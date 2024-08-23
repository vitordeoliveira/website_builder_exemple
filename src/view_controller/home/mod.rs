use crate::{
    error::ServerError,
    i18n::{Translatable, Translation, I18N},
    service::exemple_service,
    state::AppState,
};
use anyhow::Result;
use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use tracing::instrument;

#[derive(Template)]
#[template(path = "home/home.html")]
pub struct HomeTemplate {
    title: Translation,
    stringvalue: Translation,
    vec_strings: Vec<&'static str>,
    lang: I18N,
}

#[instrument]
pub async fn home(
    lang: I18N,
    State(AppState { pg_pool }): State<AppState>,
) -> Result<impl IntoResponse, ServerError> {
    let result = exemple_service(&pg_pool).await;

    let home = HomeTemplate {
        title: lang.title(),
        stringvalue: Translation(result),
        vec_strings: vec!["Rust", "is", "the", "best", "language"],
        lang: lang.clone(),
    };

    Ok((StatusCode::OK, Html(home.render().unwrap())))
}
