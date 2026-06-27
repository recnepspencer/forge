use std::collections::BTreeSet;

use crate::capability::CommandProjectionId;

use super::super::CommandProjectionDescriptor;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CommandProjectionAcceptedRegistrationProof {
    accepted_identity_texts: BTreeSet<String>,
}

impl CommandProjectionAcceptedRegistrationProof {
    pub(crate) fn from_identity_texts(accepted_identity_texts: BTreeSet<String>) -> Self {
        Self {
            accepted_identity_texts,
        }
    }

    pub(crate) fn admits(&self, descriptor: &CommandProjectionDescriptor) -> bool {
        self.accepted_identity_texts
            .contains(descriptor.id().as_str())
    }

    #[allow(dead_code)]
    pub(crate) fn admits_id(&self, id: &CommandProjectionId) -> bool {
        self.accepted_identity_texts.contains(id.as_str())
    }
}
