use std::path::PathBuf;

use api::response_models::FileInfo;
use dioxus::prelude::*;
use futures::StreamExt;

use crate::global_state::Uri;
use crate::global_state::UriMomento;

#[component]
pub(crate) fn FilesTable(uri: Signal<UriMomento>) -> Element {
    let selected = use_signal(|| None::<usize>);
    let mut files = use_signal(Vec::<FileInfo>::new);
    let mut is_loading = use_signal(|| false);

    let _ = use_resource(move || async move {
        let current_uri = uri.read().get_current_uri().unwrap_or_default();
        let stream = match current_uri {
            Uri::Path(path) => api::actions::list_files_streamed(path).await,
            Uri::Search(path, query) => api::actions::search_file_streamed(path, query).await,
        }
        .unwrap();
        files.set(vec![]);
        is_loading.set(true);
        spawn(async move {
            let mut stream = stream.into_inner();
            while let Some(file) = stream.next().await {
                if let Ok(file) = file {
                    files.write().push(file);
                }
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
                }
            },
            td { "{name}" }
            td { "{size}" }
            td { "{file_type}" }
            td { "{modified}" }
        }
    };
}
