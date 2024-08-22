use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};
use strum::Display;
use thiserror::Error;

#[derive(Error, Debug, Display)]
pub enum SysError {
    InternalServerError,

    #[error(transparent)]
    Undefined(#[from] anyhow::Error),
}

impl IntoResponse for SysError {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("{self}");

        match self {
            SysError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html("Internal Server Error"),
            )
                .into_response(),
            SysError::Undefined(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, Html("Undefined Error")).into_response()
            }
        }
    }
}
