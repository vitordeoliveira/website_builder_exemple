use anyhow::{Context, Result};
use axum::{routing::get, Router};

use website::{
    config,
    error::SysError,
    view_controller::{home, root},
    AppState,
};

// TODO: impl snapshot testting with asmaka
// TODO: setup tailwind
// TODO: setup HTMX
// TODO: setup rust make (db, jeager, tailwind, htmx)
// TODO: add metrics to opentelemetry (just doing tracing for now)
// TODO: setup workspaces
// TODO: write blogpost about i18n
// TODO: write blogpost about tracing with open telemetry
// TODO: write blogpost about snapshot testing with askama
//
// TODO: connect to kafka in another service (4FUN)

#[tokio::main]
async fn main() -> Result<(), SysError> {
    let host = env!("HOST");
    let port = env!("PORT");
    let db_connection_str = env!("DATABASE_URL");

    config::tracing::Tracing::setup()?;

    let app_state = AppState::new(db_connection_str).await;

    let listener = tokio::net::TcpListener::bind(format!("{host}:{port}"))
        .await
        .context("Failed to start tokio listener")
        .unwrap();

    let translated_pages = Router::new()
        .route("/:i18n", get(root))
        .route("/:i18n/home", get(home));

    let app = Router::new()
        .route("/", get(home))
        .merge(translated_pages)
        .with_state(app_state);
    // .layer(middleware::map_response(set_header));

    tracing::info!("router initialized, now listening on port {}", port);

    axum::serve(listener, app)
        .await
        .context("failed to serve server")
        .unwrap();

    Ok(())
}
