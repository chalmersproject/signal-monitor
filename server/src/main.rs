use std::future::ready;
use std::io::ErrorKind as IoErrorKind;
use std::net::SocketAddr;

use dotenv::from_filename as load_dotenv;
use dotenv::Error as DotenvError;

use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use axum::routing::Router;
use axum::routing::{any, get};
use axum::Server;

use anyhow::Context as AnyhowContext;
use anyhow::Result;

use tracing::{debug, info};
use tracing_subscriber::fmt::layer as fmt_tracing_layer;
use tracing_subscriber::layer::SubscriberExt as TracingSubscriberLayerExt;
use tracing_subscriber::registry as tracing_registry;
use tracing_subscriber::util::SubscriberInitExt as TracingSubscriberInitExt;
use tracing_subscriber::EnvFilter as TracingEnvFilter;

use sentry::init as init_sentry_client;
use sentry::ClientInitGuard as SentryGuard;
use sentry::ClientOptions as SentryOptions;
use sentry::IntoDsn as IntoSentryDsn;
use sentry_tracing::layer as sentry_tracing_layer;

use server::app::Server as AppServer;
use server::config::Environment;
use server::config::{env_opt, set_env};
use server::handlers::app_handler;
use server::handlers::AppExtension;

use http::StatusCode;
use portpicker::pick_unused_port;
use tokio::runtime::Runtime;

const PROJECT_NAME: &str = "signal-monitor";
const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<()> {
    let env = init_env()?;

    init_logging()?;
    let _sentry_guard = init_sentry_opt(env)?;

    // Configure app server
    let app_server_addr = {
        let host = [127, 0, 0, 1];
        let port = pick_unused_port().expect("all ports are unavailable");
        SocketAddr::from((host, port))
    };
    let app_server = AppServer::new(env);

    // Build app route
    let app_service = any(app_handler);
    let app_extension = AppExtension::new(app_server_addr);

    // Init routes
    let routes = Router::new()
        .route("/_health", get(|| ready((StatusCode::OK, "OK"))))
        .route("/", app_service.clone())
        .route("/_next/*path", app_service);

    // Build layers
    let layers = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(app_extension.into_layer());

    // Configure server
    let server_addr: SocketAddr = {
        let host = {
            let host = env_opt("HOST")?;
            host.unwrap_or_else(|| "0.0.0.0".to_owned())
        };
        let port = {
            let port = env_opt("PORT")?;
            port.unwrap_or_else(|| "3000".to_owned())
        };
        format!("{}:{}", host, port)
            .parse()
            .context("failed to parse server address")?
    };

    // Start async runtime
    let runtime = Runtime::new().context("failed to initialize runtime")?;
    runtime.block_on(async move {
        // Run app server
        info!("starting app");
        app_server
            .serve(&app_server_addr)
            .await
            .context("failed to initialize app")?;

        // Run server
        info!("listening on http://{}", &server_addr);
        Server::try_bind(&server_addr)
            .context("failed to bind to address")?
            .serve(routes.layer(layers).into_make_service())
            .await?;

        Ok(())
    })
}

fn init_env() -> Result<Environment> {
    // Read environment name
    let env = Environment::from_env().context("failed to load environment name")?;

    // Load values from .env, .env.local
    for file in [".env", ".env.local"] {
        load_dotenv(file)
            .map(|_| ())
            .or_else(|error| {
                if let DotenvError::Io(error) = &error {
                    if error.kind() == IoErrorKind::NotFound {
                        return Ok(());
                    }
                }
                Err(error)
            })
            .with_context(|| format!("failed to load environment variables from {}", file))?;
    }

    // Apply unprefixed variables
    {
        let backtrace =
            env_opt("BACKTRACE").context("failed read environment variable BACKTRACE")?;
        let backtrace = backtrace.unwrap_or_else(|| "1".to_owned());
        set_env("RUST_BACKTRACE", backtrace);
    }
    {
        let log = env_opt("LOG").context("failed to read environment variable LOG")?;
        let log = log.unwrap_or_else(|| {
            use Environment::*;
            match env {
                Development => format!("info,{}=debug", PACKAGE_NAME),
                Production => "info".to_owned(),
            }
        });
        set_env("RUST_LOG", log);
    }

    Ok(env)
}

pub fn default<T: Default>() -> T {
    T::default()
}

fn init_logging() -> Result<()> {
    debug!("initializing tracer");
    tracing_registry()
        .with(TracingEnvFilter::from_default_env())
        .with(fmt_tracing_layer())
        .with(sentry_tracing_layer())
        .try_init()
        .context("failed to initialize tracer")?;
    Ok(())
}

fn init_sentry(dsn: &str, env: Environment) -> Result<SentryGuard> {
    info!("initializing Sentry");
    let dsn = dsn.into_dsn().context("failed to parse Sentry DSN")?;
    let release = format!("{}-{}@{}", PROJECT_NAME, PACKAGE_NAME, PACKAGE_VERSION);
    let env = env.to_string();
    let options = SentryOptions {
        dsn,
        release: Some(release.into()),
        environment: Some(env.into()),
        ..default()
    };
    let guard = init_sentry_client(options);
    Ok(guard)
}

fn init_sentry_opt(env: Environment) -> Result<Option<SentryGuard>> {
    let dsn = env_opt("SENTRY_DSN").context("failed to read environment variable SENTRY_DSN")?;
    let dsn = match dsn {
        Some(dsn) => dsn,
        None => {
            info!("skipping Sentry initialization (missing DSN)");
            return Ok(None);
        }
    };
    init_sentry(&dsn, env).map(Some)
}
