use std::path::PathBuf;

use dioxus::prelude::*;

use crate::global_state::{Uri, UriMomento};

#[component]
pub(crate) fn Toolbar(uri: Signal<UriMomento>) -> Element {
    let mut search_query = use_signal(|| "".to_string());
    rsx! {
        div {
            class: "toolbar",
            button {
                class: "toolbar-button",
                onclick: move |_| {
                    let current_uri_index = uri.read().current_uri;
                    if let Some(index) = current_uri_index {
                        if index > 0 {
                            uri.write().set_current_uri(index - 1);
                        }
                    }
                },
                "Back"
            }
            button {
                class: "toolbar-button",
                onclick: move |_| {
                    let current_uri_index = uri.read().current_uri;
                    if let Some(index) = current_uri_index {
                        if index < uri.read().uris.len() - 1 {
                            uri.write().set_current_uri(index + 1);
                        }
                    }
                },
                "Front"
            }
            button {
                class: "toolbar-button",
                onclick: move |_| async move {
                    let current_uri = uri.read().get_current_uri().unwrap_or_default();
                    if let Uri::Path(path) = current_uri {
                        let mut buf = PathBuf::from(path);
                        buf.push("..");
                        let canonical_path = api::actions::canonicalize(buf.to_string_lossy().to_string()).await;
                        if let Ok(realpath) = canonical_path {
                            uri.write().add_uri(Uri::Path(realpath));
                            uri.write().set_current_to_latest();
                        }
                    }
                },
                "Up"
            }
            button {
                class: "toolbar-button",
                onclick: move |_| async move {
                    let homr_dir = api::actions::get_home_dir().await;
                    if let Ok(realpath) = homr_dir {
                        uri.write().add_uri(Uri::Path(realpath));
                        uri.write().set_current_to_latest();
                    }
                },
                "Home"
            }
            input {
                class: "toolbar-input",
                placeholder: "Search...",
                value: if let Uri::Search(_, value) = uri.read().get_current_uri().unwrap_or_default() {value} else {""},
                onkeydown: move |e| {
                    if e.key() == Key::Enter {
                        let current_uri = uri.read().get_current_uri().unwrap_or_default();
                        uri.write().add_uri(Uri::Search(current_uri.path(), search_query.cloned()));
                        uri.write().set_current_to_latest();
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
