use crate::{
    view_controller::{home, root},
    AppState,
};
use axum::{routing::get, Router};
use tower_http::services::ServeDir;

pub async fn new_app(db_connection: &str, assets_path: &str) -> axum::Router {
    let app_state = AppState::new(db_connection).await;
    let translated_pages = Router::new()
        .route("/:i18n", get(root))
        .route("/:i18n/home", get(home));

    Router::new()
        .route("/", get(root))
        .merge(translated_pages)
        .with_state(app_state)
        .nest_service("/assets", ServeDir::new(format!("{}/assets", assets_path)))
}
