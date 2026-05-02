use crate::edit::WorthTopologyEditFamily;
use crate::query::{WorthTopologyQueryEditFamilySupportStatus, WorthTopologyRuntimeSupport};

#[test]
fn current_head_runtime_partial_family_support_stays_disjoint_and_lane_backed() {
    let support = WorthTopologyRuntimeSupport::current_head_authoritative();

    assert_eq!(
        support.partially_admitted_query_edit_families(),
        &[
            WorthTopologyEditFamily::AttachBoundaryMembership,
            WorthTopologyEditFamily::AttachShellOrWireMembership,
            WorthTopologyEditFamily::RewireLoopSuccessor,
        ]
    );
    for family in support.partially_admitted_query_edit_families() {
        assert!(!support.admitted_query_edit_families().contains(family));
        assert!(support.query_edit_family_supported(*family));
        assert_eq!(
            support.query_edit_family_support_status(*family),
            WorthTopologyQueryEditFamilySupportStatus::PartiallyAdmittedByLane
        );
    }
    assert!(support.query_edit_lane_supported("CreateInnerLoopOnExistingFace"));
    assert!(support.query_edit_lane_supported("RehomeAllOwnedHalfEdgesToNewWire"));
    assert!(support.query_edit_lane_supported("SplitConnectedHalfEdgeSetIntoNewWire"));
    assert!(support.query_edit_lane_supported("SplitSingleFaceFromTwoFaceShellToNewShell"));
    assert!(support.query_edit_lane_supported("RehomeAllOwnedFacesToNewShell"));
    assert!(support.query_edit_lane_supported("RelocateHalfEdgeBeforeSuccessor"));
    assert!(support.query_edit_lane_supported("RelocateHalfEdgeSpanBeforeSuccessor"));
}
