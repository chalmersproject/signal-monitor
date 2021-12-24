use super::*;

use headers::authorization::Bearer;
use headers::Authorization;

use ::graphql::http::playground_source as graphql_playground_source;
use ::graphql::http::GraphQLPlaygroundConfig;
use ::graphql::http::ALL_WEBSOCKET_PROTOCOLS as GRAPHQL_WEBSOCKET_PROTOCOLS;
use ::graphql::Data as GraphQLData;
use ::graphql::Result as GraphQLResult;
use ::graphql::Schema as GraphQLSchema;
use ::graphql::ServerError as GraphQLError;

use graphql_axum::GraphQLProtocol as GraphQLWebsocketProtocol;
use graphql_axum::GraphQLRequest;
use graphql_axum::GraphQLResponse;
use graphql_axum::GraphQLSubscription;
use graphql_axum::GraphQLWebSocket;

use crate::api::{Mutation, Query, Subscription};

#[derive(Derivative)]
#[derivative(Debug, Clone)]
pub struct ApiExtension {
    #[derivative(Debug = "ignore")]
    schema: GraphQLSchema<Query, Mutation, Subscription>,
}

impl ApiExtension {
    pub fn new(schema: GraphQLSchema<Query, Mutation, Subscription>) -> Self {
        ApiExtension { schema }
    }

    pub fn into_layer(self) -> AddExtensionLayer<Self> {
        AddExtensionLayer::new(self)
    }
}

pub async fn api_handler(
    Extension(extension): Extension<ApiExtension>,
    ws_upgrade: Option<WebSocketUpgrade>,
    ws_protocol: Option<GraphQLWebsocketProtocol>,
    req: Request<Body>,
) -> HandlerResult<Response> {
    let ApiExtension { schema } = extension;

    if req.method() == Method::GET {
        // Try handling subscription
        if let (Some(upgrade), Some(protocol)) = (ws_upgrade, ws_protocol) {
            let res = upgrade
                .protocols(GRAPHQL_WEBSOCKET_PROTOCOLS)
                .on_upgrade(move |stream| async move {
                    trace!("handling subscription");
                    GraphQLWebSocket::new(stream, schema, protocol)
                        .serve()
                        .await
                })
                .into_response();
            return Ok(res);
        }

        // Serve playground
        let config =
            GraphQLPlaygroundConfig::new("/api").subscription_endpoint("ws://localhost:3001/api");
        let source = graphql_playground_source(config);
        let res = HtmlResponse(source);
        let res = res.into_response();
        return Ok(res);
    }

    // Handle query
    let req = {
        let mut req = RequestParts::new(req);
        match GraphQLRequest::from_request(&mut req).await {
            Ok(GraphQLRequest(req)) => req,
            Err(rejection) => {
                let res = rejection.into_response();
                return Ok(res);
            }
        }
    };
    trace!(req = %format_json(&req), "handling query");
    let res = schema.execute(req).await;
    res.errors
        .iter()
        .for_each(|error| match error.message.as_str() {
            "PersistedQueryNotFound" => (),
            _ => {
                let GraphQLError {
                    message,
                    locations,
                    path,
                    ..
                } = error;
                let locations = {
                    let locations = locations
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>();
                    to_json_string(&locations).unwrap()
                };
                let path = to_json_string(path).unwrap();
                error!(
                    %locations,
                    %path,
                    "{}", message,
                );
            }
        });
    let res = GraphQLResponse::from(res).into_response();
    Ok(res)
}
