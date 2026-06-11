use std::collections::BTreeSet;

use super::super::MosaicRegionKindDescriptor;

/// Mosaic-region-family acceptance proof produced by registration validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct MosaicRegionAcceptedRegistrationProof {
    accepted_identity_texts: BTreeSet<String>,
}

impl MosaicRegionAcceptedRegistrationProof {
    pub(crate) fn from_identity_texts(accepted_identity_texts: BTreeSet<String>) -> Self {
        Self {
            accepted_identity_texts,
        }
    }

    pub(crate) fn admits(&self, descriptor: &MosaicRegionKindDescriptor) -> bool {
        self.accepted_identity_texts
            .contains(descriptor.id().as_str())
    }
}
