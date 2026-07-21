use worth_ui::facade::{
    app::WorthUi,
    registry::{CommandDescriptor, CommandId, ComponentChildPolicy, ComponentDescriptor, ComponentId, ComponentPropSchema, ComponentStateOwnership},
};

fn main() {
    let command_id = CommandId::new("workspace.save").expect("valid command id");
    let component_id = ComponentId::new("workspace.editor").expect("valid component id");

    let app = WorthUi::app()
        .register_command(CommandDescriptor::new(command_id.clone(), "Save"))
        .register_component(ComponentDescriptor::new(
            component_id,
            ComponentPropSchema::named("workspace.editor.props"),
            ComponentChildPolicy::no_children(),
            ComponentStateOwnership::runtime_owned(),
        ))
        .freeze().expect("application preparation should succeed");

    let command_lookup = app.capabilities().index().commands().lookup(&command_id);

    let _ = command_lookup.is_found();
}
