use std::{collections::HashSet, fmt::Display, str::FromStr, sync::Arc};

use axum::{extract::Request, middleware::Next, response::IntoResponse, Extension};
use strum::{EnumIter, IntoEnumIterator};

// TODO: should be a struct that impl Display
// TODO: should work in a way that the translations should be set IN the file that use it

#[derive(Clone, Debug)]
pub struct AllowedLangs {
    pub langs: Arc<HashSet<String>>,
    pub default_lang: Lang,
}

pub async fn i18n_middleware(
    Extension(allowed_langs): Extension<AllowedLangs>,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, ()> {
    let test = Lang::as_vec();
    println!("{test:?}");

    let path = req.uri().path().to_string();
    let lang_segment = path.trim_start_matches('/').split('/').next().unwrap();

    let lang = Lang::from_str(lang_segment)?;

    let lang = if allowed_langs.langs.contains(lang.as_str()) {
        lang
    } else {
        allowed_langs.default_lang
    };

    req.extensions_mut().insert(lang);
    Ok(next.run(req).await)
}

#[derive(Clone, Debug, EnumIter, PartialEq)]
pub enum Lang {
    En,
    Fr,
    Es,
}

impl Lang {
    pub fn as_str(&self) -> &str {
        match self {
            Lang::En => "en",
            Lang::Fr => "fr",
            Lang::Es => "es",
        }
    }

    pub fn as_vec() -> Vec<String> {
        Lang::iter().map(|e| e.to_string()).collect::<Vec<String>>()
    }
}

impl FromStr for Lang {
    type Err = ();

    fn from_str(lang: &str) -> Result<Self, Self::Err> {
        Ok(match lang {
            "en" => Lang::En,
            "fr" => Lang::Fr,
            "es" => Lang::Es,
            _ => Lang::En,
        })
    }
}

impl Display for Lang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
