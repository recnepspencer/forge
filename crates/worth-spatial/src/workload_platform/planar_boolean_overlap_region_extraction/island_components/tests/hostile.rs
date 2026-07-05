use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanOverlapIslandCandidateKind::{AreaOverlap, BoundaryContact},
    PlanarBooleanOverlapIslandCandidateRow, PlanarBooleanOverlapIslandCandidateSet,
    PlanarBooleanOverlapIslandComponentCounters, PlanarBooleanOverlapIslandComponentDenialKind,
    PlanarBooleanOverlapIslandPartition,
};

#[test]
fn overlap_island_partition_rejects_mixed_islands_that_share_one_contact_basis() {
    let candidates = PlanarBooleanOverlapIslandCandidateSet::new(
        "hostile-candidate-set".to_string(),
        "hostile-request".to_string(),
        "hostile-arrangement".to_string(),
        "hostile-cell-set".to_string(),
        "hostile-order".to_string(),
        vec![
            PlanarBooleanOverlapIslandCandidateRow::new(
                "area-candidate".to_string(),
                "shared-island".to_string(),
                "area-cell".to_string(),
                "shared-neighborhood".to_string(),
                vec!["shared-boundary-component".to_string()],
                vec!["shared-segment-a".to_string()],
                vec!["left-loop".to_string(), "right-loop".to_string()],
                vec!["shared-name".to_string()],
                AreaOverlap,
            ),
            PlanarBooleanOverlapIslandCandidateRow::new(
                "boundary-candidate".to_string(),
                "shared-island".to_string(),
                "boundary-cell".to_string(),
                "shared-neighborhood".to_string(),
                vec!["shared-boundary-component".to_string()],
                vec!["shared-segment-b".to_string()],
                vec!["left-loop".to_string()],
                vec!["shared-name".to_string()],
                BoundaryContact,
            ),
        ],
        PlanarBooleanOverlapIslandComponentCounters::default(),
    );
    let denial = PlanarBooleanOverlapIslandPartition::admit(&candidates)
        .expect_err("mixed island with one contact basis should deny partition");

    assert_eq!(
        denial.kind(),
        PlanarBooleanOverlapIslandComponentDenialKind::MixedIslandPartitionDenied
    );
}

#[test]
fn overlap_island_partition_rejects_indirectly_mixed_component_groups() {
    let candidates = PlanarBooleanOverlapIslandCandidateSet::new(
        "indirect-mixed-candidate-set".to_string(),
        "indirect-mixed-request".to_string(),
        "indirect-mixed-arrangement".to_string(),
        "indirect-mixed-cell-set".to_string(),
        "indirect-mixed-order".to_string(),
        vec![
            PlanarBooleanOverlapIslandCandidateRow::new(
                "boundary-candidate-a".to_string(),
                "shared-island".to_string(),
                "boundary-cell-a".to_string(),
                "shared-neighborhood".to_string(),
                vec!["boundary-component-a".to_string()],
                vec!["shared-segment".to_string()],
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
                vec!["shared-segment".to_string()],
                vec!["shared-loop".to_string()],
                vec!["shared-name".to_string()],
                BoundaryContact,
            ),
            PlanarBooleanOverlapIslandCandidateRow::new(
                "area-candidate".to_string(),
                "shared-island".to_string(),
                "area-cell".to_string(),
                "shared-neighborhood".to_string(),
                vec!["area-boundary-component".to_string()],
                vec!["shared-segment".to_string()],
                vec!["shared-loop".to_string(), "area-loop".to_string()],
                vec!["shared-name".to_string()],
                AreaOverlap,
            ),
        ],
        PlanarBooleanOverlapIslandComponentCounters::default(),
    );
    let denial = PlanarBooleanOverlapIslandPartition::admit(&candidates)
        .expect_err("indirectly connected mixed group should deny partition");

    assert_eq!(
        denial.kind(),
        PlanarBooleanOverlapIslandComponentDenialKind::MixedIslandPartitionDenied
    );
}
