use dioxus::{logger::tracing::Level, prelude::*};

static STYLE_CSS: Asset = asset!("assets/style.css");

fn main() {
    dioxus::logger::init(Level::INFO).expect("Failed to initialize logger");
    dioxus::launch(App);
}

#[component]
pub fn App() -> Element {
    rsx! {
        document::Style {href: STYLE_CSS}
        app::App {}
    }
}
