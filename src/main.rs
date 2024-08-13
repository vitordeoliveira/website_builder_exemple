use axum::{middleware, routing::get, Extension, Router};

use website::{
    i18n::i18n_middleware,
    view::{home, root},
    AppState,
};

// TODO: impl i18n in a way that if not set on URI redirect to default
// TODO: setup error
// TODO: setup tracing with open telemetry
// TODO: setup config singleton
// TODO: impl adatper for database connections
// TODO: impl controller properly
// TODO: impl model

#[tokio::main]
async fn main() -> Result<(), ()> {
    let port = "3000";
    tracing_subscriber::fmt().init();
    tracing::info!("router initialized, now listening on port {}", port);

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{port}"))
        .await
        .unwrap();

    let state = AppState { title: "website" };
    let pages = Router::new()
        .route("/", get(root))
        .route("/home", get(home))
        .with_state(state)
        .layer(Extension("hello from extension"));

    let app = Router::new()
        .nest("/:i18n", pages)
        .layer(middleware::from_fn(i18n_middleware));

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
