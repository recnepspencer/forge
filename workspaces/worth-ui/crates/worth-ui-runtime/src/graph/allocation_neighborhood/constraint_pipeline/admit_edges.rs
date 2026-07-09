use crate::evidence::{
    UiAllocationNeighborhood, UiConstraintBoundReconciliationResult, UiConstraintPropagationDenial,
    UiConstraintPropagationEdge, UiConstraintPropagationEdgeFamily, UiMeasurementBasis,
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
use super::super::constraint_scroll_owner_planning_input::admit_scroll_owner_planning_input;
use super::super::constraint_sibling_negotiation::admit_sibling_negotiation;
use super::super::constraint_summary::derive_constraint_summary;
use super::super::constraint_viewport_planning_input::admit_viewport_planning_input;
use super::types::{
    ConstraintAuthorityContext, PropagationEdgeAdmissionParts, SpecialInputAdmissionParts,
};

use super::verify_construct::{
    apply_cyclic_bound_reconciliation_posture, verify_required_special_inputs,
};

pub(super) fn admit_propagation_edge_families(
    measurement_basis: &UiMeasurementBasis,
    neighborhood: &UiAllocationNeighborhood,
    context: &ConstraintAuthorityContext<'_>,
    required_special_families: &[UiConstraintPropagationEdgeFamily],
    observed_special_families: &[UiConstraintPropagationEdgeFamily],
) -> Result<PropagationEdgeAdmissionParts, UiConstraintPropagationDenial> {
    let special_inputs = admit_required_special_inputs(
        measurement_basis,
        neighborhood,
        context,
        required_special_families,
    )?;
    let downward =
        admit_parent_available_space(measurement_basis, neighborhood, context.allowed_families)?;
    let intrinsic = admit_child_intrinsic_contributions(
        measurement_basis,
        neighborhood,
        context.allowed_families,
    )?;
    let summary = derive_constraint_summary(
        downward.incoming_available_space(),
        downward.incoming_available_space_posture(),
        downward.bounded_min_max_requirement(),
        downward.normalization_posture(),
        context.contract.semantics().child_participation_rule(),
        context.allowed_families,
        required_special_families,
    );
    let downward_bounded_targets = downward.bounded_targets().to_vec();

    verify_required_special_inputs(
        context,
        observed_special_families,
        required_special_families,
    )?;

    let edges = assemble_initial_propagation_edges(
        context,
        &special_inputs,
        downward,
        intrinsic,
        measurement_basis,
        neighborhood,
        summary,
    )?;
    let (sibling_negotiation, equal_share_distribution, bound_reconciliation, mut edges) =
        admit_negotiation_and_reconciliation_edges(
            measurement_basis,
            neighborhood,
            context,
            summary,
            &downward_bounded_targets,
            edges,
        )?;

    edges = admit_cycle_postures(
        edges,
        context.admitted_cycle_families,
        context.neighborhood_identity_digest,
        context.contract_identity_digest,
    )?;
    let bound_reconciliation =
        apply_cyclic_bound_reconciliation_posture(bound_reconciliation.as_ref(), &mut edges);

    Ok(PropagationEdgeAdmissionParts {
        summary,
        viewport_planning_input: special_inputs.viewport_planning_input,
        scroll_owner_planning_input: special_inputs.scroll_owner_planning_input,
        portal_anchor_planning_input: special_inputs.portal_anchor_planning_input,
        sibling_negotiation,
        equal_share_distribution,
        bound_reconciliation,
        edges,
    })
}

pub(super) fn admit_required_special_inputs(
    measurement_basis: &UiMeasurementBasis,
    neighborhood: &UiAllocationNeighborhood,
    context: &ConstraintAuthorityContext<'_>,
    required_special_families: &[UiConstraintPropagationEdgeFamily],
) -> Result<SpecialInputAdmissionParts, UiConstraintPropagationDenial> {
    let viewport_planning_input = admit_viewport_planning_input(
        measurement_basis,
        neighborhood,
        required_special_families.contains(&UiConstraintPropagationEdgeFamily::ViewportInput),
        context.allowed_families,
    )?;
    let scroll_owner_planning_input = admit_scroll_owner_planning_input(
        measurement_basis,
        neighborhood,
        required_special_families.contains(&UiConstraintPropagationEdgeFamily::ScrollViewportInput),
        context.allowed_families,
    )?;
    let portal_anchor_planning_input = admit_portal_anchor_planning_input(
        measurement_basis,
        neighborhood,
        required_special_families.contains(&UiConstraintPropagationEdgeFamily::PortalAnchorInput),
        context.allowed_families,
    )?;
    Ok(SpecialInputAdmissionParts {
        viewport_planning_input,
        scroll_owner_planning_input,
        portal_anchor_planning_input,
    })
}

pub(super) fn assemble_initial_propagation_edges(
    context: &ConstraintAuthorityContext<'_>,
    special_inputs: &SpecialInputAdmissionParts,
    downward: UiConstraintDownwardAdmission,
    intrinsic: Vec<UiConstraintPropagationEdge>,
    measurement_basis: &UiMeasurementBasis,
    neighborhood: &UiAllocationNeighborhood,
    summary: crate::evidence::UiAllocationConstraintSummary,
) -> Result<Vec<UiConstraintPropagationEdge>, UiConstraintPropagationDenial> {
    let special_input_edges = assemble_special_input_edges(
        context.root_identity_digest,
        special_inputs.viewport_planning_input.as_ref(),
        special_inputs.scroll_owner_planning_input.as_ref(),
        special_inputs.portal_anchor_planning_input.as_ref(),
    );
    let mut edges =
        assemble_base_propagation_edges(downward.into_edges(), intrinsic, special_input_edges);
    edges.extend(admit_durable_resize_inputs(
        measurement_basis,
        neighborhood,
        summary,
        context.allowed_families,
    ));
    Ok(edges)
}

pub(super) fn admit_negotiation_and_reconciliation_edges(
    measurement_basis: &UiMeasurementBasis,
    neighborhood: &UiAllocationNeighborhood,
    context: &ConstraintAuthorityContext<'_>,
    summary: crate::evidence::UiAllocationConstraintSummary,
    downward_bounded_targets: &[(u64, crate::evidence::UiConstraintAxisScope)],
    mut edges: Vec<UiConstraintPropagationEdge>,
) -> Result<
    (
        Option<crate::evidence::UiConstraintSiblingNegotiationResult>,
        Option<crate::evidence::UiConstraintEqualShareDistributionResult>,
        Option<UiConstraintBoundReconciliationResult>,
        Vec<UiConstraintPropagationEdge>,
    ),
    UiConstraintPropagationDenial,
> {
    let sibling_negotiation_admission =
        admit_sibling_negotiation(neighborhood, summary, &edges, context.allowed_families)?;
    let sibling_negotiation = sibling_negotiation_admission.result().cloned();
    edges.extend(sibling_negotiation_admission.into_edges());
    let equal_share_admission = admit_equal_share_distribution(
        measurement_basis,
        neighborhood,
        summary,
        sibling_negotiation.as_ref(),
        context.allowed_families,
    )?;
    let equal_share_distribution = equal_share_admission.result().cloned();
    edges.extend(equal_share_admission.into_edges());
    let bound_reconciliation_admission = admit_bound_reconciliation(
        measurement_basis,
        neighborhood,
        summary,
        sibling_negotiation.as_ref(),
        equal_share_distribution.as_ref(),
        downward_bounded_targets,
        context.allowed_families,
    );
    let bound_reconciliation = bound_reconciliation_admission.result().cloned();
    let (_, bound_edges) = bound_reconciliation_admission.into_parts();
    edges.extend(bound_edges);
    Ok((
        sibling_negotiation,
        equal_share_distribution,
        bound_reconciliation,
        edges,
    ))
}
