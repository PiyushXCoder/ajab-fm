use dioxus::prelude::*;

#[derive(thiserror::Error, Debug)]
pub(crate) enum MyError {
    #[error("Failed to read metadata")]
    IoError(#[from] std::io::Error),
}
