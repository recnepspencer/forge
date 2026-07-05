use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::LoopFixtureEntryOrder;
use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanLoopOverlapParticipationMap, PlanarBooleanLoopOverlapParticipationRow,
    PlanarBooleanOverlapAdjacencyIndexInput, PlanarBooleanOverlapChainRegionLineageMap,
    PlanarBooleanOverlapChainRegionLineageRow,
};

use super::super::PlanarBooleanOverlapRegionAdjacencyIndex;
use super::fixtures::recovered_participation;

#[test]
fn overlap_adjacency_is_replay_stable_for_real_participation_products() {
    let canonical = recovered_participation(LoopFixtureEntryOrder::Canonical);
    let replayed = recovered_participation(LoopFixtureEntryOrder::Replayed);

    let canonical_index = PlanarBooleanOverlapRegionAdjacencyIndex::admit(
        PlanarBooleanOverlapAdjacencyIndexInput::from_participation_products(
            canonical.loop_participation_map(),
            canonical.island_participation_map(),
            canonical.chain_lineage_map(),
        ),
    )
    .expect("canonical participation should admit adjacency indexing");
    let replayed_index = PlanarBooleanOverlapRegionAdjacencyIndex::admit(
        PlanarBooleanOverlapAdjacencyIndexInput::from_participation_products(
            replayed.loop_participation_map(),
            replayed.island_participation_map(),
            replayed.chain_lineage_map(),
        ),
    )
    .expect("replayed participation should admit adjacency indexing");

    assert_eq!(canonical_index, replayed_index);
    assert_eq!(
        canonical_index.ordering_basis(),
        replayed_index.ordering_basis()
    );
    assert!(
        canonical_index
            .rows()
            .iter()
            .any(|row| !row.source_edge_identities().is_empty()),
        "adjacency rows should preserve source-edge provenance"
    );
}

#[test]
fn overlap_adjacency_aggregates_shared_connectivity_lineage_rows_into_one_neighborhood() {
    let recovered = recovered_participation(LoopFixtureEntryOrder::Canonical);
    let canonical_index = PlanarBooleanOverlapRegionAdjacencyIndex::admit(
        PlanarBooleanOverlapAdjacencyIndexInput::from_participation_products(
            recovered.loop_participation_map(),
            recovered.island_participation_map(),
            recovered.chain_lineage_map(),
        ),
    )
    .expect("canonical participation should admit adjacency indexing");

    let base_lineage_row = recovered.chain_lineage_map().rows()[0].clone();
    let extra_lineage_identity = format!("{}-aggregated", base_lineage_row.lineage_identity());
    let shared_loop_identities = base_lineage_row.participating_loop_identities().to_vec();
    let augmented_loop_map = PlanarBooleanLoopOverlapParticipationMap::new(
        recovered
            .loop_participation_map()
            .map_identity()
            .to_string(),
        recovered
            .loop_participation_map()
            .request_identity()
            .to_string(),
        recovered
            .loop_participation_map()
            .rows()
            .iter()
            .map(|row| {
                let mut overlap_chain_lineage_identities =
                    row.overlap_chain_lineage_identities().to_vec();
                if shared_loop_identities.contains(&row.canonical_loop_identity().to_string()) {
                    overlap_chain_lineage_identities.push(extra_lineage_identity.clone());
                    overlap_chain_lineage_identities.sort();
                    overlap_chain_lineage_identities.dedup();
                }
                PlanarBooleanLoopOverlapParticipationRow::new(
                    row.participation_identity().to_string(),
                    row.ledger_row_identity().to_string(),
                    row.canonical_loop_identity().to_string(),
                    row.tracked_loop_identity().to_string(),
                    row.loop_kind(),
                    row.loop_role(),
                    row.role_outcome_identity().to_string(),
                    row.island_identity().to_string(),
                    row.island_origin_loop_identity().to_string(),
                    row.island_kind(),
                    row.source_loop_identities().to_vec(),
                    row.source_loop_operand_sides().to_vec(),
                    row.source_loop_winding_signs().to_vec(),
                    row.propagated_persistent_name_identities().to_vec(),
                    overlap_chain_lineage_identities,
                )
            })
            .collect(),
    );
    let augmented_chain_map = PlanarBooleanOverlapChainRegionLineageMap::new(
        recovered.chain_lineage_map().map_identity().to_string(),
        recovered.chain_lineage_map().request_identity().to_string(),
        recovered
            .chain_lineage_map()
            .rows()
            .iter()
            .cloned()
            .chain(std::iter::once(
                PlanarBooleanOverlapChainRegionLineageRow::new(
                    format!("{}-aggregated-row", base_lineage_row.lineage_row_identity()),
                    extra_lineage_identity.clone(),
                    base_lineage_row.chain_identity().to_string(),
                    vec![format!(
                        "{}-alternate-fragment",
                        base_lineage_row.fragment_identities()[0]
                    )],
                    base_lineage_row.source_loop_identities().to_vec(),
                    base_lineage_row.source_loop_operand_sides().to_vec(),
                    base_lineage_row.source_loop_winding_signs().to_vec(),
                    vec![format!(
                        "{}-alternate-edge",
                        base_lineage_row.source_edge_identities()[0]
                    )],
                    base_lineage_row.boundary_roles().to_vec(),
                    base_lineage_row.participating_loop_identities().to_vec(),
                    base_lineage_row.participating_island_identities().to_vec(),
                    base_lineage_row
                        .propagated_persistent_name_identities()
                        .to_vec(),
                ),
            ))
            .collect(),
    );

    let augmented_index = PlanarBooleanOverlapRegionAdjacencyIndex::admit(
        PlanarBooleanOverlapAdjacencyIndexInput::from_participation_products(
            &augmented_loop_map,
            recovered.island_participation_map(),
            &augmented_chain_map,
        ),
    )
    .expect("shared-connectivity lineage rows should aggregate into one neighborhood");

    assert_eq!(
        augmented_index.rows().len(),
        canonical_index.rows().len(),
        "adding a second lineage row in the same chain neighborhood should not add a second neighborhood row"
    );
    assert!(
        augmented_index.rows().iter().any(|row| {
            row.lineage_identities().contains(&base_lineage_row.lineage_identity().to_string())
                && row.lineage_identities().contains(&extra_lineage_identity)
                && row.fragment_identities().iter().any(|id| id.contains("alternate-fragment"))
                && row.source_edge_identities().iter().any(|id| id.contains("alternate-edge"))
        }),
        "adjacency neighborhoods should aggregate lineage rows when one canonical chain connectivity neighborhood survives multiple lineage decompositions"
    );
}

