use std::{fmt::Display, str::FromStr};

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

pub trait Translatable {
    fn title(&self) -> Translation;
    fn stringvalue(&self) -> Translation;
}

impl Translatable for I18N {
    fn title(&self) -> Translation {
        let text = match self {
            I18N::En => "title",
            I18N::Fr => "titre",
            I18N::Es => "título",
            I18N::De => "titel",
        };

        Translation(text.to_string())
    }

    fn stringvalue(&self) -> Translation {
        let text = match self {
            I18N::En => "Hello from myownvalue",
            I18N::Fr => "Bonjour de ma propre valeur",
            I18N::Es => "Hola de mi propio valor",
            I18N::De => "Hallo von meinem eigenen Wert",
        };

        Translation(text.to_string())
    }
}

pub struct Translation(String);

impl Display for Translation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
