use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    Extension,
};

use crate::AppState;

#[derive(Template)]
#[template(path = "root/root.html")]
pub struct RootTemplate {
    title: String,
}

pub async fn root(
    State(state): State<AppState>,
    ext: Extension<&'static str>,
) -> impl IntoResponse {
    let title = format!("from state: {} ---- from extension: {}", state.title, ext.0);
    let root = RootTemplate { title };
    (StatusCode::OK, Html(root.render().unwrap()))
}
