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

pub(crate) fn current_batch_admission_contributor_row(
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
        ConflictFamilyContributorRowKind::BatchAdmission,
        "current_worth_touched_graph_conflict_selected_route_packet::conflict_family_batch_pre_execution_identity",
        "current_worth_touched_graph_conflict_milestone_fifteen_seed::{overlap_identity_digests,selected_conflict_plan_digests,independence_proof_digests,selected_batch_plan_digest}",
        "current_worth_touched_graph_conflict_selected_route_packet::{batch_admission_denial_witness_identity,batch_admission_denial_witness_kind}",
        "current_worth_touched_graph_conflict_selected_route_packet",
        "crates/worth-kernel/src/workload_composition/planner_owned_routing/selected_route/current.rs",
        selected_route.conflict_family_batch_pre_execution_identity(),
        vec![
            selected_route.conflict_family_conflict_pre_execution_identity(),
            selected_route.conflict_family_independence_pre_execution_identity(),
        ],
        seed.overlap_identity_digests().to_vec(),
        selected_route.selected_conflict_plan_digests().to_vec(),
        selected_route.independence_proof_digests().to_vec(),
        selected_route.selected_batch_plan_digest().to_string(),
        selected_route.batch_admission_denial_witness_identity().map(str::to_string),
        selected_route
            .batch_admission_denial_witness_kind()
            .map(ConflictFamilyDenialWitnessKind::BatchAdmission),
        &[
            "overlap_identity_digests",
            "selected_conflict_plan_digests",
            "independence_proof_digests",
            "selected_batch_plan_digest",
            "conflict_family_batch_pre_execution_identity",
            "selected_family_identity",
            "selected_product_identity_digest",
        ],
        &[
            "batch_admission_denial_witness_identity",
            "batch_admission_denial_witness_kind",
        ],
    )
}
