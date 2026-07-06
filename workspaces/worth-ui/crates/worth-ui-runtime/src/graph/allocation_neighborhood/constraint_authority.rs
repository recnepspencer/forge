use crate::declaration::stable_text_digest;
use crate::evidence::{
    UiAllocationConstraintSet, UiAllocationNeighborhood, UiBoundReconciliationPosture,
    UiConstraintBoundReconciliationResult, UiConstraintPropagationDenial,
    UiConstraintPropagationDenialReason, UiConstraintPropagationEdge,
    UiConstraintPropagationEdgeFamily, UiConstraintPropagationEdgePayload,
    UiLayoutOperatorSpecialInputRequirement, UiMeasurementBasis,
};

use super::constraint_bound_reconciliation::admit_bound_reconciliation;
use super::constraint_child_intrinsic_contribution::admit_child_intrinsic_contributions;
use super::constraint_cycle_posture::admit_cycle_postures;
use super::constraint_durable_resize_input::admit_durable_resize_inputs;
use super::constraint_equal_share_distribution::admit_equal_share_distribution;
use super::constraint_parent_available_space::admit_parent_available_space;
use super::constraint_portal_anchor_planning_input::admit_portal_anchor_planning_input;
use super::constraint_sibling_negotiation::admit_sibling_negotiation;
use super::constraint_scroll_owner_planning_input::admit_scroll_owner_planning_input;
use super::constraint_summary::{derive_constraint_summary, special_input_families_from_basis};
use super::constraint_viewport_planning_input::admit_viewport_planning_input;

