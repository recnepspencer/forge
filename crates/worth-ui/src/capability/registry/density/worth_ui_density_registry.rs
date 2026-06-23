use super::{
    FrozenDensityCapabilities, WorthUiDensityAcceptedRegistrationProof,
    WorthUiDensityTokenDescriptor,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiDensityRegistry {
    descriptors: Vec<WorthUiDensityTokenDescriptor>,
}

impl WorthUiDensityRegistry {
    pub(crate) fn empty() -> Self {
        Self {
            descriptors: Vec::new(),
        }
    }

    pub(crate) fn push(&mut self, descriptor: WorthUiDensityTokenDescriptor) {
        self.descriptors.push(descriptor);
    }

    pub(crate) fn freeze(
        self,
        accepted: &WorthUiDensityAcceptedRegistrationProof,
    ) -> FrozenDensityCapabilities {
        FrozenDensityCapabilities::from_accepted_descriptors(self.descriptors, accepted)
    }
}
