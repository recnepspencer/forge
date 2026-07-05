use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::LoopFixtureEntryOrder;
use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanLoopIslandOverlapParticipationMap, PlanarBooleanLoopIslandOverlapParticipationRow,
    PlanarBooleanLoopOverlapParticipationMap, PlanarBooleanLoopOverlapParticipationRow,
    PlanarBooleanOverlapAdjacencyIndexDenialKind, PlanarBooleanOverlapAdjacencyIndexInput,
    PlanarBooleanOverlapChainRegionLineageMap, PlanarBooleanOverlapChainRegionLineageRow,
};

use super::super::PlanarBooleanOverlapRegionAdjacencyIndex;
use super::fixtures::recovered_participation;

#[test]
fn overlap_adjacency_rejects_unindexed_discovery_before_region_ordering() {
    let recovered = recovered_participation(LoopFixtureEntryOrder::Canonical);
    let hostile_chain_row = recovered.chain_lineage_map().rows()[0].clone();
    let hostile_chain_map = PlanarBooleanOverlapChainRegionLineageMap::new(
        recovered.chain_lineage_map().map_identity().to_string(),
        recovered.chain_lineage_map().request_identity().to_string(),
        vec![PlanarBooleanOverlapChainRegionLineageRow::new(
            hostile_chain_row.lineage_row_identity().to_string(),
            hostile_chain_row.lineage_identity().to_string(),
            hostile_chain_row.chain_identity().to_string(),
            hostile_chain_row.fragment_identities().to_vec(),
            hostile_chain_row.source_loop_identities().to_vec(),
            hostile_chain_row.source_loop_operand_sides().to_vec(),
            hostile_chain_row.source_loop_winding_signs().to_vec(),
            hostile_chain_row.source_edge_identities().to_vec(),
            hostile_chain_row.boundary_roles().to_vec(),
            vec!["foreign-loop".to_string()],
            hostile_chain_row.participating_island_identities().to_vec(),
            hostile_chain_row
                .propagated_persistent_name_identities()
                .to_vec(),
        )],
    );

    let denial = PlanarBooleanOverlapRegionAdjacencyIndex::admit(
        PlanarBooleanOverlapAdjacencyIndexInput::from_participation_products(
            recovered.loop_participation_map(),
            recovered.island_participation_map(),
            &hostile_chain_map,
        ),
    )
    .expect_err("adjacency should reject unindexed neighborhood discovery");

    assert_eq!(
        denial.kind(),
        PlanarBooleanOverlapAdjacencyIndexDenialKind::DanglingAdjacencyLineageDenied
    );
}

#[test]
fn overlap_adjacency_rejects_missing_promised_lineage_before_region_ordering() {
    let recovered = recovered_participation(LoopFixtureEntryOrder::Canonical);
    let missing_lineage_identity = recovered.loop_participation_map().rows()[0]
        .overlap_chain_lineage_identities()[0]
        .to_string();
    let hostile_chain_map = PlanarBooleanOverlapChainRegionLineageMap::new(
        recovered.chain_lineage_map().map_identity().to_string(),
        recovered.chain_lineage_map().request_identity().to_string(),
        recovered
            .chain_lineage_map()
            .rows()
            .iter()
            .filter(|row| row.lineage_identity() != missing_lineage_identity)
            .cloned()
            .collect(),
    );

    let denial = PlanarBooleanOverlapRegionAdjacencyIndex::admit(
        PlanarBooleanOverlapAdjacencyIndexInput::from_participation_products(
            recovered.loop_participation_map(),
            recovered.island_participation_map(),
            &hostile_chain_map,
        ),
    )
    .expect_err("adjacency should reject missing promised chain lineage");

    assert_eq!(
        denial.kind(),
        PlanarBooleanOverlapAdjacencyIndexDenialKind::UnindexedOverlapNeighborhoodDiscoveryDenied
    );
}

#[test]
fn overlap_adjacency_rejects_duplicate_loop_participation_before_region_ordering() {
    let recovered = recovered_participation(LoopFixtureEntryOrder::Canonical);
    let duplicated_loop = recovered.loop_participation_map().rows()[0].clone();
    let mut hostile_loop_rows = recovered.loop_participation_map().rows().to_vec();
    hostile_loop_rows.push(PlanarBooleanLoopOverlapParticipationRow::new(
        "hostile-duplicate-loop".to_string(),
        duplicated_loop.ledger_row_identity().to_string(),
        duplicated_loop.canonical_loop_identity().to_string(),
        duplicated_loop.tracked_loop_identity().to_string(),
        duplicated_loop.loop_kind(),
        duplicated_loop.loop_role(),
        "hostile-duplicate-role-outcome".to_string(),
        duplicated_loop.island_identity().to_string(),
        duplicated_loop.island_origin_loop_identity().to_string(),
        duplicated_loop.island_kind(),
        duplicated_loop.source_loop_identities().to_vec(),
        duplicated_loop.source_loop_operand_sides().to_vec(),
        duplicated_loop.source_loop_winding_signs().to_vec(),
        duplicated_loop
            .propagated_persistent_name_identities()
            .to_vec(),
        duplicated_loop.overlap_chain_lineage_identities().to_vec(),
    ));
    let hostile_loop_map = PlanarBooleanLoopOverlapParticipationMap::new(
        recovered
            .loop_participation_map()
            .map_identity()
            .to_string(),
        recovered
            .loop_participation_map()
            .request_identity()
            .to_string(),
        hostile_loop_rows,
    );

    let denial = PlanarBooleanOverlapRegionAdjacencyIndex::admit(
        PlanarBooleanOverlapAdjacencyIndexInput::from_participation_products(
            &hostile_loop_map,
            recovered.island_participation_map(),
            recovered.chain_lineage_map(),
        ),
    )
    .expect_err(
        "adjacency should reject duplicate loop participation before keyed lookup construction",
    );

    assert_eq!(
        denial.kind(),
        PlanarBooleanOverlapAdjacencyIndexDenialKind::ContradictoryAdjacencyNeighborhoodDenied
    );
}

