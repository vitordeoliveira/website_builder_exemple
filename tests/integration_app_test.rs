use website::{i18n::I18N, view_controller::HomeTemplate};

mod common;
// use ::axum_test::TestServerConfig;

// #[tokio::test]
// async fn root_en() {
//     let server = common::setup().await;
//     let response = server.get("/").await;
//     response.assert_text_contains("lang_from_extractor:En");
//     let response = server.get("/en").await;
//     response.assert_text_contains("lang_from_extractor:En");
// }
//
// #[tokio::test]
// async fn root_es() {
//     let server = common::setup().await;
//     let response = server.get("/es").await;
//     response.assert_text_contains("lang_from_extractor:Es");
// }
//
// #[tokio::test]
// async fn root_de() {
//     let server = common::setup().await;
//     let response = server.get("/de").await;
//     response.assert_text_contains("lang_from_extractor:De");
// }
//
// #[tokio::test]
// async fn root_fr() {
//     let server = common::setup().await;
//     let response = server.get("/fr").await;
//     response.assert_text_contains("lang_from_extractor:Fr");
// }
//
// #[tokio::test]
// async fn root_not_language() {
//     let server = common::setup().await;
//     let response = server.get("/invalid").await;
//     response.assert_text_contains("lang_from_extractor:En");
// }

#[tokio::test]
async fn root_match() {
    let server = common::setup().await;
    let response = server.get("/invalid").await;

    let expect = HomeTemplate {
        title: website::i18n::Translation("title".to_string()),
        stringvalue: website::i18n::Translation("hello world from pg".to_string()),
        vec_strings: vec!["Rust", "is", "the", "best", "language"],
        lang: I18N::En,
    };

    assert_eq!(response.text(), expect.to_string())
}
