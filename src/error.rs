use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum SysError {
    #[error("I18N Error: {0}")]
    I18NError(String),
}

impl IntoResponse for SysError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html("INTERNAL_SERVER_ERROR".to_string()),
        )
            .into_response()
    }
}

pub enum ClientError {}
