use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

#[derive(Template)]
#[template(path = "root/root.html")]
pub struct RootTemplate {
    title: &'static str,
}

pub async fn root() -> impl IntoResponse {
    let root = RootTemplate { title: "title" };
    (StatusCode::OK, Html(root.render().unwrap()))
}
