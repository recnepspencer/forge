use worth_ui::facade::{
    ComponentChildPolicy, ComponentDescriptor, ComponentId, ComponentPropSchema,
    ComponentStateOwnership, SurfaceDescriptor, SurfaceId, SurfaceKind, SurfacePlacementClass,
    SurfaceStateClass, WorthUi,
};

use super::view_binding_fixtures::{table_view_binding, view_binding_id};

#[test]
fn admitted_view_binding_can_satisfy_surface_view_binding_reference() {
    let report = WorthUi::app()
        .register_component(component_descriptor("workspace.component.tasks"))
        .register_view_binding(table_view_binding("workspace.view_binding.tasks"))
        .register_surface(
            SurfaceDescriptor::new(
                surface_id("workspace.surface.tasks"),
                SurfaceKind::primary_content(),
                component_id("workspace.component.tasks"),
                SurfacePlacementClass::primary_region(),
                SurfaceStateClass::restorable(),
            )
            .with_view_binding(view_binding_id("workspace.view_binding.tasks")),
        )
        .freeze_with_registration_report();

    assert!(!report.has_errors());
    assert_eq!(report.accepted_snapshot().surfaces().len(), 1);
    assert_eq!(report.accepted_snapshot().view_bindings().len(), 1);
}

fn component_descriptor(id: &str) -> ComponentDescriptor {
    ComponentDescriptor::new(
        component_id(id),
        ComponentPropSchema::named(format!("{id}.props")),
        ComponentChildPolicy::no_children(),
        ComponentStateOwnership::runtime_owned(),
    )
}

fn component_id(raw_text: &str) -> ComponentId {
    ComponentId::new(raw_text).expect("valid component id")
}

fn surface_id(raw_text: &str) -> SurfaceId {
    SurfaceId::new(raw_text).expect("valid surface id")
}
