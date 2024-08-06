use axum::{routing::get, Router};

use website::view::{home, root};

// TODO: impl a state
// TODO: impl a extension
// TODO: impl a extractor
// TODO: impl a middleware

#[tokio::main]
async fn main() -> Result<(), ()> {
    let port = "3000";
    tracing_subscriber::fmt().init();
    tracing::info!("router initialized, now listening on port {}", port);

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{port}"))
        .await
        .unwrap();

    let pages = Router::new()
        .route("/", get(root))
        .route("/home", get(home));

    let app = Router::new().nest(":i18n", pages);

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
