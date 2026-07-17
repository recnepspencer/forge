use crate::declaration::stable_text_digest;
use crate::evidence::{
    UiAllocationConstraintSet, UiBoundReconciliationPosture, UiConstraintBoundReconciliationResult,
    UiConstraintPropagationDenial, UiConstraintPropagationDenialReason,
    UiConstraintPropagationEdge, UiConstraintPropagationEdgeFamily,
    UiConstraintPropagationEdgePayload,
};

use super::admission_parts::ConstraintAuthorityContext;
use super::classify_special_inputs::family_for_requirement;

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
    edges: &mut [UiConstraintPropagationEdge],
) -> Option<UiConstraintBoundReconciliationResult> {
    let reconciliation = bound_reconciliation?;
    if !edges.iter().any(|edge| {
        edge.family() == UiConstraintPropagationEdgeFamily::BoundedReconciliation
            && edge.cycle_participation_posture()
                == crate::evidence::UiConstraintCycleParticipationPosture::AdmittedFixedPoint
    }) {
        return Some(reconciliation.clone());
    }
    let cycled = UiConstraintBoundReconciliationResult::new(
        crate::evidence::UiConstraintBoundReconciliationInput {
            neighborhood_identity_digest: reconciliation.neighborhood_identity_digest(),
            axis_scope: reconciliation.axis_scope(),
            requirement: reconciliation.requirement(),
            solve_order: reconciliation.solve_order(),
            posture: UiBoundReconciliationPosture::Cyclic,
            incoming_available_space_posture: reconciliation.incoming_available_space_posture(),
            viewport_requirement: reconciliation.viewport_requirement(),
            scroll_owner_requirement: reconciliation.scroll_owner_requirement(),
            portal_anchor_requirement: reconciliation.portal_anchor_requirement(),
            unit_posture: reconciliation.unit_posture(),
            coordinate_space: reconciliation.coordinate_space(),
            rounding_posture: reconciliation.rounding_posture(),
            members: reconciliation.members().to_vec(),
        },
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
    edges: &mut [UiConstraintPropagationEdge],
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
    input: crate::evidence::UiAllocationConstraintSetInput,
) -> Result<UiAllocationConstraintSet, UiConstraintPropagationDenial> {
    Ok(UiAllocationConstraintSet::new_with_sibling_negotiation(
        super::super::UiGraphConstraintMintAuthority::mint(),
        input,
    ))
}
