use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::LoopFixtureEntryOrder;
use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanCoplanarOverlapArrangementGraph, PlanarBooleanOverlapArrangementGraphInput,
};

use super::fixtures::{
    admitted_adjacency, multi_boundary_component_adjacency, multi_cell_adjacency,
    permuted_multi_cell_adjacency,
};

#[test]
fn overlap_arrangement_is_replay_stable_for_real_adjacency_products() {
    let canonical_adjacency = admitted_adjacency(LoopFixtureEntryOrder::Canonical);
    let replayed_adjacency = admitted_adjacency(LoopFixtureEntryOrder::Replayed);

    let canonical_graph = PlanarBooleanCoplanarOverlapArrangementGraph::admit(
        PlanarBooleanOverlapArrangementGraphInput::from_adjacency(
            &canonical_adjacency,
            canonical_adjacency.ordering_basis(),
        ),
    )
    .expect("canonical adjacency should admit arrangement construction");
    let replayed_graph = PlanarBooleanCoplanarOverlapArrangementGraph::admit(
        PlanarBooleanOverlapArrangementGraphInput::from_adjacency(
            &replayed_adjacency,
            replayed_adjacency.ordering_basis(),
        ),
    )
    .expect("replayed adjacency should admit arrangement construction");

    assert_eq!(canonical_graph, replayed_graph);
    assert_eq!(canonical_graph.cell_set(), replayed_graph.cell_set());
}

#[test]
fn overlap_arrangement_cells_are_traceable_to_adjacency_and_ordering_basis() {
    let adjacency = admitted_adjacency(LoopFixtureEntryOrder::Canonical);
    let graph = PlanarBooleanCoplanarOverlapArrangementGraph::admit(
        PlanarBooleanOverlapArrangementGraphInput::from_adjacency(
            &adjacency,
            adjacency.ordering_basis(),
        ),
    )
    .expect("canonical adjacency should admit arrangement construction");

    assert_eq!(
        graph.adjacency_index_identity(),
        adjacency.adjacency_index_identity()
    );
    assert_eq!(
        graph.ordering_basis_identity(),
        adjacency.ordering_basis().basis_identity()
    );
    assert!(
        !graph.boundary_components().is_empty() && !graph.boundary_segments().is_empty(),
        "arrangement graph should expose explicit boundary topology between admitted adjacency neighborhoods and emitted cells"
    );
    assert!(
        graph
            .cell_set()
            .cells()
            .iter()
            .all(|cell| !cell.boundary_component_identities().is_empty()
                && !cell.boundary_segment_identities().is_empty()),
        "arrangement cells should carry concrete boundary topology traced to admitted adjacency provenance"
    );
}

#[test]
fn overlap_arrangement_can_lower_one_neighborhood_into_multiple_cells() {
    let adjacency = multi_cell_adjacency();
    let graph = PlanarBooleanCoplanarOverlapArrangementGraph::admit(
        PlanarBooleanOverlapArrangementGraphInput::from_adjacency(
            &adjacency,
            adjacency.ordering_basis(),
        ),
    )
    .expect("one neighborhood with multiple closed source-loop components should admit arrangement construction");

    assert_eq!(graph.rows().len(), 1);
    assert_eq!(graph.cell_set().cells().len(), 2);
    assert_eq!(graph.rows()[0].cell_identities().len(), 2);
    assert_eq!(
        graph.cell_set().cells()[0].source_loop_identities(),
        graph.cell_set().cells()[1].source_loop_identities(),
        "same-loop full-span components should remain distinct cells rather than being merged by source-loop identity"
    );
    assert!(
        graph.boundary_components().len() >= 2,
        "arrangement graph should preserve multiple closed boundary components before lowering them into cells"
    );
}

#[test]
fn overlap_arrangement_can_lower_one_face_with_multiple_boundary_components() {
    let adjacency = multi_boundary_component_adjacency();
    let graph = PlanarBooleanCoplanarOverlapArrangementGraph::admit(
        PlanarBooleanOverlapArrangementGraphInput::from_adjacency(
            &adjacency,
            adjacency.ordering_basis(),
        ),
    )
    .expect("one neighborhood with multiple walk components under one island-backed face witness should admit arrangement construction");

    assert_eq!(graph.rows().len(), 1);
    assert_eq!(graph.boundary_components().len(), 2);
    assert_eq!(graph.cell_set().cells().len(), 1);
    assert_eq!(graph.rows()[0].cell_identities().len(), 1);
    assert_eq!(
        graph.cell_set().cells()[0].boundary_component_identities().len(),
        2,
        "one arrangement cell should be able to carry multiple boundary components when one admitted island-backed witness groups them into the same face",
    );
    assert!(
        graph.cell_set().cells()[0].supporting_island_identity().is_some(),
        "multi-boundary arrangement faces should record the supporting island-backed grouping witness",
    );
}

#[test]
fn overlap_arrangement_is_stable_under_benign_segment_order_variation() {
    let canonical = multi_cell_adjacency();
    let permuted = permuted_multi_cell_adjacency();

    let canonical_graph = PlanarBooleanCoplanarOverlapArrangementGraph::admit(
        PlanarBooleanOverlapArrangementGraphInput::from_adjacency(
            &canonical,
            canonical.ordering_basis(),
        ),
    )
    .expect("canonical multi-cell adjacency should admit arrangement construction");
    let permuted_graph = PlanarBooleanCoplanarOverlapArrangementGraph::admit(
        PlanarBooleanOverlapArrangementGraphInput::from_adjacency(
            &permuted,
            permuted.ordering_basis(),
        ),
    )
    .expect("permuted multi-cell adjacency should admit arrangement construction");

    assert_eq!(canonical_graph, permuted_graph);
}
