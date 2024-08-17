mod translation;
// Flatten
pub use translation::*;

use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use std::str::FromStr;
use strum::{Display, EnumString};

use crate::error::SysError;

#[derive(Clone, EnumString, Display, PartialEq, Debug, Default)]
#[strum(ascii_case_insensitive)]
pub enum I18N {
    #[default]
    En,
    Fr,
    Es,
    De,
}

impl I18N {
    fn get_path_language(path: String) -> Self {
        let lang_segment = path
            .trim_start_matches('/')
            .split('/')
            .next()
            .unwrap_or("invalid");

        I18N::from_str(lang_segment).unwrap_or_default()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for I18N
where
    S: Send + Sync,
{
    type Rejection = SysError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let path = parts.uri.to_string();
        Ok(Self::get_path_language(path))
    }
}

#[cfg(test)]
mod tests {
    use crate::i18n::I18N;

    #[test]
    fn should_return_default_en_on_path() {
        assert_eq!(I18N::get_path_language("fakepath".to_string()), I18N::En)
    }

    #[test]
    fn should_return_german_on_path() {
        assert_eq!(I18N::get_path_language("/de".to_string()), I18N::De)
    }

    #[test]
    fn should_return_french_on_path() {
        assert_eq!(I18N::get_path_language("/fr".to_string()), I18N::Fr)
    }

    #[test]
    fn should_return_spanish_on_path() {
        assert_eq!(I18N::get_path_language("/es".to_string()), I18N::Es)
    }
}
