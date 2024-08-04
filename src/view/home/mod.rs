use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

#[derive(Template)]
#[template(path = "home/home.html")]
pub struct HomeTemplate {
    title: &'static str,
    stringvalue: &'static str,
    vec_strings: Vec<&'static str>,
}

pub async fn home() -> impl IntoResponse {
    let home = HomeTemplate {
        title: "title",
        stringvalue: "Hello from myownvalue",
        vec_strings: vec!["Rust", "is", "the", "best", "language"],
    };
    (StatusCode::OK, Html(home.render().unwrap()))
}
