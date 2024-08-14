// region
pub mod i18n;
pub mod view;
// flatten
// public
pub mod controller;

#[derive(Clone, Debug)]
pub struct AppState {
    pub title: &'static str,
}
