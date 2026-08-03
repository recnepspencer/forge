mod capability;
mod capture_cancellation;
mod capture_observation;
mod capture_request;
mod coordinate_transform;
mod pixel_artifact;
mod presentation_epoch;
mod realized_region;

pub use capability::UiHostCaptureCapability;
pub use capture_cancellation::UiHostCaptureCancellationOutcome;
pub use capture_observation::{
    UiHostCaptureAffinity, UiHostCaptureObservation, UiHostCaptureObservationOutcome,
};
pub use capture_request::{
    UiHostCaptureArtifactBudget, UiHostCaptureFrameAffinity, UiHostCaptureRequestIdentity,
    UiHostCaptureSurfaceAffinity, UiHostVisualCaptureRequest,
};
pub use coordinate_transform::{
    UiHostClientAreaObservation, UiHostCoordinateOrientation, UiHostCoordinatePosture,
    UiHostCoordinateRounding, UiHostCoordinateTransform, UiHostViewportTransformObservation,
};
pub use pixel_artifact::{UiHostPixelArtifact, UiHostPixelColorSpace};
pub use presentation_epoch::UiHostPresentationEpoch;
pub use realized_region::{
    UiHostRealizedGeometry, UiHostRealizedOrdering, UiHostRealizedRegion,
    UiHostRealizedRegionParticipation,
};
