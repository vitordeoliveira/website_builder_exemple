use axum_test::TestServer;
use website::app::new_app;

// use ::axum_test::TestServerConfig;

#[tokio::test]
async fn root_en() {
    let db_connection = env!("DATABASE_URL");
    let assets_path = env!("CARGO_MANIFEST_DIR");
    let app = new_app(db_connection, assets_path).await;
    let server = TestServer::new(app).unwrap();
    let response = server.get("/").await;
    response.assert_text_contains("lang_from_extractor:En");
    let response = server.get("/en").await;
    response.assert_text_contains("lang_from_extractor:En");
}

#[tokio::test]
async fn root_es() {
    let db_connection = env!("DATABASE_URL");
    let assets_path = env!("CARGO_MANIFEST_DIR");
    let app = new_app(db_connection, assets_path).await;
    let server = TestServer::new(app).unwrap();
    let response = server.get("/es").await;
    response.assert_text_contains("lang_from_extractor:Es");
}

#[tokio::test]
async fn root_de() {
    let db_connection = env!("DATABASE_URL");
    let assets_path = env!("CARGO_MANIFEST_DIR");
    let app = new_app(db_connection, assets_path).await;
    let server = TestServer::new(app).unwrap();
    let response = server.get("/de").await;
    response.assert_text_contains("lang_from_extractor:De");
}

#[tokio::test]
async fn root_fr() {
    let db_connection = env!("DATABASE_URL");
    let assets_path = env!("CARGO_MANIFEST_DIR");
    let app = new_app(db_connection, assets_path).await;
    let server = TestServer::new(app).unwrap();
    let response = server.get("/fr").await;
    response.assert_text_contains("lang_from_extractor:Fr");
}

#[tokio::test]
async fn root_not_language() {
    let db_connection = env!("DATABASE_URL");
    let assets_path = env!("CARGO_MANIFEST_DIR");
    let app = new_app(db_connection, assets_path).await;
    let server = TestServer::new(app).unwrap();
    let response = server.get("/invalid").await;
    response.assert_text_contains("lang_from_extractor:En");
}
