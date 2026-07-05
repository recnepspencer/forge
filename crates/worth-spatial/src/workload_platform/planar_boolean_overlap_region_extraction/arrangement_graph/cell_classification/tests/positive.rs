use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide::{
    Left, Right,
};
use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::LoopFixtureEntryOrder;
use crate::workload_platform::planar_boolean_overlap_region_extraction::arrangement_graph::PlanarBooleanOverlapArrangementBoundarySegmentRow;
use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanCoplanarOverlapArrangementGraph, PlanarBooleanOverlapArrangementCellRow,
    PlanarBooleanOverlapArrangementCellSet,
    PlanarBooleanOverlapCellContainmentEvidenceKind::{BoundaryOnly, Inside, Outside},
    PlanarBooleanOverlapCellContainmentInput, PlanarBooleanOverlapCellContainmentMap,
    PlanarBooleanOverlapCellWindingEvidenceKind::{BoundaryTopology, SupportingIslandTopology},
    PlanarBooleanOverlapCellWindingField, PlanarBooleanOverlapCellWindingFieldInput,
};

use super::fixtures::{
    admitted_graph, inside_both_multi_boundary_graph, multi_boundary_graph, multi_cell_graph,
    permuted_multi_cell_graph,
};

#[test]
fn overlap_cell_classification_is_replay_stable_for_real_arrangement_products() {
    let canonical = admitted_graph(LoopFixtureEntryOrder::Canonical);
    let replayed = admitted_graph(LoopFixtureEntryOrder::Replayed);

    let canonical_map = PlanarBooleanOverlapCellContainmentMap::admit(
        PlanarBooleanOverlapCellContainmentInput::from_arrangement(&canonical),
    )
    .expect("canonical arrangement should admit containment classification");
    let replayed_map = PlanarBooleanOverlapCellContainmentMap::admit(
        PlanarBooleanOverlapCellContainmentInput::from_arrangement(&replayed),
    )
    .expect("replayed arrangement should admit containment classification");
    let canonical_winding = PlanarBooleanOverlapCellWindingField::admit(
        PlanarBooleanOverlapCellWindingFieldInput::from_arrangement(&canonical, &canonical_map),
    )
    .expect("canonical containment should admit winding classification");
    let replayed_winding = PlanarBooleanOverlapCellWindingField::admit(
        PlanarBooleanOverlapCellWindingFieldInput::from_arrangement(&replayed, &replayed_map),
    )
    .expect("replayed containment should admit winding classification");

    assert_eq!(canonical_map, replayed_map);
    assert_eq!(canonical_winding, replayed_winding);
}

#[test]
fn overlap_cell_classification_is_stable_under_benign_segment_order_variation() {
    let canonical = multi_cell_graph();
    let permuted = permuted_multi_cell_graph();

    let canonical_map = PlanarBooleanOverlapCellContainmentMap::admit(
        PlanarBooleanOverlapCellContainmentInput::from_arrangement(&canonical),
    )
    .expect("canonical arrangement should admit containment classification");
    let permuted_map = PlanarBooleanOverlapCellContainmentMap::admit(
        PlanarBooleanOverlapCellContainmentInput::from_arrangement(&permuted),
    )
    .expect("permuted arrangement should admit containment classification");
    let canonical_winding = PlanarBooleanOverlapCellWindingField::admit(
        PlanarBooleanOverlapCellWindingFieldInput::from_arrangement(&canonical, &canonical_map),
    )
    .expect("canonical containment should admit winding classification");
    let permuted_winding = PlanarBooleanOverlapCellWindingField::admit(
        PlanarBooleanOverlapCellWindingFieldInput::from_arrangement(&permuted, &permuted_map),
    )
    .expect("permuted containment should admit winding classification");

    assert_eq!(canonical_map, permuted_map);
    assert_eq!(canonical_winding, permuted_winding);
}

