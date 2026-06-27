use std::collections::BTreeSet;

use super::SurfaceDescriptor;

/// Surface-family acceptance proof produced by registration validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SurfaceAcceptedRegistrationProof {
    accepted_identity_texts: BTreeSet<String>,
}

impl SurfaceAcceptedRegistrationProof {
    pub(crate) fn from_identity_texts(accepted_identity_texts: BTreeSet<String>) -> Self {
        Self {
            accepted_identity_texts,
        }
    }

    pub(crate) fn admits(&self, descriptor: &SurfaceDescriptor) -> bool {
        self.accepted_identity_texts
            .contains(descriptor.id().as_str())
    }
}
