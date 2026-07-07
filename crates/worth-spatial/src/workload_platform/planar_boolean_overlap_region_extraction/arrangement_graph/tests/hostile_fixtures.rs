use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide::{
    Left, Right,
};
use crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanOverlapChainBoundaryRole::{
    OverlapEndBoundary, OverlapStartBoundary,
};
use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::LoopFixtureEntryOrder;
use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanOverlapAdjacencyRow, PlanarBooleanOverlapRegionAdjacencyIndex,
};

use super::fixtures::{admitted_adjacency, multi_boundary_component_adjacency};

pub(crate) fn ambiguous_face_grouping_adjacency() -> PlanarBooleanOverlapRegionAdjacencyIndex {
    let base = multi_boundary_component_adjacency();
    let row = base.rows()[0].clone();
    let mut island_participation_identities = row.island_participation_identities().to_vec();
    island_participation_identities.push("hostile-face-witness".to_string());
    let mut participating_island_identities = row.participating_island_identities().to_vec();
    participating_island_identities.push("hostile-face-witness".to_string());
    let mut island_origin_loop_identities = row.island_origin_loop_identities().to_vec();
    island_origin_loop_identities.push(row.island_origin_loop_identities()[0].clone());
    let mut island_kinds = row.island_kinds().to_vec();
    island_kinds.push(row.island_kinds()[0]);
    let mut island_member_source_loop_identities =
        row.island_member_source_loop_identities().to_vec();
    island_member_source_loop_identities
        .push(row.island_member_source_loop_identities()[0].clone());
    let mut island_member_source_loop_operand_sides =
        row.island_member_source_loop_operand_sides().to_vec();
    island_member_source_loop_operand_sides
        .push(row.island_member_source_loop_operand_sides()[0].clone());
    let mut island_member_source_loop_winding_signs =
        row.island_member_source_loop_winding_signs().to_vec();
    island_member_source_loop_winding_signs
        .push(row.island_member_source_loop_winding_signs()[0].clone());

    PlanarBooleanOverlapRegionAdjacencyIndex::new(
        base.adjacency_index_identity().to_string(),
        base.request_identity().to_string(),
        base.loop_participation_map_identity().to_string(),
        base.island_participation_map_identity().to_string(),
        base.chain_lineage_map_identity().to_string(),
        vec![PlanarBooleanOverlapAdjacencyRow::new(
            row.adjacency_identity().to_string(),
            row.neighborhood_identity().to_string(),
            row.chain_identities().to_vec(),
            row.lineage_identities().to_vec(),
            row.loop_participation_identities().to_vec(),
            row.participating_loop_identities().to_vec(),
            row.loop_roles().to_vec(),
            island_participation_identities,
            participating_island_identities,
            island_origin_loop_identities,
            island_kinds,
            island_member_source_loop_identities,
            island_member_source_loop_operand_sides,
            island_member_source_loop_winding_signs,
            row.source_loop_identities().to_vec(),
            row.source_loop_operand_sides().to_vec(),
            row.source_loop_winding_signs().to_vec(),
            row.source_edge_identities().to_vec(),
            row.fragment_identities().to_vec(),
            row.boundary_roles().to_vec(),
            row.propagated_persistent_name_identities().to_vec(),
        )],
        base.ordering_basis().clone(),
        base.counters(),
    )
}

pub(crate) fn foreign_loop_boundary_adjacency() -> PlanarBooleanOverlapRegionAdjacencyIndex {
    let base = admitted_adjacency(LoopFixtureEntryOrder::Canonical);
    let base_row = base.rows()[0].clone();
    let primary_loop = base_row.source_loop_identities()[0].clone();
    let foreign_loop = format!("{primary_loop}:foreign");
    let source_edge = base_row.source_edge_identities()[0].clone();
    let fragment = base_row.fragment_identities()[0].clone();

    PlanarBooleanOverlapRegionAdjacencyIndex::new(
        base.adjacency_index_identity().to_string(),
        base.request_identity().to_string(),
        base.loop_participation_map_identity().to_string(),
        base.island_participation_map_identity().to_string(),
        base.chain_lineage_map_identity().to_string(),
        vec![PlanarBooleanOverlapAdjacencyRow::new(
            base_row.adjacency_identity().to_string(),
            base_row.neighborhood_identity().to_string(),
            base_row.chain_identities().to_vec(),
            base_row.lineage_identities().to_vec(),
            base_row.loop_participation_identities().to_vec(),
            base_row.participating_loop_identities().to_vec(),
            base_row.loop_roles().to_vec(),
            base_row.island_participation_identities().to_vec(),
            base_row.participating_island_identities().to_vec(),
            base_row.island_origin_loop_identities().to_vec(),
            base_row.island_kinds().to_vec(),
            base_row.island_member_source_loop_identities().to_vec(),
            base_row.island_member_source_loop_operand_sides().to_vec(),
            base_row.island_member_source_loop_winding_signs().to_vec(),
            vec![
                primary_loop.clone(),
                foreign_loop.clone(),
                foreign_loop,
                primary_loop,
            ],
            vec![base_row.source_loop_operand_sides()[0]; 4],
            vec![base_row.source_loop_winding_signs()[0]; 4],
            boundary_source_edges(&source_edge),
            boundary_fragments(&fragment),
            alternating_boundary_roles(),
            base_row.propagated_persistent_name_identities().to_vec(),
        )],
        base.ordering_basis().clone(),
        base.counters(),
    )
}