#[test]
fn overlap_cell_classification_distinguishes_area_bearing_cells_from_boundary_only_cells() {
    let area_graph = inside_both_multi_boundary_graph();
    let area_map = PlanarBooleanOverlapCellContainmentMap::admit(
        PlanarBooleanOverlapCellContainmentInput::from_arrangement(&area_graph),
    )
    .expect("multi-boundary arrangement should admit containment classification");
    let area_winding = PlanarBooleanOverlapCellWindingField::admit(
        PlanarBooleanOverlapCellWindingFieldInput::from_arrangement(&area_graph, &area_map),
    )
    .expect("multi-boundary containment should admit winding classification");

    assert_eq!(area_map.rows().len(), 2);
    let area_left = area_map
        .rows()
        .iter()
        .find(|row| row.operand_side() == Left)
        .expect("left containment row should exist");
    let area_right = area_map
        .rows()
        .iter()
        .find(|row| row.operand_side() == Right)
        .expect("right containment row should exist");
    assert_eq!(area_left.evidence_kind(), Inside);
    assert_eq!(area_right.evidence_kind(), Inside);
    let winding_left = area_winding
        .rows()
        .iter()
        .find(|row| row.operand_side() == Left)
        .expect("left winding row should exist");
    let winding_right = area_winding
        .rows()
        .iter()
        .find(|row| row.operand_side() == Right)
        .expect("right winding row should exist");
    assert!(winding_left.winding_number() > 0);
    assert!(winding_right.winding_number() > 0);
    assert_eq!(winding_left.evidence_kind(), BoundaryTopology);
    assert_eq!(winding_right.evidence_kind(), SupportingIslandTopology);

    let one_sided_area_graph = multi_boundary_graph();
    let one_sided_area_map = PlanarBooleanOverlapCellContainmentMap::admit(
        PlanarBooleanOverlapCellContainmentInput::from_arrangement(&one_sided_area_graph),
    )
    .expect("one-sided multi-boundary arrangement should admit containment classification");
    let one_sided_area_winding = PlanarBooleanOverlapCellWindingField::admit(
        PlanarBooleanOverlapCellWindingFieldInput::from_arrangement(
            &one_sided_area_graph,
            &one_sided_area_map,
        ),
    )
    .expect("one-sided multi-boundary containment should admit winding classification");
    let one_sided_right = one_sided_area_map
        .rows()
        .iter()
        .find(|row| row.operand_side() == Right)
        .expect("right containment row should exist");
    assert_eq!(one_sided_right.evidence_kind(), Outside);
    let one_sided_right_winding = one_sided_area_winding
        .rows()
        .iter()
        .find(|row| row.operand_side() == Right)
        .expect("right winding row should exist");
    assert_eq!(one_sided_right_winding.winding_number(), 0);

    let boundary_graph = multi_cell_graph();
    let boundary_map = PlanarBooleanOverlapCellContainmentMap::admit(
        PlanarBooleanOverlapCellContainmentInput::from_arrangement(&boundary_graph),
    )
    .expect("multi-cell arrangement should admit containment classification");

    assert_eq!(boundary_map.rows().len(), 4);
    for cell in boundary_graph.cell_set().cells() {
        let left_row = boundary_map
            .rows()
            .iter()
            .find(|row| row.cell_identity() == cell.cell_identity() && row.operand_side() == Left)
            .expect("left containment row should exist");
        let right_row = boundary_map
            .rows()
            .iter()
            .find(|row| row.cell_identity() == cell.cell_identity() && row.operand_side() == Right)
            .expect("right containment row should exist");

        assert_eq!(left_row.evidence_kind(), BoundaryOnly);
        assert_eq!(right_row.evidence_kind(), Outside);
    }
}

#[test]
fn overlap_cell_winding_preserves_repeated_certified_boundary_contributions() {
    let baseline_graph = inside_both_multi_boundary_graph();
    let baseline_containment = PlanarBooleanOverlapCellContainmentMap::admit(
        PlanarBooleanOverlapCellContainmentInput::from_arrangement(&baseline_graph),
    )
    .expect("baseline graph should admit containment");
    let baseline_winding = PlanarBooleanOverlapCellWindingField::admit(
        PlanarBooleanOverlapCellWindingFieldInput::from_arrangement(
            &baseline_graph,
            &baseline_containment,
        ),
    )
    .expect("baseline graph should admit winding");
    let baseline_left_winding = baseline_winding
        .rows()
        .iter()
        .find(|row| row.operand_side() == Left)
        .expect("baseline left winding row should exist");

    let graph = graph_with_extra_left_boundary_contribution(1);
    let containment = PlanarBooleanOverlapCellContainmentMap::admit(
        PlanarBooleanOverlapCellContainmentInput::from_arrangement(&graph),
    )
    .expect("repeated certified boundary contribution should still admit containment");
    let winding = PlanarBooleanOverlapCellWindingField::admit(
        PlanarBooleanOverlapCellWindingFieldInput::from_arrangement(&graph, &containment),
    )
    .expect("repeated certified boundary contribution should admit winding");

    let left_winding = winding
        .rows()
        .iter()
        .find(|row| row.operand_side() == Left)
        .expect("left winding row should exist");

    assert_eq!(left_winding.evidence_kind(), BoundaryTopology);
    assert_eq!(
        left_winding.winding_number(),
        baseline_left_winding.winding_number() + 1
    );
}

