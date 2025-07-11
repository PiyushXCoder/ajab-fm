use std::path::PathBuf;

use api::response_models::FileInfo;
use dioxus::logger::tracing;
use dioxus::prelude::*;

use crate::global_state::Uri;
use crate::global_state::UriMomento;

#[component]
pub(crate) fn FilesTable(uri: Signal<UriMomento>) -> Element {
    let selected = use_signal(|| None::<usize>);
    let mut files = use_signal(Vec::<FileInfo>::new);
    let mut is_loading = use_signal(|| false);

    let _ = use_resource(move || async move {
        let current_uri = uri.read().get_current_uri().unwrap_or_default();
        files.set(vec![]);
        is_loading.set(true);
        let file_info_list = match current_uri {
            Uri::Path(path) => {
                if path.is_empty() {
                    return;
                }
                api::actions::list_files_streamed(path).await
            }
            Uri::Search(path, query) => api::actions::search_file_streamed(path, query).await,
        };

        let file_info_list = match file_info_list {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("Error: {}", e);
                return;
            }
        };

        spawn(async move {
            for file in file_info_list {
                files.write().push(file);
            }
            is_loading.set(false);
        });
    });

    return rsx! {
        table {
            thead {
                tr {
                    th { "Name" }
                    th { "Size" }
                    th { "Type" }
                    th { "Last Modified" }
                }
            }
            tbody {
                for file in files.iter() {
                    FileRow {
                        selected: selected,
                        index: 0,
                        name: file.name.clone(),
                        size: file.size.clone(),
                        file_type: file.file_type.clone(),
                        modified: file.modified.clone(),
                        path: file.path.clone(),
                        uri,
                    }
                }
            }
        }

        if is_loading() {
            div { "Loading..." }
        }
    };
}

#[component]
fn FileRow(
    selected: Signal<Option<usize>>,
    index: usize,
    name: String,
    size: String,
    file_type: String,
    modified: String,
    path: String,
    uri: Signal<UriMomento>,
) -> Element {
    let is_selected = selected() == Some(index);
    let nav = use_navigator();
    return rsx! {
        tr {
            class: if is_selected { "highlight" } else { "" },
            onclick: move |_| selected.set(Some(index)),
            oncontextmenu: move |_| {
                println!("Right-clicked");
            },
            ondoubleclick: move |_| {
                if &file_type == "directory" {
                    let current_uri = uri.read().get_current_uri().unwrap_or_default();
                    let mut buf = PathBuf::from(current_uri.path());
                    buf.push(&name);
                    uri.write().add_uri(Uri::Path(path.clone()));
                    uri.write().set_current_to_latest();
                } else if &file_type == "plain/txt" {
                    nav.push(crate::Route::TextFileViewer{ path: path.clone() });
                }
            },
            td {
                class: "file-name",
                "{name}"
            }
            td { "{size}" }
            td { "{file_type}" }
            td { "{modified}" }
        }
    };
}
