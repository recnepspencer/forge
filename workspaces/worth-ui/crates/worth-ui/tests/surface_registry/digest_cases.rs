use super::*;

#[test]
fn different_surface_descriptor_meaning_produces_different_snapshot_digest() {
    let primary = WorthUi::app()
        .register_component(component_descriptor("workspace.component.editor"))
        .register_surface(surface_descriptor(
            "workspace.surface.editor",
            "workspace.component.editor",
        ))
        .freeze();
    let modal = WorthUi::app()
        .register_component(component_descriptor("workspace.component.editor"))
        .register_surface(SurfaceDescriptor::new(
            surface_id("workspace.surface.editor"),
            SurfaceKind::modal_content(),
            component_id("workspace.component.editor"),
            SurfacePlacementClass::modal_layer(),
            SurfaceStateClass::persistent(),
        ))
        .freeze();

    assert_ne!(
        primary.capabilities().surfaces(),
        modal.capabilities().surfaces()
    );
    assert_ne!(
        primary.capabilities().digest(),
        modal.capabilities().digest()
    );
}

#[test]
fn surface_command_slot_boundaries_affect_snapshot_digest() {
    let combined = WorthUi::app()
        .register_command(command_descriptor("workspace.command.ab", "AB"))
        .register_command(command_descriptor("workspace.command.c", "C"))
        .register_component(component_descriptor("workspace.component.editor"))
        .register_surface(
            surface_descriptor("workspace.surface.editor", "workspace.component.editor")
                .with_command_slot(command_id("workspace.command.ab"))
                .with_command_slot(command_id("workspace.command.c")),
        )
        .freeze();
    let split = WorthUi::app()
        .register_command(command_descriptor("workspace.command.a", "A"))
        .register_command(command_descriptor("workspace.command.bc", "BC"))
        .register_component(component_descriptor("workspace.component.editor"))
        .register_surface(
            surface_descriptor("workspace.surface.editor", "workspace.component.editor")
                .with_command_slot(command_id("workspace.command.a"))
                .with_command_slot(command_id("workspace.command.bc")),
        )
        .freeze();

    assert_ne!(
        combined.capabilities().surfaces(),
        split.capabilities().surfaces()
    );
    assert_ne!(
        combined.capabilities().digest(),
        split.capabilities().digest()
    );
}
