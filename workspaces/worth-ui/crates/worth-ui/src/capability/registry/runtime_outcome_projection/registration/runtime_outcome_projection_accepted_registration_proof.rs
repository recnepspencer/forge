use std::collections::BTreeSet;

use crate::capability::RuntimeOutcomeProjectionId;

use super::super::RuntimeOutcomeProjectionDescriptor;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct RuntimeOutcomeProjectionAcceptedRegistrationProof {
    accepted_identity_texts: BTreeSet<String>,
}

impl RuntimeOutcomeProjectionAcceptedRegistrationProof {
    pub(crate) fn from_identity_texts(accepted_identity_texts: BTreeSet<String>) -> Self {
        Self {
            accepted_identity_texts,
        }
    }

    pub(crate) fn admits(&self, descriptor: &RuntimeOutcomeProjectionDescriptor) -> bool {
        self.accepted_identity_texts
            .contains(descriptor.id().as_str())
    }

    #[allow(dead_code)]
    pub(crate) fn admits_id(&self, id: &RuntimeOutcomeProjectionId) -> bool {
        self.accepted_identity_texts.contains(id.as_str())
    }
}
