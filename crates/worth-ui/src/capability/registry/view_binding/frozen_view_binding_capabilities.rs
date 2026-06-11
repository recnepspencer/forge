use crate::capability::ViewBindingId;

use super::{
    FrozenViewBindingEntry, QueryViewBindingKey, ViewBindingAcceptedRegistrationProof,
    ViewBindingDescriptor,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenViewBindingCapabilities {
    entries: Vec<FrozenViewBindingEntry>,
}

impl FrozenViewBindingCapabilities {
    pub(crate) fn empty() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub(crate) fn from_accepted_descriptors(
        mut descriptors: Vec<ViewBindingDescriptor>,
        accepted_bindings: &ViewBindingAcceptedRegistrationProof,
    ) -> Self {
        descriptors.retain(|descriptor| accepted_bindings.admits(descriptor));
        descriptors.sort_by(|left, right| left.id().cmp(right.id()));
        let entries = descriptors
            .into_iter()
            .map(frozen_view_binding_entry)
            .collect();
        Self { entries }
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn entries(&self) -> &[FrozenViewBindingEntry] {
        &self.entries
    }

    pub fn get(&self, id: &ViewBindingId) -> Option<&ViewBindingDescriptor> {
        self.entries
            .binary_search_by(|entry| entry.descriptor().id().cmp(id))
            .ok()
            .map(|index| self.entries[index].descriptor())
    }

    pub(crate) fn digest_basis(&self) -> u64 {
        self.entries
            .iter()
            .fold(0x3a42_71ec_c9f2_a901, |basis, entry| {
                fold_bytes(
                    fold_view_binding_descriptor(basis, entry.descriptor()),
                    entry.query_binding_key().as_str().as_bytes(),
                )
            })
    }
}

fn frozen_view_binding_entry(descriptor: ViewBindingDescriptor) -> FrozenViewBindingEntry {
    let query_binding_key = query_binding_key(&descriptor);
    FrozenViewBindingEntry::new(descriptor, query_binding_key)
}

fn query_binding_key(descriptor: &ViewBindingDescriptor) -> QueryViewBindingKey {
    QueryViewBindingKey::from_digest_basis(format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        descriptor.id().as_str(),
        descriptor.family().digest_basis(),
        descriptor
            .query_capability()
            .expect("accepted view binding has query capability posture")
            .digest_basis(),
        descriptor
            .query_composition_profile_digest()
            .expect("accepted view binding has query composition posture"),
        view_shape_digest_basis(descriptor),
        descriptor
            .result_shape()
            .expect("accepted view binding has result shape")
            .digest_basis(),
        descriptor
            .basis_posture()
            .expect("accepted view binding has basis posture")
            .digest_basis(),
        descriptor
            .live_compatibility()
            .expect("accepted view binding has live compatibility")
            .digest_basis(),
        visible_state_bindings_digest_basis(descriptor),
        descriptor
            .denial_presentation()
            .map(|presentation| presentation.digest_basis())
            .unwrap_or("none")
    ))
}

fn view_shape_digest_basis(descriptor: &ViewBindingDescriptor) -> String {
    let view_shape = descriptor
        .view_shape()
        .expect("accepted view binding has view shape");
    format!(
        "{}|{}|{}",
        view_shape.family().as_str(),
        view_shape.focused_aspect().unwrap_or("none"),
        view_shape.grouping_aspect().unwrap_or("none")
    )
}

fn visible_state_bindings_digest_basis(descriptor: &ViewBindingDescriptor) -> String {
    descriptor
        .visible_state_bindings()
        .iter()
        .map(|binding| binding.digest_basis())
        .map(|basis| format!("{}:{basis}", basis.len()))
        .collect::<Vec<_>>()
        .join("")
}

fn fold_view_binding_descriptor(accumulator: u64, descriptor: &ViewBindingDescriptor) -> u64 {
    let with_id = fold_bytes(accumulator, descriptor.id().as_str().as_bytes());
    let with_family = fold_bytes(with_id, descriptor.family().digest_basis().as_bytes());
    let with_query = fold_optional_string(
        with_family,
        descriptor
            .query_capability()
            .map(|capability| capability.digest_basis()),
    );
    let with_composition =
        fold_optional_str(with_query, descriptor.query_composition_profile_digest());
    let with_view_shape = fold_optional_string(
        with_composition,
        descriptor
            .view_shape()
            .map(|_| view_shape_digest_basis(descriptor)),
    );
    let with_result = fold_optional_string(
        with_view_shape,
        descriptor
            .result_shape()
            .map(|result_shape| result_shape.digest_basis()),
    );
    let with_basis = fold_optional_string(
        with_result,
        descriptor.basis_posture().map(|basis| basis.digest_basis()),
    );
    let with_live = fold_optional_string(
        with_basis,
        descriptor
            .live_compatibility()
            .map(|compatibility| compatibility.digest_basis()),
    );
    let with_visible_state = descriptor
        .visible_state_bindings()
        .iter()
        .fold(with_live, |basis, binding| {
            fold_bytes(basis, binding.digest_basis().as_bytes())
        });
    fold_optional_str(
        with_visible_state,
        descriptor
            .denial_presentation()
            .map(|presentation| presentation.digest_basis()),
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

fn fold_bytes(mut accumulator: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        accumulator ^= u64::from(*byte);
        accumulator = accumulator.wrapping_mul(0x0000_0100_0000_01b3);
    }
    accumulator
}
