use std::path::PathBuf;

use dioxus::prelude::*;

#[component]
pub(crate) fn Toolbar(path: Signal<String>) -> Element {
    rsx! {
        div {
            class: "toolbar",
            button {
                class: "toolbar-button",
                onclick: move |_| {
                    let mut buf = PathBuf::from(path());
                    buf.push("..");
                    if let Ok(realpath) = buf.canonicalize() {
                        path.set(realpath.to_string_lossy().to_string());
                    }
                },
                "Back"
            }
        }
    }
}
