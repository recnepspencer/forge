use worth_ui::facade::{
    PluginCapabilityPermission, PluginContributionFamily, PluginSlotDescriptor,
    PluginSlotDiagnostics, PluginSlotId, PluginSlotOrdering, PluginSlotSupportPosture, WorthUi,
};

fn main() {
    let slot_id = PluginSlotId::new("workspace.plugin_slot.commands").expect("valid slot id");
    let app = WorthUi::app()
        .register_plugin_slot(
            PluginSlotDescriptor::new(slot_id.clone())
                .allow_family(PluginContributionFamily::command())
                .allow_family(PluginContributionFamily::component())
                .with_permission(PluginCapabilityPermission::host_granted())
                .with_ordering(PluginSlotOrdering::stable_by_plugin_then_declaration())
                .with_diagnostics(PluginSlotDiagnostics::explain_contributions())
                .with_support(PluginSlotSupportPosture::supported()),
        )
        .freeze();

    let _ = app.capabilities().plugin_slots().get(&slot_id);
}
