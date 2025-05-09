use crate::components::files_table::FilesTable;
use crate::components::toolbar::Toolbar;
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    let path = use_signal(|| "/home/piyush/".to_string());
    rsx! {
        div {
            div { {path} }
            Toolbar {path}
            FilesTable {path}
        }
    }
}
