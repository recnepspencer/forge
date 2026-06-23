use std::collections::BTreeSet;

use crate::capability::WorthUiAppearanceTokenDescriptor;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiAppearanceAcceptedRegistrationProof {
    accepted_identity_texts: BTreeSet<String>,
}

impl WorthUiAppearanceAcceptedRegistrationProof {
    pub(crate) fn from_identity_texts(identity_texts: BTreeSet<String>) -> Self {
        Self {
            accepted_identity_texts: identity_texts,
        }
    }

    pub(crate) fn admits(&self, descriptor: &WorthUiAppearanceTokenDescriptor) -> bool {
        self.accepted_identity_texts
            .contains(descriptor.id().as_str())
    }
}
