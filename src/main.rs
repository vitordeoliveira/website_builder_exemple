use axum::{routing::get, Router};

use website::view::{home, root};

#[tokio::main]
async fn main() -> Result<(), ()> {
    tracing_subscriber::fmt().init();
    let port = "3000";

    tracing::info!("router initialized, now listening on port {}", port);

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{port}"))
        .await
        .unwrap();

    let app = Router::new()
        .route("/", get(root))
        .route("/home", get(home));

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
