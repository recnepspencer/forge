use worth_query::facade::runtime::WorthQueryWriteCommand;

fn main() {
    let _ = WorthQueryWriteCommand::Insert {
        collection: "Task".to_string(),
        payload: "removed payload insert",
    };
}
