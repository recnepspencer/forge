use std::collections::BTreeSet;

use super::super::MosaicStateSlotDescriptor;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct MosaicStateSlotAcceptedRegistrationProof {
    accepted_identity_texts: BTreeSet<String>,
}

impl MosaicStateSlotAcceptedRegistrationProof {
    pub(crate) fn from_identity_texts(accepted_identity_texts: BTreeSet<String>) -> Self {
        Self {
            accepted_identity_texts,
        }
    }

    pub(crate) fn admits(&self, descriptor: &MosaicStateSlotDescriptor) -> bool {
        self.accepted_identity_texts
            .contains(descriptor.id().as_str())
    }
}
