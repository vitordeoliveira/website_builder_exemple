use website::view_controller::RootTemplate;
mod common;
// use ::axum_test::TestServerConfig;

#[tokio::test]
async fn root_en() {
    let server = common::setup().await;
    let response = server.get("/").await;
    response.assert_text_contains("lang_from_extractor:En");
    let response = server.get("/en").await;
    response.assert_text_contains("lang_from_extractor:En");
}

#[tokio::test]
async fn root_es() {
    let server = common::setup().await;
    let response = server.get("/es").await;
    response.assert_text_contains("lang_from_extractor:Es");
}

#[tokio::test]
async fn root_de() {
    let server = common::setup().await;
    let response = server.get("/de").await;
    response.assert_text_contains("lang_from_extractor:De");
}

#[tokio::test]
async fn root_fr() {
    let server = common::setup().await;
    let response = server.get("/fr").await;
    response.assert_text_contains("lang_from_extractor:Fr");
}

#[tokio::test]
async fn root_not_language() {
    let server = common::setup().await;
    let response = server.get("/invalid").await;
    response.assert_text_contains("lang_from_extractor:En");
}

#[tokio::test]
async fn root_match() {
    let server = common::setup().await;
    let response = server.get("/invalid").await;

    let expect = RootTemplate {
        title: "lang_from_extractor:En".to_string(),
    };
    assert_eq!(response.text(), expect.to_string())
}
