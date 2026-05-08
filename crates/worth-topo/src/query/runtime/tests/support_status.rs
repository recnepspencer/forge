use crate::edit::TopologyEditFamily;
use crate::query::{
    TopologyQueryEditFamilySupportStatus, TopologyQueryEditLane,
    TopologyQueryEditLaneSupportStatus, TopologyRuntimeSupport,
};

#[test]
fn current_head_runtime_partial_family_support_stays_disjoint_and_lane_backed() {
    let support = TopologyRuntimeSupport::current_head_authoritative();
    let family_rows = support.query_edit_family_support_rows();

    assert_eq!(
        family_rows
            .iter()
            .filter(|row| {
                row.status() == TopologyQueryEditFamilySupportStatus::PartiallyAdmittedByLane
            })
            .map(|row| row.family())
            .collect::<Vec<_>>()
            .as_slice(),
        &[
            TopologyEditFamily::AttachBoundaryMembership,
            TopologyEditFamily::AttachShellOrWireMembership,
            TopologyEditFamily::RewireLoopSuccessor,
        ]
    );
    for family in [
        TopologyEditFamily::AttachBoundaryMembership,
        TopologyEditFamily::AttachShellOrWireMembership,
        TopologyEditFamily::RewireLoopSuccessor,
    ] {
        assert!(!family_rows.iter().any(|row| {
            row.family() == family && row.status() == TopologyQueryEditFamilySupportStatus::Admitted
        }));
        assert_eq!(
            support.query_edit_family_support_status(family),
            TopologyQueryEditFamilySupportStatus::PartiallyAdmittedByLane
        );
    }
    for lane in [
        TopologyQueryEditLane::CreateInnerLoopOnExistingFace,
        TopologyQueryEditLane::RehomeAllOwnedHalfEdgesToNewWire,
        TopologyQueryEditLane::SplitConnectedHalfEdgeSetIntoNewWire,
        TopologyQueryEditLane::SplitSingleFaceFromTwoFaceShellToNewShell,
        TopologyQueryEditLane::RehomeAllOwnedFacesToNewShell,
        TopologyQueryEditLane::RelocateHalfEdgeBeforeSuccessor,
        TopologyQueryEditLane::RelocateHalfEdgeSpanBeforeSuccessor,
    ] {
        assert_eq!(
            support.query_edit_lane_support_status(lane),
            TopologyQueryEditLaneSupportStatus::Admitted
        );
    }
}
