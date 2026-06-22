use super::{
    FrozenMosaicStateCapabilities, MosaicStateSlotAcceptedRegistrationProof,
    MosaicStateSlotDescriptor,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct MosaicStateSlotRegistry {
    descriptors: Vec<MosaicStateSlotDescriptor>,
}

impl MosaicStateSlotRegistry {
    pub(crate) fn empty() -> Self {
        Self {
            descriptors: Vec::new(),
        }
    }

    pub(crate) fn push(&mut self, descriptor: MosaicStateSlotDescriptor) {
        self.descriptors.push(descriptor);
    }

    pub(crate) fn freeze(
        self,
        accepted_slots: &MosaicStateSlotAcceptedRegistrationProof,
    ) -> FrozenMosaicStateCapabilities {
        FrozenMosaicStateCapabilities::from_accepted_descriptors(self.descriptors, accepted_slots)
    }
}
