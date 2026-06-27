use super::{
    FrozenRuntimeOutcomeProjectionCapabilities, RuntimeOutcomeProjectionAcceptedRegistrationProof,
    RuntimeOutcomeProjectionDescriptor,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct RuntimeOutcomeProjectionRegistry {
    descriptors: Vec<RuntimeOutcomeProjectionDescriptor>,
}

impl RuntimeOutcomeProjectionRegistry {
    pub(crate) fn empty() -> Self {
        Self {
            descriptors: Vec::new(),
        }
    }

    pub(crate) fn push(&mut self, descriptor: RuntimeOutcomeProjectionDescriptor) {
        self.descriptors.push(descriptor);
    }

    pub(crate) fn freeze(
        self,
        accepted_projections: &RuntimeOutcomeProjectionAcceptedRegistrationProof,
    ) -> FrozenRuntimeOutcomeProjectionCapabilities {
        FrozenRuntimeOutcomeProjectionCapabilities::from_accepted_descriptors(
            self.descriptors,
            accepted_projections,
        )
    }
}
