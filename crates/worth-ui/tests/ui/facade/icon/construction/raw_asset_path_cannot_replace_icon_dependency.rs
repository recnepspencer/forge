use worth_ui::facade::{CommandDescriptor, CommandId};

fn main() {
    let _ = CommandDescriptor::new(CommandId::new("workspace.command.save").unwrap(), "Save")
        .with_icon("assets/icons/save.svg");
}
