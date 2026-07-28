mod capture_basis;
mod cost;
mod evidence;
mod geometry;
mod hit_test;
mod identity_trace;
mod outcome;
mod overlay;
mod pixel_artifact;
mod point_adjudication;
mod query_budget;
mod region_adjudication;
mod spatial_index;
mod visible_region;

pub use capture_basis::{UiVisualSnapshotAffinity, UiVisualSnapshotRelation};
pub use cost::{UiVisualInspectionCostLane, UiVisualInspectionCostReceipt};
pub use evidence::{
    UiVisualSnapshotArtifactPosture, UiVisualSnapshotEvidence, UiVisualSnapshotEvidenceInput,
};
pub use geometry::{
    UiClientPhysicalPixel, UiClientPhysicalRect, UiHostSurfaceLogicalPoint,
    UiNativeScreenPhysicalPixel, UiViewportLogicalPoint, UiVisualCoordinateDenial,
    UiVisualCoordinateObservation, UiVisualCoordinateObservationInput,
    UiVisualCoordinateOrientation, UiVisualCoordinateRounding,
};
pub use hit_test::{UiVisualHitTestOutcome, UiVisualHitTestTarget};
pub use identity_trace::{
    UiVisualAuthoredProvenance, UiVisualDeclarationRef, UiVisualEvidenceRef, UiVisualGraphNodeRef,
    UiVisualIdentityTrace, UiVisualIdentityTraceInput, UiVisualMountedNodeRef,
};
pub use outcome::{
    UiVisualSnapshotDenial, UiVisualSnapshotIndeterminate, UiVisualSnapshotOmission,
    UiVisualSnapshotSuperseded,
};
pub use overlay::UiVisualOverlayDenial;
pub use pixel_artifact::{
    UiVisualDerivedPixelArtifactInput, UiVisualNativePixelArtifactInput, UiVisualPixelArtifact,
    UiVisualPixelArtifactValidity, UiVisualPixelCaptureSource, UiVisualPixelColorSpace,
    UiVisualPixelFormat, UiVisualPixelRetentionDisposition,
};
pub use point_adjudication::UiVisualPointAdjudication;
pub use query_budget::UiVisualQueryBudget;
pub use region_adjudication::{
    UiVisualRegionAdjudication, UiVisualRegionCompleteness, UiVisualRegionIntersection,
};
pub use spatial_index::{UiHitTestRegionIndexIdentity, UiVisibleRegionIndexIdentity};
pub use visible_region::{
    UiVisualContributorStack, UiVisualVisibleContributor, UiVisualVisibleOutcome,
};
