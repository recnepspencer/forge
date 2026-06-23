use std::collections::BTreeSet;

use crate::capability::WorthUiDensityTokenDescriptor;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiDensityAcceptedRegistrationProof {
    accepted_identity_texts: BTreeSet<String>,
}

impl WorthUiDensityAcceptedRegistrationProof {
    pub(crate) fn from_identity_texts(identity_texts: BTreeSet<String>) -> Self {
        Self {
            accepted_identity_texts: identity_texts,
        }
    }

    pub(crate) fn admits(&self, descriptor: &WorthUiDensityTokenDescriptor) -> bool {
        self.accepted_identity_texts
            .contains(descriptor.id().as_str())
    }
}
