use lms_plugin_api::{
    Plugin,
    bindings,
};

pub struct HelloPlug {}

impl HelloPlug {
    fn create() -> String {
        String::from(r#"
{
    "type": "common_message",
    "name": "hello plugin",
    "discription": "Hello plugin discription"
}
    "#)
    }

    fn open(query: &str) -> String {
        format!(r#"
{{
    "message": "Hello, {query}"
}}
        "#)
    }
}

impl Plugin for HelloPlug {
    fn query(query: String) -> String {
        let query = &query[..];
        match query {
            "open" => Self::open(query),
            _ => Self::create(),
        }
    }
}

bindings::export!(HelloPlug with_types_in bindings);