use worth_ui::facade::{
    declaration::{CommandCategory, CommandDescriptor, CommandId, PluginCapabilityPermission, PluginSlotDescriptor, PluginSlotDiagnostics, PluginSlotId, PluginSlotOrdering, PluginSlotSupportPosture},
};

fn main() {
    let _command = CommandDescriptor {
        id: CommandId::new("workspace.save").expect("valid command id"),
        label: String::from("Save"),
        description: None,
        icon: None,
        default_shortcut_reference: None,
        category: CommandCategory::Application,
        projection_eligibility: None,
    };

    let _plugin_slot = PluginSlotDescriptor {
        id: PluginSlotId::new("workspace.plugin_slot.commands").expect("valid plugin slot id"),
        allowed_families: Vec::new(),
        permission: Some(PluginCapabilityPermission::host_granted()),
        ordering: Some(PluginSlotOrdering::stable_by_plugin_then_declaration()),
        diagnostics: Some(PluginSlotDiagnostics::explain_contributions()),
        support: Some(PluginSlotSupportPosture::supported()),
        contribution_reference: None,
        global_mutation_hooks: Vec::new(),
    };
}
