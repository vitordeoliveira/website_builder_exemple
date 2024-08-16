use anyhow::Result;
use axum::{routing::get, Router};

use website::{
    view::{home, root},
    AppState,
};

// TODO: setup error
// TODO: setup tracing with open telemetry
// TODO: setup config singleton
// TODO: impl adatper for database connections
// TODO: impl controller properly
// TODO: impl model
// TODO: impl snapshot testting with asmaka
// TODO: setup workspaces
// TODO: write blogpost about i18n
// TODO: write blogpost about tracing with open telemetry
// TODO: write blogpost about snapshot testing with askama

#[tokio::main]
async fn main() -> Result<()> {
    let port = "3000";
    tracing_subscriber::fmt().init();
    tracing::info!("router initialized, now listening on port {}", port);

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{port}"))
        .await
        .unwrap();

    let state = AppState { title: "website" };
    let pages = Router::new()
        .route("/:i18n", get(root))
        .route("/:i18n/home", get(home));

    let app = Router::new()
        .route("/", get(home))
        .merge(pages)
        .with_state(state);

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
