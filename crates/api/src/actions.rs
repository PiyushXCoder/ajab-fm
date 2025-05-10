use crate::response_models::FileInfo;
#[allow(unused_imports)]
use chrono::{DateTime, Local};
use dioxus::prelude::*;
#[allow(unused_imports)]
use rust_search::SearchBuilder;
use server_fn::codec::{JsonStream, StreamingJson};

#[server]
pub async fn get_home_dir() -> Result<String, ServerFnError> {
    let home_dir = dirs::home_dir()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    Ok(home_dir)
}

#[server(output = StreamingJson)]
pub async fn list_files_streamed(path: String) -> Result<JsonStream<FileInfo>, ServerFnError> {
    let (tx, rx) = futures::channel::mpsc::unbounded::<Result<FileInfo, ServerFnError>>();
    tokio::spawn(async move {
        let read_dir = match std::fs::read_dir(&path) {
            Ok(rd) => rd,
            Err(e) => {
                let _ = tx.unbounded_send(Err(ServerFnError::from(e)));
                return;
            }
        };

        for entry in read_dir {
            let entry = match entry {
                Ok(e) => e,
                Err(e) => {
                    let _ = tx.unbounded_send(Err(ServerFnError::from(e)));
                    continue;
                }
            };

            let result = FileInfo::from_dir_entry(entry);
            if tx.unbounded_send(result).is_err() {
                break;
            }
        }
    });

    Ok(JsonStream::new(rx))
}

#[server(output = StreamingJson)]
pub async fn search_file_streamed(
    path: String,
    filename: String,
) -> Result<JsonStream<FileInfo>, ServerFnError> {
    let (tx, rx) = futures::channel::mpsc::unbounded::<Result<FileInfo, ServerFnError>>();
    tokio::spawn(async move {
        let search = SearchBuilder::default()
            .location(&path)
            .search_input(filename)
            .limit(100) // results to return
            .strict()
            .hidden()
            .build();

        for entry in search {
            let result = FileInfo::from_path(entry);
            if tx.unbounded_send(result).is_err() {
                break;
            }
        }
    });

    Ok(JsonStream::new(rx))
}
