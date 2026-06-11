use std::collections::BTreeSet;

use super::super::ThemeTokenDescriptor;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ThemeTokenAcceptedRegistrationProof {
    accepted_identity_texts: BTreeSet<String>,
}

impl ThemeTokenAcceptedRegistrationProof {
    pub(crate) fn from_identity_texts(identity_texts: BTreeSet<String>) -> Self {
        Self {
            accepted_identity_texts: identity_texts,
        }
    }

    pub(crate) fn admits(&self, descriptor: &ThemeTokenDescriptor) -> bool {
        self.accepted_identity_texts
            .contains(descriptor.id().as_str())
    }
}
