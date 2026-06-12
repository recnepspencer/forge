use std::collections::BTreeSet;

use super::super::IconDescriptor;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct IconAcceptedRegistrationProof {
    accepted_identity_texts: BTreeSet<String>,
}

impl IconAcceptedRegistrationProof {
    pub(crate) fn from_identity_texts(identity_texts: BTreeSet<String>) -> Self {
        Self {
            accepted_identity_texts: identity_texts,
        }
    }

    pub(crate) fn admits(&self, descriptor: &IconDescriptor) -> bool {
        self.accepted_identity_texts
            .contains(descriptor.id().as_str())
    }
}
