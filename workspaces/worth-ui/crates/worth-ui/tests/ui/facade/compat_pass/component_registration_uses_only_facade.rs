use worth_ui::facade::{
    ComponentChildPolicy, ComponentDescriptor, ComponentId, ComponentPropSchema,
    ComponentStateOwnership, WorthUi,
};

fn main() {
    let _app = WorthUi::app()
        .register_component(ComponentDescriptor::new(
            ComponentId::new("workspace.component.editor").expect("valid component id"),
            ComponentPropSchema::named("workspace.editor.props"),
            ComponentChildPolicy::no_children(),
            ComponentStateOwnership::runtime_owned(),
        ))
        .freeze().expect("application preparation should succeed");
}
