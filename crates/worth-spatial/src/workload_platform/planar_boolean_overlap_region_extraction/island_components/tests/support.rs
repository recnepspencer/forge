use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::LoopFixtureEntryOrder;
use crate::workload_platform::planar_boolean_overlap_region_extraction::arrangement_graph::cell_classification::tests::fixtures::{
    admitted_graph, inside_both_multi_boundary_graph, multi_cell_graph, permuted_multi_cell_graph,
};
use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanCoplanarOverlapArrangementGraph, PlanarBooleanOverlapCellContainmentInput,
    PlanarBooleanOverlapCellContainmentMap, PlanarBooleanOverlapCellWindingField,
    PlanarBooleanOverlapCellWindingFieldInput, PlanarBooleanOverlapIslandCandidateInput,
    PlanarBooleanOverlapIslandCandidateSet, PlanarBooleanOverlapIslandComponentBundle,
    PlanarBooleanOverlapIslandPartition,
};

pub(super) fn admitted_candidates(
    arrangement: &PlanarBooleanCoplanarOverlapArrangementGraph,
) -> PlanarBooleanOverlapIslandCandidateSet {
    let containment = PlanarBooleanOverlapCellContainmentMap::admit(
        PlanarBooleanOverlapCellContainmentInput::from_arrangement(arrangement),
    )
    .expect("fixture arrangement should admit containment");
    let winding = PlanarBooleanOverlapCellWindingField::admit(
        PlanarBooleanOverlapCellWindingFieldInput::from_arrangement(arrangement, &containment),
    )
    .expect("fixture arrangement should admit winding");
    PlanarBooleanOverlapIslandCandidateSet::admit(
        PlanarBooleanOverlapIslandCandidateInput::from_cell_classification(
            arrangement,
            &containment,
            &winding,
        ),
    )
    .expect("fixture arrangement should admit island candidates")
}

pub(super) fn replayed_real_arrangements() -> (
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

pub(super) fn permuted_boundary_graph() -> PlanarBooleanCoplanarOverlapArrangementGraph {
    permuted_multi_cell_graph()
}

pub(super) fn admitted_partition(
    arrangement: &PlanarBooleanCoplanarOverlapArrangementGraph,
) -> PlanarBooleanOverlapIslandPartition {
    let candidates = admitted_candidates(arrangement);
    PlanarBooleanOverlapIslandPartition::admit(&candidates)
        .expect("fixture candidates should admit island partition")
}

pub(super) fn admitted_bundle(
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
