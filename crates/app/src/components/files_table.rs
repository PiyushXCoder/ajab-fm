use std::path::PathBuf;

use dioxus::prelude::*;

use crate::uri::URI;

#[component]
pub(crate) fn FilesTable(uri: Signal<URI>) -> Element {
    let selected = use_signal(|| None::<usize>);
    let files = use_resource(move || async move {
        let uri = uri();
        match uri {
            URI::Path(path) => api::actions::list_files(path).await,
            URI::Search(path, query) => api::actions::search_file(path, query).await,
        }
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

                if let Some(Ok(files)) = files.cloned() {
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
    uri: Signal<URI>,
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
                    let mut buf = PathBuf::from(uri().path());
                    buf.push(&name);
                    uri.set(URI::Path(path.clone()));
                }
            },
            td { "{name}" }
            td { "{size}" }
            td { "{file_type}" }
            td { "{modified}" }
        }
    };
}
