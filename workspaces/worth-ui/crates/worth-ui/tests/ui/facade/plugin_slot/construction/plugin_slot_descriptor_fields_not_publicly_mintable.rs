use worth_ui::facade::{
    registry::{PluginCapabilityPermission, PluginSlotDescriptor, PluginSlotDiagnostics, PluginSlotId, PluginSlotOrdering, PluginSlotSupportPosture},
};

fn main() {
    let _ = PluginSlotDescriptor {
        id: PluginSlotId::new("workspace.plugin_slot.commands").expect("valid slot id"),
        allowed_families: Vec::new(),
        permission: Some(PluginCapabilityPermission::host_granted()),
        ordering: Some(PluginSlotOrdering::stable_by_plugin_then_declaration()),
        diagnostics: Some(PluginSlotDiagnostics::explain_contributions()),
        support: Some(PluginSlotSupportPosture::supported()),
        contribution_reference: None,
        global_mutation_hooks: Vec::new(),
    };
}
