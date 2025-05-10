pub(crate) mod components;
pub(crate) mod global_state;
mod views;

use dioxus::prelude::*;
use views::Home;

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[route("/")]
    Home(),
}

#[component]
pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
