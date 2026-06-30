use super::{
    CommandProjectionAcceptedRegistrationProof, CommandProjectionDescriptor,
    FrozenCommandProjectionCapabilities,
};

/// Builder-owned command projection registry lane.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CommandProjectionRegistry {
    descriptors: Vec<CommandProjectionDescriptor>,
}

impl CommandProjectionRegistry {
    pub(crate) fn empty() -> Self {
        Self {
            descriptors: Vec::new(),
        }
    }

    pub(crate) fn push(&mut self, descriptor: CommandProjectionDescriptor) {
        self.descriptors.push(descriptor);
    }

    pub(crate) fn freeze(
        self,
        accepted_projections: &CommandProjectionAcceptedRegistrationProof,
    ) -> FrozenCommandProjectionCapabilities {
        FrozenCommandProjectionCapabilities::from_accepted_descriptors(
            self.descriptors,
            accepted_projections,
        )
    }
}
