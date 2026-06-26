use std::collections::BTreeSet;

use crate::capability::SettingId;

use super::super::SettingDescriptor;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SettingAcceptedRegistrationProof {
    accepted_identity_texts: BTreeSet<String>,
}

impl SettingAcceptedRegistrationProof {
    pub(crate) fn from_identity_texts(accepted_identity_texts: BTreeSet<String>) -> Self {
        Self {
            accepted_identity_texts,
        }
    }

    pub(crate) fn admits(&self, descriptor: &SettingDescriptor) -> bool {
        self.accepted_identity_texts
            .contains(descriptor.id().as_str())
    }

    #[allow(dead_code)]
    pub(crate) fn admits_id(&self, id: &SettingId) -> bool {
        self.accepted_identity_texts.contains(id.as_str())
    }
}
