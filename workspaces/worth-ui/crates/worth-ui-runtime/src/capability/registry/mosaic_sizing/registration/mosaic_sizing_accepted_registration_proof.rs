use std::collections::BTreeSet;

use super::super::MosaicSizingContractDescriptor;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct MosaicSizingAcceptedRegistrationProof {
    accepted_identity_texts: BTreeSet<String>,
}

impl MosaicSizingAcceptedRegistrationProof {
    pub(crate) fn from_identity_texts(accepted_identity_texts: BTreeSet<String>) -> Self {
        Self {
            accepted_identity_texts,
        }
    }

    pub(crate) fn admits(&self, descriptor: &MosaicSizingContractDescriptor) -> bool {
        self.accepted_identity_texts
            .contains(descriptor.id().as_str())
    }
}
