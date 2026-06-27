use super::{NativeCapabilityDescriptor, NativeCapabilityKey};

/// Frozen native capability entry with derived platform-support key.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenNativeCapabilityEntry {
    descriptor: NativeCapabilityDescriptor,
    key: NativeCapabilityKey,
}

impl FrozenNativeCapabilityEntry {
    pub(crate) fn new(descriptor: NativeCapabilityDescriptor, key: NativeCapabilityKey) -> Self {
        Self { descriptor, key }
    }

    pub fn descriptor(&self) -> &NativeCapabilityDescriptor {
        &self.descriptor
    }

    pub fn key(&self) -> &NativeCapabilityKey {
        &self.key
    }
}
