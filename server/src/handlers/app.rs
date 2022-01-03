use super::*;

use std::io::ErrorKind as IoErrorKind;
use std::net::SocketAddr;

use http::header;
use http::{HeaderMap, Uri};

use header::{HeaderName, HeaderValue};
use header::{CONNECTION, TE, TRAILER, TRANSFER_ENCODING, UPGRADE};
use header::{PROXY_AUTHENTICATE, PROXY_AUTHORIZATION};

use hyper::client::{Client, HttpConnector};
use hyper::upgrade::OnUpgrade;

use tokio::io::copy_bidirectional as async_copy_bidirectional;

#[derive(Debug)]
pub struct AppExtension {
    client: Client<HttpConnector>,
    server_addr: SocketAddr,
}

impl AppExtension {
    pub fn new(server_addr: SocketAddr) -> Self {
        AppExtension {
            client: default(),
            server_addr,
        }
    }

    pub fn into_layer(self) -> AddExtensionLayer<Arc<Self>> {
        AddExtensionLayer::new(Arc::new(self))
    }
}

pub async fn app_handler(
    Extension(extension): Extension<Arc<AppExtension>>,
    mut req: Request<Body>,
) -> HandlerResult<Response<Body>> {
    let AppExtension {
        client,
        server_addr,
    } = &*extension;

    // Extract upgrade, modify client request to forward to server
    let client_upgrade = req.extensions_mut().remove::<OnUpgrade>();
    let req = {
        set_headers(&mut req);
        set_uri(&mut req, server_addr);
        req
    };

    // Forward request to server, extract upgrade
    let mut res = client
        .request(req)
        .await
        .context("failed to forward request")?;
    let server_upgrade = res.extensions_mut().remove::<OnUpgrade>();

    // If both connections are upgradeable, perform full duplex streaming
    if let (Some(client_upgrade), Some(server_upgrade)) = (client_upgrade, server_upgrade) {
        spawn(async move {
            let mut client_conn = match client_upgrade.await {
                Ok(conn) => conn,
                Err(error) => {
                    error!(?error, "client connection upgrade failed");
                    return;
                }
            };
            let mut server_conn = match server_upgrade.await {
                Ok(conn) => conn,
                Err(error) => {
                    error!(?error, "server connection upgrade failed");
                    return;
                }
            };
            match async_copy_bidirectional(&mut client_conn, &mut server_conn).await {
                Err(error) if error.kind() == IoErrorKind::NotConnected => (),
                Err(error) => error!(?error, "failed to proxy upgraded connection"),
                _ => (),
            };
        });
    }

    Ok(res)
}

static HOP_HEADERS: [HeaderName; 5] = [
    TE,
    TRAILER,
    TRANSFER_ENCODING,
    PROXY_AUTHENTICATE,
    PROXY_AUTHORIZATION,
];

fn set_headers(req: &mut Request<Body>) {
    let headers = req.headers_mut();
    HOP_HEADERS.iter().for_each(|key| {
        headers.remove(key);
    });
    let connection = headers
        .get(CONNECTION)
        .map(HeaderValue::to_str)
        .map(Result::ok)
        .flatten()
        .unwrap_or_default();
    if !connection.eq_ignore_ascii_case("upgrade") {
        headers.remove(CONNECTION);
        headers.remove(UPGRADE);
    }
}

fn set_uri(req: &mut Request<Body>, server_addr: &SocketAddr) {
    let mut uri = Uri::builder()
        .scheme("http")
        .authority(server_addr.to_string());
    if let Some(p_and_q) = req.uri().path_and_query() {
        uri = uri.path_and_query(p_and_q.clone());
    }
    *req.uri_mut() = uri.build().unwrap();
}
