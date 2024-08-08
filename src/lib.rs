// region
pub mod view;
// flatten
// public
pub mod controller;

#[derive(Clone)]
pub struct AppState {
    pub title: &'static str,
}
