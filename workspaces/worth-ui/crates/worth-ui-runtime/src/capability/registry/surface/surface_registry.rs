use super::{FrozenSurfaceCapabilities, SurfaceAcceptedRegistrationProof, SurfaceDescriptor};

/// Builder-owned surface registry lane.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SurfaceRegistry {
    descriptors: Vec<SurfaceDescriptor>,
}

impl SurfaceRegistry {
    pub(crate) fn empty() -> Self {
        Self {
            descriptors: Vec::new(),
        }
    }

    pub(crate) fn push(&mut self, descriptor: SurfaceDescriptor) {
        self.descriptors.push(descriptor);
    }

    pub(crate) fn freeze(
        self,
        accepted_surfaces: &SurfaceAcceptedRegistrationProof,
    ) -> FrozenSurfaceCapabilities {
        FrozenSurfaceCapabilities::from_accepted_descriptors(self.descriptors, accepted_surfaces)
    }
}
