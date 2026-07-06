use super::support::{
    admitted_bundle, admitted_candidates, admitted_partition, area_graph, boundary_graph,
    permuted_boundary_graph, replayed_real_arrangements,
};
use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanOverlapIslandCandidateKind::{AreaOverlap, BoundaryContact},
    PlanarBooleanOverlapIslandCandidateRow, PlanarBooleanOverlapIslandCandidateSet,
    PlanarBooleanOverlapIslandComponentCounters, PlanarBooleanOverlapIslandPartition,
};

#[test]
fn overlap_island_partition_is_replay_stable_for_real_arrangement_products() {
    let (canonical, replayed) = replayed_real_arrangements();
    let canonical_candidates = admitted_candidates(&canonical);
    let replayed_candidates = admitted_candidates(&replayed);
    let canonical_partition = crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapIslandPartition::admit(&canonical_candidates)
        .expect("canonical candidates should admit partition");
    let replayed_partition = crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapIslandPartition::admit(&replayed_candidates)
        .expect("replayed candidates should admit partition");

    assert_eq!(canonical_candidates, replayed_candidates);
    assert_eq!(canonical_partition, replayed_partition);
}

#[test]
fn overlap_island_partition_is_stable_under_benign_order_variation() {
    let canonical = boundary_graph();
    let permuted = permuted_boundary_graph();
    let canonical_candidates = admitted_candidates(&canonical);
    let permuted_candidates = admitted_candidates(&permuted);
    let canonical_partition = crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapIslandPartition::admit(&canonical_candidates)
        .expect("canonical candidates should admit partition");
    let permuted_partition = crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapIslandPartition::admit(&permuted_candidates)
        .expect("permuted candidates should admit partition");

    assert_eq!(canonical_candidates, permuted_candidates);
    assert_eq!(canonical_partition, permuted_partition);
}

#[test]
fn overlap_island_partition_keeps_boundary_contact_and_area_components_separate() {
    let area_partition = admitted_partition(&area_graph());
    assert_eq!(area_partition.overlap_islands().rows().len(), 1);
    assert!(area_partition
        .boundary_contact_components()
        .rows()
        .is_empty());
    assert_eq!(area_partition.area_overlap_components().rows().len(), 1);

    let boundary_partition = admitted_partition(&boundary_graph());
    assert_eq!(boundary_partition.overlap_islands().rows().len(), 1);
    assert_eq!(
        boundary_partition
            .boundary_contact_components()
            .rows()
            .len(),
        2
    );
    assert!(boundary_partition
        .area_overlap_components()
        .rows()
        .is_empty());
}

#[test]
fn overlap_island_component_bundle_is_the_ordinary_phase_seven_lowering_surface() {
    let arrangement = area_graph();
    let direct_candidates = admitted_candidates(&arrangement);
    let direct_partition = crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapIslandPartition::admit(&direct_candidates)
        .expect("fixture candidates should admit partition");
    let bundle = admitted_bundle(&arrangement);

    assert_eq!(bundle.island_candidates(), &direct_candidates);
    assert_eq!(bundle.island_partition(), &direct_partition);
    assert_eq!(bundle.overlap_islands(), direct_partition.overlap_islands());
    assert_eq!(
        bundle.area_overlap_components(),
        direct_partition.area_overlap_components()
    );
    assert_eq!(
        bundle.boundary_contact_components(),
        direct_partition.boundary_contact_components()
    );
}

