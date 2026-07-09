use worth_query::facade::WorthQueryOperationInput;

fn main() {
    let _ = WorthQueryOperationInput::new("payload", serde_json::json!({"raw": "json"}));
}
