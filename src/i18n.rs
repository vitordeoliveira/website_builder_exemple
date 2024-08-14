use std::str::FromStr;

use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use strum::{Display, EnumString};

// TODO: should be a struct that impl Display
// TODO: should work in a way that the translations should be set IN the file that use it

#[derive(Clone, EnumString, Display)]
#[strum(ascii_case_insensitive)]
pub enum I18N {
    En,
    Fr,
    Es,
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
