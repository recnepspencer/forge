//! Constraint set admission — proof-flow TOC for allocation neighborhood planning.
//!
//! collect authority → classify specials → admit edges → verify edge authority → construct set
//!
//! Each step lives in a named module so future readers can open the transition they care about
//! without reconstructing the pipeline from a bag of private functions.

mod admit_edges;
mod classify_special_inputs;
mod collect_authority;
mod types;
mod verify_construct;

use crate::evidence::{
    UiAllocationConstraintSet, UiAllocationNeighborhood, UiConstraintPropagationDenial,
    UiMeasurementBasis,
};

use super::constraint_summary::special_input_families_from_basis;
use admit_edges::admit_propagation_edge_families;
use classify_special_inputs::classify_special_input_requirements;
use collect_authority::collect_constraint_authority_context;
use verify_construct::{construct_constraint_set, verify_unique_edge_authority};

/// Orchestration: admits a constraint set only after authority collection, special-input
/// classification, edge admission, and unique-edge verification.
pub(super) fn admit_constraint_set(
    measurement_basis: &UiMeasurementBasis,
    neighborhood: &UiAllocationNeighborhood,
) -> Result<UiAllocationConstraintSet, UiConstraintPropagationDenial> {
    let context = collect_constraint_authority_context(measurement_basis, neighborhood);
    let required_special_families = classify_special_input_requirements(&context);
    let observed_special_families = special_input_families_from_basis(measurement_basis);
    let mut parts = admit_propagation_edge_families(
        measurement_basis,
        neighborhood,
        &context,
        &required_special_families,
        &observed_special_families,
    )?;
    verify_unique_edge_authority(
        &mut parts.edges,
        context.neighborhood_identity_digest,
        context.contract_identity_digest,
    )?;
    construct_constraint_set(
        context.neighborhood_identity_digest,
        context.contract.identity(),
        parts.summary,
        parts.viewport_planning_input,
        parts.scroll_owner_planning_input,
        parts.portal_anchor_planning_input,
        parts.sibling_negotiation,
        parts.equal_share_distribution,
        parts.bound_reconciliation,
        parts.edges,
    )
}
