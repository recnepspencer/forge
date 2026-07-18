use worth_ui::facade::WorthUi;

use super::icon_assertions::{assert_dependency_diagnostics, assert_registered_icon_ids};
use super::icon_fixtures::{
    command_descriptor, command_icon, component_descriptor, denied_projection_with_icon, icon_id,
    runtime_outcome_icon, surface_descriptor, surface_icon, surface_id,
};

#[test]
fn command_references_missing_icon_rejected() {
    let report = WorthUi::app()
        .register_command(
            command_descriptor("workspace.command.save", "Save")
                .with_icon(icon_id("workspace.icon.save")),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().commands().is_empty());
    assert_dependency_diagnostics(
        report.registration_diagnostics(),
        &[("workspace.command.save", "icon", "workspace.icon.save")],
    );
}

#[test]
fn command_icon_reference_resolves_against_registered_icon() {
    let app = WorthUi::app()
        .register_icon(command_icon("workspace.icon.save"))
        .register_command(
            command_descriptor("workspace.command.save", "Save")
                .with_icon(icon_id("workspace.icon.save")),
        )
        .freeze()
        .expect("application preparation should succeed");

    assert_registered_icon_ids(app.capabilities().icons(), &["workspace.icon.save"]);
    assert_eq!(app.capabilities().commands().len(), 1);
}

#[test]
fn surface_references_missing_icon_rejected() {
    let report = WorthUi::app()
        .register_component(component_descriptor("workspace.component.editor"))
        .register_surface(
            surface_descriptor("workspace.surface.editor", "workspace.component.editor")
                .with_icon(icon_id("workspace.icon.surface")),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().surfaces().is_empty());
    assert_dependency_diagnostics(
        report.registration_diagnostics(),
        &[("workspace.surface.editor", "icon", "workspace.icon.surface")],
    );
}

#[test]
fn surface_icon_reference_survives_freeze_when_registered() {
    let app = WorthUi::app()
        .register_icon(surface_icon("workspace.icon.surface"))
        .register_component(component_descriptor("workspace.component.editor"))
        .register_surface(
            surface_descriptor("workspace.surface.editor", "workspace.component.editor")
                .with_icon(icon_id("workspace.icon.surface")),
        )
        .freeze()
        .expect("application preparation should succeed");

    let descriptor = app
        .capabilities()
        .surfaces()
        .get(&surface_id("workspace.surface.editor"))
        .expect("registered surface");
    assert_eq!(descriptor.icon(), Some(&icon_id("workspace.icon.surface")));
}

#[test]
fn runtime_outcome_projection_references_missing_icon_rejected() {
    let report = WorthUi::app()
        .register_runtime_outcome_projection(denied_projection_with_icon(
            "workspace.outcome.denied",
            "workspace.icon.denied",
        ))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report
        .accepted_snapshot()
        .runtime_outcome_projections()
        .is_empty());
    assert_dependency_diagnostics(
        report.registration_diagnostics(),
        &[("workspace.outcome.denied", "icon", "workspace.icon.denied")],
    );
}

#[test]
fn rejected_icon_dependency_does_not_poison_valid_icons() {
    let report = WorthUi::app()
        .register_icon(command_icon("workspace.icon.valid"))
        .register_command(
            command_descriptor("workspace.command.missing", "Missing")
                .with_icon(icon_id("workspace.icon.missing")),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_registered_icon_ids(
        report.accepted_snapshot().icons(),
        &["workspace.icon.valid"],
    );
    assert_dependency_diagnostics(
        report.registration_diagnostics(),
        &[(
            "workspace.command.missing",
            "icon",
            "workspace.icon.missing",
        )],
    );
}

#[test]
fn runtime_outcome_icon_reference_resolves_when_registered() {
    let app = WorthUi::app()
        .register_icon(runtime_outcome_icon("workspace.icon.denied"))
        .register_runtime_outcome_projection(denied_projection_with_icon(
            "workspace.outcome.denied",
            "workspace.icon.denied",
        ))
        .freeze()
        .expect("application preparation should succeed");

    assert_eq!(app.capabilities().runtime_outcome_projections().len(), 1);
}
