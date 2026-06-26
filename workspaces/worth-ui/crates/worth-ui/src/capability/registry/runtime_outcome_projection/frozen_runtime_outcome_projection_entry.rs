use super::{RuntimeOutcomeProjectionDescriptor, RuntimeOutcomeProjectionKey};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenRuntimeOutcomeProjectionEntry {
    descriptor: RuntimeOutcomeProjectionDescriptor,
    key: RuntimeOutcomeProjectionKey,
}

impl FrozenRuntimeOutcomeProjectionEntry {
    pub(crate) fn new(
        descriptor: RuntimeOutcomeProjectionDescriptor,
        key: RuntimeOutcomeProjectionKey,
    ) -> Self {
        Self { descriptor, key }
    }

    pub fn descriptor(&self) -> &RuntimeOutcomeProjectionDescriptor {
        &self.descriptor
    }

    pub fn key(&self) -> &RuntimeOutcomeProjectionKey {
        &self.key
    }
}
