use crate::projection::runtime_boundary::query_runtime::mutation_support::ALL_QUERY_MUTATION_FAMILIES;
use crate::projection::runtime_boundary::query_runtime::{
    TopologyQueryMutationFamilySupportStatus, TopologyQueryMutationLane,
    TopologyQueryMutationLaneExecutionShape, TopologyQueryMutationLaneSupportStatus,
    TopologyRuntimeSupport,
};
use crate::topology_operators::TopologyMutationFamily;

#[test]
fn current_head_runtime_support_reports_typed_mutation_family_and_lane_rows() {
    let support = TopologyRuntimeSupport::current_head_authoritative();

    assert_eq!(
        support.query_mutation_family_support_rows().len(),
        ALL_QUERY_MUTATION_FAMILIES.len()
    );
    assert_eq!(
        support.query_mutation_lane_support_rows().len(),
        TopologyQueryMutationLane::ALL.len()
    );

    let attach_boundary = support
        .query_mutation_family_support_rows()
        .iter()
        .find(|row| row.family() == TopologyMutationFamily::AttachBoundaryMembership)
        .expect("attach-boundary family row should exist");
    assert_eq!(
        attach_boundary.status(),
        TopologyQueryMutationFamilySupportStatus::PartiallyAdmittedByLane
    );
    assert_eq!(
        attach_boundary.admitted_lanes(),
        &[TopologyQueryMutationLane::CreateInnerLoopOnExistingFace]
    );
    assert!(!attach_boundary.row_digest().is_empty());

    let relocate_span = support
        .query_mutation_lane_support_rows()
        .iter()
        .find(|row| row.lane() == TopologyQueryMutationLane::RelocateHalfEdgeSpanBeforeSuccessor)
        .expect("relocate-span lane row should exist");
    assert_eq!(
        relocate_span.status(),
        TopologyQueryMutationLaneSupportStatus::Admitted
    );
    assert_eq!(
        relocate_span.execution_shape(),
        TopologyQueryMutationLaneExecutionShape::GraphComposition
    );
    assert!(!relocate_span.row_digest().is_empty());
}

#[test]
fn snapshot_runtime_blocks_typed_mutation_family_and_lane_rows() {
    let support = TopologyRuntimeSupport::snapshot_read_only();

    assert_eq!(
        support.query_mutation_family_support_rows().len(),
        ALL_QUERY_MUTATION_FAMILIES.len()
    );
    assert_eq!(
        support.query_mutation_lane_support_rows().len(),
        TopologyQueryMutationLane::ALL.len()
    );
    for row in support.query_mutation_family_support_rows() {
        assert_eq!(
            row.status(),
            TopologyQueryMutationFamilySupportStatus::Denied
        );
        assert!(row.admitted_lanes().is_empty());
    }
    for row in support.query_mutation_lane_support_rows() {
        assert_eq!(row.status(), TopologyQueryMutationLaneSupportStatus::Denied);
    }
    assert_eq!(
        support.query_mutation_lane_support_status(TopologyQueryMutationLane::RewireLoopEndpoint),
        TopologyQueryMutationLaneSupportStatus::Denied
    );
}
