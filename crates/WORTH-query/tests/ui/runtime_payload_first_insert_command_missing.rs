use worth_query::facade::WorthQueryWriteCommand;

fn main() {
    let _ = WorthQueryWriteCommand::Insert {
        collection: "Task".to_string(),
        payload: "removed payload insert",
    };
}
