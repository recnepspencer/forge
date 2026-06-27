use std::collections::BTreeSet;

use crate::capability::NativeCapabilityId;

use super::super::NativeCapabilityDescriptor;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct NativeCapabilityAcceptedRegistrationProof {
    accepted_identity_texts: BTreeSet<String>,
}

impl NativeCapabilityAcceptedRegistrationProof {
    pub(crate) fn from_identity_texts(accepted_identity_texts: BTreeSet<String>) -> Self {
        Self {
            accepted_identity_texts,
        }
    }

    pub(crate) fn admits(&self, descriptor: &NativeCapabilityDescriptor) -> bool {
        self.accepted_identity_texts
            .contains(descriptor.id().as_str())
    }

    #[allow(dead_code)]
    pub(crate) fn admits_id(&self, id: &NativeCapabilityId) -> bool {
        self.accepted_identity_texts.contains(id.as_str())
    }
}
