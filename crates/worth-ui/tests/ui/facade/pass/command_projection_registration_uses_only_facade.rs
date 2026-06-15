use worth_ui::facade::{
    CommandCategory, CommandDescriptor, CommandId, CommandProjectionCommandReference,
    CommandProjectionDescriptor, CommandProjectionId, CommandProjectionSurface, WorthUi,
};

fn main() {
    let command_id = CommandId::new("workspace.command.save").expect("valid command id");
    let projection_id =
        CommandProjectionId::new("workspace.projection.toolbar").expect("valid projection id");

    let app = WorthUi::app()
        .register_command(CommandDescriptor::new(command_id.clone(), "Save"))
        .register_command_projection(
            CommandProjectionDescriptor::new(
                projection_id.clone(),
                CommandProjectionSurface::toolbar(),
            )
            .with_command_reference(CommandProjectionCommandReference::command(command_id))
            .with_eligible_category(CommandCategory::Workspace)
            .show_shortcuts()
            .show_readiness(),
        )
        .freeze();

    let _ = app.capabilities().command_projections().get(&projection_id);
}
