use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

// region
pub mod i18n;
pub mod view;
// flatten
// public
pub mod controller;

#[derive(Clone)]
pub struct AppState {
    pub title: &'static str,
}

pub struct ExtractTitle(String);

#[async_trait]
impl<S> FromRequestParts<S> for ExtractTitle
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(_parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        println!("->> {:<12} - ExtractUserAgent", "EXTRACTOR");

        Ok(ExtractTitle("extractor-title".to_string()))

        // if let Some(user_agent) = parts.headers.get(USER_AGENT) {
        //     Ok(ExtractTitle("extractor-title".to_string()))
        // } else {
        //     Err((StatusCode::BAD_REQUEST, "`User-Agent` header is missing"))
        // }
    }
}
