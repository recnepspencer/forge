use worth_ui::facade::{
    CapabilityDiagnosticCode, SurfaceDescriptor, SurfaceKind, SurfacePlacementClass,
    SurfaceStateClass, WorthUi,
};

#[path = "surface_registry/adversarial_cases.rs"]
mod adversarial_cases;
#[path = "surface_registry/digest_cases.rs"]
mod digest_cases;
#[path = "surface_registry/surface_registry_assertions.rs"]
mod surface_registry_assertions;
#[path = "surface_registry/surface_registry_fixtures.rs"]
mod surface_registry_fixtures;

use surface_registry_assertions::{
    assert_dependency_diagnostics, assert_diagnostic_codes, assert_diagnostic_codes_and_identities,
    assert_registered_surface_ids,
};
use surface_registry_fixtures::{
    command_descriptor, command_id, component_descriptor, component_id, surface_descriptor,
    surface_id, view_binding_id,
};

#[test]
fn equivalent_app_defined_surfaces_produce_equivalent_entries() {
    let first = WorthUi::app()
        .register_component(component_descriptor("workspace.component.editor"))
        .register_component(component_descriptor("workspace.component.sidebar"))
        .register_surface(surface_descriptor(
            "workspace.surface.editor",
            "workspace.component.editor",
        ))
        .register_surface(surface_descriptor(
            "workspace.surface.sidebar",
            "workspace.component.sidebar",
        ))
        .freeze();
    let second = WorthUi::app()
        .register_surface(surface_descriptor(
            "workspace.surface.sidebar",
            "workspace.component.sidebar",
        ))
        .register_component(component_descriptor("workspace.component.sidebar"))
        .register_surface(surface_descriptor(
            "workspace.surface.editor",
            "workspace.component.editor",
        ))
        .register_component(component_descriptor("workspace.component.editor"))
        .freeze();

    assert_eq!(
        first.capabilities().surfaces(),
        second.capabilities().surfaces()
    );
    assert_eq!(
        first.capabilities().digest(),
        second.capabilities().digest()
    );
    assert_registered_surface_ids(
        first.capabilities().surfaces(),
        &["workspace.surface.editor", "workspace.surface.sidebar"],
    );
}

#[test]
fn surface_references_missing_component_rejected() {
    let report = WorthUi::app()
        .register_surface(surface_descriptor(
            "workspace.surface.editor",
            "workspace.component.editor",
        ))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().surfaces().is_empty());
    assert_dependency_diagnostics(
        report.registration_diagnostics(),
        &[(
            CapabilityDiagnosticCode::MissingDependency,
            "workspace.surface.editor",
            "component",
            "workspace.component.editor",
        )],
    );
}

#[test]
fn surface_missing_component_does_not_poison_valid_surface() {
    let report = WorthUi::app()
        .register_component(component_descriptor("workspace.component.valid"))
        .register_surface(surface_descriptor(
            "workspace.surface.valid",
            "workspace.component.valid",
        ))
        .register_surface(surface_descriptor(
            "workspace.surface.missing",
            "workspace.component.missing",
        ))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_registered_surface_ids(
        report.accepted_snapshot().surfaces(),
        &["workspace.surface.valid"],
    );
    assert_dependency_diagnostics(
        report.registration_diagnostics(),
        &[(
            CapabilityDiagnosticCode::MissingDependency,
            "workspace.surface.missing",
            "component",
            "workspace.component.missing",
        )],
    );
}

#[test]
fn duplicate_surface_id_rejected_before_snapshot_freeze() {
    let report = WorthUi::app()
        .register_component(component_descriptor("workspace.component.editor"))
        .register_surface(surface_descriptor(
            "workspace.surface.editor",
            "workspace.component.editor",
        ))
        .register_surface(surface_descriptor(
            "workspace.surface.editor",
            "workspace.component.editor",
        ))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().surfaces().is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[
            CapabilityDiagnosticCode::DuplicateCapabilityId,
            CapabilityDiagnosticCode::DuplicateCapabilityId,
        ],
    );
}

#[test]
fn duplicate_surface_id_rejects_only_the_duplicate_identity() {
    let report = WorthUi::app()
        .register_component(component_descriptor("workspace.component.valid"))
        .register_component(component_descriptor("workspace.component.editor"))
        .register_surface(surface_descriptor(
            "workspace.surface.valid",
            "workspace.component.valid",
        ))
        .register_surface(surface_descriptor(
            "workspace.surface.editor",
            "workspace.component.editor",
        ))
        .register_surface(surface_descriptor(
            "workspace.surface.editor",
            "workspace.component.editor",
        ))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_registered_surface_ids(
        report.accepted_snapshot().surfaces(),
        &["workspace.surface.valid"],
    );
    assert_diagnostic_codes_and_identities(
        report.registration_diagnostics(),
        &[
            (
                CapabilityDiagnosticCode::DuplicateCapabilityId,
                "workspace.surface.editor",
            ),
            (
                CapabilityDiagnosticCode::DuplicateCapabilityId,
                "workspace.surface.editor",
            ),
        ],
    );
}

