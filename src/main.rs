use dioxus::prelude::*;

static STYLE_CSS: Asset = asset!("assets/style.css");

fn main() {
    dioxus::launch(App);
}

#[component]
pub fn App() -> Element {
    rsx! {
        document::Style {href: STYLE_CSS}
        app::App {}
    }
}
