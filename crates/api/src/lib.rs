#[allow(unused_imports)]
use chrono::{DateTime, Local};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FileInfo {
    pub name: String,
    pub size: String,
    pub file_type: String,
    pub modified: String,
}

#[server]
pub async fn list_files(path: String) -> Result<Vec<FileInfo>, ServerFnError> {
    let read_dir_result =
        std::fs::read_dir(&path).map_err(|_| ServerFnError::new("Failed to read directory"))?;

    let dirs = read_dir_result
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| ServerFnError::new("Failed to collect directory entries"))?;

    let mut files = Vec::new();
    for entry in dirs {
        let metadata = entry
            .metadata()
            .map_err(|_| ServerFnError::new("Failed to read metadata"))?;
        let name = entry
            .file_name()
            .into_string()
            .map_err(|_| ServerFnError::new("Failed to convert file name"))?;
        let size = format!("{:.2} KB", metadata.len() as f64 / 1024.0);
        println!("{:?}", entry.path());

        let file_type;

        if metadata.is_dir() {
            file_type = "directory".to_string();
        } else if metadata.is_file() {
            file_type = infer::get_from_path(entry.path())
                .ok()
                .and_then(|r| r)
                .map(|t| t.extension().to_string())
                .unwrap_or("plain/txt".to_string());
        } else if metadata.is_symlink() {
            file_type = "symlink".to_string();
        } else {
            file_type = "unknown".to_string();
        }
        let modified = metadata
            .modified()
            .map_err(|_| ServerFnError::new("Failed to read modified time"))?;
        let modified: DateTime<Local> = modified.into();
        let modified = modified.format("%Y-%m-%d %H:%M:%S").to_string();

        files.push(FileInfo {
            name,
            size,
            file_type,
            modified,
        });
    }
    Ok(files)
}
