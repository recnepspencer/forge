//! Evidence topology grouped by named transition family.
//!
//! Lifecycle order: construction → measurement → planning → obligation.

pub(crate) mod construction;
mod dummy_measurement_family;
pub(crate) mod layout_operator;
pub(crate) mod measurement;
pub(crate) mod obligation;
pub(crate) mod planning;
pub(crate) mod shared;

pub(crate) use construction::{
    evidence_authority_binding, evidence_family_summary, evidence_handle, evidence_identity,
    evidence_ref, evidence_slice, order_refs, preflight_evidence_expansion, with_retention_posture,
    UiEvidenceSliceAssembly, UiEvidenceSliceAssemblyInput,
};
pub use layout_operator::{
    UiLayoutOperatorChildParticipationRule, UiLayoutOperatorContainmentKind,
    UiLayoutOperatorContractIdentity, UiLayoutOperatorCrossAxis, UiLayoutOperatorDenialPolicy,
    UiLayoutOperatorFamily, UiLayoutOperatorIntrinsicReturnPolicy, UiLayoutOperatorOverflowPolicy,
    UiLayoutOperatorPlanningContract, UiLayoutOperatorPlanningSemantics,
    UiLayoutOperatorPrimaryAxis, UiLayoutOperatorSiblingGroupingRule, UiLayoutOperatorSizingMode,
    UiLayoutOperatorSlotParticipationKind, UiLayoutOperatorSpecialInputRequirement,
};
pub use measurement::{
    admit_measurement_basis, certify_measurement_basis_determinism,
    certify_measurement_basis_determinism_for_scenarios, consume_declared_measurement_projection_facts,
    MeasurementEvidenceInput,
    UiChildIntrinsicMeasurementEvidence, UiChildIntrinsicMeasurementSource,
    UiCurrentMeasurementResult, UiMeasurementBasis, UiMeasurementBasisCertificationHostRequest,
    UiMeasurementBasisCertificationOutcome, UiMeasurementBasisCertificationReport,
    UiMeasurementBasisCertificationScenario, UiMeasurementBasisCertificationScenarioError,
    UiMeasurementBasisDenial, UiMeasurementBasisDeterminismPosture, UiMeasurementBasisGeneration,
    UiMeasurementBasisPosture, UiMeasurementCoordinateSpace, UiMeasurementDependencyLineage,
    UiMeasurementDependencyLineageEntry, UiMeasurementDependencyLineageKind,
    UiMeasurementDependencyMap, UiMeasurementDependencyMapEntry, UiMeasurementEvidenceCategory,
    UiMeasurementEvidenceSlot, UiMeasurementGenerationCompatibility,
    UiMeasurementNeighborhoodClassHint, UiMeasurementResult, UiMeasurementRoundingPosture,
    UiMeasurementSiblingResizeSupport, UiMeasurementSiblingResizeSupportSource,
    UiMeasurementUnitPosture, UiMeasurementValue, UiProjectionFactObservation,
    UiProjectionFactReceipt, UiProjectionFactReceiptDenial,
};
pub use obligation::{
    UiInspectionObligationEvidenceReceipt, UiInspectionObligationReasonProjection,
};
pub use planning::{
    certify_allocation_planning_determinism, certify_allocation_planning_suite,
    UiAllocationConstraintSet, UiAllocationConstraintSetIdentity,
    UiAllocationConstraintSummary, UiAllocationNeighborhood, UiAllocationNeighborhoodClass,
    UiAllocationNeighborhoodIdentity, UiAllocationNeighborhoodMember,
    UiAllocationNeighborhoodMemberRole, UiAllocationNeighborhoodMembershipRule,
    UiAllocationPlanningCertificationReport, UiAllocationPlanningCertificationSuiteKind,
    UiAllocationPlanningCostClass, UiAllocationPlanningCostReceipt,
    UiAllocationPlanningDeniedBroadeningReason, UiAllocationPlanningDeterminismPosture,
    UiAllocationPlanningEvidenceDetail, UiAllocationPlanningEvidenceFamily,
    UiAllocationPlanningInspectionReceipt, UiAllocationSolveConvergencePosture,
    UiAllocationSolvePass, UiAllocationSolveRemainderPolicy, UiAllocationSolveTrace,
    UiBoundReconciliationPosture, UiBoundReconciliationSolveOrder, UiConstraintAvailableSpacePosture,
    UiConstraintAxisScope, UiConstraintBoundReconciliationMember, UiConstraintBoundReconciliationResult,
    UiConstraintChildIntrinsicContribution, UiConstraintCycleParticipationPosture,
    UiConstraintEqualShareDistributionPolicy, UiConstraintEqualShareDistributionResult,
    UiConstraintEqualShareMember, UiConstraintEqualSharePosture, UiConstraintEqualShareSolveOrder,
    UiConstraintHostIntrinsicKind, UiConstraintIntrinsicSourcePosture,
    UiConstraintNormalizationPosture, UiConstraintParentAvailableSpace,
    UiConstraintPortalAnchorPlanningInputResult, UiConstraintPropagationDenial,
    UiConstraintPropagationDenialReason, UiConstraintPropagationEdge,
    UiConstraintPropagationEdgeFamily, UiConstraintPropagationEdgePayload,
    UiConstraintResizeInputPosture, UiConstraintScrollOwnerPlanningInputResult,
    UiConstraintSiblingNegotiationFixedPointPolicy, UiConstraintSiblingNegotiationGroup,
    UiConstraintSiblingNegotiationMember, UiConstraintSiblingNegotiationResult,
    UiConstraintSiblingNegotiationSolveOrder, UiConstraintSpecialInputPosture,
    UiConstraintViewportPlanningInputResult, UiPortalAnchorPlanningInputPosture,
    UiPortalAnchorPlanningInputSolveOrder, UiScrollOwnerPlanningInputPosture,
    UiScrollOwnerPlanningInputSolveOrder, UiViewportPlanningInputPosture,
    UiViewportPlanningInputSolveOrder, UiConstraintBoundedMinMaxRequirement,
    UiConstraintEqualShareGroup, UiConstraintResizePermissionPosture,
    UiConstraintSiblingNegotiationMode,
};
pub(crate) use measurement::{
    admit_declared_measurement_projection_fact_receipt, derive_measurement_dependency_map,
    derive_measurement_neighborhood_class_hint, project_measurement_inspection_compatibility_view,
    project_measurement_inspection_denial_view, project_measurement_inspection_view,
};
#[cfg(test)]
pub(crate) use measurement::{
    host_measurement_request_shape_digest, measurement_result_identity_digest,
};
pub(crate) use planning::{
    convergence_posture_for_cycle_and_denial, project_allocation_planning_inspection_receipt,
    remainder_policy_for_equal_share,
};
pub use shared::evidence_expansion::UiEvidenceExpansion;
pub use shared::evidence_family::UiEvidenceFamily;
pub use shared::evidence_family_summary::UiEvidenceFamilySummary;
pub use shared::evidence_handle::UiEvidenceHandle;
pub use shared::evidence_identity::UiEvidenceIdentity;
pub use shared::evidence_materialized_detail::UiEvidenceMaterializedDetail;
pub use shared::evidence_reference::UiEvidenceRef;
pub use shared::evidence_slice::UiEvidenceSlice;
pub use shared::evidence_slice_ref::UiEvidenceSliceRef;
pub(crate) use shared::query_measurement_fact_family_digest::query_measurement_fact_family_set_digest;
pub use worth_ui_inspection::{
    UiEvidenceAuthorityArtifactIdentity, UiEvidenceAuthorityBinding, UiEvidenceAuthorityGeneration,
    UiEvidenceAuthorityKind, UiEvidenceExpansionOutcome, UiEvidenceMaterializationPosture,
    UiEvidenceRetentionPosture, UiEvidenceSliceOmission, UiInspectionCostReceipt,
};
pub(crate) use construction::cost_receipt::UiInspectionCostMetrics;
