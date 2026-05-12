use forge_query::facade::ForgeQueryWriteCommand;
use serde_json::json;

fn main() {
    let _ = ForgeQueryWriteCommand::Insert {
        collection: "Task".to_string(),
        payload: json!({
            "identity": { "id": "task-1" },
            "title": { "value": "removed payload insert" },
        }),
    };
}
