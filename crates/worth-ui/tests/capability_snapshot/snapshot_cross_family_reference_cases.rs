use worth_ui::facade::{CapabilityDiagnosticCode, SnapshotReferenceViolationKind, WorthUi};

use super::snapshot_assertions::{diagnostic_codes, violation_kinds};
use super::snapshot_fixtures::{
    command_id, command_with_icon, component, deferred_native_capability, deferred_plugin_slot,
    icon_id, plugin_slot_referencing,
};

#[test]
fn snapshot_missing_cross_family_reference_rejected() {
    let report = WorthUi::app()
        .register_command(command_with_icon("command.save", "icon.missing"))
        .register_component(
            component("component.editor").with_command_binding_slot(command_id("command.save")),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_eq!(
        diagnostic_codes(report.registration_diagnostics()),
        vec![CapabilityDiagnosticCode::MissingDependency]
    );
    assert_eq!(
        violation_kinds(report.accepted_snapshot().validation_summary()),
        vec![SnapshotReferenceViolationKind::MissingCrossFamilyReference]
    );
    assert!(
        report
            .accepted_snapshot()
            .index()
            .icons()
            .lookup(&icon_id("icon.missing"))
            .counters()
            .families_scanned()
            == 0
    );
}

#[test]
fn snapshot_deferred_entry_used_as_admitted_rejected() {
    let app = WorthUi::app()
        .register_plugin_slot(deferred_plugin_slot("plugin.slot.deferred"))
        .register_plugin_slot(plugin_slot_referencing(
            "plugin.slot.consumer",
            "plugin.slot.deferred",
        ))
        .freeze();

    assert_eq!(
        violation_kinds(app.capabilities().validation_summary()),
        vec![SnapshotReferenceViolationKind::DeferredEntryUsedAsAdmitted]
    );
    assert!(!app
        .capabilities()
        .validation_summary()
        .lowering_is_admissible());
}

#[test]
fn snapshot_deferred_entry_not_used_as_admitted_remains_later_lowering_vocabulary() {
    let app = WorthUi::app()
        .register_native_capability(deferred_native_capability("platform.native.clipboard"))
        .freeze();

    assert!(app
        .capabilities()
        .validation_summary()
        .lowering_is_admissible());
}
