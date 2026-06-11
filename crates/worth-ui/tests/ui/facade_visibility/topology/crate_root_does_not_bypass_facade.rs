use worth_ui::{CommandDescriptor, CommandId, WorthUi};

fn main() {
    let _ = WorthUi::app().register_command(CommandDescriptor::new(
        CommandId::new("workspace.save").expect("valid command id"),
        "Save",
    ));
}