#[test]
fn surface_claims_unsupported_placement_class_rejected() {
    let report = WorthUi::app()
        .register_component(component_descriptor("workspace.component.editor"))
        .register_surface(SurfaceDescriptor::new(
            surface_id("workspace.surface.editor"),
            SurfaceKind::primary_content(),
            component_id("workspace.component.editor"),
            SurfacePlacementClass::unsupported_for_diagnostics("floating-dock"),
            SurfaceStateClass::restorable(),
        ))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().surfaces().is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::UnsupportedSurfacePlacementClass],
    );
}

#[test]
fn surface_uses_invalid_state_class_rejected() {
    let report = WorthUi::app()
        .register_component(component_descriptor("workspace.component.editor"))
        .register_surface(SurfaceDescriptor::new(
            surface_id("workspace.surface.editor"),
            SurfaceKind::primary_content(),
            component_id("workspace.component.editor"),
            SurfacePlacementClass::primary_region(),
            SurfaceStateClass::invalid_for_diagnostics("ambient-memory"),
        ))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().surfaces().is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::InvalidSurfaceStateClass],
    );
}

#[test]
fn platform_builtin_surface_domain_name_rejected() {
    let report = WorthUi::app()
        .register_component(component_descriptor("workspace.component.editor"))
        .register_surface(SurfaceDescriptor::new(
            surface_id("workspace.surface.editor"),
            SurfaceKind::product_domain_name_for_diagnostics("project explorer"),
            component_id("workspace.component.editor"),
            SurfacePlacementClass::primary_region(),
            SurfaceStateClass::restorable(),
        ))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().surfaces().is_empty());
    assert_diagnostic_codes_and_identities(
        report.registration_diagnostics(),
        &[(
            CapabilityDiagnosticCode::ProductDomainSurfaceKind,
            "workspace.surface.editor",
        )],
    );
}

#[test]
fn surface_view_binding_reference_fails_closed_until_view_binding_registry_exists() {
    let report = WorthUi::app()
        .register_component(component_descriptor("workspace.component.editor"))
        .register_surface(
            surface_descriptor("workspace.surface.editor", "workspace.component.editor")
                .with_view_binding(view_binding_id("workspace.view_binding.editor")),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().surfaces().is_empty());
    assert_dependency_diagnostics(
        report.registration_diagnostics(),
        &[(
            CapabilityDiagnosticCode::MissingDependency,
            "workspace.surface.editor",
            "view_binding",
            "workspace.view_binding.editor",
        )],
    );
}

#[test]
fn surface_references_missing_command_slot_rejected() {
    let report = WorthUi::app()
        .register_component(component_descriptor("workspace.component.editor"))
        .register_surface(
            surface_descriptor("workspace.surface.editor", "workspace.component.editor")
                .with_command_slot(command_id("workspace.command.open")),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().surfaces().is_empty());
    assert_dependency_diagnostics(
        report.registration_diagnostics(),
        &[(
            CapabilityDiagnosticCode::MissingDependency,
            "workspace.surface.editor",
            "command",
            "workspace.command.open",
        )],
    );
}

#[test]
fn surface_command_slot_resolves_against_registered_command() {
    let app = WorthUi::app()
        .register_command(command_descriptor(
            "workspace.command.open",
            "Open Workspace",
        ))
        .register_component(component_descriptor("workspace.component.editor"))
        .register_surface(
            surface_descriptor("workspace.surface.editor", "workspace.component.editor")
                .with_command_slot(command_id("workspace.command.open")),
        )
        .freeze();

    let descriptor = app
        .capabilities()
        .surfaces()
        .get(&surface_id("workspace.surface.editor"))
        .expect("registered surface");
    assert_eq!(
        descriptor.command_slots(),
        &[command_id("workspace.command.open")]
    );
}

#[test]
fn surface_metadata_survives_freeze() {
    let app = WorthUi::app()
        .register_component(component_descriptor("workspace.component.overlay"))
        .register_surface(
            SurfaceDescriptor::new(
                surface_id("workspace.surface.overlay"),
                SurfaceKind::overlay_content(),
                component_id("workspace.component.overlay"),
                SurfacePlacementClass::overlay_layer(),
                SurfaceStateClass::ephemeral(),
            )
            .with_label("Overlay"),
        )
        .freeze();

    let descriptor = app
        .capabilities()
        .surfaces()
        .get(&surface_id("workspace.surface.overlay"))
        .expect("registered surface");
    assert_eq!(descriptor.kind(), &SurfaceKind::overlay_content());
    assert_eq!(
        descriptor.placement_class(),
        &SurfacePlacementClass::overlay_layer()
    );
    assert_eq!(descriptor.state_class(), &SurfaceStateClass::ephemeral());
    assert_eq!(descriptor.label(), Some("Overlay"));
}
