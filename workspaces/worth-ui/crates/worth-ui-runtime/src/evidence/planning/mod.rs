pub(crate) mod allocation_solve;
pub(crate) mod certification;
pub(crate) mod constraint_propagation;
pub(crate) mod constraint_set;
pub(crate) mod inspection_receipt;
pub(crate) mod neighborhood;
#[cfg(test)]
mod tests;

pub use allocation_solve::{
    UiAllocationSolveConvergencePosture, UiAllocationSolvePass, UiAllocationSolveRemainderPolicy,
    UiAllocationSolveTrace,
};
pub(crate) use allocation_solve::{
    convergence_posture_for_cycle_and_denial, remainder_policy_for_equal_share,
};
pub use certification::{
    certify_allocation_planning_determinism, certify_allocation_planning_suite,
    UiAllocationPlanningCertificationReport, UiAllocationPlanningCertificationSuiteKind,
    UiAllocationPlanningDeterminismPosture,
};
pub use constraint_propagation::{
    UiBoundReconciliationPosture, UiBoundReconciliationSolveOrder,
    UiConstraintAvailableSpacePosture, UiConstraintAxisScope,
    UiConstraintBoundReconciliationMember, UiConstraintBoundReconciliationResult,
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
    UiConstraintSiblingNegotiationSolveOrder, UiConstraintViewportPlanningInputResult,
    UiPortalAnchorPlanningInputPosture,
    UiPortalAnchorPlanningInputSolveOrder, UiScrollOwnerPlanningInputPosture,
    UiScrollOwnerPlanningInputSolveOrder, UiViewportPlanningInputPosture,
    UiViewportPlanningInputSolveOrder,
};
pub use constraint_set::{
    UiAllocationConstraintSet, UiAllocationConstraintSetIdentity, UiAllocationConstraintSummary,
    UiConstraintBoundedMinMaxRequirement, UiConstraintEqualShareGroup,
    UiConstraintResizePermissionPosture, UiConstraintSiblingNegotiationMode,
    UiConstraintSpecialInputPosture,
};
pub use inspection_receipt::{
    UiAllocationPlanningCostClass,
    UiAllocationPlanningCostReceipt, UiAllocationPlanningDeniedBroadeningReason,
    UiAllocationPlanningEvidenceDetail, UiAllocationPlanningEvidenceFamily,
    UiAllocationPlanningInspectionReceipt,
};
pub(crate) use inspection_receipt::project_allocation_planning_inspection_receipt;
pub use neighborhood::{
    UiAllocationNeighborhood, UiAllocationNeighborhoodClass, UiAllocationNeighborhoodIdentity,
    UiAllocationNeighborhoodMember, UiAllocationNeighborhoodMemberRole,
    UiAllocationNeighborhoodMembershipRule,
};