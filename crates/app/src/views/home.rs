use crate::components::files_table::FilesTable;
use crate::components::toolbar::Toolbar;
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    let home_dir = use_resource(move || async move { api::actions::get_home_dir().await }).cloned();
    let path = if let Some(Ok(home_dir)) = home_dir {
        use_signal(move || home_dir)
    } else {
        println!("Failed to get home dir");
        use_signal(move || String::from("/"))
    };

    rsx! {
        div {
            div { {path} }
            Toolbar {path}
            FilesTable {path}
        }
    }
}
