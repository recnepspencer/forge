use worth_ui::facade::{
    CommandDescriptor, CommandId, IconDescriptor, IconFamily, IconId, IconSourceDescriptor, WorthUi,
};

fn main() {
    let icon_id = IconId::new("workspace.icon.save").unwrap();

    let _app = WorthUi::app()
        .register_icon(IconDescriptor::new(
            icon_id.clone(),
            IconFamily::command(),
            IconSourceDescriptor::symbol("save"),
        ))
        .register_command(
            CommandDescriptor::new(CommandId::new("workspace.command.save").unwrap(), "Save")
                .with_icon(icon_id),
        )
        .freeze().expect("application preparation should succeed");
}
