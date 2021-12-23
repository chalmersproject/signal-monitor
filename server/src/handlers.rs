use super::*;

use axum::body::Body;
use axum::extract::Extension;
use axum::response::{IntoResponse, Response};
use axum::{AddExtensionLayer, Json};

use http::Request;

mod app;

pub use app::*;

#[derive(Debug, From)]
pub struct HandlerError(Error);

impl IntoResponse for HandlerError {
    fn into_response(self) -> Response {
        let HandlerError(error) = self;
        let message = format!("{:#}", &error);
        let errors = json!({ "errors": [{ "message": message }] });
        Json(errors).into_response()
    }
}

pub type HandlerResult<T> = Result<T, HandlerError>;
