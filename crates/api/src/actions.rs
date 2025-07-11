#![allow(unused_imports)]
use crate::response_models::FileInfo;
#[allow(unused_imports)]
use chrono::{DateTime, Local};
use dioxus::prelude::*;
#[allow(unused_imports)]
use rust_search::SearchBuilder;
use std::io::Read;
#[allow(unused_imports)]
use std::path::PathBuf;

#[server]
pub async fn canonicalize(path: String) -> Result<String, ServerFnError> {
    let buf = PathBuf::from(path);
    let result = buf.canonicalize()?;
    Ok(result.to_string_lossy().to_string())
}

#[server]
pub async fn get_home_dir() -> Result<String, ServerFnError> {
    let home_dir = dirs::home_dir()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    Ok(home_dir)
}

#[server]
pub async fn list_files_streamed(path: String) -> Result<Vec<FileInfo>, ServerFnError> {
    let read_dir = std::fs::read_dir(&path)?;
    let mut file_info_list = Vec::new();

    for entry in read_dir {
        let entry = entry?;
        let result = FileInfo::from_dir_entry(entry)?;
        file_info_list.push(result);
    }

    Ok(file_info_list)
}

#[server]
pub async fn search_file_streamed(
    path: String,
    filename: String,
) -> Result<Vec<FileInfo>, ServerFnError> {
    let mut file_info_list = Vec::new();
    let search = SearchBuilder::default()
        .location(&path)
        .search_input(filename)
        .limit(100) // results to return
        .strict()
        .hidden()
        .build();

    for entry in search {
        let result = FileInfo::from_path(entry)?;
        file_info_list.push(result);
    }

    Ok(file_info_list)
}

#[server]
pub async fn read_text_file(path: String) -> Result<String, ServerFnError> {
    let file = std::fs::File::open(&path)?;
    let mut reader = std::io::BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    Ok(contents)
}
