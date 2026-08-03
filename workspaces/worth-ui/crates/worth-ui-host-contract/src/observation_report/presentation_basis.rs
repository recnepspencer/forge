#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostObservationPresentationBasis {
    frame: crate::UiMountedFrameIdentity,
    binding: crate::UiSurfaceBindingGeneration,
    epoch: crate::UiHostPresentationEpoch,
}

impl UiHostObservationPresentationBasis {
    pub const fn new(
        frame: crate::UiMountedFrameIdentity,
        binding: crate::UiSurfaceBindingGeneration,
        epoch: crate::UiHostPresentationEpoch,
    ) -> Self {
        Self {
            frame,
            binding,
            epoch,
        }
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
