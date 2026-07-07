use crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanOverlapChainBoundaryRole;
use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::LoopFixtureEntryOrder;
use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanCoplanarOverlapArrangementGraph, PlanarBooleanOverlapAdjacencyIndexInput,
    PlanarBooleanOverlapArrangementGraphDenialKind, PlanarBooleanOverlapArrangementGraphInput,
    PlanarBooleanOverlapChainRegionLineageMap, PlanarBooleanOverlapChainRegionLineageRow,
    PlanarBooleanOverlapRegionAdjacencyIndex,
};

use super::fixtures::recovered_participation;
use super::hostile_fixtures::{
    ambiguous_face_grouping_adjacency, foreign_loop_boundary_adjacency,
    role_contradiction_adjacency,
};

#[test]
fn overlap_arrangement_rejects_neighborhood_without_concrete_cell_substrate() {
    let recovered = recovered_participation(LoopFixtureEntryOrder::Canonical);
    let hostile_row = recovered.chain_lineage_map().rows()[0].clone();
    let hostile_chain_map = PlanarBooleanOverlapChainRegionLineageMap::new(
        recovered.chain_lineage_map().map_identity().to_string(),
        recovered.chain_lineage_map().request_identity().to_string(),
        recovered
            .chain_lineage_map()
            .rows()
            .iter()
            .map(|row| {
                if row.lineage_identity() == hostile_row.lineage_identity() {
                    PlanarBooleanOverlapChainRegionLineageRow::new(
                        row.lineage_row_identity().to_string(),
                        row.lineage_identity().to_string(),
                        row.chain_identity().to_string(),
                        row.fragment_identities().to_vec(),
                        row.source_loop_identities().to_vec(),
                        row.source_loop_operand_sides().to_vec(),
                        row.source_loop_winding_signs().to_vec(),
                        Vec::new(),
                        Vec::<PlanarBooleanOverlapChainBoundaryRole>::new(),
                        row.participating_loop_identities().to_vec(),
                        row.participating_island_identities().to_vec(),
                        row.propagated_persistent_name_identities().to_vec(),
                    )
                } else {
                    row.clone()
                }
            })
            .collect(),
    );
    let adjacency = PlanarBooleanOverlapRegionAdjacencyIndex::admit(
        PlanarBooleanOverlapAdjacencyIndexInput::from_participation_products(
            recovered.loop_participation_map(),
            recovered.island_participation_map(),
            &hostile_chain_map,
        ),
    )
    .expect("phase four should admit adjacency even when phase-five cell substrate is missing");

    let denial = PlanarBooleanCoplanarOverlapArrangementGraph::admit(
        PlanarBooleanOverlapArrangementGraphInput::from_adjacency(
            &adjacency,
            adjacency.ordering_basis(),
        ),
    )
    .expect_err("arrangement should reject neighborhoods without concrete cell substrate");

    assert_eq!(
        denial.kind(),
        PlanarBooleanOverlapArrangementGraphDenialKind::NoConcreteCellSubstrateDenied
    );
}

#[test]
fn overlap_arrangement_rejects_role_sequences_that_cannot_close_one_subdivision_component() {
    let adjacency = role_contradiction_adjacency();

    let denial = PlanarBooleanCoplanarOverlapArrangementGraph::admit(
        PlanarBooleanOverlapArrangementGraphInput::from_adjacency(
            &adjacency,
            adjacency.ordering_basis(),
        ),
    )
    .expect_err("arrangement should reject contradictory boundary role walks even when vector counts stay aligned");

    assert_eq!(
        denial.kind(),
        PlanarBooleanOverlapArrangementGraphDenialKind::AmbiguousArrangementSegmentOrderingDenied
    );
}

#[test]
fn overlap_arrangement_rejects_foreign_segment_loop_authority() {
    let adjacency = foreign_loop_boundary_adjacency();

    let denial = PlanarBooleanCoplanarOverlapArrangementGraph::admit(
        PlanarBooleanOverlapArrangementGraphInput::from_adjacency(
            &adjacency,
            adjacency.ordering_basis(),
        ),
    )
    .expect_err("arrangement should reject segment source-loop identities that are not certified by phase-four participation authority");

    assert_eq!(
        denial.kind(),
        PlanarBooleanOverlapArrangementGraphDenialKind::ContradictoryArrangementNeighborhoodDenied
    );
}

#[test]
fn overlap_arrangement_rejects_ambiguous_face_grouping_witnesses() {
    let adjacency = ambiguous_face_grouping_adjacency();

    let denial = PlanarBooleanCoplanarOverlapArrangementGraph::admit(
        PlanarBooleanOverlapArrangementGraphInput::from_adjacency(
            &adjacency,
            adjacency.ordering_basis(),
        ),
    )
    .expect_err(
        "arrangement should reject boundary-walk components that match multiple admitted island-backed face witnesses",
    );

    assert_eq!(
        denial.kind(),
        PlanarBooleanOverlapArrangementGraphDenialKind::ContradictoryArrangementNeighborhoodDenied
    );
}
