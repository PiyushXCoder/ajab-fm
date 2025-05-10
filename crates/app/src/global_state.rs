use dioxus::signals::GlobalSignal;
use std::fmt::Display;

pub(crate) static GLOBAL_STATE: GlobalSignal<UriMomento> = GlobalSignal::new(UriMomento::new);

#[derive(Clone, Debug)]
pub(crate) struct UriMomento {
    pub(crate) uris: Vec<Uri>,
    pub(crate) current_uri: Option<usize>,
}

impl UriMomento {
    pub(crate) fn new() -> Self {
        Self {
            uris: Vec::new(),
            current_uri: None,
        }
    }

    pub(crate) fn add_uri(&mut self, uri: Uri) {
        self.uris.push(uri);
    }

    pub(crate) fn set_current_to_latest(&mut self) {
        if !self.uris.is_empty() {
            self.current_uri = Some(self.uris.len() - 1);
        }
    }

    pub(crate) fn set_current_uri(&mut self, index: usize) {
        if index < self.uris.len() {
            self.current_uri = Some(index);
        }
    }

    pub(crate) fn get_current_uri(&self) -> Option<Uri> {
        self.current_uri
            .and_then(|index| self.uris.get(index))
            .cloned()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Uri {
    Path(String),
    Search(String, String),
}

impl Display for Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Uri::Path(path) => write!(f, "{}", path),
            Uri::Search(path, query) => write!(f, "Path:{} Query:{}", path, query),
        }
    }
}

impl Default for Uri {
    fn default() -> Self {
        Uri::Path(String::new())
    }
}

impl Uri {
    pub fn path(&self) -> String {
        match self {
            Uri::Path(path) => path.clone(),
            Uri::Search(path, _) => path.clone(),
        }
    }
}