pub(super) fn admit_constraint_set(
    measurement_basis: &UiMeasurementBasis,
    neighborhood: &UiAllocationNeighborhood,
) -> Result<UiAllocationConstraintSet, UiConstraintPropagationDenial> {
    let contract = neighborhood.layout_operator_planning_contract();
    let contract_identity = contract.identity();
    let neighborhood_identity_digest = neighborhood.identity().identity_digest();
    let allowed_families = contract.semantics().allowed_propagation_families();
    let admitted_cycle_families = contract.semantics().admitted_cycle_families();
    let root = neighborhood
        .members()
        .iter()
        .find(|member| {
            matches!(
                member.role(),
                crate::evidence::UiAllocationNeighborhoodMemberRole::Root
            )
        })
        .expect("allocation neighborhood must preserve a root member");
    let root_identity_digest = root.identity_digest();
    let observed_special_families = special_input_families_from_basis(measurement_basis);
    let required_special_families = contract
        .semantics()
        .special_input_requirements()
        .iter()
        .copied()
        .map(family_for_requirement)
        .collect::<Vec<_>>();
    let viewport_required = required_special_families
        .contains(&UiConstraintPropagationEdgeFamily::ViewportInput);
    let scroll_owner_required = required_special_families
        .contains(&UiConstraintPropagationEdgeFamily::ScrollViewportInput);
    let portal_anchor_required = required_special_families
        .contains(&UiConstraintPropagationEdgeFamily::PortalAnchorInput);
    let viewport_planning_input = admit_viewport_planning_input(
        measurement_basis,
        neighborhood,
        viewport_required,
        allowed_families,
    )?;
    let scroll_owner_planning_input = admit_scroll_owner_planning_input(
        measurement_basis,
        neighborhood,
        scroll_owner_required,
        allowed_families,
    )?;
    let portal_anchor_planning_input = admit_portal_anchor_planning_input(
        measurement_basis,
        neighborhood,
        portal_anchor_required,
        allowed_families,
    )?;
    let intrinsic =
        admit_child_intrinsic_contributions(measurement_basis, neighborhood, allowed_families)?;
    let downward = admit_parent_available_space(measurement_basis, neighborhood, allowed_families)?;
    let summary = derive_constraint_summary(
        downward.incoming_available_space(),
        downward.incoming_available_space_posture(),
        downward.bounded_min_max_requirement(),
        downward.normalization_posture(),
        contract.semantics().child_participation_rule(),
        allowed_families,
        &required_special_families,
    );
    let downward_bounded_targets = downward.bounded_targets().to_vec();

    for requirement in contract.semantics().special_input_requirements() {
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
                neighborhood_identity_digest,
                contract_identity.identity_digest(),
                Some(required_family),
                contract.semantics().identity_digest(),
            ));
        }
    }

    let mut edges = Vec::new();
    edges.extend(downward.into_edges());
    edges.extend(intrinsic);
    if let Some(viewport_planning_input) = viewport_planning_input.as_ref() {
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
    if let Some(scroll_owner_planning_input) = scroll_owner_planning_input.as_ref() {
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
    if let Some(portal_anchor_planning_input) = portal_anchor_planning_input.as_ref() {
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
    edges.extend(admit_durable_resize_inputs(
        measurement_basis,
        neighborhood,
        summary,
        allowed_families,
    ));

    let sibling_negotiation_admission =
        admit_sibling_negotiation(neighborhood, summary, &edges, allowed_families)?;
    let sibling_negotiation = sibling_negotiation_admission.result().cloned();
    edges.extend(sibling_negotiation_admission.into_edges());
    let equal_share_admission = admit_equal_share_distribution(
        measurement_basis,
        neighborhood,
        summary,
        sibling_negotiation.as_ref(),
        allowed_families,
    )?;
    let equal_share_distribution = equal_share_admission.result().cloned();
    edges.extend(equal_share_admission.into_edges());
    let bound_reconciliation_admission = admit_bound_reconciliation(
        measurement_basis,
        neighborhood,
        summary,
        sibling_negotiation.as_ref(),
        equal_share_distribution.as_ref(),
        &downward_bounded_targets,
        allowed_families,
    );
    let mut bound_reconciliation = bound_reconciliation_admission.result().cloned();
    let (_, bound_edges) = bound_reconciliation_admission.into_parts();
    edges.extend(bound_edges);

    for family in required_special_families {
        if matches!(
            family,
            UiConstraintPropagationEdgeFamily::ViewportInput
                | UiConstraintPropagationEdgeFamily::ScrollViewportInput
                | UiConstraintPropagationEdgeFamily::PortalAnchorInput
        ) {
            continue;
        }
    }

    let mut edges = admit_cycle_postures(
        edges,
        admitted_cycle_families,
        neighborhood_identity_digest,
        contract_identity.identity_digest(),
    )?;
    if let Some(reconciliation) = bound_reconciliation.as_ref() {
        if edges.iter().any(|edge| {
            edge.family() == UiConstraintPropagationEdgeFamily::BoundedReconciliation
                && edge.cycle_participation_posture()
                    == crate::evidence::UiConstraintCycleParticipationPosture::AdmittedFixedPoint
        }) {
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
            for edge in &mut edges {
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
            bound_reconciliation = Some(cycled);
        }
    }
    edges.sort_unstable_by_key(UiConstraintPropagationEdge::canonical_sort_key);
    for window in edges.windows(2) {
        if window[0].identity_digest() == window[1].identity_digest() {
            return Err(UiConstraintPropagationDenial::new(
                UiConstraintPropagationDenialReason::DuplicateEdgeAuthority,
                neighborhood_identity_digest,
                contract_identity.identity_digest(),
                Some(window[0].family()),
                window[0].identity_digest()
                    ^ stable_text_digest("worth-ui.constraint-duplicate-edge").rotate_left(7),
            ));
        }
    }

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

fn family_for_requirement(
    requirement: UiLayoutOperatorSpecialInputRequirement,
) -> UiConstraintPropagationEdgeFamily {
    match requirement {
        UiLayoutOperatorSpecialInputRequirement::ViewportExtent => {
            UiConstraintPropagationEdgeFamily::ViewportInput
        }
        UiLayoutOperatorSpecialInputRequirement::ScrollViewportExtent => {
            UiConstraintPropagationEdgeFamily::ScrollViewportInput
        }
        UiLayoutOperatorSpecialInputRequirement::PortalAnchorRect => {
            UiConstraintPropagationEdgeFamily::PortalAnchorInput
        }
    }
}
