use super::{
    FrozenPluginSlotCapabilities, PluginSlotAcceptedRegistrationProof, PluginSlotDescriptor,
};

/// Builder-owned plugin slot registry lane.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PluginSlotRegistry {
    descriptors: Vec<PluginSlotDescriptor>,
}

impl PluginSlotRegistry {
    pub(crate) fn empty() -> Self {
        Self {
            descriptors: Vec::new(),
        }
    }

    pub(crate) fn push(&mut self, descriptor: PluginSlotDescriptor) {
        self.descriptors.push(descriptor);
    }

    pub(crate) fn freeze(
        self,
        accepted_slots: &PluginSlotAcceptedRegistrationProof,
    ) -> FrozenPluginSlotCapabilities {
        FrozenPluginSlotCapabilities::from_accepted_descriptors(self.descriptors, accepted_slots)
    }
}
