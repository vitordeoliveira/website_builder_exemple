use crate::i18n::{Translatable, Translation, I18N};
use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use sqlx::PgPool;
use sqlx::Row;
use tracing::instrument;
use tracing::Instrument;

#[derive(Template)]
#[template(path = "home/home.html")]
pub struct HomeTemplate {
    title: Translation,
    stringvalue: Translation,
    vec_strings: Vec<&'static str>,
    lang: I18N,
}

#[instrument]
pub async fn home(lang: I18N, State(pool): State<PgPool>) -> impl IntoResponse {
    let result = sqlx::query(r#"select 'hello world from pg' as "message!""#)
        .fetch_one(&pool)
        .instrument(tracing::info_span!("db_query"))
        .await
        .unwrap();

    let home = HomeTemplate {
        title: lang.title(),
        stringvalue: Translation(result.get("message!")),
        vec_strings: vec!["Rust", "is", "the", "best", "language"],
        lang: lang.clone(),
    };

    (StatusCode::OK, Html(home.render().unwrap()))
}
