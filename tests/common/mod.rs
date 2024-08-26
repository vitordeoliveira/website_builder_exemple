use axum_test::TestServer;
use website::app::new_app;

pub async fn setup() -> TestServer {
    let db_connection = env!("DATABASE_URL");
    let assets_path = env!("CARGO_MANIFEST_DIR");
    let app = new_app(db_connection, assets_path).await;
    TestServer::new(app).unwrap()
}