#[test]
fn overlap_adjacency_keeps_disconnected_shared_participants_as_separate_neighborhoods() {
    let recovered = recovered_participation(LoopFixtureEntryOrder::Canonical);
    let canonical_index = PlanarBooleanOverlapRegionAdjacencyIndex::admit(
        PlanarBooleanOverlapAdjacencyIndexInput::from_participation_products(
            recovered.loop_participation_map(),
            recovered.island_participation_map(),
            recovered.chain_lineage_map(),
        ),
    )
    .expect("canonical participation should admit adjacency indexing");

    let base_lineage_row = recovered.chain_lineage_map().rows()[0].clone();
    let extra_lineage_identity = format!("{}-disconnected", base_lineage_row.lineage_identity());
    let shared_loop_identities = base_lineage_row.participating_loop_identities().to_vec();
    let augmented_loop_map = PlanarBooleanLoopOverlapParticipationMap::new(
        recovered
            .loop_participation_map()
            .map_identity()
            .to_string(),
        recovered
            .loop_participation_map()
            .request_identity()
            .to_string(),
        recovered
            .loop_participation_map()
            .rows()
            .iter()
            .map(|row| {
                let mut overlap_chain_lineage_identities =
                    row.overlap_chain_lineage_identities().to_vec();
                if shared_loop_identities.contains(&row.canonical_loop_identity().to_string()) {
                    overlap_chain_lineage_identities.push(extra_lineage_identity.clone());
                    overlap_chain_lineage_identities.sort();
                    overlap_chain_lineage_identities.dedup();
                }
                PlanarBooleanLoopOverlapParticipationRow::new(
                    row.participation_identity().to_string(),
                    row.ledger_row_identity().to_string(),
                    row.canonical_loop_identity().to_string(),
                    row.tracked_loop_identity().to_string(),
                    row.loop_kind(),
                    row.loop_role(),
                    row.role_outcome_identity().to_string(),
                    row.island_identity().to_string(),
                    row.island_origin_loop_identity().to_string(),
                    row.island_kind(),
                    row.source_loop_identities().to_vec(),
                    row.source_loop_operand_sides().to_vec(),
                    row.source_loop_winding_signs().to_vec(),
                    row.propagated_persistent_name_identities().to_vec(),
                    overlap_chain_lineage_identities,
                )
            })
            .collect(),
    );
    let augmented_chain_map = PlanarBooleanOverlapChainRegionLineageMap::new(
        recovered.chain_lineage_map().map_identity().to_string(),
        recovered.chain_lineage_map().request_identity().to_string(),
        recovered
            .chain_lineage_map()
            .rows()
            .iter()
            .cloned()
            .chain(std::iter::once(
                PlanarBooleanOverlapChainRegionLineageRow::new(
                    format!(
                        "{}-disconnected-row",
                        base_lineage_row.lineage_row_identity()
                    ),
                    extra_lineage_identity,
                    format!("{}-disconnected", base_lineage_row.chain_identity()),
                    vec![format!(
                        "{}-disconnected-fragment",
                        base_lineage_row.fragment_identities()[0]
                    )],
                    base_lineage_row.source_loop_identities().to_vec(),
                    base_lineage_row.source_loop_operand_sides().to_vec(),
                    base_lineage_row.source_loop_winding_signs().to_vec(),
                    vec![format!(
                        "{}-disconnected-edge",
                        base_lineage_row.source_edge_identities()[0]
                    )],
                    base_lineage_row.boundary_roles().to_vec(),
                    base_lineage_row.participating_loop_identities().to_vec(),
                    base_lineage_row.participating_island_identities().to_vec(),
                    base_lineage_row
                        .propagated_persistent_name_identities()
                        .to_vec(),
                ),
            ))
            .collect(),
    );

    let augmented_index = PlanarBooleanOverlapRegionAdjacencyIndex::admit(
        PlanarBooleanOverlapAdjacencyIndexInput::from_participation_products(
            &augmented_loop_map,
            recovered.island_participation_map(),
            &augmented_chain_map,
        ),
    )
    .expect(
        "disconnected neighborhoods with shared participants should still admit adjacency indexing",
    );

    assert_eq!(
        augmented_index.rows().len(),
        canonical_index.rows().len() + 1,
        "adding a disconnected lineage neighborhood with the same participants should add a distinct adjacency row"
    );
}

