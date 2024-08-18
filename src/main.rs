use anyhow::Result;
use axum::{routing::get, Router};

use opentelemetry::{
    trace::{TraceError, TracerProvider as _},
    KeyValue,
};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::trace as sdktrace;
use opentelemetry_sdk::{runtime, Resource};
use opentelemetry_semantic_conventions::resource::SERVICE_NAME;
use tracing::instrument;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use website::view::{home, root};
// TODO: setup tracing with open telemetry
// TODO: setup config singleton
// TODO: impl adatper for database connections
// TODO: impl controller properly
// TODO: setup error (ClientError)
// TODO: impl model
// TODO: impl snapshot testting with asmaka
// TODO: setup workspaces
// TODO: write blogpost about i18n
// TODO: write blogpost about tracing with open telemetry
// TODO: write blogpost about snapshot testing with askama

fn init_tracer_provider() -> Result<opentelemetry_sdk::trace::TracerProvider, TraceError> {
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://localhost:4317"),
        )
        .with_trace_config(
            sdktrace::Config::default().with_resource(Resource::new(vec![KeyValue::new(
                SERVICE_NAME,
                "website-builder",
            )])),
        )
        .install_batch(runtime::Tokio)
}

#[tokio::main]
#[instrument]
async fn main() -> Result<()> {
    let port = "3000";
    // Create a new OpenTelemetry trace pipeline that prints to stdout
    // let provider = TracerProvider::builder()
    //     .with_simple_exporter(stdout::SpanExporter::default())
    //     .build();

    let tracer = init_tracer_provider().unwrap().tracer("my_app");

    // Create a tracing layer with the configured tracer
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(telemetry)
        .init();

    tracing::trace!("router initialized, now listening on port {}", port);
    // tracing::error!("router initialized, now listening on port {}", port);

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{port}"))
        .await
        .unwrap();

    let translated_pages = Router::new()
        .route("/:i18n", get(root))
        .route("/:i18n/home", get(home));

    let app = Router::new().route("/", get(home)).merge(translated_pages);

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
