use super::{FrozenIconCapabilities, IconAcceptedRegistrationProof, IconDescriptor};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct IconRegistry {
    descriptors: Vec<IconDescriptor>,
}

impl IconRegistry {
    pub(crate) fn empty() -> Self {
        Self {
            descriptors: Vec::new(),
        }
    }

    pub(crate) fn push(&mut self, descriptor: IconDescriptor) {
        self.descriptors.push(descriptor);
    }

    pub(crate) fn freeze(
        self,
        accepted_icons: &IconAcceptedRegistrationProof,
    ) -> FrozenIconCapabilities {
        FrozenIconCapabilities::from_accepted_descriptors(self.descriptors, accepted_icons)
    }
}
