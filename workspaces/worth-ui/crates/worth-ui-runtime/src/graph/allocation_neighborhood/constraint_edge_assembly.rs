use crate::evidence::{
    UiConstraintPortalAnchorPlanningInputResult, UiConstraintPropagationEdge,
    UiConstraintPropagationEdgeFamily, UiConstraintPropagationEdgePayload,
    UiConstraintScrollOwnerPlanningInputResult, UiConstraintViewportPlanningInputResult,
};

pub(super) fn assemble_special_input_edges(
    root_identity_digest: u64,
    viewport_planning_input: Option<&UiConstraintViewportPlanningInputResult>,
    scroll_owner_planning_input: Option<&UiConstraintScrollOwnerPlanningInputResult>,
    portal_anchor_planning_input: Option<&UiConstraintPortalAnchorPlanningInputResult>,
) -> Vec<UiConstraintPropagationEdge> {
    let mut edges = Vec::new();
    if let Some(viewport_planning_input) = viewport_planning_input {
        edges.push(UiConstraintPropagationEdge::new(
            UiConstraintPropagationEdgeFamily::ViewportInput,
            root_identity_digest,
            root_identity_digest,
            UiConstraintPropagationEdgePayload::ViewportInput {
                viewport_identity_digest: viewport_planning_input.identity_digest(),
                solve_order: viewport_planning_input.solve_order(),
                posture: viewport_planning_input.posture(),
                planning_time_only: viewport_planning_input.is_planning_time_only(),
            },
            crate::evidence::UiConstraintCycleParticipationPosture::Acyclic,
        ));
    }
    if let Some(scroll_owner_planning_input) = scroll_owner_planning_input {
        edges.push(UiConstraintPropagationEdge::new(
            UiConstraintPropagationEdgeFamily::ScrollViewportInput,
            root_identity_digest,
            root_identity_digest,
            UiConstraintPropagationEdgePayload::ScrollViewportInput {
                scroll_identity_digest: scroll_owner_planning_input.identity_digest(),
                solve_order: scroll_owner_planning_input.solve_order(),
                posture: scroll_owner_planning_input.posture(),
                planning_time_only: scroll_owner_planning_input.is_planning_time_only(),
            },
            crate::evidence::UiConstraintCycleParticipationPosture::Acyclic,
        ));
    }
    if let Some(portal_anchor_planning_input) = portal_anchor_planning_input {
        edges.push(UiConstraintPropagationEdge::new(
            UiConstraintPropagationEdgeFamily::PortalAnchorInput,
            root_identity_digest,
            root_identity_digest,
            UiConstraintPropagationEdgePayload::PortalAnchorInput {
                portal_identity_digest: portal_anchor_planning_input.identity_digest(),
                solve_order: portal_anchor_planning_input.solve_order(),
                posture: portal_anchor_planning_input.posture(),
                planning_time_only: portal_anchor_planning_input.is_planning_time_only(),
            },
            crate::evidence::UiConstraintCycleParticipationPosture::Acyclic,
        ));
    }
    edges
}

pub(super) fn assemble_base_propagation_edges(
    downward_edges: Vec<UiConstraintPropagationEdge>,
    intrinsic_edges: Vec<UiConstraintPropagationEdge>,
    special_input_edges: Vec<UiConstraintPropagationEdge>,
) -> Vec<UiConstraintPropagationEdge> {
    let mut edges = downward_edges;
    edges.extend(intrinsic_edges);
    edges.extend(special_input_edges);
    edges
}