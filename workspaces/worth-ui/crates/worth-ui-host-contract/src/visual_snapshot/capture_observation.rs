use super::{
    UiHostCaptureRequestIdentity, UiHostCoordinateTransform, UiHostPixelArtifact,
    UiHostPresentationEpoch, UiHostRealizedRegion,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostCaptureAffinity {
    request: UiHostCaptureRequestIdentity,
    copy_epoch: UiHostPresentationEpoch,
}

#[derive(Debug)]
pub struct UiHostCaptureObservation {
    affinity: UiHostCaptureAffinity,
    transform: UiHostCoordinateTransform,
    regions: Box<[UiHostRealizedRegion]>,
    pixels: Option<UiHostPixelArtifact>,
}

#[derive(Debug)]
#[must_use = "capture observation may retain a pending host obligation and must be handled"]
pub enum UiHostCaptureObservationOutcome {
    Pending,
    Captured(UiHostCaptureObservation),
    SupersededBeforeReadback,
    CaptureAffinityIndeterminate,
    ReadbackCompletionIndeterminate,
    Unsupported,
    CapacityExceeded,
}

impl UiHostCaptureAffinity {
    #[doc(hidden)]
    pub const fn observed_by_host(
        request: UiHostCaptureRequestIdentity,
        copy_epoch: UiHostPresentationEpoch,
    ) -> Self {
        Self {
            request,
            copy_epoch,
        }
    }

    pub const fn request(self) -> UiHostCaptureRequestIdentity {
        self.request
    }

    pub const fn copy_epoch(self) -> UiHostPresentationEpoch {
        self.copy_epoch
    }
}

impl UiHostCaptureObservation {
    #[doc(hidden)]
    pub fn observed_by_host(
        affinity: UiHostCaptureAffinity,
        transform: UiHostCoordinateTransform,
        regions: Vec<UiHostRealizedRegion>,
        pixels: Option<UiHostPixelArtifact>,
    ) -> Self {
        Self {
            affinity,
            transform,
            regions: regions.into_boxed_slice(),
            pixels,
        }
    }

    pub const fn affinity(&self) -> UiHostCaptureAffinity {
        self.affinity
    }

    pub const fn transform(&self) -> UiHostCoordinateTransform {
        self.transform
    }

    pub fn regions(&self) -> &[UiHostRealizedRegion] {
        &self.regions
    }

    pub const fn pixels(&self) -> Option<&UiHostPixelArtifact> {
        self.pixels.as_ref()
    }

    pub fn into_parts(
        self,
    ) -> (
        UiHostCaptureAffinity,
        UiHostCoordinateTransform,
        Box<[UiHostRealizedRegion]>,
        Option<UiHostPixelArtifact>,
    ) {
        (self.affinity, self.transform, self.regions, self.pixels)
    }
}
