use crate::response_models::FileInfo;
#[allow(unused_imports)]
use chrono::{DateTime, Local};
use dioxus::prelude::*;

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

