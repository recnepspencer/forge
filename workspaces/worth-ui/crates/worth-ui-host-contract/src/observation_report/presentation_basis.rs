#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostObservationPresentationBasis {
    host_surface: crate::UiHostSurfaceIdentity,
    frame: crate::UiMountedFrameIdentity,
    binding: crate::UiSurfaceBindingGeneration,
    epoch: crate::UiHostPresentationEpoch,
}

impl UiHostObservationPresentationBasis {
    pub const fn new(
        host_surface: crate::UiHostSurfaceIdentity,
        frame: crate::UiMountedFrameIdentity,
        binding: crate::UiSurfaceBindingGeneration,
        epoch: crate::UiHostPresentationEpoch,
    ) -> Self {
        Self {
            host_surface,
            frame,
            binding,
            epoch,
        }
    }

    pub const fn host_surface(self) -> crate::UiHostSurfaceIdentity {
        self.host_surface
    }

    pub const fn frame(self) -> crate::UiMountedFrameIdentity {
        self.frame
    }

    pub const fn binding(self) -> crate::UiSurfaceBindingGeneration {
        self.binding
    }

    pub const fn epoch(self) -> crate::UiHostPresentationEpoch {
        self.epoch
    }
}
