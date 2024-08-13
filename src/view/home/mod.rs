use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    Extension,
};

use crate::i18n::Lang;

#[derive(Template)]
#[template(path = "home/home.html")]
pub struct HomeTemplate {
    title: &'static str,
    stringvalue: &'static str,
    vec_strings: Vec<&'static str>,
    lang: Lang,
}

pub async fn home(Extension(lang_ext): Extension<Lang>) -> impl IntoResponse {
    let home = HomeTemplate {
        title: "title",
        stringvalue: "Hello from myownvalue",
        vec_strings: vec!["Rust", "is", "the", "best", "language"],
        lang: lang_ext,
    };
    (StatusCode::OK, Html(home.render().unwrap()))
}
