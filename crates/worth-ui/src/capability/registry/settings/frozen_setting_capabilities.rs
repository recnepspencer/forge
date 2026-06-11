use crate::capability::SettingId;

use super::{FrozenSettingEntry, SettingAcceptedRegistrationProof, SettingDescriptor, SettingKey};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenSettingCapabilities {
    entries: Vec<FrozenSettingEntry>,
}

impl FrozenSettingCapabilities {
    pub(crate) fn empty() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub(crate) fn from_accepted_descriptors(
        mut descriptors: Vec<SettingDescriptor>,
        accepted_settings: &SettingAcceptedRegistrationProof,
    ) -> Self {
        descriptors.retain(|descriptor| accepted_settings.admits(descriptor));
        descriptors.sort_by(|left, right| left.id().cmp(right.id()));
        let entries = descriptors.into_iter().map(frozen_setting_entry).collect();
        Self { entries }
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn entries(&self) -> &[FrozenSettingEntry] {
        &self.entries
    }

    pub fn get(&self, id: &SettingId) -> Option<&SettingDescriptor> {
        self.entries
            .binary_search_by(|entry| entry.descriptor().id().cmp(id))
            .ok()
            .map(|index| self.entries[index].descriptor())
    }

    pub(crate) fn digest_basis(&self) -> u64 {
        self.entries
            .iter()
            .fold(0xb21f_57d0_8a44_9e23, |basis, entry| {
                fold_setting_key(basis, entry.key())
            })
    }
}

fn frozen_setting_entry(descriptor: SettingDescriptor) -> FrozenSettingEntry {
    let key = SettingKey::from_descriptor(&descriptor);
    FrozenSettingEntry::new(descriptor, key)
}

fn fold_setting_key(accumulator: u64, key: &SettingKey) -> u64 {
    fold_bytes(accumulator, key.configuration_basis().as_bytes())
}

fn fold_bytes(mut accumulator: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        accumulator ^= u64::from(*byte);
        accumulator = accumulator.wrapping_mul(0x0000_0100_0000_01b3);
    }
    accumulator
}
