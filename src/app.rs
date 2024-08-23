use axum::{routing::get, Router};

use crate::{
    view_controller::{home, root},
    AppState,
};

pub async fn new_app(db_connection_str: &str) -> axum::Router {
    let app_state = AppState::new(db_connection_str).await;
    let translated_pages = Router::new()
        .route("/:i18n", get(root))
        .route("/:i18n/home", get(home));

    Router::new()
        .route("/", get(root))
        .merge(translated_pages)
        .with_state(app_state)
}
