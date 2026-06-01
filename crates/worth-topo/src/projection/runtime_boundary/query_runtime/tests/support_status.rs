use crate::projection::runtime_boundary::query_runtime::{
    TopologyQueryMutationFamilySupportStatus, TopologyQueryMutationLane,
    TopologyQueryMutationLaneSupportStatus, TopologyRuntimeSupport,
};
use crate::topology_operators::TopologyMutationFamily;

#[test]
fn current_head_runtime_partial_family_support_stays_disjoint_and_lane_backed() {
    let support = TopologyRuntimeSupport::current_head_authoritative();
    let family_rows = support.query_mutation_family_support_rows();

    assert_eq!(
        family_rows
            .iter()
            .filter(|row| {
                row.status() == TopologyQueryMutationFamilySupportStatus::PartiallyAdmittedByLane
            })
            .map(|row| row.family())
            .collect::<Vec<_>>()
            .as_slice(),
        &[
            TopologyMutationFamily::AttachBoundaryMembership,
            TopologyMutationFamily::AttachShellOrWireMembership,
            TopologyMutationFamily::RewireLoopSuccessor,
        ]
    );
    for family in [
        TopologyMutationFamily::AttachBoundaryMembership,
        TopologyMutationFamily::AttachShellOrWireMembership,
        TopologyMutationFamily::RewireLoopSuccessor,
    ] {
        assert!(!family_rows.iter().any(|row| {
            row.family() == family
                && row.status() == TopologyQueryMutationFamilySupportStatus::Admitted
        }));
        assert_eq!(
            support.query_mutation_family_support_status(family),
            TopologyQueryMutationFamilySupportStatus::PartiallyAdmittedByLane
        );
    }
    for lane in [
        TopologyQueryMutationLane::CreateInnerLoopOnExistingFace,
        TopologyQueryMutationLane::RehomeAllOwnedHalfEdgesToNewWire,
        TopologyQueryMutationLane::SplitConnectedHalfEdgeSetIntoNewWire,
        TopologyQueryMutationLane::SplitSingleFaceFromTwoFaceShellToNewShell,
        TopologyQueryMutationLane::RehomeAllOwnedFacesToNewShell,
        TopologyQueryMutationLane::RelocateHalfEdgeBeforeSuccessor,
        TopologyQueryMutationLane::RelocateHalfEdgeSpanBeforeSuccessor,
    ] {
        assert_eq!(
            support.query_mutation_lane_support_status(lane),
            TopologyQueryMutationLaneSupportStatus::Admitted
        );
    }
}
