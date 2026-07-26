use worth_ui::facade::declaration::{
    CommandDescriptor, CommandId, ComponentChildPolicy, ComponentDescriptor, ComponentId,
    ComponentPropSchema, ComponentStateOwnership, SurfaceDescriptor, SurfaceId, SurfaceKind,
    SurfacePlacementClass, SurfaceStateClass, ViewBindingId,
};

pub(crate) fn command_descriptor(id: &str, label: &str) -> CommandDescriptor {
    CommandDescriptor::new(command_id(id), label)
}

pub(crate) fn component_descriptor(id: &str) -> ComponentDescriptor {
    ComponentDescriptor::new(
        component_id(id),
        ComponentPropSchema::named(format!("{id}.props")),
        ComponentChildPolicy::no_children(),
        ComponentStateOwnership::runtime_owned(),
    )
}

pub(crate) fn surface_descriptor(id: &str, component_id: &str) -> SurfaceDescriptor {
    SurfaceDescriptor::new(
        surface_id(id),
        SurfaceKind::primary_content(),
        self::component_id(component_id),
        SurfacePlacementClass::primary_region(),
        SurfaceStateClass::restorable(),
    )
}

pub(crate) fn component_id(raw_text: &str) -> ComponentId {
    ComponentId::new(raw_text).expect("valid component id")
}

pub(crate) fn command_id(raw_text: &str) -> CommandId {
    CommandId::new(raw_text).expect("valid command id")
}

pub(crate) fn surface_id(raw_text: &str) -> SurfaceId {
    SurfaceId::new(raw_text).expect("valid surface id")
}

pub(crate) fn view_binding_id(raw_text: &str) -> ViewBindingId {
    ViewBindingId::new(raw_text).expect("valid view binding id")
}
