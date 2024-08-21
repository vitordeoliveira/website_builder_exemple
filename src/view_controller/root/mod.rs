use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};
use tracing::instrument;

use crate::i18n::I18N;

#[derive(Template)]
#[template(path = "root/root.html")]
pub struct RootTemplate {
    title: String,
}

#[instrument]
pub async fn root(i18n: I18N) -> impl IntoResponse {
    let title = format!("lang_from_extractor:{i18n}");
    let root = RootTemplate { title };
    (StatusCode::OK, Html(root.render().unwrap()))
}
