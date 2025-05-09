use std::path::PathBuf;

use dioxus::prelude::*;

use crate::uri::URI;

#[component]
pub(crate) fn Toolbar(uri: Signal<URI>) -> Element {
    let mut search_query = use_signal(|| "".to_string());
    rsx! {
        div {
            class: "toolbar",
            button {
                class: "toolbar-button",
                onclick: move |_| {
                    if let URI::Path(path) = uri() {
                        let mut buf = PathBuf::from(path);
                        buf.push("..");
                        if let Ok(realpath) = buf.canonicalize() {
                            uri.set(URI::Path(realpath.to_string_lossy().to_string()));
                        }
                    }
                },
                "Back"
            }
            input {
                class: "toolbar-input",
                placeholder: "Search...",
                value: if let URI::Search(_, value) = uri() {value} else {""},
                onkeydown: move |e| {
                    if e.key() == Key::Enter {
                        uri.set(URI::Search(uri.cloned().path(), search_query.cloned()));
                    }
                },
                oninput: move |e| {
                    let value = e.value();
                    search_query.set(value.clone());
                }
            }
        }
    }
}
