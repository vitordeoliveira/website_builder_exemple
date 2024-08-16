mod translation;
// Flatten
pub use translation::*;

use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use std::str::FromStr;
use strum::{Display, EnumString};

use crate::error::SysError;

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
    type Rejection = SysError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let path = parts.uri.to_string();
        let lang_segment = path
            .trim_start_matches('/')
            .split('/')
            .next()
            .unwrap_or("en");

        let lang = I18N::from_str(lang_segment).map_err(|e| SysError::I18NError(e.to_string()))?;

        Ok(lang)
    }
}

#[cfg(test)]
#[test]
fn should_return_i18n_error() {
    let lang_segment = "invalid_lang";
    let result = I18N::from_str(lang_segment).map_err(|e| SysError::I18NError(e.to_string()));

    match result {
        Err(err_msg) => {
            assert!(err_msg
                .to_string()
                .contains("I18N Error: Matching variant not found"));
        }
        Ok(_) => {
            panic!("Expected SysError::I18NError, got something else.");
        }
    }
}
