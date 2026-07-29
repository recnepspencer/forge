mod closeout;
mod closure_report;
pub(crate) mod evidence;
mod inspection_cost_receipt;
mod measurement;
mod rebind;
mod replay;
mod scope_support_row;
mod snapshot;
mod support_report;

pub use closeout::{
    UiInspectionAiHarnessLane, UiInspectionClosedSemanticLane, UiInspectionCloseoutGuarantee,
    UiInspectionCloseoutNonGoal, UiInspectionCloseoutReport, UiInspectionCostLane,
    UiInspectionDerivedIndexLane, UiInspectionRefLifecycleLane, UiInspectionSliceLane,
};
pub use closure_report::UiInspectionClosureReport;
pub use inspection_cost_receipt::UiInspectionCostReceipt;
pub use measurement::{
    UiInspectionMeasurementBasisInput, UiInspectionMeasurementBasisPosture,
    UiInspectionMeasurementBasisSource, UiInspectionMeasurementChildIntrinsicSource,
    UiInspectionMeasurementDenialPosture, UiInspectionMeasurementDependencyLineageEntry,
    UiInspectionMeasurementDependencyLineageKind, UiInspectionMeasurementEvidenceCategory,
    UiInspectionMeasurementEvidenceSlot, UiInspectionMeasurementEvidenceView,
    UiInspectionMeasurementEvidenceViewInput, UiInspectionMeasurementFailureSource,
    UiInspectionMeasurementGenerationCompatibility, UiInspectionMeasurementNeighborhoodClassHint,
    UiInspectionMeasurementOwnershipPosture, UiInspectionMeasurementQueryFactFamily,
    UiInspectionMeasurementQueryUnsupportedReason, UiInspectionQueryWorldCompatibilityFailure,
};
pub use rebind::{
    UiRebindDecisionDisposition, UiRebindDecisionIndex, UiRebindDecisionIndexDenial,
    UiRebindDecisionKey, UiRebindDecisionLookup, UiRebindDecisionRecord,
    UiRebindDecisionRecordInput, UiRebindDecisionStopPoint, UiRebindStructuralCost,
};
pub use scope_support_row::UiInspectionScopeSupportRow;
pub use snapshot::{
    UiClientPhysicalPixel, UiClientPhysicalRect, UiHitTestRegionIndexIdentity,
    UiHostSurfaceLogicalPoint, UiNativeScreenPhysicalPixel, UiViewportLogicalPoint,
    UiVisibleRegionIndexIdentity, UiVisualAuthoredProvenance, UiVisualComparisonPixelPolicy,
    UiVisualContributorStack, UiVisualCoordinateDenial, UiVisualCoordinateObservation,
    UiVisualCoordinateObservationInput, UiVisualCoordinateOrientation, UiVisualCoordinateRounding,
    UiVisualDeclarationRef, UiVisualDerivedPixelArtifactInput, UiVisualEvidenceRef,
    UiVisualGraphNodeRef, UiVisualHitTestOutcome, UiVisualHitTestTarget,
    UiVisualIdentityContinuity, UiVisualIdentityTrace, UiVisualIdentityTraceInput,
    UiVisualInspectionCostLane, UiVisualInspectionCostReceipt, UiVisualMountedNodeRef,
    UiVisualNativePixelArtifactInput, UiVisualOverlayDenial, UiVisualPixelArtifact,
    UiVisualPixelArtifactValidity, UiVisualPixelCaptureSource, UiVisualPixelColorSpace,
    UiVisualPixelFormat, UiVisualPixelRetentionDisposition, UiVisualPointAdjudication,
    UiVisualQueryBudget, UiVisualRegionAdjudication, UiVisualRegionCompleteness,
    UiVisualRegionIntersection, UiVisualSnapshotAffinity, UiVisualSnapshotArtifactPosture,
    UiVisualSnapshotComparison, UiVisualSnapshotComparisonBudget,
    UiVisualSnapshotComparisonBudgetDenial, UiVisualSnapshotComparisonCost,
    UiVisualSnapshotComparisonDenial, UiVisualSnapshotComparisonDenialKind,
    UiVisualSnapshotComparisonExpiry, UiVisualSnapshotComparisonIncompatibility,
    UiVisualSnapshotComparisonInput, UiVisualSnapshotComparisonOmission,
    UiVisualSnapshotComparisonOutcome, UiVisualSnapshotDenial, UiVisualSnapshotEvidence,
    UiVisualSnapshotEvidenceInput, UiVisualSnapshotIndeterminate, UiVisualSnapshotOmission,
    UiVisualSnapshotRelation, UiVisualSnapshotSuperseded, UiVisualVisibleContributor,
    UiVisualVisibleOutcome,
};
pub use support_report::UiInspectionSupportReport;
