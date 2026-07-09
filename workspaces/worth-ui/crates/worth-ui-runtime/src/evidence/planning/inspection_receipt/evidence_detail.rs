use crate::evidence::{
    UiAllocationConstraintSetIdentity, UiAllocationConstraintSummary, UiAllocationNeighborhood,
    UiAllocationSolveTrace, UiConstraintPortalAnchorPlanningInputResult,
    UiConstraintPropagationEdge, UiConstraintResizePermissionPosture,
    UiConstraintScrollOwnerPlanningInputResult, UiConstraintViewportPlanningInputResult,
};
use crate::runtime::{WorthUiAllocationPlanningDenial, WorthUiAllocationPlanningInspection};
use worth_ui_inspection::UiAllocationPlanningQuestion;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAllocationPlanningEvidenceDetail {
    constraint_set_identity: Option<UiAllocationConstraintSetIdentity>,
    neighborhood: UiAllocationNeighborhood,
    constraint_summary: Option<UiAllocationConstraintSummary>,
    viewport_planning_input: Option<UiConstraintViewportPlanningInputResult>,
    scroll_owner_planning_input: Option<UiConstraintScrollOwnerPlanningInputResult>,
    portal_anchor_planning_input: Option<UiConstraintPortalAnchorPlanningInputResult>,
    propagation_edges: Box<[UiConstraintPropagationEdge]>,
    solve_trace: UiAllocationSolveTrace,
    denial: Option<WorthUiAllocationPlanningDenial>,
}

impl UiAllocationPlanningEvidenceDetail {
    pub(crate) fn from_inspection(inspection: &WorthUiAllocationPlanningInspection) -> Self {
        Self {
            constraint_set_identity: inspection.constraint_set_identity(),
            neighborhood: inspection.neighborhood().clone(),
            constraint_summary: inspection.constraint_summary(),
            viewport_planning_input: inspection.viewport_planning_input().cloned(),
            scroll_owner_planning_input: inspection.scroll_owner_planning_input().cloned(),
            portal_anchor_planning_input: inspection.portal_anchor_planning_input().cloned(),
            propagation_edges: inspection.propagation_edges().to_vec().into_boxed_slice(),
            solve_trace: inspection.solve_trace().clone(),
            denial: inspection.denial().cloned(),
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

    pub fn propagation_edges(&self) -> &[UiConstraintPropagationEdge] {
        &self.propagation_edges
    }

    pub fn solve_trace(&self) -> &UiAllocationSolveTrace {
        &self.solve_trace
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

    pub fn durable_resize_posture(&self) -> Option<UiConstraintResizePermissionPosture> {
        self.constraint_summary
            .map(|summary| summary.resize_permission_posture())
    }

    pub fn answers(&self, question: UiAllocationPlanningQuestion) -> bool {
        match question {
            UiAllocationPlanningQuestion::Neighborhood => true,
            UiAllocationPlanningQuestion::PropagationEdges => true,
            UiAllocationPlanningQuestion::SpecialInputs => {
                self.constraint_summary.is_some()
                    || self.viewport_planning_input.is_some()
                    || self.scroll_owner_planning_input.is_some()
                    || self.portal_anchor_planning_input.is_some()
            }
            UiAllocationPlanningQuestion::DurableResizePosture => self.constraint_summary.is_some(),
            _ => false,
        }
    }

    pub fn denial(&self) -> Option<&WorthUiAllocationPlanningDenial> {
        self.denial.as_ref()
    }
}
