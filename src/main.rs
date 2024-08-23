use anyhow::{Context, Result};

use website::{app, config, error::ServerError};

// TODO: setup rust make (db, jeager, tailwind, htmx)
// TODO: setup unit test
// TODO: impl snapshot testting with asmaka
// TODO: add metrics to opentelemetry (just doing tracing for now)

// Extras
// TODO: setup workspaces
// TODO: write blogpost about i18n
// TODO: write blogpost about tracing with open telemetry
// TODO: write blogpost about snapshot testing with askama
// TODO: connect to kafka in another service (4FUN)

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    let host = env!("HOST");
    let port = env!("PORT");
    let db_connection = env!("DATABASE_URL");
    let assets_path = env!("CARGO_MANIFEST_DIR");

    config::tracing::Tracing::setup()?;

    let listener = tokio::net::TcpListener::bind(format!("{host}:{port}"))
        .await
        .context("Failed to start tokio listener")
        .unwrap();

    let app = app::new_app(db_connection, assets_path).await;

    tracing::info!("router initialized, now listening on port {}", port);

    axum::serve(listener, app)
        .await
        .context("failed to serve server")
        .unwrap();

    Ok(())
}
