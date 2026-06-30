use super::{CommandAcceptedRegistrationProof, CommandDescriptor, FrozenCommandCapabilities};

/// Builder-owned command registry lane.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CommandRegistry {
    descriptors: Vec<CommandDescriptor>,
}

impl CommandRegistry {
    pub(crate) fn empty() -> Self {
        Self {
            descriptors: Vec::new(),
        }
    }

    pub(crate) fn push(&mut self, descriptor: CommandDescriptor) {
        self.descriptors.push(descriptor);
    }

    pub(crate) fn freeze(
        self,
        accepted_commands: &CommandAcceptedRegistrationProof,
    ) -> FrozenCommandCapabilities {
        FrozenCommandCapabilities::from_accepted_descriptors(self.descriptors, accepted_commands)
    }
}
