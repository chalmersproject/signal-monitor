use super::*;

use axum::body::boxed as boxed_body;
use axum::body::Body;
use axum::extract::ws::WebSocketUpgrade;
use axum::extract::Json as JsonExtractor;
use axum::extract::TypedHeader as HeaderExtractor;
use axum::extract::{Extension, FromRequest, RequestParts};
use axum::response::Html as HtmlResponse;
use axum::response::Json as JsonResponse;
use axum::response::{IntoResponse, Response};

use axum::AddExtensionLayer;

use http::{Method, Request, StatusCode};

mod api;
mod app;

pub use self::api::*;
pub use self::app::*;

#[derive(Debug, From)]
pub struct HandlerError(Error);

impl IntoResponse for HandlerError {
    fn into_response(self) -> Response {
        let HandlerError(error) = self;
        let message = format!("{:#}", &error);
        let errors = json!({ "errors": [{ "message": message }] });
        JsonResponse(errors).into_response()
    }
}

pub type HandlerResult<T> = Result<T, HandlerError>;
