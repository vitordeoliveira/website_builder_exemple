use std::str::FromStr;

use axum::{
    extract::Request,
    http::Uri,
    middleware::{self, Next},
    response::{IntoResponse, Redirect},
    routing::get,
    Extension, Router,
};

use website::{
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

async fn i18n_middleware(mut request: Request, next: Next) -> impl IntoResponse {
    // *request.uri_mut() = Uri::from_str("/en/home").unwrap();
    let response = next.run(request).await;
    // // do something with `response`...o
    // dbg!(response);
    response
    // Redirect::temporary("/en/home")
}

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
        .with_state(state)
        .layer(Extension("hello from extension"))
        .layer(middleware::from_fn(my_middleware))
        .route("/home", get(home));

    let app = Router::new().nest("/:i18n", pages);

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
