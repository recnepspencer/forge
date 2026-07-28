mod capture_handle;
mod capture_progression;
mod grant;
mod identity;
mod identity_trace;
mod overlay;
mod overlay_registry;
mod point_adjudication;
mod receipt;
mod region_adjudication;
mod region_occlusion;
mod registry;
mod spatial;
pub(crate) mod structural_reservation;
mod target;

pub(crate) use capture_handle::{
    UiPendingDerivedRegionCapture, UiPendingDerivedRegionInput, UiPendingVisualCaptureRoute,
};
pub use capture_handle::{
    UiPendingVisualCapture, UiVisualCancellationPosture, UiVisualCancellationReceipt,
    UiVisualCapturePoll, UiVisualSnapshotOutcome,
};
pub(crate) use capture_progression::{
    UiIndexedVisualCapture, UiIndexedVisualCaptureParts, UiObservedHostVisualCapture,
    UiPinnedVisualCaptureInput, UiRequestedHostVisualCapture, UiValidatedHostVisualCapture,
    UiValidatedHostVisualCaptureInput, UiVisualCaptureIntent,
};
pub use grant::{
    UiVisualGeometryGrant, UiVisualGrantLifetime, UiVisualGrantScope, UiVisualGrantSurfaceScope,
    UiVisualOverlayGrant, UiVisualPixelCaptureGrant, WorthUiVisualInspectionAuthority,
};
pub use identity::UiVisualSnapshotIdentity;
pub(crate) use identity_trace::resolve_identity_trace;
pub(crate) use overlay::{
    seal_cleared_overlay, seal_overlay_target, seal_pending_overlay, seal_published_overlay,
    UiClearingVisualOverlay, UiPublishingVisualOverlay, UiVisualOverlaySelection,
    UiVisualOverlayTargetInput,
};
pub use overlay::{
    UiClearedVisualOverlayReceipt, UiPendingVisualOverlay, UiPublishedVisualOverlay,
    UiVisualOverlayClearFailure, UiVisualOverlayIdentity, UiVisualOverlayPublicationFailure,
    UiVisualOverlayTarget,
};
pub use overlay_registry::UiVisualOverlayShutdownReport;
pub(crate) use overlay_registry::{UiPendingVisualOverlayRegistration, UiVisualOverlayRegistry};
pub(crate) use point_adjudication::{adjudicate_point, UiPointAdjudicationInput};
pub(crate) use receipt::{UiRetainedVisualSnapshotSource, UiVisualSnapshotSealInput};
pub use receipt::{
    UiSnapshotClientPixel, UiSnapshotClientRegion, UiVisualCoordinateScope,
    UiVisualSnapshotDisposalReceipt, UiVisualSnapshotReceipt,
};
pub(crate) use region_adjudication::{adjudicate_region, UiRegionAdjudicationInput};
pub use registry::UiVisualCaptureShutdownReport;
pub(crate) use registry::{
    UiVisualCaptureRegistrationDenial, UiVisualCaptureRegistrationLease, UiVisualCaptureRegistry,
    UiVisualResourceReservation, UiVisualRetainedResourceUsage, UiVisualSnapshotResourceLease,
};
pub(crate) use spatial::{
    validate_and_index, UiHitTestRegionIndex, UiSpatialIndexBuildCost, UiSpatialValidationDenial,
    UiVisibleOpacity, UiVisibleRegionIndex, UiVisibleRegionRecord,
};
pub(crate) use target::{
    into_capture_route, seal_current_surface_target, seal_mounted_node_target, seal_region_target,
    seal_retained_surface_target, UiDerivedRegionTargetSource, UiVisualSurfaceCaptureBasis,
    UiVisualTargetRoute,
};
pub use target::{
    UiClientRegionVisualTarget, UiCurrentPresentedSurfaceTarget, UiMountedNodeVisualTarget,
    UiRetainedPresentedSurfaceTarget, UiVisualTarget,
};
