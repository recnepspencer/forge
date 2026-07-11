pub use crate::access::planning::{
    S8AccessPlanCostEstimate, S8AccessPlanSelection, S8DeterministicSelectionRule,
    S8PlanFingerprint, S8PlanSelectionDenied, S8PlanningCapabilityGrant, S8SelectedAccessPlan,
    S8SelectionCandidateAudit, S8SelectionCandidateEligibility, S8SelectionCandidateOutcome,
    S8SelectionCandidateRejection,
};
pub use crate::access::shape::{
    access_shapes, S8AccessAuthorityPosture, S8AccessLaneClassification, S8AccessShape,
    S8AccessShapeContract, S8AccessShapeDetail, S8AccessShapeUnsupportedDenial,
    S8AccessStaleDisposition, S8BatchPointBasis, S8BoundedScanBasis, S8ChunkTreeWalkBasis,
    S8CoalescedPageReadBasis, S8DegradedExactScanBasis, S8DegradedExactScanRequest,
    S8ExpectedCounterClass, S8FullDeclaredScanBasis, S8GroupedPrefixBasis, S8MaintenanceReadBasis,
    S8ManifestGraphWalkBasis, S8MultiRangeBasis, S8MutationAccessBasis, S8PrefixBasis,
    S8RangeBasis, S8SortedBatchBasis, S8StreamingContinuationBasis, S8StreamingReadBasis,
};
pub use crate::facade::{access_planning, deterministic_plan_selection};
pub use crate::materialization::{
    S8AbsenceAuthorityClass, S8CoverageBasisKind, S8CoverageGapClass, S8CoverageGapWitness,
    S8LayoutCoverageWitness, S8LayoutMaterializationState, S8LayoutWatermark,
    S8MaterializationCompleteness, S8MaterializationDenial, S8MaterializationStateClass,
    S8PhysicalAbsenceProof, S8PhysicalCoverageBasis, S8PrefixCompletenessWitness,
    S8RangeCompletenessWitness,
};
