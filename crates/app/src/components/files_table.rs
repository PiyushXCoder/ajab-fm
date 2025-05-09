use std::path::PathBuf;

use dioxus::prelude::*;

#[component]
pub(crate) fn FilesTable(path: Signal<String>) -> Element {
    let selected = use_signal(|| None::<usize>);
    let files = use_resource(move || async move {
        let path = path();
        api::actions::list_files(path).await
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
                            path: path
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
    path: Signal<String>,
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
                    let mut buf = PathBuf::from(path());
                    buf.push(&name);
                    path.set(buf.to_string_lossy().to_string());
                }
            },
            td { "{name}" }
            td { "{size}" }
            td { "{file_type}" }
            td { "{modified}" }
        }
    };
}
