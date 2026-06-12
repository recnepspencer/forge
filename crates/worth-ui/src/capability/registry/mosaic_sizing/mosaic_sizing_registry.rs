use super::{
    FrozenMosaicSizingCapabilities, MosaicSizingAcceptedRegistrationProof,
    MosaicSizingContractDescriptor,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct MosaicSizingRegistry {
    descriptors: Vec<MosaicSizingContractDescriptor>,
}

impl MosaicSizingRegistry {
    pub(crate) fn empty() -> Self {
        Self {
            descriptors: Vec::new(),
        }
    }

    pub(crate) fn push(&mut self, descriptor: MosaicSizingContractDescriptor) {
        self.descriptors.push(descriptor);
    }

    pub(crate) fn freeze(
        self,
        accepted_contracts: &MosaicSizingAcceptedRegistrationProof,
    ) -> FrozenMosaicSizingCapabilities {
        FrozenMosaicSizingCapabilities::from_accepted_descriptors(
            self.descriptors,
            accepted_contracts,
        )
    }
}