#[test]
fn overlap_cell_winding_preserves_signed_cancellation_for_repeated_boundary_contributions() {
    let baseline_graph = inside_both_multi_boundary_graph();
    let baseline_containment = PlanarBooleanOverlapCellContainmentMap::admit(
        PlanarBooleanOverlapCellContainmentInput::from_arrangement(&baseline_graph),
    )
    .expect("baseline graph should admit containment");
    let baseline_winding = PlanarBooleanOverlapCellWindingField::admit(
        PlanarBooleanOverlapCellWindingFieldInput::from_arrangement(
            &baseline_graph,
            &baseline_containment,
        ),
    )
    .expect("baseline graph should admit winding");
    let baseline_left_winding = baseline_winding
        .rows()
        .iter()
        .find(|row| row.operand_side() == Left)
        .expect("baseline left winding row should exist");

    let graph = graph_with_extra_left_boundary_contribution(-1);
    let containment = PlanarBooleanOverlapCellContainmentMap::admit(
        PlanarBooleanOverlapCellContainmentInput::from_arrangement(&graph),
    )
    .expect("cancelling certified boundary contribution should still admit containment");
    let winding = PlanarBooleanOverlapCellWindingField::admit(
        PlanarBooleanOverlapCellWindingFieldInput::from_arrangement(&graph, &containment),
    )
    .expect("cancelling certified boundary contribution should admit winding");

    let left_winding = winding
        .rows()
        .iter()
        .find(|row| row.operand_side() == Left)
        .expect("left winding row should exist");

    assert_eq!(left_winding.evidence_kind(), BoundaryTopology);
    assert_eq!(
        left_winding.winding_number(),
        baseline_left_winding.winding_number() - 1
    );
}

fn graph_with_extra_left_boundary_contribution(
    extra_winding_sign: i8,
) -> PlanarBooleanCoplanarOverlapArrangementGraph {
    let base = inside_both_multi_boundary_graph();
    let template_segment = base
        .boundary_segments()
        .iter()
        .find(|segment| segment.operand_side() == Left)
        .expect("fixture arrangement should contain one left boundary segment")
        .clone();
    let extra_segment_identity = format!(
        "{}:extra-left-winding:{}",
        template_segment.segment_identity(),
        extra_winding_sign
    );
    let extra_segment = PlanarBooleanOverlapArrangementBoundarySegmentRow::new(
        extra_segment_identity.clone(),
        template_segment.neighborhood_identity().to_string(),
        template_segment.source_loop_identity().to_string(),
        template_segment.operand_side(),
        extra_winding_sign,
        template_segment.source_edge_identity().to_string(),
        format!(
            "{}:extra-left-winding:{}",
            template_segment.fragment_identity(),
            extra_winding_sign
        ),
        template_segment.boundary_role(),
        base.boundary_segments().len(),
    );

    let mut boundary_segments = base.boundary_segments().to_vec();
    boundary_segments.push(extra_segment);

    let base_cell = base.cell_set().cells()[0].clone();
    let mut boundary_segment_identities = base_cell.boundary_segment_identities().to_vec();
    boundary_segment_identities.push(extra_segment_identity);
    let rebuilt_cell = PlanarBooleanOverlapArrangementCellRow::new(
        base_cell.cell_identity().to_string(),
        base_cell.arrangement_identity().to_string(),
        base_cell.neighborhood_identity().to_string(),
        base_cell.source_loop_identities().to_vec(),
        base_cell.supporting_island_identity().map(str::to_string),
        base_cell
            .supporting_island_member_source_loop_identities()
            .to_vec(),
        base_cell
            .supporting_island_member_source_loop_operand_sides()
            .to_vec(),
        base_cell
            .supporting_island_member_source_loop_winding_signs()
            .to_vec(),
        base_cell.chain_identities().to_vec(),
        base_cell.lineage_identities().to_vec(),
        base_cell.participating_loop_identities().to_vec(),
        base_cell.participating_island_identities().to_vec(),
        base_cell.boundary_component_identities().to_vec(),
        boundary_segment_identities,
        base_cell.propagated_persistent_name_identities().to_vec(),
    );
    let rebuilt_cell_set = PlanarBooleanOverlapArrangementCellSet::new(
        base.cell_set().cell_set_identity().to_string(),
        base.cell_set().request_identity().to_string(),
        base.cell_set().adjacency_index_identity().to_string(),
        base.cell_set().ordering_basis_identity().to_string(),
        vec![rebuilt_cell],
    );

    PlanarBooleanCoplanarOverlapArrangementGraph::new(
        base.arrangement_graph_identity().to_string(),
        base.request_identity().to_string(),
        base.adjacency_index_identity().to_string(),
        base.ordering_basis_identity().to_string(),
        base.rows().to_vec(),
        base.boundary_components().to_vec(),
        boundary_segments,
        rebuilt_cell_set,
        base.counters(),
    )
}
