use worth_ui::facade::{
    ComponentChildPolicy, ComponentDescriptor, ComponentId, ComponentPropSchema,
    ComponentStateOwnership, SurfaceDescriptor, SurfaceId, SurfaceKind, SurfacePlacementClass,
    SurfaceStateClass, WorthUi,
};

fn main() {
    let _app = WorthUi::app()
        .register_component(ComponentDescriptor::new(
            ComponentId::new("workspace.component.editor").expect("valid component id"),
            ComponentPropSchema::named("workspace.editor.props"),
            ComponentChildPolicy::no_children(),
            ComponentStateOwnership::runtime_owned(),
        ))
        .register_surface(SurfaceDescriptor::new(
            SurfaceId::new("workspace.surface.editor").expect("valid surface id"),
            SurfaceKind::primary_content(),
            ComponentId::new("workspace.component.editor").expect("valid component id"),
            SurfacePlacementClass::primary_region(),
            SurfaceStateClass::restorable(),
        ))
        .freeze().expect("application preparation should succeed");
}
