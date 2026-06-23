use super::{
    FrozenAppearanceCapabilities, WorthUiAppearanceAcceptedRegistrationProof,
    WorthUiAppearanceTokenDescriptor,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiAppearanceRegistry {
    descriptors: Vec<WorthUiAppearanceTokenDescriptor>,
}

impl WorthUiAppearanceRegistry {
    pub(crate) fn empty() -> Self {
        Self {
            descriptors: Vec::new(),
        }
    }

    pub(crate) fn push(&mut self, descriptor: WorthUiAppearanceTokenDescriptor) {
        self.descriptors.push(descriptor);
    }

    pub(crate) fn freeze(
        self,
        accepted: &WorthUiAppearanceAcceptedRegistrationProof,
    ) -> FrozenAppearanceCapabilities {
        FrozenAppearanceCapabilities::from_accepted_descriptors(self.descriptors, accepted)
    }
}
