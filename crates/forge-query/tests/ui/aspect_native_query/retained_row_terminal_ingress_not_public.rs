use forge_query::facade::ForgeQueryRetainedMaterializedRow;

fn main() {
    let _ = ForgeQueryRetainedMaterializedRow::from_terminal_json_row(serde_json::json!({
        "value": "terminal"
    }));
}
