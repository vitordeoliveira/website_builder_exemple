use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
};

use crate::{i18n::I18N, AppState, ExtractTitle};

#[derive(Template)]
#[template(path = "root/root.html")]
pub struct RootTemplate {
    title: String,
}

pub async fn root(
    i18n: I18N,
    State(AppState { title }): State<AppState>,
    ExtractTitle(extract_title): ExtractTitle,
) -> impl IntoResponse {
    let title = format!(
        "from state: {} ---- from extractor {} ---- lang_from_extractor:{i18n}",
        title, extract_title
    );
    let root = RootTemplate { title };
    (StatusCode::OK, Html(root.render().unwrap()))
}
