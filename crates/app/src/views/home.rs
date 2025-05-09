use crate::components::files_table::FilesTable;
use crate::components::toolbar::Toolbar;
use crate::uri::URI;
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    let home_dir = use_resource(move || async move { api::actions::get_home_dir().await });
    let mut uri = use_signal(move || match home_dir.cloned() {
        Some(Ok(path)) => URI::Path(path),
        _ => URI::Path("".to_string()),
    });

    use_effect(move || {
        if let Some(Ok(path)) = home_dir.cloned() {
            uri.set(URI::Path(path));
        }
    });

    println!("[Home] uri: {:?}", uri());

    rsx! {
        div {
            div { {uri.to_string() } }
            Toolbar {uri}
            FilesTable {uri}
        }
    }
}
