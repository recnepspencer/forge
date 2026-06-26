use super::{
    FrozenMosaicPlacementCapabilities, MosaicPlacementAcceptedRegistrationProof,
    MosaicPlacementPolicyDescriptor,
};

/// Builder-owned mosaic placement policy registry lane.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct MosaicPlacementRegistry {
    descriptors: Vec<MosaicPlacementPolicyDescriptor>,
}

impl MosaicPlacementRegistry {
    pub(crate) fn empty() -> Self {
        Self {
            descriptors: Vec::new(),
        }
    }

    pub(crate) fn push(&mut self, descriptor: MosaicPlacementPolicyDescriptor) {
        self.descriptors.push(descriptor);
    }

    pub(crate) fn freeze(
        self,
        accepted_policies: &MosaicPlacementAcceptedRegistrationProof,
    ) -> FrozenMosaicPlacementCapabilities {
        FrozenMosaicPlacementCapabilities::from_accepted_descriptors(
            self.descriptors,
            accepted_policies,
        )
    }
}
