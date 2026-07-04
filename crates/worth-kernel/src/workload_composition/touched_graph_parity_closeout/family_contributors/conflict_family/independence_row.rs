use crate::workload_composition::{
    current_worth_touched_graph_conflict_milestone_fifteen_seed,
    current_worth_touched_graph_conflict_selected_route_packet,
};

use super::error::{
    ConflictFamilyContributorCatalogError, ConflictFamilyContributorCatalogErrorKind,
};
use super::row::{
    ConflictFamilyContributorCatalogRow, ConflictFamilyContributorRowKind,
    ConflictFamilyDenialWitnessKind,
};

pub(crate) fn current_independence_contributor_row(
) -> Result<ConflictFamilyContributorCatalogRow, ConflictFamilyContributorCatalogError> {
    let selected_route =
        current_worth_touched_graph_conflict_selected_route_packet().map_err(|error| {
            ConflictFamilyContributorCatalogError::new(
                ConflictFamilyContributorCatalogErrorKind::CurrentSurfaceUnavailable,
                error.detail(),
            )
        })?;
    let seed = current_worth_touched_graph_conflict_milestone_fifteen_seed().map_err(|error| {
        ConflictFamilyContributorCatalogError::new(
            ConflictFamilyContributorCatalogErrorKind::CurrentSurfaceUnavailable,
            format!("{error:?}"),
        )
    })?;

    ConflictFamilyContributorCatalogRow::new(
        ConflictFamilyContributorRowKind::Independence,
        "current_worth_touched_graph_conflict_selected_route_packet::conflict_family_independence_pre_execution_identity",
        "current_worth_touched_graph_conflict_milestone_fifteen_seed::{overlap_identity_digests,independence_proof_digests}",
        "current_worth_touched_graph_conflict_selected_route_packet::{conflict_independence_denial_witness_identity,conflict_independence_denial_witness_kind}",
        "current_worth_touched_graph_conflict_public_facade",
        "crates/worth-kernel/src/workload_composition/planner_owned_routing/public_facade/current.rs",
        selected_route.conflict_family_independence_pre_execution_identity(),
        vec![],
        seed.overlap_identity_digests().to_vec(),
        vec![],
        selected_route.independence_proof_digests().to_vec(),
        String::new(),
        selected_route.conflict_independence_denial_witness_identity().map(str::to_string),
        selected_route
            .conflict_independence_denial_witness_kind()
            .map(ConflictFamilyDenialWitnessKind::ConflictIndependence),
        &[
            "overlap_identity_digests",
            "independence_proof_digests",
            "conflict_family_independence_pre_execution_identity",
            "selected_family_identity",
            "selected_product_identity_digest",
        ],
        &[
            "conflict_independence_denial_witness_identity",
            "conflict_independence_denial_witness_kind",
        ],
    )
}
