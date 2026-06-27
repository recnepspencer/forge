mod descriptor;
mod frozen_plugin_slot_capabilities;
mod frozen_plugin_slot_entry;
mod plugin_slot_key;
mod plugin_slot_registry;
mod registration;

pub use descriptor::{
    PluginCapabilityPermission, PluginContributionFamily, PluginSlotContributionReference,
    PluginSlotDescriptor, PluginSlotDiagnostics, PluginSlotGlobalMutationHook, PluginSlotOrdering,
    PluginSlotSupportPosture,
};
pub use frozen_plugin_slot_capabilities::FrozenPluginSlotCapabilities;
pub use frozen_plugin_slot_entry::FrozenPluginSlotEntry;
pub use plugin_slot_key::PluginSlotKey;
pub(crate) use plugin_slot_registry::PluginSlotRegistry;
pub(crate) use registration::PluginSlotAcceptedRegistrationProof;
