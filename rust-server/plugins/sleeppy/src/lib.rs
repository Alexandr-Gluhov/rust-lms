use lms_plugin_api::{bindings, Plugin};

pub struct SleeppyPlug {}

impl SleeppyPlug {
    fn create() -> String {
        String::from(
            r#"
{
    "type": "common_message",
    "name": "sleeppy plug",
    "discription": "open and wait"
}
    "#,
        )
    }

    fn open(query: &str) -> String {
        format!(
            r#"
{{
    "message": "woke up"
}}
        "#
        )
    }
}

impl Plugin for SleeppyPlug {
    fn query(query: String) -> String {
        let query = &query[..];
        match query {
            "open" => Self::open(query),
            _ => Self::create(),
        }
    }
}

bindings::export!(SleeppyPlug with_types_in bindings);
