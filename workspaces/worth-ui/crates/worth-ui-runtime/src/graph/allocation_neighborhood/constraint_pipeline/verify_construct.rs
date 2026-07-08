use crate::declaration::stable_text_digest;
use crate::evidence::{
    UiAllocationConstraintSet, UiAllocationNeighborhood, UiBoundReconciliationPosture,
    UiConstraintBoundReconciliationResult, UiConstraintPropagationDenial,
    UiConstraintPropagationDenialReason, UiConstraintPropagationEdge,
    UiConstraintPropagationEdgeFamily, UiConstraintPropagationEdgePayload,
    UiLayoutOperatorContractIdentity, UiMeasurementBasis,
};

use super::super::constraint_bound_reconciliation::admit_bound_reconciliation;
use super::super::constraint_child_intrinsic_contribution::admit_child_intrinsic_contributions;
use super::super::constraint_cycle_posture::admit_cycle_postures;
use super::super::constraint_durable_resize_input::admit_durable_resize_inputs;
use super::super::constraint_edge_assembly::{
    assemble_base_propagation_edges, assemble_special_input_edges,
};
use super::super::constraint_equal_share_distribution::admit_equal_share_distribution;
use super::super::constraint_parent_available_space::{
    admit_parent_available_space, UiConstraintDownwardAdmission,
};
use super::super::constraint_portal_anchor_planning_input::admit_portal_anchor_planning_input;
use super::super::constraint_sibling_negotiation::admit_sibling_negotiation;
use super::super::constraint_scroll_owner_planning_input::admit_scroll_owner_planning_input;
use super::super::constraint_summary::derive_constraint_summary;
use super::super::constraint_viewport_planning_input::admit_viewport_planning_input;
use super::classify_special_inputs::family_for_requirement;
use super::types::{
    ConstraintAuthorityContext, PropagationEdgeAdmissionParts, SpecialInputAdmissionParts,
};


pub(super) fn verify_required_special_inputs(
    context: &ConstraintAuthorityContext<'_>,
    observed_special_families: &[UiConstraintPropagationEdgeFamily],
    _required_special_families: &[UiConstraintPropagationEdgeFamily],
) -> Result<(), UiConstraintPropagationDenial> {
    for requirement in context.special_input_requirements {
        let required_family = family_for_requirement(*requirement);
        if matches!(
            required_family,
            UiConstraintPropagationEdgeFamily::ViewportInput
                | UiConstraintPropagationEdgeFamily::ScrollViewportInput
                | UiConstraintPropagationEdgeFamily::PortalAnchorInput
        ) {
            continue;
        }
        if !observed_special_families.contains(&required_family) {
            return Err(UiConstraintPropagationDenial::new(
                UiConstraintPropagationDenialReason::MissingRequiredSpecialInput,
                context.neighborhood_identity_digest,
                context.contract_identity_digest,
                Some(required_family),
                context.contract.semantics().identity_digest(),
            ));
        }
    }
    Ok(())
}

pub(super) fn apply_cyclic_bound_reconciliation_posture(
    bound_reconciliation: Option<&UiConstraintBoundReconciliationResult>,
    edges: &mut Vec<UiConstraintPropagationEdge>,
) -> Option<UiConstraintBoundReconciliationResult> {
    let Some(reconciliation) = bound_reconciliation else {
        return None;
    };
    if !edges.iter().any(|edge| {
        edge.family() == UiConstraintPropagationEdgeFamily::BoundedReconciliation
            && edge.cycle_participation_posture()
                == crate::evidence::UiConstraintCycleParticipationPosture::AdmittedFixedPoint
    }) {
        return Some(reconciliation.clone());
    }
    let cycled = UiConstraintBoundReconciliationResult::new(
        reconciliation.neighborhood_identity_digest(),
        reconciliation.axis_scope(),
        reconciliation.requirement(),
        reconciliation.solve_order(),
        UiBoundReconciliationPosture::Cyclic,
        reconciliation.incoming_available_space_posture(),
        reconciliation.viewport_requirement(),
        reconciliation.scroll_owner_requirement(),
        reconciliation.portal_anchor_requirement(),
        reconciliation.unit_posture(),
        reconciliation.coordinate_space(),
        reconciliation.rounding_posture(),
        reconciliation.members().to_vec(),
    );
    for edge in edges.iter_mut() {
        if edge.family() == UiConstraintPropagationEdgeFamily::BoundedReconciliation {
            *edge = UiConstraintPropagationEdge::new(
                UiConstraintPropagationEdgeFamily::BoundedReconciliation,
                edge.source_member_identity_digest(),
                edge.target_member_identity_digest(),
                UiConstraintPropagationEdgePayload::BoundedReconciliation {
                    axis_scope: cycled.axis_scope(),
                    reconciliation_identity_digest: cycled.identity_digest(),
                    solve_order: cycled.solve_order(),
                    posture: cycled.posture(),
                },
                edge.cycle_participation_posture(),
            );
        }
    }
    Some(cycled)
}

pub(super) fn verify_unique_edge_authority(
    edges: &mut Vec<UiConstraintPropagationEdge>,
    neighborhood_identity_digest: u64,
    contract_identity_digest: u64,
) -> Result<(), UiConstraintPropagationDenial> {
    edges.sort_unstable_by_key(UiConstraintPropagationEdge::canonical_sort_key);
    for window in edges.windows(2) {
        if window[0].identity_digest() == window[1].identity_digest() {
            return Err(UiConstraintPropagationDenial::new(
                UiConstraintPropagationDenialReason::DuplicateEdgeAuthority,
                neighborhood_identity_digest,
                contract_identity_digest,
                Some(window[0].family()),
                window[0].identity_digest()
                    ^ stable_text_digest("worth-ui.constraint-duplicate-edge").rotate_left(7),
            ));
        }
    }
    Ok(())
}

pub(super) fn construct_constraint_set(
    neighborhood_identity_digest: u64,
    contract_identity: UiLayoutOperatorContractIdentity,
    summary: crate::evidence::UiAllocationConstraintSummary,
    viewport_planning_input: Option<crate::evidence::UiConstraintViewportPlanningInputResult>,
    scroll_owner_planning_input: Option<crate::evidence::UiConstraintScrollOwnerPlanningInputResult>,
    portal_anchor_planning_input: Option<crate::evidence::UiConstraintPortalAnchorPlanningInputResult>,
    sibling_negotiation: Option<crate::evidence::UiConstraintSiblingNegotiationResult>,
    equal_share_distribution: Option<crate::evidence::UiConstraintEqualShareDistributionResult>,
    bound_reconciliation: Option<UiConstraintBoundReconciliationResult>,
    edges: Vec<UiConstraintPropagationEdge>,
) -> Result<UiAllocationConstraintSet, UiConstraintPropagationDenial> {
    Ok(UiAllocationConstraintSet::new_with_sibling_negotiation(
        neighborhood_identity_digest,
        contract_identity,
        summary,
        viewport_planning_input,
        scroll_owner_planning_input,
        portal_anchor_planning_input,
        sibling_negotiation,
        equal_share_distribution,
        bound_reconciliation,
        edges,
    ))
}

