use crate::capability::MosaicStateSlotId;

use super::{
    FrozenMosaicStateSlotEntry, MosaicStateReconciliationKey,
    MosaicStateSlotAcceptedRegistrationProof, MosaicStateSlotDescriptor,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenMosaicStateCapabilities {
    entries: Vec<FrozenMosaicStateSlotEntry>,
}

impl FrozenMosaicStateCapabilities {
    #[cfg(test)]
    pub(crate) fn empty() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub(crate) fn from_accepted_descriptors(
        mut descriptors: Vec<MosaicStateSlotDescriptor>,
        accepted_slots: &MosaicStateSlotAcceptedRegistrationProof,
    ) -> Self {
        descriptors.retain(|descriptor| accepted_slots.admits(descriptor));
        descriptors.sort_by(|left, right| left.id().cmp(right.id()));
        let entries = descriptors
            .into_iter()
            .map(frozen_mosaic_state_slot_entry)
            .collect();
        Self { entries }
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn entries(&self) -> &[FrozenMosaicStateSlotEntry] {
        &self.entries
    }

    pub fn get(&self, id: &MosaicStateSlotId) -> Option<&MosaicStateSlotDescriptor> {
        self.entries
            .binary_search_by(|entry| entry.descriptor().id().cmp(id))
            .ok()
            .map(|index| self.entries[index].descriptor())
    }

    pub(crate) fn digest_basis(&self) -> u64 {
        self.entries
            .iter()
            .fold(0xf7ad_01b5_b42c_d779, |basis, entry| {
                fold_bytes(
                    fold_mosaic_state_descriptor(basis, entry.descriptor()),
                    entry.reconciliation_key().as_str().as_bytes(),
                )
            })
    }
}

fn frozen_mosaic_state_slot_entry(
    descriptor: MosaicStateSlotDescriptor,
) -> FrozenMosaicStateSlotEntry {
    let reconciliation_key = reconciliation_key(&descriptor);
    FrozenMosaicStateSlotEntry::new(descriptor, reconciliation_key)
}

fn reconciliation_key(descriptor: &MosaicStateSlotDescriptor) -> MosaicStateReconciliationKey {
    MosaicStateReconciliationKey::from_digest_basis(format!(
        "{}|{}|{}|{}|{}|{}",
        descriptor.id().as_str(),
        descriptor.kind().digest_basis(),
        descriptor
            .owner_identity()
            .expect("accepted state slot has owner identity")
            .digest_basis(),
        descriptor
            .persistence_policy()
            .expect("accepted state slot has persistence policy")
            .digest_basis(),
        descriptor
            .replacement_rule()
            .expect("accepted state slot has replacement rule")
            .digest_basis(),
        descriptor
            .truth_posture()
            .expect("accepted state slot has truth posture")
            .digest_basis()
    ))
}

fn fold_mosaic_state_descriptor(accumulator: u64, descriptor: &MosaicStateSlotDescriptor) -> u64 {
    let with_id = fold_bytes(accumulator, descriptor.id().as_str().as_bytes());
    let with_kind = fold_bytes(with_id, descriptor.kind().digest_basis().as_bytes());
    let with_owner = fold_optional_string(
        with_kind,
        descriptor
            .owner_identity()
            .map(|identity| identity.digest_basis()),
    );
    let with_persistence = fold_optional_str(
        with_owner,
        descriptor
            .persistence_policy()
            .map(|policy| policy.digest_basis()),
    );
    let with_replacement = fold_optional_str(
        with_persistence,
        descriptor
            .replacement_rule()
            .map(|rule| rule.digest_basis()),
    );
    let with_truth = fold_optional_str(
        with_replacement,
        descriptor
            .truth_posture()
            .map(|posture| posture.digest_basis()),
    );
    fold_optional_str(with_truth, descriptor.label())
}

fn fold_optional_string(accumulator: u64, value: Option<String>) -> u64 {
    match value {
        Some(value) => fold_bytes(fold_bytes(accumulator, b"some"), value.as_bytes()),
        None => fold_bytes(accumulator, b"none"),
    }
}

fn fold_optional_str(accumulator: u64, value: Option<&str>) -> u64 {
    match value {
        Some(value) => fold_bytes(fold_bytes(accumulator, b"some"), value.as_bytes()),
        None => fold_bytes(accumulator, b"none"),
    }
}

fn fold_bytes(mut accumulator: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        accumulator ^= u64::from(*byte);
        accumulator = accumulator.wrapping_mul(0x0000_0100_0000_01b3);
    }
    accumulator
}
