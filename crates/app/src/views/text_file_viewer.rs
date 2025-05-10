use dioxus::prelude::*;

#[component]
pub fn TextFileViewer(path: ReadOnlySignal<String>) -> Element {
    let nav = use_navigator();
    let file_content = use_resource(move || async move {
        let content = api::actions::read_text_file(path.to_string()).await;
        content.unwrap_or_default()
    });

    rsx! {
        div {
            h1 { "Text File Viewer" }
            p { "Path: {path}" }
            button {

                onclick: move |_| {
                    nav.go_back();
                },
                "Back to File Browser"
            }
            hr {  }
            pre {
                {file_content.cloned()}
            }

        }
    }
}
