use crate::components::files_table::FilesTable;
use crate::components::toolbar::Toolbar;
use crate::global_state::{GLOBAL_STATE, Uri};
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    let mut uri = GLOBAL_STATE.signal();
    use_future(move || async move {
        let home_dir = api::actions::get_home_dir().await;
        if let Ok(path) = home_dir {
            uri.write().add_uri(Uri::Path(path.clone()));
            uri.write().set_current_to_latest();
        }
    });
    rsx! {
        div {
            div { {uri.read().get_current_uri().unwrap_or_default().to_string()} }
            Toolbar {uri}
            FilesTable {uri}
        }
    }
}
