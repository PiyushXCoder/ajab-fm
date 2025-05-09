use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum URI {
    Path(String),
    Search(String, String),
}

impl Display for URI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            URI::Path(path) => write!(f, "{}", path),
            URI::Search(path, query) => write!(f, "Path:{} Query:{}", path, query),
        }
    }
}

impl URI {
    pub fn path(&self) -> String {
        match self {
            URI::Path(path) => path.clone(),
            URI::Search(path, _) => path.clone(),
        }
    }
}
