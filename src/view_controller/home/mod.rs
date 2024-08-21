use crate::{
    i18n::{Translatable, Translation, I18N},
    model::exemple_service,
    state::AppState,
};
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
pub async fn home(lang: I18N, State(AppState { pg_pool }): State<AppState>) -> impl IntoResponse {
    let result = exemple_service(&pg_pool).await;

    let home = HomeTemplate {
        title: lang.title(),
        stringvalue: Translation(result),
        vec_strings: vec!["Rust", "is", "the", "best", "language"],
        lang: lang.clone(),
    };

    (StatusCode::OK, Html(home.render().unwrap()))
}
