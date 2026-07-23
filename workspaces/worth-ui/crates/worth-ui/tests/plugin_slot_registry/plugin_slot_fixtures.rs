use worth_ui::facade::registry::{
    PluginCapabilityPermission, PluginContributionFamily, PluginSlotDescriptor,
    PluginSlotDiagnostics, PluginSlotId, PluginSlotOrdering, PluginSlotSupportPosture,
};

pub(crate) fn plugin_slot(id: &str) -> PluginSlotDescriptor {
    PluginSlotDescriptor::new(plugin_slot_id(id))
        .allow_family(PluginContributionFamily::command())
        .with_permission(PluginCapabilityPermission::host_granted())
        .with_ordering(PluginSlotOrdering::stable_by_plugin_then_declaration())
        .with_diagnostics(PluginSlotDiagnostics::explain_contributions())
        .with_support(PluginSlotSupportPosture::supported())
}

pub(crate) fn plugin_slot_id(raw_text: &str) -> PluginSlotId {
    PluginSlotId::new(raw_text).expect("valid plugin slot id")
}
