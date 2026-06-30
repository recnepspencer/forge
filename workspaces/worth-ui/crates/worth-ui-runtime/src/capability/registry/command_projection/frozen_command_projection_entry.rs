use super::{CommandProjectionDescriptor, CommandProjectionKey};

/// Frozen command projection entry with derived projection key.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenCommandProjectionEntry {
    descriptor: CommandProjectionDescriptor,
    key: CommandProjectionKey,
}

impl FrozenCommandProjectionEntry {
    pub(crate) fn new(descriptor: CommandProjectionDescriptor, key: CommandProjectionKey) -> Self {
        Self { descriptor, key }
    }

    pub fn descriptor(&self) -> &CommandProjectionDescriptor {
        &self.descriptor
    }

    pub fn key(&self) -> &CommandProjectionKey {
        &self.key
    }
}
