use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::LoopFixtureEntryOrder;
use crate::workload_platform::planar_boolean_overlap_region_extraction::arrangement_graph::cell_classification::tests::fixtures::{
    admitted_graph, inside_both_multi_boundary_graph, multi_cell_graph, permuted_multi_cell_graph,
};
use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanCoplanarOverlapArrangementGraph, PlanarBooleanOverlapCellContainmentInput,
    PlanarBooleanOverlapCellContainmentMap, PlanarBooleanOverlapCellWindingField,
    PlanarBooleanOverlapCellWindingFieldInput, PlanarBooleanOverlapIslandCandidateInput,
    PlanarBooleanBoundaryContactClassificationBundle, PlanarBooleanOverlapIslandComponentBundle,
};

pub(super) fn admitted_bundle(
    island_bundle: &PlanarBooleanOverlapIslandComponentBundle,
) -> PlanarBooleanBoundaryContactClassificationBundle {
    island_bundle
        .classify_boundary_contact_components()
        .expect("fixture island component bundle should admit boundary contact classification")
}

pub(super) fn admitted_island_bundle(
    arrangement: &PlanarBooleanCoplanarOverlapArrangementGraph,
) -> PlanarBooleanOverlapIslandComponentBundle {
    let containment = PlanarBooleanOverlapCellContainmentMap::admit(
        PlanarBooleanOverlapCellContainmentInput::from_arrangement(arrangement),
    )
    .expect("fixture arrangement should admit containment");
    let winding = PlanarBooleanOverlapCellWindingField::admit(
        PlanarBooleanOverlapCellWindingFieldInput::from_arrangement(arrangement, &containment),
    )
    .expect("fixture arrangement should admit winding");
    PlanarBooleanOverlapIslandComponentBundle::admit(
        PlanarBooleanOverlapIslandCandidateInput::from_cell_classification(
            arrangement,
            &containment,
            &winding,
        ),
    )
    .expect("fixture arrangement should admit island component bundle")
}

pub(super) fn admitted_from_arrangement_boundary() -> PlanarBooleanBoundaryContactClassificationBundle {
    let bundle = admitted_island_bundle(&boundary_graph());
    admitted_bundle(&bundle)
}

pub(super) fn admitted_from_arrangement_area() -> PlanarBooleanBoundaryContactClassificationBundle {
    let bundle = admitted_island_bundle(&area_graph());
    admitted_bundle(&bundle)
}

pub(super) fn replayed_real_bundles() -> (
    PlanarBooleanOverlapIslandComponentBundle,
    PlanarBooleanOverlapIslandComponentBundle,
) {
    let (canonical, replayed) = replayed_real_arrangements();
    (
        admitted_island_bundle(&canonical),
        admitted_island_bundle(&replayed),
    )
}

pub(super) fn permuted_boundary_bundle() -> (
    PlanarBooleanOverlapIslandComponentBundle,
    PlanarBooleanOverlapIslandComponentBundle,
) {
    (
        admitted_island_bundle(&boundary_graph()),
        admitted_island_bundle(&permuted_boundary_graph()),
    )
}

fn replayed_real_arrangements() -> (
    PlanarBooleanCoplanarOverlapArrangementGraph,
    PlanarBooleanCoplanarOverlapArrangementGraph,
) {
    (
        admitted_graph(LoopFixtureEntryOrder::Canonical),
        admitted_graph(LoopFixtureEntryOrder::Replayed),
    )
}

pub(super) fn area_graph() -> PlanarBooleanCoplanarOverlapArrangementGraph {
    inside_both_multi_boundary_graph()
}

pub(super) fn boundary_graph() -> PlanarBooleanCoplanarOverlapArrangementGraph {
    multi_cell_graph()
}

fn permuted_boundary_graph() -> PlanarBooleanCoplanarOverlapArrangementGraph {
    permuted_multi_cell_graph()
}