#[test]
fn overlap_adjacency_ignores_benign_participant_order_variation() {
    let recovered = recovered_participation(LoopFixtureEntryOrder::Canonical);
    let reordered_chain_map = PlanarBooleanOverlapChainRegionLineageMap::new(
        recovered.chain_lineage_map().map_identity().to_string(),
        recovered.chain_lineage_map().request_identity().to_string(),
        recovered
            .chain_lineage_map()
            .rows()
            .iter()
            .map(|row| {
                let mut participating_loop_identities =
                    row.participating_loop_identities().to_vec();
                participating_loop_identities.reverse();
                let mut participating_island_identities =
                    row.participating_island_identities().to_vec();
                participating_island_identities.reverse();
                PlanarBooleanOverlapChainRegionLineageRow::new(
                    row.lineage_row_identity().to_string(),
                    row.lineage_identity().to_string(),
                    row.chain_identity().to_string(),
                    row.fragment_identities().to_vec(),
                    row.source_loop_identities().to_vec(),
                    row.source_loop_operand_sides().to_vec(),
                    row.source_loop_winding_signs().to_vec(),
                    row.source_edge_identities().to_vec(),
                    row.boundary_roles().to_vec(),
                    participating_loop_identities,
                    participating_island_identities,
                    row.propagated_persistent_name_identities().to_vec(),
                )
            })
            .collect(),
    );

    let canonical_index = PlanarBooleanOverlapRegionAdjacencyIndex::admit(
        PlanarBooleanOverlapAdjacencyIndexInput::from_participation_products(
            recovered.loop_participation_map(),
            recovered.island_participation_map(),
            recovered.chain_lineage_map(),
        ),
    )
    .expect("canonical participation should admit adjacency indexing");
    let reordered_index = PlanarBooleanOverlapRegionAdjacencyIndex::admit(
        PlanarBooleanOverlapAdjacencyIndexInput::from_participation_products(
            recovered.loop_participation_map(),
            recovered.island_participation_map(),
            &reordered_chain_map,
        ),
    )
    .expect("benign participant-order variation should preserve adjacency admission");

    assert_eq!(canonical_index, reordered_index);
}
