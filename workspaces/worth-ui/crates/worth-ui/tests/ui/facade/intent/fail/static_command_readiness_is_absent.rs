use worth_ui::facade::declaration::{CommandDescriptor, CommandId};

fn main() {
    let _ = CommandDescriptor::new(
        CommandId::new("compile.command").expect("valid command id"),
        "Compile command",
    )
    .with_readiness(true);
}