#[test]
fn overlap_island_partition_keeps_separable_mixed_components_in_one_island() {
    let candidates = PlanarBooleanOverlapIslandCandidateSet::new(
        "mixed-candidate-set".to_string(),
        "mixed-request".to_string(),
        "mixed-arrangement".to_string(),
        "mixed-cell-set".to_string(),
        "mixed-order".to_string(),
        vec![
            PlanarBooleanOverlapIslandCandidateRow::new(
                "area-candidate".to_string(),
                "shared-island".to_string(),
                "area-cell".to_string(),
                "shared-neighborhood".to_string(),
                vec!["area-boundary-component".to_string()],
                vec!["area-segment".to_string()],
                vec!["area-loop-left".to_string(), "area-loop-right".to_string()],
                vec!["shared-name".to_string()],
                AreaOverlap,
            ),
            PlanarBooleanOverlapIslandCandidateRow::new(
                "boundary-candidate".to_string(),
                "shared-island".to_string(),
                "boundary-cell".to_string(),
                "shared-neighborhood".to_string(),
                vec!["boundary-component".to_string()],
                vec!["boundary-segment".to_string()],
                vec!["boundary-loop".to_string()],
                vec!["shared-name".to_string()],
                BoundaryContact,
            ),
        ],
        PlanarBooleanOverlapIslandComponentCounters::default(),
    );
    let partition = PlanarBooleanOverlapIslandPartition::admit(&candidates)
        .expect("mixed island with disjoint contact bases should partition");

    assert_eq!(partition.overlap_islands().rows().len(), 1);
    assert_eq!(partition.boundary_contact_components().rows().len(), 1);
    assert_eq!(partition.area_overlap_components().rows().len(), 1);
}

#[test]
fn overlap_island_partition_keeps_disconnected_same_kind_components_distinct() {
    let candidates = PlanarBooleanOverlapIslandCandidateSet::new(
        "disconnected-candidate-set".to_string(),
        "disconnected-request".to_string(),
        "disconnected-arrangement".to_string(),
        "disconnected-cell-set".to_string(),
        "disconnected-order".to_string(),
        vec![
            PlanarBooleanOverlapIslandCandidateRow::new(
                "boundary-candidate-a".to_string(),
                "shared-island".to_string(),
                "boundary-cell-a".to_string(),
                "shared-neighborhood".to_string(),
                vec!["boundary-component-a".to_string()],
                vec!["boundary-segment-a".to_string()],
                vec!["boundary-loop-a".to_string()],
                vec!["shared-name".to_string()],
                BoundaryContact,
            ),
            PlanarBooleanOverlapIslandCandidateRow::new(
                "boundary-candidate-b".to_string(),
                "shared-island".to_string(),
                "boundary-cell-b".to_string(),
                "shared-neighborhood".to_string(),
                vec!["boundary-component-b".to_string()],
                vec!["boundary-segment-b".to_string()],
                vec!["boundary-loop-b".to_string()],
                vec!["shared-name".to_string()],
                BoundaryContact,
            ),
        ],
        PlanarBooleanOverlapIslandComponentCounters::default(),
    );
    let partition = PlanarBooleanOverlapIslandPartition::admit(&candidates)
        .expect("disconnected same-kind candidates should partition into distinct components");

    assert_eq!(partition.overlap_islands().rows().len(), 1);
    assert_eq!(partition.boundary_contact_components().rows().len(), 2);
    assert!(partition.area_overlap_components().rows().is_empty());
}

#[test]
fn overlap_island_partition_keeps_same_loop_disjoint_boundary_contacts_distinct() {
    let candidates = PlanarBooleanOverlapIslandCandidateSet::new(
        "same-loop-boundary-candidate-set".to_string(),
        "same-loop-boundary-request".to_string(),
        "same-loop-boundary-arrangement".to_string(),
        "same-loop-boundary-cell-set".to_string(),
        "same-loop-boundary-order".to_string(),
        vec![
            PlanarBooleanOverlapIslandCandidateRow::new(
                "boundary-candidate-a".to_string(),
                "shared-island".to_string(),
                "boundary-cell-a".to_string(),
                "shared-neighborhood".to_string(),
                vec!["boundary-component-a".to_string()],
                vec!["boundary-segment-a".to_string()],
                vec!["shared-loop".to_string()],
                vec!["shared-name".to_string()],
                BoundaryContact,
            ),
            PlanarBooleanOverlapIslandCandidateRow::new(
                "boundary-candidate-b".to_string(),
                "shared-island".to_string(),
                "boundary-cell-b".to_string(),
                "shared-neighborhood".to_string(),
                vec!["boundary-component-b".to_string()],
                vec!["boundary-segment-b".to_string()],
                vec!["shared-loop".to_string()],
                vec!["shared-name".to_string()],
                BoundaryContact,
            ),
        ],
        PlanarBooleanOverlapIslandComponentCounters::default(),
    );
    let partition = PlanarBooleanOverlapIslandPartition::admit(&candidates)
        .expect("same-loop disjoint boundary contacts should remain separate");

    assert_eq!(partition.overlap_islands().rows().len(), 1);
    assert_eq!(partition.boundary_contact_components().rows().len(), 2);
    assert!(partition.area_overlap_components().rows().is_empty());
}

