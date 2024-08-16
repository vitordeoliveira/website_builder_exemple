use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum SysError {}

impl IntoResponse for SysError {
    fn into_response(self) -> axum::response::Response {
        // error!("{self}");
        // match self {
        //     SysError::I18NError(path) => {
        //         let mut segments = path
        //             .trim_start_matches('/')
        //             .splitn(2, '/')
        //             .collect::<Vec<&str>>();
        //         segments[0] = "en";
        //         let new_path = format!("/{}", segments.join("/"));
        //         Redirect::temporary(&new_path).into_response()
        //     }
        // }

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html("internal server error"),
        )
            .into_response()
    }
}

pub enum ClientError {}
