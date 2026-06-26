use crate::capability::RuntimeOutcomeProjectionId;

use super::{
    FrozenRuntimeOutcomeProjectionEntry, RuntimeOutcomeProjectionAcceptedRegistrationProof,
    RuntimeOutcomeProjectionDescriptor, RuntimeOutcomeProjectionKey,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenRuntimeOutcomeProjectionCapabilities {
    entries: Vec<FrozenRuntimeOutcomeProjectionEntry>,
}

impl FrozenRuntimeOutcomeProjectionCapabilities {
    pub(crate) fn empty() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub(crate) fn from_accepted_descriptors(
        mut descriptors: Vec<RuntimeOutcomeProjectionDescriptor>,
        accepted_projections: &RuntimeOutcomeProjectionAcceptedRegistrationProof,
    ) -> Self {
        descriptors.retain(|descriptor| accepted_projections.admits(descriptor));
        descriptors.sort_by(|left, right| left.id().cmp(right.id()));
        let entries = descriptors
            .into_iter()
            .map(frozen_runtime_outcome_projection_entry)
            .collect();
        Self { entries }
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn entries(&self) -> &[FrozenRuntimeOutcomeProjectionEntry] {
        &self.entries
    }

    pub fn get(
        &self,
        id: &RuntimeOutcomeProjectionId,
    ) -> Option<&RuntimeOutcomeProjectionDescriptor> {
        self.entries
            .binary_search_by(|entry| entry.descriptor().id().cmp(id))
            .ok()
            .map(|index| self.entries[index].descriptor())
    }

    pub(crate) fn digest_basis(&self) -> u64 {
        self.entries
            .iter()
            .fold(0x4973_2f4b_1357_bdd1, |basis, entry| {
                fold_bytes(
                    fold_runtime_outcome_projection_descriptor(basis, entry.descriptor()),
                    entry.key().projection_basis().as_bytes(),
                )
            })
    }
}

fn frozen_runtime_outcome_projection_entry(
    descriptor: RuntimeOutcomeProjectionDescriptor,
) -> FrozenRuntimeOutcomeProjectionEntry {
    let key = runtime_outcome_projection_key(&descriptor);
    FrozenRuntimeOutcomeProjectionEntry::new(descriptor, key)
}

fn runtime_outcome_projection_key(
    descriptor: &RuntimeOutcomeProjectionDescriptor,
) -> RuntimeOutcomeProjectionKey {
    let runtime_identity_basis = format!(
        "{}|{}",
        descriptor.family().digest_basis(),
        descriptor
            .source()
            .expect("accepted runtime outcome projection has runtime source")
            .digest_basis()
    );
    let projection_basis = format!(
        "{}|{}|{}|{}|{}",
        descriptor.id().as_str(),
        runtime_identity_basis,
        descriptor
            .presentation()
            .map(|presentation| presentation.digest_basis())
            .unwrap_or_else(|| "none".to_string()),
        descriptor
            .denial_posture()
            .map(|posture| length_prefixed(posture.digest_basis()))
            .unwrap_or_else(|| "none".to_string()),
        descriptor
            .recovery_posture()
            .map(|posture| length_prefixed(posture.digest_basis()))
            .unwrap_or_else(|| "none".to_string())
    );
    RuntimeOutcomeProjectionKey::new(runtime_identity_basis, projection_basis)
}

fn fold_runtime_outcome_projection_descriptor(
    accumulator: u64,
    descriptor: &RuntimeOutcomeProjectionDescriptor,
) -> u64 {
    let with_id = fold_bytes(accumulator, descriptor.id().as_str().as_bytes());
    let with_family = fold_bytes(with_id, descriptor.family().digest_basis().as_bytes());
    let with_source = fold_optional_string(
        with_family,
        descriptor.source().map(|source| source.digest_basis()),
    );
    let with_presentation = fold_optional_string(
        with_source,
        descriptor
            .presentation()
            .map(|presentation| presentation.digest_basis()),
    );
    let with_denial = fold_optional_str(
        with_presentation,
        descriptor
            .denial_posture()
            .map(|posture| posture.digest_basis()),
    );
    fold_optional_str(
        with_denial,
        descriptor
            .recovery_posture()
            .map(|posture| posture.digest_basis()),
    )
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

fn length_prefixed(value: &str) -> String {
    format!("{}:{value}", value.len())
}

fn fold_bytes(mut accumulator: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        accumulator ^= u64::from(*byte);
        accumulator = accumulator.wrapping_mul(0x0000_0100_0000_01b3);
    }
    accumulator
}
