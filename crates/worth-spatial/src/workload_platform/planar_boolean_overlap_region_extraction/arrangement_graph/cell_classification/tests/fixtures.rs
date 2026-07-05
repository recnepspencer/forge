use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::LoopFixtureEntryOrder;
use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanCoplanarOverlapArrangementGraph, PlanarBooleanOverlapAdjacencyRow,
    PlanarBooleanOverlapArrangementGraphInput, PlanarBooleanOverlapRegionAdjacencyIndex,
};

use crate::workload_platform::planar_boolean_overlap_region_extraction::arrangement_graph::tests::fixtures::{
    admitted_adjacency, inside_both_multi_boundary_adjacency, multi_boundary_component_adjacency,
    multi_cell_adjacency, permuted_multi_cell_adjacency,
};

pub(crate) fn admitted_graph(
    order: LoopFixtureEntryOrder,
) -> PlanarBooleanCoplanarOverlapArrangementGraph {
    let adjacency = admitted_adjacency(order);
    PlanarBooleanCoplanarOverlapArrangementGraph::admit(
        PlanarBooleanOverlapArrangementGraphInput::from_adjacency(
            &adjacency,
            adjacency.ordering_basis(),
        ),
    )
    .expect("fixture adjacency should admit arrangement construction")
}

pub(super) fn multi_boundary_graph() -> PlanarBooleanCoplanarOverlapArrangementGraph {
    let adjacency = multi_boundary_component_adjacency();
    PlanarBooleanCoplanarOverlapArrangementGraph::admit(
        PlanarBooleanOverlapArrangementGraphInput::from_adjacency(
            &adjacency,
            adjacency.ordering_basis(),
        ),
    )
    .expect("fixture adjacency should admit multi-boundary arrangement construction")
}

pub(crate) fn inside_both_multi_boundary_graph() -> PlanarBooleanCoplanarOverlapArrangementGraph {
    let adjacency = inside_both_multi_boundary_adjacency();
    PlanarBooleanCoplanarOverlapArrangementGraph::admit(
        PlanarBooleanOverlapArrangementGraphInput::from_adjacency(
            &adjacency,
            adjacency.ordering_basis(),
        ),
    )
    .expect("fixture adjacency should admit inside-both multi-boundary arrangement construction")
}

pub(super) fn ambiguous_hidden_right_winding_graph() -> PlanarBooleanCoplanarOverlapArrangementGraph
{
    let base = inside_both_multi_boundary_adjacency();
    let row = base.rows()[0].clone();
    let mut participating_loop_identities = row.participating_loop_identities().to_vec();
    let second_hidden_right_loop = format!(
        "{}:right-contained-second",
        row.island_origin_loop_identities()[0]
    );
    participating_loop_identities.push(second_hidden_right_loop.clone());
    let mut island_member_source_loop_identities =
        row.island_member_source_loop_identities().to_vec();
    island_member_source_loop_identities[0].push(second_hidden_right_loop);
    let mut island_member_source_loop_operand_sides =
        row.island_member_source_loop_operand_sides().to_vec();
    island_member_source_loop_operand_sides[0].push(
        crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide::Right,
    );
    let mut island_member_source_loop_winding_signs =
        row.island_member_source_loop_winding_signs().to_vec();
    island_member_source_loop_winding_signs[0].push(1);
    let rebuilt_row = PlanarBooleanOverlapAdjacencyRow::new(
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
        row.source_loop_operand_sides().to_vec(),
        row.source_loop_winding_signs().to_vec(),
        row.source_edge_identities().to_vec(),
        row.fragment_identities().to_vec(),
        row.boundary_roles().to_vec(),
        row.propagated_persistent_name_identities().to_vec(),
    );
    let adjacency = PlanarBooleanOverlapRegionAdjacencyIndex::new(
        base.adjacency_index_identity().to_string(),
        base.request_identity().to_string(),
        base.loop_participation_map_identity().to_string(),
        base.island_participation_map_identity().to_string(),
        base.chain_lineage_map_identity().to_string(),
        vec![rebuilt_row],
        base.ordering_basis().clone(),
        base.counters(),
    );

    PlanarBooleanCoplanarOverlapArrangementGraph::admit(
        PlanarBooleanOverlapArrangementGraphInput::from_adjacency(
            &adjacency,
            adjacency.ordering_basis(),
        ),
    )
    .expect("fixture adjacency should admit arrangement construction before winding denial")
}

pub(super) fn cancelled_hidden_right_winding_graph() -> PlanarBooleanCoplanarOverlapArrangementGraph
{
    let base = inside_both_multi_boundary_adjacency();
    let row = base.rows()[0].clone();
    let mut participating_loop_identities = row.participating_loop_identities().to_vec();
    let second_hidden_right_loop = format!(
        "{}:right-contained-cancelling",
        row.island_origin_loop_identities()[0]
    );
    participating_loop_identities.push(second_hidden_right_loop.clone());
    let mut island_member_source_loop_identities =
        row.island_member_source_loop_identities().to_vec();
    island_member_source_loop_identities[0].push(second_hidden_right_loop);
    let mut island_member_source_loop_operand_sides =
        row.island_member_source_loop_operand_sides().to_vec();
    island_member_source_loop_operand_sides[0].push(
        crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide::Right,
    );
    let mut island_member_source_loop_winding_signs =
        row.island_member_source_loop_winding_signs().to_vec();
    island_member_source_loop_winding_signs[0].push(-1);
    let rebuilt_row = PlanarBooleanOverlapAdjacencyRow::new(
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
        row.source_loop_operand_sides().to_vec(),
        row.source_loop_winding_signs().to_vec(),
        row.source_edge_identities().to_vec(),
        row.fragment_identities().to_vec(),
        row.boundary_roles().to_vec(),
        row.propagated_persistent_name_identities().to_vec(),
    );
    let adjacency = PlanarBooleanOverlapRegionAdjacencyIndex::new(
        base.adjacency_index_identity().to_string(),
        base.request_identity().to_string(),
        base.loop_participation_map_identity().to_string(),
        base.island_participation_map_identity().to_string(),
        base.chain_lineage_map_identity().to_string(),
        vec![rebuilt_row],
        base.ordering_basis().clone(),
        base.counters(),
    );

    PlanarBooleanCoplanarOverlapArrangementGraph::admit(
        PlanarBooleanOverlapArrangementGraphInput::from_adjacency(
            &adjacency,
            adjacency.ordering_basis(),
        ),
    )
    .expect("fixture adjacency should admit arrangement construction before containment denial")
}

pub(crate) fn multi_cell_graph() -> PlanarBooleanCoplanarOverlapArrangementGraph {
    let adjacency = multi_cell_adjacency();
    PlanarBooleanCoplanarOverlapArrangementGraph::admit(
        PlanarBooleanOverlapArrangementGraphInput::from_adjacency(
            &adjacency,
            adjacency.ordering_basis(),
        ),
    )
    .expect("fixture adjacency should admit multi-cell arrangement construction")
}

pub(crate) fn permuted_multi_cell_graph() -> PlanarBooleanCoplanarOverlapArrangementGraph {
    let adjacency = permuted_multi_cell_adjacency();
    PlanarBooleanCoplanarOverlapArrangementGraph::admit(
        PlanarBooleanOverlapArrangementGraphInput::from_adjacency(
            &adjacency,
            adjacency.ordering_basis(),
        ),
    )
    .expect("fixture adjacency should admit multi-cell arrangement construction")
}
