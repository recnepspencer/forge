use worth_ui::facade::{AdmittedCapability, CommandId};

fn main() {
    let id = CommandId::new("app.command.save").expect("valid command id");
    let _admitted = AdmittedCapability { id };
}
