use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

use crate::i18n::{Translatable, Translation, I18N};

#[derive(Template)]
#[template(path = "home/home.html")]
pub struct HomeTemplate {
    title: Translation,
    stringvalue: Translation,
    vec_strings: Vec<&'static str>,
    lang: I18N,
}

pub async fn home(lang: I18N) -> impl IntoResponse {
    let home = HomeTemplate {
        title: lang.title(),
        stringvalue: lang.stringvalue(),
        vec_strings: vec!["Rust", "is", "the", "best", "language"],
        lang: lang.clone(),
    };
    (StatusCode::OK, Html(home.render().unwrap()))
}
