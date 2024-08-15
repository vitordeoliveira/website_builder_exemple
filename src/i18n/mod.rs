mod translation;
// Flatten
pub use translation::*;

use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use std::str::FromStr;
use strum::{Display, EnumString};

#[derive(Clone, EnumString, Display)]
#[strum(ascii_case_insensitive)]
pub enum I18N {
    En,
    Fr,
    Es,
    De,
}

#[async_trait]
impl<S> FromRequestParts<S> for I18N
where
    S: Send + Sync,
{
    type Rejection = ();

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let path = parts.uri.to_string();
        let lang_segment = path.trim_start_matches('/').split('/').next().unwrap();

        let lang = I18N::from_str(lang_segment).unwrap_or(I18N::En);
        Ok(lang)
    }
}
