use askama::Template;
use axum::{
    extract::{Path, State},
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
    Path(lang): Path<String>,
    State(AppState { title }): State<AppState>,
    Extension(extension_title): Extension<&'static str>,
    ExtractTitle(extract_title): ExtractTitle,
) -> impl IntoResponse {
    let title = format!(
        "from state: {} ---- from extension: {} ---- from extractor {} ---- lang:{}",
        title, extension_title, extract_title, lang
    );
    let root = RootTemplate { title };
    (StatusCode::OK, Html(root.render().unwrap()))
}
