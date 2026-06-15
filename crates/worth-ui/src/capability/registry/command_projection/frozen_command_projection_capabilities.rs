use crate::capability::CommandProjectionId;

use super::{
    CommandProjectionAcceptedRegistrationProof, CommandProjectionDescriptor, CommandProjectionKey,
    FrozenCommandProjectionEntry,
};

/// Canonical frozen command projection capability index.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenCommandProjectionCapabilities {
    entries: Vec<FrozenCommandProjectionEntry>,
}

impl FrozenCommandProjectionCapabilities {
    pub(crate) fn empty() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub(crate) fn from_accepted_descriptors(
        mut descriptors: Vec<CommandProjectionDescriptor>,
        accepted_projections: &CommandProjectionAcceptedRegistrationProof,
    ) -> Self {
        descriptors.retain(|descriptor| accepted_projections.admits(descriptor));
        descriptors.sort_by(|left, right| left.id().cmp(right.id()));
        let entries = descriptors
            .into_iter()
            .map(CommandProjectionDescriptor::canonicalized_for_freeze)
            .map(frozen_entry)
            .collect();
        Self { entries }
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn entries(&self) -> &[FrozenCommandProjectionEntry] {
        &self.entries
    }

    pub fn get(&self, id: &CommandProjectionId) -> Option<&CommandProjectionDescriptor> {
        self.entries
            .binary_search_by(|entry| entry.descriptor().id().cmp(id))
            .ok()
            .map(|index| self.entries[index].descriptor())
    }

    pub(crate) fn digest_basis(&self) -> u64 {
        self.entries
            .iter()
            .fold(0x45c9_1894_92f8_7d15, |basis, entry| {
                fold_bytes(basis, entry.key().projection_basis().as_bytes())
            })
    }
}

fn frozen_entry(descriptor: CommandProjectionDescriptor) -> FrozenCommandProjectionEntry {
    let key = CommandProjectionKey::from_descriptor(&descriptor);
    FrozenCommandProjectionEntry::new(descriptor, key)
}

fn fold_bytes(mut accumulator: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        accumulator ^= u64::from(*byte);
        accumulator = accumulator.wrapping_mul(0x0000_0100_0000_01b3);
    }
    accumulator
}
