use worth_ui::facade::{
    CapabilityDiagnosticCode, PluginContributionFamily, PluginSlotDescriptor,
    PluginSlotDiagnostics, PluginSlotGlobalMutationHook, PluginSlotOrdering,
    PluginSlotSupportPosture, WorthUi,
};

use super::plugin_slot_assertions::assert_diagnostic_codes;
use super::plugin_slot_fixtures::{plugin_slot, plugin_slot_id};

#[test]
fn plugin_contribution_to_unsupported_family_rejected() {
    let report = WorthUi::app()
        .register_plugin_slot(
            plugin_slot("workspace.plugin_slot.unsupported").allow_family(
                PluginContributionFamily::unsupported_for_diagnostics("ad_hoc_renderer"),
            ),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().plugin_slots().is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::UnsupportedPluginContributionFamily],
    );
}

#[test]
fn plugin_contribution_without_permission_rejected() {
    let report = WorthUi::app()
        .register_plugin_slot(
            PluginSlotDescriptor::new(plugin_slot_id("workspace.plugin_slot.commands"))
                .allow_family(PluginContributionFamily::command())
                .with_ordering(PluginSlotOrdering::stable_by_plugin_then_declaration())
                .with_diagnostics(PluginSlotDiagnostics::explain_contributions())
                .with_support(PluginSlotSupportPosture::supported()),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::MissingPluginSlotPermission],
    );
}

#[test]
fn plugin_slot_cannot_be_arbitrary_global_mutation_hook() {
    let report = WorthUi::app()
        .register_plugin_slot(
            plugin_slot("workspace.plugin_slot.global_mutation")
                .allow_family(
                    PluginContributionFamily::arbitrary_global_mutation_hook_for_diagnostics(),
                )
                .with_global_mutation_hook_for_diagnostics(
                    PluginSlotGlobalMutationHook::opaque_callback_for_diagnostics(),
                ),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[
            CapabilityDiagnosticCode::UnsupportedPluginContributionFamily,
            CapabilityDiagnosticCode::PluginSlotArbitraryGlobalMutationHook,
        ],
    );
}

#[test]
fn plugin_slot_reports_all_missing_required_postures() {
    let report = WorthUi::app()
        .register_plugin_slot(PluginSlotDescriptor::new(plugin_slot_id(
            "workspace.plugin_slot.empty",
        )))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[
            CapabilityDiagnosticCode::MissingPluginSlotContributionFamily,
            CapabilityDiagnosticCode::MissingPluginSlotPermission,
            CapabilityDiagnosticCode::MissingPluginSlotOrdering,
            CapabilityDiagnosticCode::MissingPluginSlotDiagnostics,
            CapabilityDiagnosticCode::MissingPluginSlotSupportPosture,
        ],
    );
}

#[test]
fn plugin_slot_support_posture_participates_in_snapshot_digest() {
    let supported = WorthUi::app()
        .register_plugin_slot(plugin_slot("workspace.plugin_slot.commands"))
        .freeze()
        .expect("application preparation should succeed");
    let deferred = WorthUi::app()
        .register_plugin_slot(
            plugin_slot("workspace.plugin_slot.commands")
                .with_support(PluginSlotSupportPosture::deferred()),
        )
        .freeze()
        .expect("application preparation should succeed");

    assert_ne!(
        supported.capabilities().plugin_slots(),
        deferred.capabilities().plugin_slots()
    );
    assert_ne!(
        supported.capabilities().digest(),
        deferred.capabilities().digest()
    );
}
