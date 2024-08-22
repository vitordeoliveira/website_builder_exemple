use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};
use strum::Display;
use thiserror::Error;

#[derive(Error, Debug, Display)]
pub enum ServerError {
    InternalServerError,

    #[error(transparent)]
    Undefined(#[from] anyhow::Error),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("{self}");

        match self {
            ServerError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html("Internal Server Error"),
            )
                .into_response(),
            ServerError::Undefined(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, Html("Undefined Error")).into_response()
            }
        }
    }
}
