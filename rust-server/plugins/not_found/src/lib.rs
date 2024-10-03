use lms_plugin_api::{bindings, Plugin};

pub struct NotFound {}

impl Plugin for NotFound {
    fn query(query: String) -> String {
        format!(
            r#"
    {{
        "type": "common_message",
        "name": "Not found",
        "discription": "Plugin {query} not found"
        }}
        "#
        )
    }
}

bindings::export!(NotFound with_types_in bindings);
