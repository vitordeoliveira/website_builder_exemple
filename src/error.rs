use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Internal Server Error {0}")]
    InternalServer(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html("INTERNAL_SERVER_ERROR".to_string()),
        )
            .into_response()
    }
}
