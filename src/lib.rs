// region
// flatten
// public
pub mod controller;
pub mod i18n;
pub mod view;

#[derive(Clone, Debug)]
pub struct AppState {
    pub title: &'static str,
}
