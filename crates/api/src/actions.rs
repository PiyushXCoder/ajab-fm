use crate::response_models::FileInfo;
#[allow(unused_imports)]
use chrono::{DateTime, Local};
use dioxus::prelude::*;
use rust_search::SearchBuilder;

#[server]
pub async fn list_files(path: String) -> Result<Vec<FileInfo>, ServerFnError> {
    let read_dir_result = std::fs::read_dir(&path)?;

    let dirs = read_dir_result.collect::<Result<Vec<_>, _>>()?;

    let mut files = Vec::new();
    for entry in dirs {
        let file_info = FileInfo::from_dir_entry(entry)?;
        files.push(file_info);
    }
    Ok(files)
}

#[server]
pub async fn get_home_dir() -> Result<String, ServerFnError> {
    let home_dir = dirs::home_dir()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    Ok(home_dir)
}

// #[server]
pub async fn search_file(path: String, filename: String) -> Result<Vec<FileInfo>, ServerFnError> {
    let search: Vec<String> = SearchBuilder::default()
        .location(&path)
        .search_input(filename)
        .limit(100) // results to return
        .strict()
        .hidden()
        .build()
        .collect();

    let mut files = Vec::new();
    for entry in search {
        let file_info = FileInfo::from_path(entry)?;
        files.push(file_info);
    }
    Ok(files)
}
