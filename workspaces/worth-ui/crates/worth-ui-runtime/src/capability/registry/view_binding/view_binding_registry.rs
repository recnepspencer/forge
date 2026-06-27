use super::{
    FrozenViewBindingCapabilities, ViewBindingAcceptedRegistrationProof, ViewBindingDescriptor,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ViewBindingRegistry {
    descriptors: Vec<ViewBindingDescriptor>,
}

impl ViewBindingRegistry {
    pub(crate) fn empty() -> Self {
        Self {
            descriptors: Vec::new(),
        }
    }

    pub(crate) fn push(&mut self, descriptor: ViewBindingDescriptor) {
        self.descriptors.push(descriptor);
    }

    pub(crate) fn freeze(
        self,
        accepted_bindings: &ViewBindingAcceptedRegistrationProof,
    ) -> FrozenViewBindingCapabilities {
        FrozenViewBindingCapabilities::from_accepted_descriptors(
            self.descriptors,
            accepted_bindings,
        )
    }
}
