use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
};

use crate::AppState;

#[derive(Template)]
#[template(path = "root/root.html")]
pub struct RootTemplate {
    title: &'static str,
}

pub async fn root(State(state): State<AppState>) -> impl IntoResponse {
    let root = RootTemplate { title: state.title };
    (StatusCode::OK, Html(root.render().unwrap()))
}
