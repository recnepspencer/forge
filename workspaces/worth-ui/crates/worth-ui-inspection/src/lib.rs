//! Public inspection surfaces grouped by lifecycle authority:
//! target identity → query admission → receipt projection → evidence contract → posture → scope.

mod allocation;
mod evidence_contract;
mod facade;
mod posture;
mod query;
mod receipt;
mod scope;
mod target;

// Target identity lane
pub use target::{
    UiAuthoredSourceProvenanceRef, UiInspectionAspectName, UiInspectionDeclarationIdentity,
    UiInspectionTarget, UiSourceArtifactGeneration, UiSourceArtifactIdentity,
};

// Evidence contract lane
pub use allocation::{
    UiAllocationInspectionAnchorPosture, UiAllocationInspectionAttemptResult,
    UiAllocationInspectionAxis, UiAllocationInspectionBounds,
    UiAllocationInspectionCoordinateSpace, UiAllocationInspectionDenialFamily,
    UiAllocationInspectionDeniedAttempt, UiAllocationInspectionEdgeReference,
    UiAllocationInspectionEvidenceFamily, UiAllocationInspectionEvidenceRef,
    UiAllocationInspectionFreshnessPosture, UiAllocationInspectionGeometry,
    UiAllocationInspectionGraphNodeIdentity, UiAllocationInspectionInvalidationFamily,
    UiAllocationInspectionKnowledge, UiAllocationInspectionNeighborhoodIdentity,
    UiAllocationInspectionPlanningBasisIdentity, UiAllocationInspectionPortalAnchorTargetIdentity,
    UiAllocationInspectionReceipt, UiAllocationInspectionReceiptIdentity,
    UiAllocationInspectionReceiptProjection, UiAllocationInspectionReuseDenialPosture,
    UiAllocationInspectionReusePosture, UiAllocationInspectionSelection,
    UiAllocationInspectionStreamFamily,
};
pub use evidence_contract::{
    UiEvidenceAuthorityArtifactIdentity, UiEvidenceAuthorityBinding, UiEvidenceAuthorityGeneration,
    UiEvidenceAuthorityKind, UiEvidenceExpansionOutcome, UiEvidenceFamily,
    UiEvidenceMaterializationPosture, UiEvidenceRetentionPosture,
    UiInspectionForeignEvidenceCitation, UiInspectionForeignEvidenceRef,
    UiInspectionQueryForeignEvidenceArtifactKind, UiInspectionQueryForeignEvidenceCitation,
    UiInspectionQueryForeignEvidenceKind, UiInspectionQueryForeignEvidenceRef,
};

// Posture lane
pub use posture::{
    UiInspectionAdmissionPosture, UiInspectionDeferredPosture, UiInspectionDiagnosticOnlyPosture,
    UiInspectionMilestoneExpectation, UiInspectionPosture, UiInspectionSupportPosture,
    UiInspectionSupportReason, UiInspectionSupportStatus, UiInspectionSupportWorld,
    UiInspectionUnsupportedPosture, UiInspectionWrongWorldPosture,
};

// Query admission lane
pub use query::{
    UiAllocationPlanningQuestion, UiEvidenceBudget, UiEvidenceLinkKind, UiEvidenceRichness,
    UiInspectionAspectRelevanceDetail, UiInspectionEvidenceSource,
    UiInspectionObligationRelevanceDetail, UiInspectionQuery, UiInspectionRelevance,
    UiInspectionRelevanceAdmission, UiInspectionRelevanceOutcome, UiInspectionTargetClass,
    UiRelevanceFamily, UiRelevanceFilter,
};

// Receipt projection lane
pub use receipt::evidence::{
    UiEvidenceSliceOmission, UiInspectionAdmissionHostCapability, UiInspectionAdmissionQueryBasis,
    UiInspectionAdmissionStaleEvidence, UiInspectionObligationDecision,
    UiInspectionObligationDenialPosture, UiInspectionObligationDispatchPosture,
    UiInspectionObligationFamily, UiInspectionObligationLegalityReason,
    UiInspectionObligationNonSelectionReason, UiInspectionObligationSelectionReason,
    UiInspectionObligationSupportSelectionPosture, UiInspectionObligationVerdictClass,
    UiInspectionObligationVerdictPosture, UiInspectionObligationWorldProfileClass,
    UiInspectionSelectionBudget, UiInspectionSupportRowSchemaKind, UiInspectionTouchAspectPosture,
    UiInspectionTouchOriginClass, UiInspectionTouchRuntimeLane, UiInspectionTouchTargetClass,
};
pub use receipt::{
    UiInspectionAiHarnessLane, UiInspectionClosedSemanticLane, UiInspectionCloseoutGuarantee,
    UiInspectionCloseoutNonGoal, UiInspectionCloseoutReport, UiInspectionClosureReport,
    UiInspectionCostLane, UiInspectionCostReceipt, UiInspectionDerivedIndexLane,
    UiInspectionMeasurementBasisInput, UiInspectionMeasurementBasisPosture,
    UiInspectionMeasurementBasisSource, UiInspectionMeasurementChildIntrinsicSource,
    UiInspectionMeasurementDenialPosture, UiInspectionMeasurementDependencyLineageEntry,
    UiInspectionMeasurementDependencyLineageKind, UiInspectionMeasurementEvidenceCategory,
    UiInspectionMeasurementEvidenceSlot, UiInspectionMeasurementEvidenceView,
    UiInspectionMeasurementEvidenceViewInput, UiInspectionMeasurementFailureSource,
    UiInspectionMeasurementGenerationCompatibility, UiInspectionMeasurementNeighborhoodClassHint,
    UiInspectionMeasurementOwnershipPosture, UiInspectionMeasurementQueryFactFamily,
    UiInspectionMeasurementQueryUnsupportedReason, UiInspectionQueryWorldCompatibilityFailure,
    UiInspectionRefLifecycleLane, UiInspectionScopeSupportRow, UiInspectionSliceLane,
    UiInspectionSupportReport,
};

// Scope lane
pub use scope::UiInspectionScope;

// Facade inventory lane
pub use facade::{UiInspectionScopeInventory, RUNTIME_INSPECTION_SCOPE_INVENTORY};
