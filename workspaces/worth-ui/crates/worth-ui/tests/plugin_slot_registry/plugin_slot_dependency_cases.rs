use worth_ui::facade::{
    app::WorthUi,
    declaration::{PluginSlotContributionReference, PluginSlotDescriptor},
    diagnostics::CapabilityDiagnosticCode,
};

use super::plugin_slot_assertions::{
    assert_dependency_diagnostics, assert_diagnostic_codes, assert_registered_plugin_slot_ids,
};
use super::plugin_slot_fixtures::{plugin_slot, plugin_slot_id};

#[test]
fn plugin_contribution_to_unknown_slot_rejected() {
    let report = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_plugin_slot(
            plugin_slot("workspace.plugin_contribution.inspectors").with_contribution_reference(
                PluginSlotContributionReference::slot(plugin_slot_id(
                    "workspace.plugin_slot.inspectors",
                )),
            ),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().plugin_slots().is_empty());
    assert_dependency_diagnostics(
        report.registration_diagnostics(),
        &[(
            "workspace.plugin_contribution.inspectors",
            "plugin_slot",
            "workspace.plugin_slot.inspectors",
        )],
    );
}

#[test]
fn plugin_contribution_slot_reference_resolves_against_registered_slot() {
    let app = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_plugin_slot(plugin_slot("workspace.plugin_slot.inspectors"))
        .register_plugin_slot(
            plugin_slot("workspace.plugin_contribution.inspectors").with_contribution_reference(
                PluginSlotContributionReference::slot(plugin_slot_id(
                    "workspace.plugin_slot.inspectors",
                )),
            ),
        )
        .freeze()
        .expect("application preparation should succeed");

    assert_registered_plugin_slot_ids(
        app.capabilities().plugin_slots(),
        &[
            "workspace.plugin_contribution.inspectors",
            "workspace.plugin_slot.inspectors",
        ],
    );
}

#[test]
fn plugin_contribution_reference_to_duplicate_slot_target_rejected() {
    let report = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_plugin_slot(plugin_slot("workspace.plugin_slot.inspectors"))
        .register_plugin_slot(plugin_slot("workspace.plugin_slot.inspectors"))
        .register_plugin_slot(
            plugin_slot("workspace.plugin_contribution.inspectors").with_contribution_reference(
                PluginSlotContributionReference::slot(plugin_slot_id(
                    "workspace.plugin_slot.inspectors",
                )),
            ),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().plugin_slots().is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[
            CapabilityDiagnosticCode::DuplicateCapabilityId,
            CapabilityDiagnosticCode::DuplicateCapabilityId,
            CapabilityDiagnosticCode::MissingDependency,
        ],
    );
    assert_dependency_diagnostics(
        report.registration_diagnostics(),
        &[(
            "workspace.plugin_contribution.inspectors",
            "plugin_slot",
            "workspace.plugin_slot.inspectors",
        )],
    );
}

#[test]
fn invalid_plugin_contribution_does_not_poison_valid_slot() {
    let report = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_plugin_slot(plugin_slot("workspace.plugin_slot.commands"))
        .register_plugin_slot(PluginSlotDescriptor::new(plugin_slot_id(
            "workspace.plugin_slot.empty",
        )))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_registered_plugin_slot_ids(
        report.accepted_snapshot().plugin_slots(),
        &["workspace.plugin_slot.commands"],
    );
}
