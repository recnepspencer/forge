use forge_query::facade::ForgeQueryAspectValue;
use serde_json::Value;

fn main() {
    let _aspect = ForgeQueryAspectValue {
        aspect_path: "title.value".to_string(),
        value: Value::String("Buy milk".to_string()),
    };
}
