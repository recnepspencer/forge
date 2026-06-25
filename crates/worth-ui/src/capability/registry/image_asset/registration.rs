use std::collections::BTreeSet;

use super::ImageAssetDescriptor;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ImageAssetAcceptedRegistrationProof {
    accepted_identity_texts: BTreeSet<String>,
}

impl ImageAssetAcceptedRegistrationProof {
    pub(crate) fn from_identity_texts(identity_texts: BTreeSet<String>) -> Self {
        Self {
            accepted_identity_texts: identity_texts,
        }
    }

    pub(crate) fn admits(&self, descriptor: &ImageAssetDescriptor) -> bool {
        self.accepted_identity_texts
            .contains(descriptor.id().as_str())
    }
}
