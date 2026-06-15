use crate::capability::PluginSlotId;

use super::{
    FrozenPluginSlotEntry, PluginSlotAcceptedRegistrationProof, PluginSlotDescriptor, PluginSlotKey,
};

/// Canonical frozen plugin contribution-slot capability index.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenPluginSlotCapabilities {
    entries: Vec<FrozenPluginSlotEntry>,
}

impl FrozenPluginSlotCapabilities {
    pub(crate) fn empty() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub(crate) fn from_accepted_descriptors(
        mut descriptors: Vec<PluginSlotDescriptor>,
        accepted_slots: &PluginSlotAcceptedRegistrationProof,
    ) -> Self {
        descriptors.retain(|descriptor| accepted_slots.admits(descriptor));
        descriptors.sort_by(|left, right| left.id().cmp(right.id()));
        let entries = descriptors
            .into_iter()
            .map(PluginSlotDescriptor::canonicalized_for_freeze)
            .map(frozen_plugin_slot_entry)
            .collect();
        Self { entries }
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn entries(&self) -> &[FrozenPluginSlotEntry] {
        &self.entries
    }

    pub fn get(&self, id: &PluginSlotId) -> Option<&PluginSlotDescriptor> {
        self.entries
            .binary_search_by(|entry| entry.descriptor().id().cmp(id))
            .ok()
            .map(|index| self.entries[index].descriptor())
    }

    pub(crate) fn digest_basis(&self) -> u64 {
        self.entries
            .iter()
            .fold(0x9d1a_8a6f_e02f_1c73, |basis, entry| {
                fold_bytes(basis, entry.key().admission_basis().as_bytes())
            })
    }
}

fn frozen_plugin_slot_entry(descriptor: PluginSlotDescriptor) -> FrozenPluginSlotEntry {
    let key = PluginSlotKey::from_descriptor(&descriptor);
    FrozenPluginSlotEntry::new(descriptor, key)
}

fn fold_bytes(mut accumulator: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        accumulator ^= u64::from(*byte);
        accumulator = accumulator.wrapping_mul(0x0000_0100_0000_01b3);
    }
    accumulator
}
