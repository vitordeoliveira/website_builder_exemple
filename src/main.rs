use std::{collections::HashSet, sync::Arc};

use axum::{
    extract::Request,
    middleware::{self, Next},
    response::IntoResponse,
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

#[derive(Clone, Debug)]
struct AllowedLangs {
    langs: Arc<HashSet<String>>,
    default_lang: String,
}

async fn i18n_middleware(
    Extension(allowed_langs): Extension<AllowedLangs>,
    mut req: Request,
    next: Next,
) -> impl IntoResponse {
    let path = req.uri().path().to_string();
    let lang_segment = path.trim_start_matches('/').split('/').next().unwrap();

    let lang = if allowed_langs.langs.contains(lang_segment) {
        lang_segment.to_string()
    } else {
        allowed_langs.default_lang.clone()
    };

    req.extensions_mut().insert(lang);
    next.run(req).await
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let port = "3000";
    tracing_subscriber::fmt().init();
    tracing::info!("router initialized, now listening on port {}", port);
    let allowed_langs = vec!["en".to_string(), "fr".to_string(), "es".to_string()]
        .into_iter()
        .collect::<HashSet<_>>();

    let default_lang = "en".to_string();

    let allowed_langs = AllowedLangs {
        langs: Arc::new(allowed_langs),
        default_lang,
    };
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
        .layer(middleware::from_fn(i18n_middleware))
        .layer(Extension(allowed_langs));

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
