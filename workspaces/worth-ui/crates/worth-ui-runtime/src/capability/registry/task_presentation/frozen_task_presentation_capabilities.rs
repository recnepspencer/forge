use crate::capability::TaskPresentationId;

use super::{
    FrozenTaskPresentationEntry, TaskPresentationAcceptedRegistrationProof,
    TaskPresentationDescriptor, TaskPresentationKey,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenTaskPresentationCapabilities {
    entries: Vec<FrozenTaskPresentationEntry>,
}

impl FrozenTaskPresentationCapabilities {
    #[cfg(test)]
    pub(crate) fn empty() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub(crate) fn from_accepted_descriptors(
        mut descriptors: Vec<TaskPresentationDescriptor>,
        accepted_task_presentations: &TaskPresentationAcceptedRegistrationProof,
    ) -> Self {
        descriptors.retain(|descriptor| accepted_task_presentations.admits(descriptor));
        descriptors.sort_by(|left, right| left.id().cmp(right.id()));
        let entries = descriptors
            .into_iter()
            .map(frozen_task_presentation_entry)
            .collect();
        Self { entries }
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn entries(&self) -> &[FrozenTaskPresentationEntry] {
        &self.entries
    }

    pub fn get(&self, id: &TaskPresentationId) -> Option<&TaskPresentationDescriptor> {
        self.entries
            .binary_search_by(|entry| entry.descriptor().id().cmp(id))
            .ok()
            .map(|index| self.entries[index].descriptor())
    }

    pub(crate) fn digest_basis(&self) -> u64 {
        self.entries
            .iter()
            .fold(0xf0a7_2d19_ba55_6c31, |basis, entry| {
                fold_task_presentation_key(basis, entry.key())
            })
    }
}

fn frozen_task_presentation_entry(
    descriptor: TaskPresentationDescriptor,
) -> FrozenTaskPresentationEntry {
    let key = TaskPresentationKey::from_descriptor(&descriptor);
    FrozenTaskPresentationEntry::new(descriptor, key)
}

fn fold_task_presentation_key(accumulator: u64, key: &TaskPresentationKey) -> u64 {
    fold_bytes(accumulator, key.projection_basis().as_bytes())
}

fn fold_bytes(mut accumulator: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        accumulator ^= u64::from(*byte);
        accumulator = accumulator.wrapping_mul(0x0000_0100_0000_01b3);
    }
    accumulator
}
