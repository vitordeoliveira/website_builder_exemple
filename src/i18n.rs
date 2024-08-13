use std::str::FromStr;

use axum::{extract::Request, middleware::Next, response::IntoResponse};
use strum::{Display, EnumString};

// TODO: should be a struct that impl Display
// TODO: should work in a way that the translations should be set IN the file that use it

#[derive(Clone, EnumString, Display)]
#[strum(ascii_case_insensitive)]
pub enum Lang {
    En,
    Fr,
    Es,
}

pub async fn i18n_middleware(mut req: Request, next: Next) -> Result<impl IntoResponse, ()> {
    let path = req.uri().path().to_string();
    let lang_segment = path.trim_start_matches('/').split('/').next().unwrap();

    let lang = Lang::from_str(lang_segment).unwrap_or(Lang::En);

    req.extensions_mut().insert(lang);
    Ok(next.run(req).await)
}
