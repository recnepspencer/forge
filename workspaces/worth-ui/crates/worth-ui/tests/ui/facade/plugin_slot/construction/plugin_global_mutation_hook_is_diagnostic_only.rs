use worth_ui::facade::{
    registry::{PluginContributionFamily, PluginSlotDescriptor, PluginSlotGlobalMutationHook, PluginSlotId},
};

fn main() {
    let _ = PluginSlotDescriptor::new(
        PluginSlotId::new("workspace.plugin_slot.global_mutation").expect("valid slot id"),
    )
    .allow_family(PluginContributionFamily::global_mutation_hook())
    .with_global_mutation_hook(PluginSlotGlobalMutationHook::opaque_callback());
}
