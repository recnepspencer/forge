use crate::evidence::{
    convergence_posture_for_cycle_and_denial, remainder_policy_for_equal_share,
    UiAllocationConstraintSetIdentity, UiAllocationConstraintSummary, UiAllocationNeighborhood,
    UiAllocationSolvePass, UiAllocationSolveTrace, UiConstraintCycleParticipationPosture,
    UiConstraintPortalAnchorPlanningInputResult, UiConstraintPropagationEdge,
    UiConstraintResizePermissionPosture, UiConstraintScrollOwnerPlanningInputResult,
    UiConstraintViewportPlanningInputResult, UiMeasurementBasisDenial,
};
use crate::runtime::{
    WorthUiAllocationPlanning, WorthUiAllocationPlanningCounters, WorthUiAllocationPlanningDenial,
    WorthUiPlanLoweringDenial,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiAllocationPlanningInspection {
    constraint_set_identity: Option<UiAllocationConstraintSetIdentity>,
    neighborhood: UiAllocationNeighborhood,
    constraint_summary: Option<UiAllocationConstraintSummary>,
    viewport_planning_input: Option<UiConstraintViewportPlanningInputResult>,
    scroll_owner_planning_input: Option<UiConstraintScrollOwnerPlanningInputResult>,
    portal_anchor_planning_input: Option<UiConstraintPortalAnchorPlanningInputResult>,
    propagation_edges: Box<[UiConstraintPropagationEdge]>,
    denial: Option<WorthUiAllocationPlanningDenial>,
    counters: WorthUiAllocationPlanningCounters,
    solve_trace: UiAllocationSolveTrace,
}

impl WorthUiAllocationPlanningInspection {
    pub(crate) fn from_planning(planning: &WorthUiAllocationPlanning) -> Self {
        let constraint_set = planning.allocation_constraint_set();
        Self {
            solve_trace: solve_trace_for(planning, constraint_set),
            constraint_set_identity: constraint_set
                .map(|retained_constraint_set| retained_constraint_set.identity()),
            neighborhood: planning.allocation_neighborhood().clone(),
            constraint_summary: constraint_set
                .map(|retained_constraint_set| retained_constraint_set.summary()),
            viewport_planning_input: constraint_set.and_then(|retained_constraint_set| {
                retained_constraint_set.viewport_planning_input().cloned()
            }),
            scroll_owner_planning_input: constraint_set.and_then(|retained_constraint_set| {
                retained_constraint_set
                    .scroll_owner_planning_input()
                    .cloned()
            }),
            portal_anchor_planning_input: constraint_set.and_then(|retained_constraint_set| {
                retained_constraint_set
                    .portal_anchor_planning_input()
                    .cloned()
            }),
            propagation_edges: constraint_set
                .map(|retained_constraint_set| retained_constraint_set.propagation_edges().to_vec())
                .unwrap_or_default()
                .into_boxed_slice(),
            denial: planning.denial_posture().cloned(),
            counters: planning.counters(),
        }
    }

    pub fn planning_identity_digest(&self) -> u64 {
        self.solve_trace().planning_identity_digest()
    }

    pub fn measurement_basis_identity_digest(&self) -> u64 {
        self.neighborhood()
            .identity()
            .measurement_basis_identity_digest()
    }

    pub fn neighborhood_identity_digest(&self) -> u64 {
        self.neighborhood().identity().identity_digest()
    }

    pub fn constraint_set_identity(&self) -> Option<UiAllocationConstraintSetIdentity> {
        self.constraint_set_identity
    }

    pub fn constraint_set_identity_digest(&self) -> Option<u64> {
        self.constraint_set_identity
            .map(UiAllocationConstraintSetIdentity::identity_digest)
    }

    pub fn neighborhood(&self) -> &UiAllocationNeighborhood {
        &self.neighborhood
    }

    pub fn constraint_summary(&self) -> Option<UiAllocationConstraintSummary> {
        self.constraint_summary
    }

    pub fn viewport_planning_input(&self) -> Option<&UiConstraintViewportPlanningInputResult> {
        self.viewport_planning_input.as_ref()
    }

    pub fn scroll_owner_planning_input(
        &self,
    ) -> Option<&UiConstraintScrollOwnerPlanningInputResult> {
        self.scroll_owner_planning_input.as_ref()
    }

    pub fn portal_anchor_planning_input(
        &self,
    ) -> Option<&UiConstraintPortalAnchorPlanningInputResult> {
        self.portal_anchor_planning_input.as_ref()
    }

    pub fn propagation_edges(&self) -> &[UiConstraintPropagationEdge] {
        &self.propagation_edges
    }

    pub fn durable_resize_posture(&self) -> Option<UiConstraintResizePermissionPosture> {
        self.constraint_summary
            .map(|summary| summary.resize_permission_posture())
    }

    pub fn denial(&self) -> Option<&WorthUiAllocationPlanningDenial> {
        self.denial.as_ref()
    }

    pub fn measurement_basis_denial(&self) -> Option<&UiMeasurementBasisDenial> {
        self.denial
            .as_ref()
            .and_then(WorthUiAllocationPlanningDenial::measurement_basis_denial)
    }

    pub fn constraint_set_denial(&self) -> Option<&crate::evidence::UiConstraintPropagationDenial> {
        self.denial
            .as_ref()
            .and_then(WorthUiAllocationPlanningDenial::constraint_set_denial)
    }

    pub fn plan_lowering_denial(&self) -> Option<&WorthUiPlanLoweringDenial> {
        self.denial
            .as_ref()
            .and_then(WorthUiAllocationPlanningDenial::plan_lowering_denial)
    }

    pub fn counters(&self) -> WorthUiAllocationPlanningCounters {
        self.counters
    }

    pub fn solve_trace(&self) -> &UiAllocationSolveTrace {
        &self.solve_trace
    }
}

fn solve_trace_for(
    planning: &WorthUiAllocationPlanning,
    constraint_set: Option<&crate::evidence::UiAllocationConstraintSet>,
) -> UiAllocationSolveTrace {
    let mut pass_order = Vec::new();
    if constraint_set
        .and_then(|retained_constraint_set| retained_constraint_set.viewport_planning_input())
        .is_some()
    {
        pass_order.push(UiAllocationSolvePass::ViewportInput);
    }
    if constraint_set
        .and_then(|retained_constraint_set| retained_constraint_set.scroll_owner_planning_input())
        .is_some()
    {
        pass_order.push(UiAllocationSolvePass::ScrollOwnerInput);
    }
    if constraint_set
        .and_then(|retained_constraint_set| retained_constraint_set.portal_anchor_planning_input())
        .is_some()
    {
        pass_order.push(UiAllocationSolvePass::PortalAnchorInput);
    }
    if constraint_set
        .and_then(|retained_constraint_set| retained_constraint_set.sibling_negotiation())
        .is_some()
    {
        pass_order.push(UiAllocationSolvePass::SiblingNegotiation);
    }
    if constraint_set
        .and_then(|retained_constraint_set| retained_constraint_set.equal_share_distribution())
        .is_some()
    {
        pass_order.push(UiAllocationSolvePass::EqualShareDistribution);
    }
    if constraint_set
        .and_then(|retained_constraint_set| retained_constraint_set.bound_reconciliation())
        .is_some()
    {
        pass_order.push(UiAllocationSolvePass::BoundedReconciliation);
    }
    if constraint_set.is_some_and(|retained_constraint_set| {
        retained_constraint_set
            .summary()
            .resize_permission_posture()
            == UiConstraintResizePermissionPosture::DurableAuthorityLane
    }) {
        pass_order.push(UiAllocationSolvePass::DurableResizeInput);
    }

    let cycle_posture = planning
        .allocation_constraint_set()
        .map(|retained_constraint_set| {
            highest_cycle_posture(retained_constraint_set.propagation_edges())
        });
    let fixed_point_policy = planning
        .allocation_constraint_set()
        .and_then(|retained_constraint_set| retained_constraint_set.sibling_negotiation())
        .map(|negotiation| negotiation.fixed_point_policy());
    let remainder_policy = remainder_policy_for_equal_share(
        planning
            .allocation_constraint_set()
            .and_then(|retained_constraint_set| retained_constraint_set.equal_share_distribution())
            .map(|distribution| distribution.policy()),
    );
    let normalization_posture =
        planning
            .allocation_constraint_set()
            .and_then(|retained_constraint_set| {
                retained_constraint_set
                    .propagation_edges()
                    .iter()
                    .find_map(|edge| {
                        edge.payload()
                            .parent_available_space()
                            .map(|available_space| available_space.normalization_posture())
                    })
            });

    UiAllocationSolveTrace::new(
        planning.planning_identity_digest(),
        pass_order,
        remainder_policy,
        convergence_posture_for_cycle_and_denial(
            cycle_posture,
            fixed_point_policy,
            planning.denial_posture().map(|denial| denial.reason()),
        ),
        planning
            .allocation_constraint_set()
            .map(|retained_constraint_set| {
                retained_constraint_set
                    .summary()
                    .resize_permission_posture()
            }),
        normalization_posture,
    )
}

fn highest_cycle_posture(
    propagation_edges: &[UiConstraintPropagationEdge],
) -> UiConstraintCycleParticipationPosture {
    propagation_edges
        .iter()
        .map(UiConstraintPropagationEdge::cycle_participation_posture)
        .max_by_key(|posture| match posture {
            UiConstraintCycleParticipationPosture::Acyclic => 0_u8,
            UiConstraintCycleParticipationPosture::AdmittedFixedPoint => 1_u8,
        })
        .unwrap_or(UiConstraintCycleParticipationPosture::Acyclic)
}
