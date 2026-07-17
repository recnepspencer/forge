use crate::capability::ViewBindingId;

use super::{
    FrozenViewBindingEntry, ViewBindingAcceptedRegistrationProof, ViewBindingDescriptor,
    WorthUiViewBindingIdentity,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenViewBindingCapabilities {
    entries: Vec<FrozenViewBindingEntry>,
}

impl FrozenViewBindingCapabilities {
    #[cfg(test)]
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
            .map(|descriptor| {
                let identity = WorthUiViewBindingIdentity::from_descriptor(&descriptor);
                FrozenViewBindingEntry::new(descriptor, identity)
            })
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
        self.get_entry(id).map(FrozenViewBindingEntry::descriptor)
    }

    pub fn get_entry(&self, id: &ViewBindingId) -> Option<&FrozenViewBindingEntry> {
        self.entries
            .binary_search_by(|entry| entry.descriptor().id().cmp(id))
            .ok()
            .map(|index| &self.entries[index])
    }

    pub(crate) fn digest_basis(&self) -> u64 {
        self.entries
            .iter()
            .fold(0x3a42_71ec_c9f2_a901, |basis, entry| {
                basis
                    ^ entry
                        .descriptor()
                        .definition()
                        .digest()
                        .as_u64()
                        .rotate_left(17)
                    ^ entry.identity().as_u64().rotate_left(29)
            })
    }
}
