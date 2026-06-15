use crate::capability::NativeCapabilityId;

use super::{
    FrozenNativeCapabilityEntry, NativeCapabilityAcceptedRegistrationProof,
    NativeCapabilityDescriptor, NativeCapabilityKey,
};

/// Canonical frozen native capability support index.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenNativeCapabilities {
    entries: Vec<FrozenNativeCapabilityEntry>,
}

impl FrozenNativeCapabilities {
    pub(crate) fn empty() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub(crate) fn from_accepted_descriptors(
        mut descriptors: Vec<NativeCapabilityDescriptor>,
        accepted_native_capabilities: &NativeCapabilityAcceptedRegistrationProof,
    ) -> Self {
        descriptors.retain(|descriptor| accepted_native_capabilities.admits(descriptor));
        descriptors.sort_by(|left, right| left.id().cmp(right.id()));
        let entries = descriptors
            .into_iter()
            .map(NativeCapabilityDescriptor::canonicalized_for_freeze)
            .map(frozen_native_capability_entry)
            .collect();
        Self { entries }
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn entries(&self) -> &[FrozenNativeCapabilityEntry] {
        &self.entries
    }

    pub fn get(&self, id: &NativeCapabilityId) -> Option<&NativeCapabilityDescriptor> {
        self.entries
            .binary_search_by(|entry| entry.descriptor().id().cmp(id))
            .ok()
            .map(|index| self.entries[index].descriptor())
    }

    pub(crate) fn digest_basis(&self) -> u64 {
        self.entries
            .iter()
            .fold(0x6c62_272e_07bb_0142, |basis, entry| {
                fold_bytes(basis, entry.key().support_basis().as_bytes())
            })
    }
}

fn frozen_native_capability_entry(
    descriptor: NativeCapabilityDescriptor,
) -> FrozenNativeCapabilityEntry {
    let key = NativeCapabilityKey::from_descriptor(&descriptor);
    FrozenNativeCapabilityEntry::new(descriptor, key)
}

fn fold_bytes(mut accumulator: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        accumulator ^= u64::from(*byte);
        accumulator = accumulator.wrapping_mul(0x0000_0100_0000_01b3);
    }
    accumulator
}
