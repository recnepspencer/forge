use std::collections::BTreeSet;

use super::ComponentDescriptor;

/// Component-family acceptance proof produced by registration validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ComponentAcceptedRegistrationProof {
    accepted_identity_texts: BTreeSet<String>,
}

impl ComponentAcceptedRegistrationProof {
    pub(crate) fn from_identity_texts(accepted_identity_texts: BTreeSet<String>) -> Self {
        Self {
            accepted_identity_texts,
        }
    }

    pub(crate) fn admits(&self, descriptor: &ComponentDescriptor) -> bool {
        self.accepted_identity_texts
            .contains(descriptor.id().as_str())
    }
}