#[test]
fn overlap_island_partition_keeps_same_loop_mixed_candidates_separable_without_contact_overlap() {
    let candidates = PlanarBooleanOverlapIslandCandidateSet::new(
        "same-loop-mixed-candidate-set".to_string(),
        "same-loop-mixed-request".to_string(),
        "same-loop-mixed-arrangement".to_string(),
        "same-loop-mixed-cell-set".to_string(),
        "same-loop-mixed-order".to_string(),
        vec![
            PlanarBooleanOverlapIslandCandidateRow::new(
                "area-candidate".to_string(),
                "shared-island".to_string(),
                "area-cell".to_string(),
                "shared-neighborhood".to_string(),
                vec!["area-boundary-component".to_string()],
                vec!["area-segment".to_string()],
                vec!["shared-loop".to_string(), "area-loop-right".to_string()],
                vec!["shared-name".to_string()],
                AreaOverlap,
            ),
            PlanarBooleanOverlapIslandCandidateRow::new(
                "boundary-candidate".to_string(),
                "shared-island".to_string(),
                "boundary-cell".to_string(),
                "shared-neighborhood".to_string(),
                vec!["boundary-component".to_string()],
                vec!["boundary-segment".to_string()],
                vec!["shared-loop".to_string()],
                vec!["shared-name".to_string()],
                BoundaryContact,
            ),
        ],
        PlanarBooleanOverlapIslandComponentCounters::default(),
    );
    let partition = PlanarBooleanOverlapIslandPartition::admit(&candidates)
        .expect("same-loop mixed candidates without shared contact should remain separable");

    assert_eq!(partition.overlap_islands().rows().len(), 1);
    assert_eq!(partition.boundary_contact_components().rows().len(), 1);
    assert_eq!(partition.area_overlap_components().rows().len(), 1);
}

#[test]
fn overlap_island_partition_keeps_area_envelope_separate_from_boundary_edges() {
    let candidates = PlanarBooleanOverlapIslandCandidateSet::new(
        "area-envelope-candidate-set".to_string(),
        "area-envelope-request".to_string(),
        "area-envelope-arrangement".to_string(),
        "area-envelope-cell-set".to_string(),
        "area-envelope-order".to_string(),
        vec![
            PlanarBooleanOverlapIslandCandidateRow::new(
                "area-envelope-candidate".to_string(),
                "shared-island".to_string(),
                "area-cell".to_string(),
                "shared-neighborhood".to_string(),
                vec![
                    "boundary-component-a".to_string(),
                    "boundary-component-b".to_string(),
                ],
                vec!["segment-a".to_string(), "segment-b".to_string()],
                vec!["left-loop".to_string(), "right-loop".to_string()],
                vec!["shared-name".to_string()],
                AreaOverlap,
            ),
            PlanarBooleanOverlapIslandCandidateRow::new(
                "boundary-candidate-a".to_string(),
                "shared-island".to_string(),
                "boundary-cell-a".to_string(),
                "shared-neighborhood".to_string(),
                vec!["boundary-component-a".to_string()],
                vec!["segment-a".to_string()],
                vec!["left-loop".to_string()],
                vec!["shared-name".to_string()],
                BoundaryContact,
            ),
            PlanarBooleanOverlapIslandCandidateRow::new(
                "boundary-candidate-b".to_string(),
                "shared-island".to_string(),
                "boundary-cell-b".to_string(),
                "shared-neighborhood".to_string(),
                vec!["boundary-component-b".to_string()],
                vec!["segment-b".to_string()],
                vec!["right-loop".to_string()],
                vec!["shared-name".to_string()],
                BoundaryContact,
            ),
        ],
        PlanarBooleanOverlapIslandComponentCounters::default(),
    );
    let partition = PlanarBooleanOverlapIslandPartition::admit(&candidates)
        .expect("area envelope should partition separately from its boundary-edge witnesses");

    assert_eq!(partition.overlap_islands().rows().len(), 1);
    assert_eq!(partition.boundary_contact_components().rows().len(), 2);
    assert_eq!(partition.area_overlap_components().rows().len(), 1);
}
