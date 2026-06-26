use forge_query::facade::ForgeQueryOperationInput;

fn main() {
    let _ = ForgeQueryOperationInput::new("payload", serde_json::json!({"raw": "json"}));
}
