use worth_ui::facade::{
    registry::{CommandDescriptor, CommandId},
};

fn main() {
    let _ = CommandDescriptor::new(
        CommandId::new("workspace.open").expect("valid command id"),
        "Open Workspace",
    )
    .with_readiness(true);
}
