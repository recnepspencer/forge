use super::{SettingDescriptor, SettingKey};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenSettingEntry {
    descriptor: SettingDescriptor,
    key: SettingKey,
}

impl FrozenSettingEntry {
    pub(crate) fn new(descriptor: SettingDescriptor, key: SettingKey) -> Self {
        Self { descriptor, key }
    }

    pub fn descriptor(&self) -> &SettingDescriptor {
        &self.descriptor
    }

    pub fn key(&self) -> &SettingKey {
        &self.key
    }
}
