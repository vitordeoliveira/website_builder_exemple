use crate::{view_controller::home, AppState};
use anyhow::Result;
use axum::{routing::get, Router};
use tower_http::services::ServeDir;

pub async fn new_app(db_connection: &str, assets_path: &str) -> Result<axum::Router> {
    let app_state = AppState::new(db_connection).await?;
    let translated_pages = Router::new().route("/:i18n", get(home));

    Ok(Router::new()
        .route("/", get(home))
        .merge(translated_pages)
        .with_state(app_state)
        .nest_service("/assets", ServeDir::new(format!("{}/assets", assets_path))))
}
