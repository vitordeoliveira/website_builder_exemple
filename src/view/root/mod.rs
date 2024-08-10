use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    Extension,
};

use crate::{AppState, ExtractTitle};

#[derive(Template)]
#[template(path = "root/root.html")]
pub struct RootTemplate {
    title: String,
}

pub async fn root(
    State(AppState { title }): State<AppState>,
    Extension(extension_title): Extension<&'static str>,
    ExtractTitle(extract_title): ExtractTitle,
) -> impl IntoResponse {
    let title = format!(
        "from state: {} ---- from extension: {} ---- from extractor {}",
        title, extension_title, extract_title
    );
    let root = RootTemplate { title };
    (StatusCode::OK, Html(root.render().unwrap()))
}
