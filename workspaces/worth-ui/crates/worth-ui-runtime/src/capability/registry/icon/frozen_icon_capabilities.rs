use crate::capability::IconId;

use super::{FrozenIconEntry, IconAcceptedRegistrationProof, IconDescriptor, IconKey};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenIconCapabilities {
    entries: Vec<FrozenIconEntry>,
}

impl FrozenIconCapabilities {
    #[cfg(test)]
    pub(crate) fn empty() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub(crate) fn from_accepted_descriptors(
        mut descriptors: Vec<IconDescriptor>,
        accepted_icons: &IconAcceptedRegistrationProof,
    ) -> Self {
        descriptors.retain(|descriptor| accepted_icons.admits(descriptor));
        descriptors.sort_by(|left, right| left.id().cmp(right.id()));
        let entries = descriptors.into_iter().map(frozen_icon_entry).collect();
        Self { entries }
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn entries(&self) -> &[FrozenIconEntry] {
        &self.entries
    }

    pub fn get(&self, id: &IconId) -> Option<&IconDescriptor> {
        self.entries
            .binary_search_by(|entry| entry.descriptor().id().cmp(id))
            .ok()
            .map(|index| self.entries[index].descriptor())
    }

    pub(crate) fn digest_basis(&self) -> u64 {
        self.entries
            .iter()
            .fold(0x1c0f_5a77_d3e2_4491, |basis, entry| {
                fold_bytes(basis, entry.key().projection_basis().as_bytes())
            })
    }
}

fn frozen_icon_entry(descriptor: IconDescriptor) -> FrozenIconEntry {
    let key = IconKey::from_descriptor(&descriptor);
    FrozenIconEntry::new(descriptor, key)
}

fn fold_bytes(mut accumulator: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        accumulator ^= u64::from(*byte);
        accumulator = accumulator.wrapping_mul(0x0000_0100_0000_01b3);
    }
    accumulator
}
