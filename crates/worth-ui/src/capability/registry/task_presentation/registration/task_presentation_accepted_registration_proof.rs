use std::collections::BTreeSet;

use crate::capability::TaskPresentationId;

use super::super::TaskPresentationDescriptor;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TaskPresentationAcceptedRegistrationProof {
    accepted_identity_texts: BTreeSet<String>,
}

impl TaskPresentationAcceptedRegistrationProof {
    pub(crate) fn from_identity_texts(accepted_identity_texts: BTreeSet<String>) -> Self {
        Self {
            accepted_identity_texts,
        }
    }

    pub(crate) fn admits(&self, descriptor: &TaskPresentationDescriptor) -> bool {
        self.accepted_identity_texts
            .contains(descriptor.id().as_str())
    }

    #[allow(dead_code)]
    pub(crate) fn admits_id(&self, id: &TaskPresentationId) -> bool {
        self.accepted_identity_texts.contains(id.as_str())
    }
}
