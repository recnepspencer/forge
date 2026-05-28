use crate::topology_operators::TopologyEditFamily;

use super::edit_support::{
    TopologyQueryEditFamilySupportStatus, TopologyQueryEditLane,
    TopologyQueryEditLaneExecutionShape, TopologyQueryEditLaneSupportStatus,
    TopologyRuntimeEditFamilySupportRow, TopologyRuntimeEditLaneSupportRow,
};

pub(super) fn current_head_edit_lane_support_row(
    lane: TopologyQueryEditLane,
) -> TopologyRuntimeEditLaneSupportRow {
    use TopologyQueryEditLaneExecutionShape as Shape;
    use TopologyQueryEditLaneSupportStatus as Status;

    let (execution_shape, reason) = match lane {
        TopologyQueryEditLane::CreateTopologyEntity => (
            Shape::ScalarMutation,
            "current-head runtime admits scalar topology entity creation through query-native mutation authoring",
        ),
        TopologyQueryEditLane::CreateInnerLoopOnExistingFace => (
            Shape::GraphComposition,
            "current-head runtime admits inner-loop membership creation through query graph composition",
        ),
        TopologyQueryEditLane::RehomeAllOwnedHalfEdgesToNewWire => (
            Shape::GraphComposition,
            "current-head runtime admits half-edge wire rehome through query graph composition",
        ),
        TopologyQueryEditLane::SplitConnectedHalfEdgeSetIntoNewWire => (
            Shape::GraphComposition,
            "current-head runtime admits connected half-edge wire split through query graph composition",
        ),
        TopologyQueryEditLane::SplitSingleFaceFromTwoFaceShellToNewShell => (
            Shape::GraphComposition,
            "current-head runtime admits single-face shell split through query graph composition",
        ),
        TopologyQueryEditLane::RehomeAllOwnedFacesToNewShell => (
            Shape::GraphComposition,
            "current-head runtime admits shell face rehome through query graph composition",
        ),
        TopologyQueryEditLane::RetireTopologyEntity => (
            Shape::ScalarMutation,
            "current-head runtime admits scalar topology entity retirement through query-native mutation authoring",
        ),
        TopologyQueryEditLane::DetachBoundaryMembership => (
            Shape::ScalarMutation,
            "current-head runtime admits scalar boundary-membership detach through query-native mutation authoring",
        ),
        TopologyQueryEditLane::DetachRadialAdjacency => (
            Shape::ScalarMutation,
            "current-head runtime admits scalar radial-adjacency detach through query-native mutation authoring",
        ),
        TopologyQueryEditLane::DetachShellOrWireMembership => (
            Shape::ScalarMutation,
            "current-head runtime admits scalar shell-or-wire membership detach through query-native mutation authoring",
        ),
        TopologyQueryEditLane::RelocateHalfEdgeBeforeSuccessor => (
            Shape::GraphComposition,
            "current-head runtime admits single-edge successor relocation through query graph composition",
        ),
        TopologyQueryEditLane::RelocateHalfEdgeSpanBeforeSuccessor => (
            Shape::GraphComposition,
            "current-head runtime admits successor-span relocation through query graph composition",
        ),
        TopologyQueryEditLane::RewireLoopEndpoint => (
            Shape::ScalarMutation,
            "current-head runtime admits scalar loop-endpoint rewires through query-native mutation authoring",
        ),
        TopologyQueryEditLane::SpliceRadialAdjacency => (
            Shape::ScalarMutation,
            "current-head runtime admits scalar radial-adjacency splice through query-native mutation authoring",
        ),
    };

    TopologyRuntimeEditLaneSupportRow::new(lane, Status::Admitted, execution_shape, reason)
}

pub(super) fn current_head_edit_family_support_row(
    family: TopologyEditFamily,
) -> TopologyRuntimeEditFamilySupportRow {
    use TopologyQueryEditFamilySupportStatus as Status;
    use TopologyQueryEditLane as Lane;

    let (status, admitted_lanes, reason) = match family {
        TopologyEditFamily::CreateTopologyEntity => (
            Status::Admitted,
            vec![Lane::CreateTopologyEntity],
            "current-head runtime admits scalar topology entity creation",
        ),
        TopologyEditFamily::RetireTopologyEntity => (
            Status::Admitted,
            vec![Lane::RetireTopologyEntity],
            "current-head runtime admits scalar topology entity retirement",
        ),
        TopologyEditFamily::AttachBoundaryMembership => (
            Status::PartiallyAdmittedByLane,
            vec![Lane::CreateInnerLoopOnExistingFace],
            "current-head runtime admits boundary-membership attachment only through the inner-loop composed program",
        ),
        TopologyEditFamily::AttachShellOrWireMembership => (
            Status::PartiallyAdmittedByLane,
            vec![
                Lane::RehomeAllOwnedHalfEdgesToNewWire,
                Lane::SplitConnectedHalfEdgeSetIntoNewWire,
                Lane::SplitSingleFaceFromTwoFaceShellToNewShell,
                Lane::RehomeAllOwnedFacesToNewShell,
            ],
            "current-head runtime admits shell-or-wire membership attachment only through named composed programs",
        ),
        TopologyEditFamily::DetachBoundaryMembership => (
            Status::Admitted,
            vec![Lane::DetachBoundaryMembership],
            "current-head runtime admits scalar boundary-membership detach",
        ),
        TopologyEditFamily::RewireLoopSuccessor => (
            Status::PartiallyAdmittedByLane,
            vec![
                Lane::RelocateHalfEdgeBeforeSuccessor,
                Lane::RelocateHalfEdgeSpanBeforeSuccessor,
            ],
            "current-head runtime admits loop-successor rewires only through composed relocation programs",
        ),
        TopologyEditFamily::RewireLoopEndpoint => (
            Status::Admitted,
            vec![Lane::RewireLoopEndpoint],
            "current-head runtime admits scalar loop-endpoint rewires",
        ),
        TopologyEditFamily::DetachShellOrWireMembership => (
            Status::Admitted,
            vec![Lane::DetachShellOrWireMembership],
            "current-head runtime admits scalar shell-or-wire membership detach",
        ),
        TopologyEditFamily::SpliceRadialAdjacency => (
            Status::Admitted,
            vec![Lane::SpliceRadialAdjacency],
            "current-head runtime admits scalar radial-adjacency splice",
        ),
        TopologyEditFamily::DetachRadialAdjacency => (
            Status::Admitted,
            vec![Lane::DetachRadialAdjacency],
            "current-head runtime admits scalar radial-adjacency detach",
        ),
    };

    TopologyRuntimeEditFamilySupportRow::new(family, status, admitted_lanes, reason)
}




