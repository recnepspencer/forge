use worth_ui::{CommandDescriptor, CommandId, WorthUi};

fn main() {
    let _ = WorthUi::app().with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse()).register_command(CommandDescriptor::new(
        CommandId::new("workspace.save").expect("valid command id"),
        "Save",
    ));
}
