use std::collections::BTreeSet;

use super::super::ViewBindingDescriptor;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ViewBindingAcceptedRegistrationProof {
    accepted_identity_texts: BTreeSet<String>,
}

impl ViewBindingAcceptedRegistrationProof {
    pub(crate) fn from_identity_texts(accepted_identity_texts: BTreeSet<String>) -> Self {
        Self {
            accepted_identity_texts,
        }
    }

    pub(crate) fn admits(&self, descriptor: &ViewBindingDescriptor) -> bool {
        self.accepted_identity_texts
            .contains(descriptor.id().as_str())
    }
}
