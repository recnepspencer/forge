use super::{
    FrozenMosaicRegionCapabilities, MosaicRegionAcceptedRegistrationProof,
    MosaicRegionKindDescriptor,
};

/// Builder-owned mosaic region kind registry lane.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct MosaicRegionRegistry {
    descriptors: Vec<MosaicRegionKindDescriptor>,
}

impl MosaicRegionRegistry {
    pub(crate) fn empty() -> Self {
        Self {
            descriptors: Vec::new(),
        }
    }

    pub(crate) fn push(&mut self, descriptor: MosaicRegionKindDescriptor) {
        self.descriptors.push(descriptor);
    }

    pub(crate) fn freeze(
        self,
        accepted_regions: &MosaicRegionAcceptedRegistrationProof,
    ) -> FrozenMosaicRegionCapabilities {
        FrozenMosaicRegionCapabilities::from_accepted_descriptors(
            self.descriptors,
            accepted_regions,
        )
    }
}
