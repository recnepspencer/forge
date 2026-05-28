use crate::projection::runtime_boundary::query_runtime::edit_support::ALL_QUERY_EDIT_FAMILIES;
use crate::projection::runtime_boundary::query_runtime::{
    TopologyQueryEditFamilySupportStatus, TopologyQueryEditLane,
    TopologyQueryEditLaneExecutionShape, TopologyQueryEditLaneSupportStatus,
    TopologyRuntimeSupport,
};
use crate::topology_operators::TopologyEditFamily;

#[test]
fn current_head_runtime_support_reports_typed_edit_family_and_lane_rows() {
    let support = TopologyRuntimeSupport::current_head_authoritative();

    assert_eq!(
        support.query_edit_family_support_rows().len(),
        ALL_QUERY_EDIT_FAMILIES.len()
    );
    assert_eq!(
        support.query_edit_lane_support_rows().len(),
        TopologyQueryEditLane::ALL.len()
    );

    let attach_boundary = support
        .query_edit_family_support_rows()
        .iter()
        .find(|row| row.family() == TopologyEditFamily::AttachBoundaryMembership)
        .expect("attach-boundary family row should exist");
    assert_eq!(
        attach_boundary.status(),
        TopologyQueryEditFamilySupportStatus::PartiallyAdmittedByLane
    );
    assert_eq!(
        attach_boundary.admitted_lanes(),
        &[TopologyQueryEditLane::CreateInnerLoopOnExistingFace]
    );
    assert!(!attach_boundary.row_digest().is_empty());

    let relocate_span = support
        .query_edit_lane_support_rows()
        .iter()
        .find(|row| row.lane() == TopologyQueryEditLane::RelocateHalfEdgeSpanBeforeSuccessor)
        .expect("relocate-span lane row should exist");
    assert_eq!(
        relocate_span.status(),
        TopologyQueryEditLaneSupportStatus::Admitted
    );
    assert_eq!(
        relocate_span.execution_shape(),
        TopologyQueryEditLaneExecutionShape::GraphComposition
    );
    assert!(!relocate_span.row_digest().is_empty());
}

#[test]
fn snapshot_runtime_blocks_typed_edit_family_and_lane_rows() {
    let support = TopologyRuntimeSupport::snapshot_read_only();

    assert_eq!(
        support.query_edit_family_support_rows().len(),
        ALL_QUERY_EDIT_FAMILIES.len()
    );
    assert_eq!(
        support.query_edit_lane_support_rows().len(),
        TopologyQueryEditLane::ALL.len()
    );
    for row in support.query_edit_family_support_rows() {
        assert_eq!(row.status(), TopologyQueryEditFamilySupportStatus::Denied);
        assert!(row.admitted_lanes().is_empty());
    }
    for row in support.query_edit_lane_support_rows() {
        assert_eq!(row.status(), TopologyQueryEditLaneSupportStatus::Denied);
    }
    assert_eq!(
        support.query_edit_lane_support_status(TopologyQueryEditLane::RewireLoopEndpoint),
        TopologyQueryEditLaneSupportStatus::Denied
    );
}