#[test]
fn overlap_adjacency_rejects_duplicate_island_participation_before_region_ordering() {
    let recovered = recovered_participation(LoopFixtureEntryOrder::Canonical);
    let duplicated_island = recovered.island_participation_map().rows()[0].clone();
    let mut hostile_island_rows = recovered.island_participation_map().rows().to_vec();
    hostile_island_rows.push(PlanarBooleanLoopIslandOverlapParticipationRow::new(
        "hostile-duplicate-island".to_string(),
        duplicated_island.island_identity().to_string(),
        duplicated_island.island_origin_loop_identity().to_string(),
        duplicated_island.island_kind(),
        duplicated_island.member_loop_identities().to_vec(),
        duplicated_island.member_source_loop_identities().to_vec(),
        duplicated_island
            .member_source_loop_operand_sides()
            .to_vec(),
        duplicated_island
            .member_source_loop_winding_signs()
            .to_vec(),
        vec!["hostile-duplicate-role-outcome".to_string()],
        duplicated_island
            .propagated_persistent_name_identities()
            .to_vec(),
    ));
    let hostile_island_map = PlanarBooleanLoopIslandOverlapParticipationMap::new(
        recovered
            .island_participation_map()
            .map_identity()
            .to_string(),
        recovered
            .island_participation_map()
            .request_identity()
            .to_string(),
        hostile_island_rows,
    );

    let denial = PlanarBooleanOverlapRegionAdjacencyIndex::admit(
        PlanarBooleanOverlapAdjacencyIndexInput::from_participation_products(
            recovered.loop_participation_map(),
            &hostile_island_map,
            recovered.chain_lineage_map(),
        ),
    )
    .expect_err(
        "adjacency should reject duplicate island participation before keyed lookup construction",
    );

    assert_eq!(
        denial.kind(),
        PlanarBooleanOverlapAdjacencyIndexDenialKind::ContradictoryAdjacencyNeighborhoodDenied
    );
}

#[test]
fn overlap_adjacency_rejects_incidental_iteration_order_ties() {
    let recovered = recovered_participation(LoopFixtureEntryOrder::Canonical);
    let base_lineage_row = recovered.chain_lineage_map().rows()[0].clone();
    let duplicated_island = recovered.island_participation_map().rows()[0].clone();
    let hostile_island_identity = format!("{}-tie", duplicated_island.island_identity());
    let hostile_island_map = PlanarBooleanLoopIslandOverlapParticipationMap::new(
        recovered
            .island_participation_map()
            .map_identity()
            .to_string(),
        recovered
            .island_participation_map()
            .request_identity()
            .to_string(),
        recovered
            .island_participation_map()
            .rows()
            .iter()
            .cloned()
            .chain(std::iter::once(
                PlanarBooleanLoopIslandOverlapParticipationRow::new(
                    "hostile-tie-island".to_string(),
                    hostile_island_identity.clone(),
                    duplicated_island.island_origin_loop_identity().to_string(),
                    duplicated_island.island_kind(),
                    duplicated_island.member_loop_identities().to_vec(),
                    duplicated_island.member_source_loop_identities().to_vec(),
                    duplicated_island
                        .member_source_loop_operand_sides()
                        .to_vec(),
                    duplicated_island
                        .member_source_loop_winding_signs()
                        .to_vec(),
                    duplicated_island.member_role_outcome_identities().to_vec(),
                    duplicated_island
                        .propagated_persistent_name_identities()
                        .to_vec(),
                ),
            ))
            .collect(),
    );
    let hostile_chain_map = PlanarBooleanOverlapChainRegionLineageMap::new(
        recovered.chain_lineage_map().map_identity().to_string(),
        recovered.chain_lineage_map().request_identity().to_string(),
        recovered
            .chain_lineage_map()
            .rows()
            .iter()
            .map(|row| {
                if row.lineage_identity() == base_lineage_row.lineage_identity() {
                    let mut participating_island_identities =
                        row.participating_island_identities().to_vec();
                    participating_island_identities.push(hostile_island_identity.clone());
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
                        row.participating_loop_identities().to_vec(),
                        participating_island_identities,
                        row.propagated_persistent_name_identities().to_vec(),
                    )
                } else {
                    row.clone()
                }
            })
            .collect(),
    );

    let denial = PlanarBooleanOverlapRegionAdjacencyIndex::admit(
        PlanarBooleanOverlapAdjacencyIndexInput::from_participation_products(
            recovered.loop_participation_map(),
            &hostile_island_map,
            &hostile_chain_map,
        ),
    )
    .expect_err("adjacency should reject incidental tie-break neighborhoods");

    assert_eq!(
        denial.kind(),
        PlanarBooleanOverlapAdjacencyIndexDenialKind::IncidentalIterationOrderTieBreakDenied
    );
}