pub(crate) fn role_contradiction_adjacency() -> PlanarBooleanOverlapRegionAdjacencyIndex {
    let base = admitted_adjacency(LoopFixtureEntryOrder::Canonical);
    let base_row = base.rows()[0].clone();
    let source_loop = base_row.source_loop_identities()[0].clone();
    let source_edge = base_row.source_edge_identities()[0].clone();
    let fragment = base_row.fragment_identities()[0].clone();

    PlanarBooleanOverlapRegionAdjacencyIndex::new(
        base.adjacency_index_identity().to_string(),
        base.request_identity().to_string(),
        base.loop_participation_map_identity().to_string(),
        base.island_participation_map_identity().to_string(),
        base.chain_lineage_map_identity().to_string(),
        vec![PlanarBooleanOverlapAdjacencyRow::new(
            base_row.adjacency_identity().to_string(),
            base_row.neighborhood_identity().to_string(),
            base_row.chain_identities().to_vec(),
            base_row.lineage_identities().to_vec(),
            base_row.loop_participation_identities().to_vec(),
            base_row.participating_loop_identities().to_vec(),
            base_row.loop_roles().to_vec(),
            base_row.island_participation_identities().to_vec(),
            base_row.participating_island_identities().to_vec(),
            base_row.island_origin_loop_identities().to_vec(),
            base_row.island_kinds().to_vec(),
            base_row.island_member_source_loop_identities().to_vec(),
            base_row.island_member_source_loop_operand_sides().to_vec(),
            base_row.island_member_source_loop_winding_signs().to_vec(),
            vec![
                source_loop.clone(),
                source_loop.clone(),
                source_loop.clone(),
                source_loop,
            ],
            vec![base_row.source_loop_operand_sides()[0]; 4],
            vec![base_row.source_loop_winding_signs()[0]; 4],
            vec![
                format!("{source_edge}:start-a"),
                format!("{source_edge}:start-b"),
                format!("{source_edge}:end-a"),
                format!("{source_edge}:end-b"),
            ],
            vec![
                format!("{fragment}:start-a"),
                format!("{fragment}:start-b"),
                format!("{fragment}:end-a"),
                format!("{fragment}:end-b"),
            ],
            vec![
                OverlapStartBoundary,
                OverlapStartBoundary,
                OverlapEndBoundary,
                OverlapEndBoundary,
            ],
            base_row.propagated_persistent_name_identities().to_vec(),
        )],
        base.ordering_basis().clone(),
        base.counters(),
    )
}

pub(crate) fn inside_both_multi_boundary_adjacency() -> PlanarBooleanOverlapRegionAdjacencyIndex {
    let base = multi_boundary_component_adjacency();
    let row = base.rows()[0].clone();
    let mut participating_loop_identities = row.participating_loop_identities().to_vec();
    let right_loop = format!("{}:right-contained", row.island_origin_loop_identities()[0]);
    participating_loop_identities.push(right_loop.clone());
    let mut island_member_source_loop_identities =
        row.island_member_source_loop_identities().to_vec();
    island_member_source_loop_identities[0].push(right_loop);
    let mut island_member_source_loop_operand_sides =
        row.island_member_source_loop_operand_sides().to_vec();
    island_member_source_loop_operand_sides[0].push(Right);
    let mut island_member_source_loop_winding_signs =
        row.island_member_source_loop_winding_signs().to_vec();
    island_member_source_loop_winding_signs[0].push(1);

    PlanarBooleanOverlapRegionAdjacencyIndex::new(
        base.adjacency_index_identity().to_string(),
        base.request_identity().to_string(),
        base.loop_participation_map_identity().to_string(),
        base.island_participation_map_identity().to_string(),
        base.chain_lineage_map_identity().to_string(),
        vec![PlanarBooleanOverlapAdjacencyRow::new(
            row.adjacency_identity().to_string(),
            row.neighborhood_identity().to_string(),
            row.chain_identities().to_vec(),
            row.lineage_identities().to_vec(),
            row.loop_participation_identities().to_vec(),
            participating_loop_identities,
            row.loop_roles().to_vec(),
            row.island_participation_identities().to_vec(),
            row.participating_island_identities().to_vec(),
            row.island_origin_loop_identities().to_vec(),
            row.island_kinds().to_vec(),
            island_member_source_loop_identities,
            island_member_source_loop_operand_sides,
            island_member_source_loop_winding_signs,
            row.source_loop_identities().to_vec(),
            vec![Left; row.source_loop_identities().len()],
            row.source_loop_winding_signs().to_vec(),
            row.source_edge_identities().to_vec(),
            row.fragment_identities().to_vec(),
            row.boundary_roles().to_vec(),
            row.propagated_persistent_name_identities().to_vec(),
        )],
        base.ordering_basis().clone(),
        base.counters(),
    )
}

fn boundary_source_edges(source_edge: &str) -> Vec<String> {
    vec![
        format!("{source_edge}:000-boundary-a-start"),
        format!("{source_edge}:001-boundary-a-end"),
        format!("{source_edge}:002-boundary-b-start"),
        format!("{source_edge}:003-boundary-b-end"),
    ]
}

fn boundary_fragments(fragment: &str) -> Vec<String> {
    vec![
        format!("{fragment}:000-boundary-a-start"),
        format!("{fragment}:001-boundary-a-end"),
        format!("{fragment}:002-boundary-b-start"),
        format!("{fragment}:003-boundary-b-end"),
    ]
}

fn alternating_boundary_roles() -> Vec<
    crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanOverlapChainBoundaryRole,
> {
    vec![
        OverlapStartBoundary,
        OverlapEndBoundary,
        OverlapStartBoundary,
        OverlapEndBoundary,
    ]
}
