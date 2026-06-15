use super::{PluginSlotDescriptor, PluginSlotKey};

/// Frozen plugin slot entry with derived contribution admission key.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenPluginSlotEntry {
    descriptor: PluginSlotDescriptor,
    key: PluginSlotKey,
}

impl FrozenPluginSlotEntry {
    pub(crate) fn new(descriptor: PluginSlotDescriptor, key: PluginSlotKey) -> Self {
        Self { descriptor, key }
    }

    pub fn descriptor(&self) -> &PluginSlotDescriptor {
        &self.descriptor
    }

    pub fn key(&self) -> &PluginSlotKey {
        &self.key
    }
}
