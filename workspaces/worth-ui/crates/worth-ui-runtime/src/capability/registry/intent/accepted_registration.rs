use std::collections::BTreeSet;

use super::IntentDefinitionDescriptor;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct IntentDefinitionAcceptedRegistrationProof {
    accepted_identity_texts: BTreeSet<String>,
}

impl IntentDefinitionAcceptedRegistrationProof {
    pub(crate) fn from_identity_texts(accepted_identity_texts: BTreeSet<String>) -> Self {
        Self {
            accepted_identity_texts,
        }
    }

    pub(crate) fn admits(&self, descriptor: &IntentDefinitionDescriptor) -> bool {
        self.accepted_identity_texts
            .contains(descriptor.id().as_str())
    }
}
