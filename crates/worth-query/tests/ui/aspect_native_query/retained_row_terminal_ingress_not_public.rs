use worth_query::facade::runtime::WorthQueryRetainedMaterializedRow;

fn main() {
    let _ = WorthQueryRetainedMaterializedRow::from_terminal_json_row(serde_json::json!({
        "value": "terminal"
    }));
}
