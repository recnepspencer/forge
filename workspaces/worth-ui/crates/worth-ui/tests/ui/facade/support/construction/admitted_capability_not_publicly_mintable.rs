use worth_ui::facade::{
    declaration::CommandId,
    support::AdmittedCapability,
};

fn main() {
    let id = CommandId::new("app.command.save").expect("valid command id");
    let _admitted = AdmittedCapability { id };
}
