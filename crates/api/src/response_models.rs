use std::fs::DirEntry;

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
    pub path: String,
}

impl FileInfo {
    #[allow(dead_code)]
    pub(crate) fn from_dir_entry(value: DirEntry) -> Result<Self, ServerFnError> {
        let metadata = value.metadata()?;
        let name = value.file_name().into_string().unwrap_or("-".to_string());
        let size = format!("{:.2} KB", metadata.len() as f64 / 1024.0);

        let file_type = if metadata.is_dir() {
            "directory".to_string()
        } else if metadata.is_file() {
            infer::get_from_path(value.path())?
                .map(|filetype| filetype.extension().to_string())
                .unwrap_or("plain/txt".to_string())
        } else if metadata.is_symlink() {
            "symlink".to_string()
        } else {
            "unknown".to_string()
        };

        let modified: DateTime<Local> = (metadata.modified()?).into();
        let modified = modified.format("%Y-%m-%d %H:%M:%S").to_string();

        Ok(FileInfo {
            name,
            size,
            file_type,
            modified,
            path: value.path().to_string_lossy().to_string(),
        })
    }

    #[allow(dead_code)]
    pub(crate) fn from_path(path: String) -> Result<Self, ServerFnError> {
        let metadata = std::fs::metadata(&path)?;
        let path_buf = std::path::PathBuf::from(&path);
        let name = path_buf
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let size = format!("{:.2} KB", metadata.len() as f64 / 1024.0);

        let file_type = if metadata.is_dir() {
            "directory".to_string()
        } else if metadata.is_file() {
            infer::get_from_path(path.clone())?
                .map(|filetype| filetype.extension().to_string())
                .unwrap_or("plain/txt".to_string())
        } else if metadata.is_symlink() {
            "symlink".to_string()
        } else {
            "unknown".to_string()
        };

        let modified: DateTime<Local> = (metadata.modified()?).into();
        let modified = modified.format("%Y-%m-%d %H:%M:%S").to_string();

        Ok(FileInfo {
            name,
            size,
            file_type,
            modified,
            path,
        })
    }
}
