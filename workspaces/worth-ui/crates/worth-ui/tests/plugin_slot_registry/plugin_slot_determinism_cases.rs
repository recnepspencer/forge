use worth_ui::facade::{
    app::WorthUi,
    declaration::{
        PluginCapabilityPermission, PluginContributionFamily, PluginSlotDescriptor,
        PluginSlotDiagnostics, PluginSlotOrdering, PluginSlotSupportPosture,
    },
};

use super::plugin_slot_assertions::assert_registered_plugin_slot_ids;
use super::plugin_slot_fixtures::{plugin_slot, plugin_slot_id};

#[test]
fn equivalent_plugin_slots_produce_equivalent_admitted_families() {
    let first = WorthUi::app()
        .register_plugin_slot(plugin_slot("workspace.plugin_slot.commands"))
        .register_plugin_slot(plugin_slot("workspace.plugin_slot.views"))
        .freeze()
        .expect("application preparation should succeed");
    let second = WorthUi::app()
        .register_plugin_slot(plugin_slot("workspace.plugin_slot.views"))
        .register_plugin_slot(plugin_slot("workspace.plugin_slot.commands"))
        .freeze()
        .expect("application preparation should succeed");

    assert_eq!(
        first.capabilities().plugin_slots(),
        second.capabilities().plugin_slots()
    );
    assert_eq!(
        first.capabilities().digest(),
        second.capabilities().digest()
    );
    assert_registered_plugin_slot_ids(
        first.capabilities().plugin_slots(),
        &[
            "workspace.plugin_slot.commands",
            "workspace.plugin_slot.views",
        ],
    );
}

#[test]
fn equivalent_plugin_slot_families_are_canonicalized() {
    let single = WorthUi::app()
        .register_plugin_slot(
            PluginSlotDescriptor::new(plugin_slot_id("workspace.plugin_slot.inspectors"))
                .allow_family(PluginContributionFamily::command())
                .allow_family(PluginContributionFamily::component())
                .with_permission(PluginCapabilityPermission::host_granted())
                .with_ordering(PluginSlotOrdering::stable_by_plugin_then_declaration())
                .with_diagnostics(PluginSlotDiagnostics::explain_contributions())
                .with_support(PluginSlotSupportPosture::supported()),
        )
        .freeze()
        .expect("application preparation should succeed");
    let reordered_with_duplicate = WorthUi::app()
        .register_plugin_slot(
            PluginSlotDescriptor::new(plugin_slot_id("workspace.plugin_slot.inspectors"))
                .allow_family(PluginContributionFamily::component())
                .allow_family(PluginContributionFamily::command())
                .allow_family(PluginContributionFamily::command())
                .with_permission(PluginCapabilityPermission::host_granted())
                .with_ordering(PluginSlotOrdering::stable_by_plugin_then_declaration())
                .with_diagnostics(PluginSlotDiagnostics::explain_contributions())
                .with_support(PluginSlotSupportPosture::supported()),
        )
        .freeze()
        .expect("application preparation should succeed");

    assert_eq!(
        single.capabilities().plugin_slots(),
        reordered_with_duplicate.capabilities().plugin_slots()
    );
    assert_eq!(
        single.capabilities().digest(),
        reordered_with_duplicate.capabilities().digest()
    );
    assert_eq!(
        single
            .capabilities()
            .plugin_slots()
            .entries()
            .first()
            .expect("slot")
            .descriptor()
            .allowed_families()
            .to_vec(),
        vec![
            PluginContributionFamily::command(),
            PluginContributionFamily::component(),
        ]
    );
}

#[test]
fn all_domain_agnostic_builtin_plugin_contribution_families_are_admitted() {
    let app = WorthUi::app()
        .register_plugin_slot(all_builtin_contribution_family_slot())
        .freeze()
        .expect("application preparation should succeed");

    assert_eq!(app.capabilities().plugin_slots().len(), 1);
    assert_eq!(
        app.capabilities()
            .plugin_slots()
            .entries()
            .first()
            .expect("slot")
            .descriptor()
            .allowed_families()
            .to_vec(),
        all_builtin_contribution_families()
    );
}

#[test]
fn different_plugin_slot_permission_changes_snapshot_digest() {
    let host_granted = WorthUi::app()
        .register_plugin_slot(plugin_slot("workspace.plugin_slot.commands"))
        .freeze()
        .expect("application preparation should succeed");
    let user_consent = WorthUi::app()
        .register_plugin_slot(
            plugin_slot("workspace.plugin_slot.commands")
                .with_permission(PluginCapabilityPermission::user_consent()),
        )
        .freeze()
        .expect("application preparation should succeed");

    assert_ne!(
        host_granted.capabilities().plugin_slots(),
        user_consent.capabilities().plugin_slots()
    );
    assert_ne!(
        host_granted.capabilities().digest(),
        user_consent.capabilities().digest()
    );
}

fn all_builtin_contribution_family_slot() -> PluginSlotDescriptor {
    all_builtin_contribution_families()
        .into_iter()
        .fold(
            PluginSlotDescriptor::new(plugin_slot_id("workspace.plugin_slot.all_builtin_families")),
            PluginSlotDescriptor::allow_family,
        )
        .with_permission(PluginCapabilityPermission::host_granted())
        .with_ordering(PluginSlotOrdering::stable_by_plugin_then_declaration())
        .with_diagnostics(PluginSlotDiagnostics::explain_contributions())
        .with_support(PluginSlotSupportPosture::supported())
}

fn all_builtin_contribution_families() -> Vec<PluginContributionFamily> {
    vec![
        PluginContributionFamily::command(),
        PluginContributionFamily::component(),
        PluginContributionFamily::surface(),
        PluginContributionFamily::setting(),
        PluginContributionFamily::view_binding(),
        PluginContributionFamily::theme_token(),
        PluginContributionFamily::icon(),
        PluginContributionFamily::command_projection(),
        PluginContributionFamily::task_presentation(),
        PluginContributionFamily::runtime_outcome_projection(),
        PluginContributionFamily::native_capability_request(),
    ]
}
