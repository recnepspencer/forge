//! Narrow inspection bridge — dispatch classifier, admission, routing, and bridge receipts.

mod admission;
mod boundary_access;
mod dispatch;
pub(crate) mod obligation_routes;
mod routes;
pub(crate) mod support_routing;
mod visual_snapshot;

pub use super::inspection::UiInspectionAiHarness;
pub use super::inspection_observation::UiInspectionFacadeObservation;
pub use super::inspection_receipt::UiInspectionReceipt;
pub use super::measurement_inspection_evidence::UiMeasurementInspectionEvidenceBundle;
pub use dispatch::{classify_inspection_dispatch, InspectionDispatchLane};
pub(crate) use routes::route_inspection;
pub use visual_snapshot::{
    UiClearedVisualOverlayReceipt, UiClientRegionVisualTarget, UiCurrentPresentedSurfaceTarget,
    UiMountedNodeVisualTarget, UiPendingVisualCapture, UiPendingVisualOverlay,
    UiPublishedVisualOverlay, UiRetainedPresentedSurfaceTarget, UiSnapshotClientPixel,
    UiSnapshotClientRegion, UiUnbudgetedVisualSnapshotComparisonRequest,
    UiVisualCancellationPosture, UiVisualCancellationReceipt, UiVisualCapturePoll,
    UiVisualCaptureShutdownReport, UiVisualCoordinateScope, UiVisualGeometryGrant,
    UiVisualGrantLifetime, UiVisualGrantScope, UiVisualGrantSurfaceScope,
    UiVisualOverlayClearFailure, UiVisualOverlayGrant, UiVisualOverlayIdentity,
    UiVisualOverlayPublicationFailure, UiVisualOverlayShutdownReport, UiVisualOverlayTarget,
    UiVisualPixelCaptureGrant, UiVisualSnapshotComparisonGrant, UiVisualSnapshotComparisonRequest,
    UiVisualSnapshotDisposalReceipt, UiVisualSnapshotIdentity, UiVisualSnapshotOutcome,
    UiVisualSnapshotReceipt, UiVisualTarget, WorthUiVisualInspectionAuthority,
};
pub use worth_ui_inspection::UiInspectionClosureReport;
