use crate::evidence::{
    UiConstraintBoundReconciliationResult, UiConstraintPropagationEdge,
    UiConstraintPropagationEdgeFamily, UiLayoutOperatorSpecialInputRequirement,
};

pub(super) struct ConstraintAuthorityContext<'a> {
    pub(super) contract: &'a crate::evidence::UiLayoutOperatorPlanningContract,
    pub(super) neighborhood_identity_digest: u64,
    pub(super) contract_identity_digest: u64,
    pub(super) allowed_families: &'a [UiConstraintPropagationEdgeFamily],
    pub(super) admitted_cycle_families: &'a [UiConstraintPropagationEdgeFamily],
    pub(super) root_identity_digest: u64,
    pub(super) special_input_requirements: &'a [UiLayoutOperatorSpecialInputRequirement],
}

pub(super) struct PropagationEdgeAdmissionParts {
    pub(super) summary: crate::evidence::UiAllocationConstraintSummary,
    pub(super) viewport_planning_input: Option<crate::evidence::UiConstraintViewportPlanningInputResult>,
    pub(super) scroll_owner_planning_input: Option<crate::evidence::UiConstraintScrollOwnerPlanningInputResult>,
    pub(super) portal_anchor_planning_input: Option<crate::evidence::UiConstraintPortalAnchorPlanningInputResult>,
    pub(super) sibling_negotiation: Option<crate::evidence::UiConstraintSiblingNegotiationResult>,
    pub(super) equal_share_distribution: Option<crate::evidence::UiConstraintEqualShareDistributionResult>,
    pub(super) bound_reconciliation: Option<UiConstraintBoundReconciliationResult>,
    pub(super) edges: Vec<UiConstraintPropagationEdge>,
}

pub(super) struct SpecialInputAdmissionParts {
    pub(super) viewport_planning_input: Option<crate::evidence::UiConstraintViewportPlanningInputResult>,
    pub(super) scroll_owner_planning_input:
        Option<crate::evidence::UiConstraintScrollOwnerPlanningInputResult>,
    pub(super) portal_anchor_planning_input:
        Option<crate::evidence::UiConstraintPortalAnchorPlanningInputResult>,
}
