use worth_ui::facade::{
    registry::{CommandId, ComponentId, SurfaceDescriptor, SurfaceId, SurfaceKind, SurfacePlacementClass, SurfaceStateClass},
};

fn main() {
    let _descriptor = SurfaceDescriptor {
        id: SurfaceId::new("workspace.surface.editor").expect("valid surface id"),
        kind: SurfaceKind::primary_content(),
        component_id: ComponentId::new("workspace.component.editor").expect("valid component id"),
        placement_class: SurfacePlacementClass::primary_region(),
        state_class: SurfaceStateClass::restorable(),
        command_slots: Vec::<CommandId>::new(),
        label: None,
        view_binding: None,
    };
}
