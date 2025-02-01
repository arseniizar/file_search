use dioxus::prelude::*;

#[component]
pub fn app() -> Element {
    rsx! {
        div {
            h1 { "Dioxus SSR Frontend" }
            form {
                action: "/search",
                method: "get",
                input { r#type: "text", name: "query", placeholder: "Search query" }
                button { "Search" }
            }
            form {
                id: "add-file-form",
                action: "/add",
                method: "post",
                input { r#type: "text", name: "path", placeholder: "File path" }
                input { r#type: "text", name: "name", placeholder: "File name" }
                input { r#type: "text", name: "modified_time", placeholder: "Modified time" }
                button { r#type: "submit", "Add File" }
            }
             script {
                src: "web/static/js/form-handler.js",
                defer: true
            }
        }
    }
}
