use crate::topology_operators::TopologyMutationFamily;

use super::mutation_support::{
    TopologyQueryMutationFamilySupportStatus, TopologyQueryMutationLane,
    TopologyQueryMutationLaneExecutionShape, TopologyQueryMutationLaneSupportStatus,
    TopologyRuntimeMutationFamilySupportRow, TopologyRuntimeMutationLaneSupportRow,
};

pub(super) fn current_head_mutation_lane_support_row(
    lane: TopologyQueryMutationLane,
) -> TopologyRuntimeMutationLaneSupportRow {
    use TopologyQueryMutationLaneExecutionShape as Shape;
    use TopologyQueryMutationLaneSupportStatus as Status;

    let (execution_shape, reason) = match lane {
        TopologyQueryMutationLane::CreateTopologyEntity => (
            Shape::ScalarMutation,
            "current-head runtime admits scalar topology entity creation through query-native mutation authoring",
        ),
        TopologyQueryMutationLane::CreateInnerLoopOnExistingFace => (
            Shape::GraphComposition,
            "current-head runtime admits inner-loop membership creation through query graph composition",
        ),
        TopologyQueryMutationLane::RehomeAllOwnedHalfEdgesToNewWire => (
            Shape::GraphComposition,
            "current-head runtime admits half-edge wire rehome through query graph composition",
        ),
        TopologyQueryMutationLane::SplitConnectedHalfEdgeSetIntoNewWire => (
            Shape::GraphComposition,
            "current-head runtime admits connected half-edge wire split through query graph composition",
        ),
        TopologyQueryMutationLane::SplitSingleFaceFromTwoFaceShellToNewShell => (
            Shape::GraphComposition,
            "current-head runtime admits single-face shell split through query graph composition",
        ),
        TopologyQueryMutationLane::RehomeAllOwnedFacesToNewShell => (
            Shape::GraphComposition,
            "current-head runtime admits shell face rehome through query graph composition",
        ),
        TopologyQueryMutationLane::RetireTopologyEntity => (
            Shape::ScalarMutation,
            "current-head runtime admits scalar topology entity retirement through query-native mutation authoring",
        ),
        TopologyQueryMutationLane::DetachBoundaryMembership => (
            Shape::ScalarMutation,
            "current-head runtime admits scalar boundary-membership detach through query-native mutation authoring",
        ),
        TopologyQueryMutationLane::DetachRadialAdjacency => (
            Shape::ScalarMutation,
            "current-head runtime admits scalar radial-adjacency detach through query-native mutation authoring",
        ),
        TopologyQueryMutationLane::DetachShellOrWireMembership => (
            Shape::ScalarMutation,
            "current-head runtime admits scalar shell-or-wire membership detach through query-native mutation authoring",
        ),
        TopologyQueryMutationLane::RelocateHalfEdgeBeforeSuccessor => (
            Shape::GraphComposition,
            "current-head runtime admits single-edge successor relocation through query graph composition",
        ),
        TopologyQueryMutationLane::RelocateHalfEdgeSpanBeforeSuccessor => (
            Shape::GraphComposition,
            "current-head runtime admits successor-span relocation through query graph composition",
        ),
        TopologyQueryMutationLane::RewireLoopEndpoint => (
            Shape::ScalarMutation,
            "current-head runtime admits scalar loop-endpoint rewires through query-native mutation authoring",
        ),
        TopologyQueryMutationLane::SpliceRadialAdjacency => (
            Shape::ScalarMutation,
            "current-head runtime admits scalar radial-adjacency splice through query-native mutation authoring",
        ),
    };

    TopologyRuntimeMutationLaneSupportRow::new(lane, Status::Admitted, execution_shape, reason)
}

pub(super) fn current_head_mutation_family_support_row(
    family: TopologyMutationFamily,
) -> TopologyRuntimeMutationFamilySupportRow {
    use TopologyQueryMutationFamilySupportStatus as Status;
    use TopologyQueryMutationLane as Lane;

    let (status, admitted_lanes, reason) = match family {
        TopologyMutationFamily::CreateTopologyEntity => (
            Status::Admitted,
            vec![Lane::CreateTopologyEntity],
            "current-head runtime admits scalar topology entity creation",
        ),
        TopologyMutationFamily::RetireTopologyEntity => (
            Status::Admitted,
            vec![Lane::RetireTopologyEntity],
            "current-head runtime admits scalar topology entity retirement",
        ),
        TopologyMutationFamily::AttachBoundaryMembership => (
            Status::PartiallyAdmittedByLane,
            vec![Lane::CreateInnerLoopOnExistingFace],
            "current-head runtime admits boundary-membership attachment only through the inner-loop composed program",
        ),
        TopologyMutationFamily::AttachShellOrWireMembership => (
            Status::PartiallyAdmittedByLane,
            vec![
                Lane::RehomeAllOwnedHalfEdgesToNewWire,
                Lane::SplitConnectedHalfEdgeSetIntoNewWire,
                Lane::SplitSingleFaceFromTwoFaceShellToNewShell,
                Lane::RehomeAllOwnedFacesToNewShell,
            ],
            "current-head runtime admits shell-or-wire membership attachment only through named composed programs",
        ),
        TopologyMutationFamily::DetachBoundaryMembership => (
            Status::Admitted,
            vec![Lane::DetachBoundaryMembership],
            "current-head runtime admits scalar boundary-membership detach",
        ),
        TopologyMutationFamily::RewireLoopSuccessor => (
            Status::PartiallyAdmittedByLane,
            vec![
                Lane::RelocateHalfEdgeBeforeSuccessor,
                Lane::RelocateHalfEdgeSpanBeforeSuccessor,
            ],
            "current-head runtime admits loop-successor rewires only through composed relocation programs",
        ),
        TopologyMutationFamily::RewireLoopEndpoint => (
            Status::Admitted,
            vec![Lane::RewireLoopEndpoint],
            "current-head runtime admits scalar loop-endpoint rewires",
        ),
        TopologyMutationFamily::DetachShellOrWireMembership => (
            Status::Admitted,
            vec![Lane::DetachShellOrWireMembership],
            "current-head runtime admits scalar shell-or-wire membership detach",
        ),
        TopologyMutationFamily::SpliceRadialAdjacency => (
            Status::Admitted,
            vec![Lane::SpliceRadialAdjacency],
            "current-head runtime admits scalar radial-adjacency splice",
        ),
        TopologyMutationFamily::DetachRadialAdjacency => (
            Status::Admitted,
            vec![Lane::DetachRadialAdjacency],
            "current-head runtime admits scalar radial-adjacency detach",
        ),
    };

    TopologyRuntimeMutationFamilySupportRow::new(family, status, admitted_lanes, reason)
}
