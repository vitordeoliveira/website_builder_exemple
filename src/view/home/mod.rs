use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

use crate::i18n::I18N;

#[derive(Template)]
#[template(path = "home/home.html")]
pub struct HomeTemplate {
    title: &'static str,
    stringvalue: &'static str,
    vec_strings: Vec<&'static str>,
    lang: I18N,
}

pub async fn home(lang: I18N) -> impl IntoResponse {
    let home = HomeTemplate {
        title: "title",
        stringvalue: "Hello from myownvalue",
        vec_strings: vec!["Rust", "is", "the", "best", "language"],
        lang,
    };
    (StatusCode::OK, Html(home.render().unwrap()))
}
