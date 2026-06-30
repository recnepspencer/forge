use super::{IconDescriptor, IconKey};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenIconEntry {
    descriptor: IconDescriptor,
    key: IconKey,
}

impl FrozenIconEntry {
    pub(crate) fn new(descriptor: IconDescriptor, key: IconKey) -> Self {
        Self { descriptor, key }
    }

    pub fn descriptor(&self) -> &IconDescriptor {
        &self.descriptor
    }

    pub fn key(&self) -> &IconKey {
        &self.key
    }
}
