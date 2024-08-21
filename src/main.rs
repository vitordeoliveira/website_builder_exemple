use anyhow::Result;
use axum::{routing::get, Router};

use website::{
    config,
    view::{home, root},
};
use sqlx::postgres::PgPoolOptions;
use website::{config, view_controller::home, view_controller::root};

// TODO: setup config singleton
// TODO: impl adatper for database connections
// TODO: impl controller properly
// TODO: setup error (ClientError)
// TODO: impl model
// TODO: impl snapshot testting with asmaka
// TODO: add metrics to opentelemetry (just doing tracing for now)
// TODO: setup workspaces
// TODO: write blogpost about i18n
// TODO: write blogpost about tracing with open telemetry
// TODO: write blogpost about snapshot testing with askama

#[tokio::main]
async fn main() -> Result<()> {
    let host = env!("HOST");
    let port = env!("PORT");

    config::tracing::Tracing::setup()?;

    let listener = tokio::net::TcpListener::bind(format!("{host}:{port}"))
        .await
        .unwrap();

    let translated_pages = Router::new()
        .route("/:i18n", get(root))
        .route("/:i18n/home", get(home));

    let app = Router::new().route("/", get(home)).merge(translated_pages);

    axum::serve(listener, app).await.unwrap();

    tracing::info!("router initialized, now listening on port {}", port);

    Ok(())
}
