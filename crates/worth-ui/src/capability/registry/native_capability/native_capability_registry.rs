use super::{
    FrozenNativeCapabilities, NativeCapabilityAcceptedRegistrationProof, NativeCapabilityDescriptor,
};

/// Builder-owned native capability registry lane.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct NativeCapabilityRegistry {
    descriptors: Vec<NativeCapabilityDescriptor>,
}

impl NativeCapabilityRegistry {
    pub(crate) fn empty() -> Self {
        Self {
            descriptors: Vec::new(),
        }
    }

    pub(crate) fn push(&mut self, descriptor: NativeCapabilityDescriptor) {
        self.descriptors.push(descriptor);
    }

    pub(crate) fn freeze(
        self,
        accepted_native_capabilities: &NativeCapabilityAcceptedRegistrationProof,
    ) -> FrozenNativeCapabilities {
        FrozenNativeCapabilities::from_accepted_descriptors(
            self.descriptors,
            accepted_native_capabilities,
        )
    }
}
