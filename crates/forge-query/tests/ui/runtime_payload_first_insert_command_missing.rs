use forge_query::facade::ForgeQueryWriteCommand;

fn main() {
    let _ = ForgeQueryWriteCommand::Insert {
        collection: "Task".to_string(),
        payload: "removed payload insert",
    };
}
