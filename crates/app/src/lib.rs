pub(crate) mod components;
pub(crate) mod global_state;
mod views;

use dioxus::prelude::*;
use views::{Home, TextFileViewer};

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/text_file_viewer/:path")]
    TextFileViewer { path: String },
}

#[component]
pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
