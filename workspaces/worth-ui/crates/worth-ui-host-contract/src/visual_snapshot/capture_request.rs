use super::UiHostPresentationEpoch;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UiHostCaptureRequestIdentity(u64);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostVisualCaptureRequest {
    identity: UiHostCaptureRequestIdentity,
    host_session_identity: u64,
    frame: crate::UiMountedFrameIdentity,
    presentation_attempt: crate::UiMountedPresentationAttemptIdentity,
    host_surface: crate::UiHostSurfaceIdentity,
    binding: crate::UiSurfaceBindingGeneration,
    expected_epoch: UiHostPresentationEpoch,
    pixels_requested: bool,
    maximum_pixel_bytes: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostCaptureFrameAffinity {
    frame: crate::UiMountedFrameIdentity,
    presentation_attempt: crate::UiMountedPresentationAttemptIdentity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostCaptureSurfaceAffinity {
    host_session_identity: u64,
    host_surface: crate::UiHostSurfaceIdentity,
    binding: crate::UiSurfaceBindingGeneration,
    expected_epoch: UiHostPresentationEpoch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostCaptureArtifactBudget {
    pixels_requested: bool,
    maximum_pixel_bytes: u64,
}

impl UiHostCaptureRequestIdentity {
    #[doc(hidden)]
    pub const fn issued_by_runtime(value: u64) -> Self {
        Self(value)
    }

    pub const fn diagnostic_value(self) -> u64 {
        self.0
    }
}

impl UiHostVisualCaptureRequest {
    #[doc(hidden)]
    pub const fn admitted_by_runtime(
        identity: UiHostCaptureRequestIdentity,
        frame: UiHostCaptureFrameAffinity,
        surface: UiHostCaptureSurfaceAffinity,
        artifacts: UiHostCaptureArtifactBudget,
    ) -> Self {
        Self {
            identity,
            host_session_identity: surface.host_session_identity,
            frame: frame.frame,
            presentation_attempt: frame.presentation_attempt,
            host_surface: surface.host_surface,
            binding: surface.binding,
            expected_epoch: surface.expected_epoch,
            pixels_requested: artifacts.pixels_requested,
            maximum_pixel_bytes: artifacts.maximum_pixel_bytes,
        }
    }

    pub const fn identity(self) -> UiHostCaptureRequestIdentity {
        self.identity
    }

    pub const fn host_session_identity(self) -> u64 {
        self.host_session_identity
    }

    pub const fn frame(self) -> crate::UiMountedFrameIdentity {
        self.frame
    }

    pub const fn presentation_attempt(self) -> crate::UiMountedPresentationAttemptIdentity {
        self.presentation_attempt
    }

    pub const fn host_surface(self) -> crate::UiHostSurfaceIdentity {
        self.host_surface
    }

    pub const fn binding(self) -> crate::UiSurfaceBindingGeneration {
        self.binding
    }

    pub const fn expected_epoch(self) -> UiHostPresentationEpoch {
        self.expected_epoch
    }

    pub const fn pixels_requested(self) -> bool {
        self.pixels_requested
    }

    pub const fn maximum_pixel_bytes(self) -> u64 {
        self.maximum_pixel_bytes
    }
}

impl UiHostCaptureFrameAffinity {
    #[doc(hidden)]
    pub const fn observed_by_runtime(
        frame: crate::UiMountedFrameIdentity,
        presentation_attempt: crate::UiMountedPresentationAttemptIdentity,
    ) -> Self {
        Self {
            frame,
            presentation_attempt,
        }
    }
}

impl UiHostCaptureSurfaceAffinity {
    #[doc(hidden)]
    pub const fn observed_by_runtime(
        host_session_identity: u64,
        host_surface: crate::UiHostSurfaceIdentity,
        binding: crate::UiSurfaceBindingGeneration,
        expected_epoch: UiHostPresentationEpoch,
    ) -> Self {
        Self {
            host_session_identity,
            host_surface,
            binding,
            expected_epoch,
        }
    }
}

impl UiHostCaptureArtifactBudget {
    #[doc(hidden)]
    pub const fn admitted_by_runtime(pixels_requested: bool, maximum_pixel_bytes: u64) -> Self {
        Self {
            pixels_requested,
            maximum_pixel_bytes,
        }
    }
}
