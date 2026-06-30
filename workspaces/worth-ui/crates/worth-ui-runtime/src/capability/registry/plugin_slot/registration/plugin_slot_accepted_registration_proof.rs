use std::collections::BTreeSet;

use crate::capability::PluginSlotId;

use super::super::PluginSlotDescriptor;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PluginSlotAcceptedRegistrationProof {
    accepted_identity_texts: BTreeSet<String>,
}

impl PluginSlotAcceptedRegistrationProof {
    pub(crate) fn from_identity_texts(accepted_identity_texts: BTreeSet<String>) -> Self {
        Self {
            accepted_identity_texts,
        }
    }

    pub(crate) fn admits(&self, descriptor: &PluginSlotDescriptor) -> bool {
        self.accepted_identity_texts
            .contains(descriptor.id().as_str())
    }

    #[allow(dead_code)]
    pub(crate) fn admits_id(&self, id: &PluginSlotId) -> bool {
        self.accepted_identity_texts.contains(id.as_str())
    }
}
