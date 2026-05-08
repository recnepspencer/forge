use crate::edit::TopologyEditFamily;

use super::contracts::TopologyRuntimeSupport;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopologyQueryEditFamilySupportStatus {
    Denied,
    PartiallyAdmittedByLane,
    Admitted,
}

impl TopologyQueryEditFamilySupportStatus {
    pub fn is_supported(self) -> bool {
        !matches!(self, Self::Denied)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TopologyQueryEditLane {
    CreateTopologyEntity,
    CreateInnerLoopOnExistingFace,
    RehomeAllOwnedHalfEdgesToNewWire,
    SplitConnectedHalfEdgeSetIntoNewWire,
    SplitSingleFaceFromTwoFaceShellToNewShell,
    RehomeAllOwnedFacesToNewShell,
    RetireTopologyEntity,
    DetachBoundaryMembership,
    DetachRadialAdjacency,
    DetachShellOrWireMembership,
    RelocateHalfEdgeBeforeSuccessor,
    RelocateHalfEdgeSpanBeforeSuccessor,
    RewireLoopEndpoint,
    SpliceRadialAdjacency,
}

impl TopologyQueryEditLane {
    pub const ALL: [Self; 14] = [
        Self::CreateTopologyEntity,
        Self::CreateInnerLoopOnExistingFace,
        Self::RehomeAllOwnedHalfEdgesToNewWire,
        Self::SplitConnectedHalfEdgeSetIntoNewWire,
        Self::SplitSingleFaceFromTwoFaceShellToNewShell,
        Self::RehomeAllOwnedFacesToNewShell,
        Self::RetireTopologyEntity,
        Self::DetachBoundaryMembership,
        Self::DetachRadialAdjacency,
        Self::DetachShellOrWireMembership,
        Self::RelocateHalfEdgeBeforeSuccessor,
        Self::RelocateHalfEdgeSpanBeforeSuccessor,
        Self::RewireLoopEndpoint,
        Self::SpliceRadialAdjacency,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::CreateTopologyEntity => "CreateTopologyEntity",
            Self::CreateInnerLoopOnExistingFace => "CreateInnerLoopOnExistingFace",
            Self::RehomeAllOwnedHalfEdgesToNewWire => "RehomeAllOwnedHalfEdgesToNewWire",
            Self::SplitConnectedHalfEdgeSetIntoNewWire => "SplitConnectedHalfEdgeSetIntoNewWire",
            Self::SplitSingleFaceFromTwoFaceShellToNewShell => {
                "SplitSingleFaceFromTwoFaceShellToNewShell"
            }
            Self::RehomeAllOwnedFacesToNewShell => "RehomeAllOwnedFacesToNewShell",
            Self::RetireTopologyEntity => "RetireTopologyEntity",
            Self::DetachBoundaryMembership => "DetachBoundaryMembership",
            Self::DetachRadialAdjacency => "DetachRadialAdjacency",
            Self::DetachShellOrWireMembership => "DetachShellOrWireMembership",
            Self::RelocateHalfEdgeBeforeSuccessor => "RelocateHalfEdgeBeforeSuccessor",
            Self::RelocateHalfEdgeSpanBeforeSuccessor => "RelocateHalfEdgeSpanBeforeSuccessor",
            Self::RewireLoopEndpoint => "RewireLoopEndpoint",
            Self::SpliceRadialAdjacency => "SpliceRadialAdjacency",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopologyQueryEditLaneSupportStatus {
    Denied,
    Admitted,
}

impl TopologyQueryEditLaneSupportStatus {
    pub fn is_admitted(self) -> bool {
        matches!(self, Self::Admitted)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopologyQueryEditLaneExecutionShape {
    ScalarMutation,
    GraphComposition,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyRuntimeEditFamilySupportRow {
    family: TopologyEditFamily,
    status: TopologyQueryEditFamilySupportStatus,
    admitted_lanes: Vec<TopologyQueryEditLane>,
    reason: String,
    row_digest: String,
}

impl TopologyRuntimeEditFamilySupportRow {
    pub fn family(&self) -> TopologyEditFamily {
        self.family
    }

    pub fn status(&self) -> TopologyQueryEditFamilySupportStatus {
        self.status
    }

    pub fn admitted_lanes(&self) -> &[TopologyQueryEditLane] {
        &self.admitted_lanes
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }

    pub(super) fn new(
        family: TopologyEditFamily,
        status: TopologyQueryEditFamilySupportStatus,
        admitted_lanes: Vec<TopologyQueryEditLane>,
        reason: impl Into<String>,
    ) -> Self {
        let reason = reason.into();
        let row_digest = format!(
            "family={family:?};status={status:?};lanes={};reason={reason}",
            admitted_lanes
                .iter()
                .map(|lane| lane.as_str())
                .collect::<Vec<_>>()
                .join(",")
        );
        Self {
            family,
            status,
            admitted_lanes,
            reason,
            row_digest,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyRuntimeEditLaneSupportRow {
    lane: TopologyQueryEditLane,
    status: TopologyQueryEditLaneSupportStatus,
    execution_shape: TopologyQueryEditLaneExecutionShape,
    reason: String,
    row_digest: String,
}

impl TopologyRuntimeSupport {
    pub fn query_edit_family_support_rows(&self) -> &[TopologyRuntimeEditFamilySupportRow] {
        &self.query_edit_family_support_rows
    }

    pub fn query_edit_lane_support_rows(&self) -> &[TopologyRuntimeEditLaneSupportRow] {
        &self.query_edit_lane_support_rows
    }

    pub fn query_edit_lane_support_status(
        &self,
        lane: TopologyQueryEditLane,
    ) -> TopologyQueryEditLaneSupportStatus {
        self.query_edit_lane_support_rows
            .iter()
            .find(|row| row.lane == lane)
            .map(TopologyRuntimeEditLaneSupportRow::status)
            .unwrap_or_else(|| {
                panic!(" runtime edit-lane support rows should cover every declared lane")
            })
    }
}

impl TopologyRuntimeEditLaneSupportRow {
    pub fn lane(&self) -> TopologyQueryEditLane {
        self.lane
    }

    pub fn status(&self) -> TopologyQueryEditLaneSupportStatus {
        self.status
    }

    pub fn execution_shape(&self) -> TopologyQueryEditLaneExecutionShape {
        self.execution_shape
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }

    pub(super) fn new(
        lane: TopologyQueryEditLane,
        status: TopologyQueryEditLaneSupportStatus,
        execution_shape: TopologyQueryEditLaneExecutionShape,
        reason: impl Into<String>,
    ) -> Self {
        let reason = reason.into();
        let row_digest = format!(
            "lane={:?};status={status:?};shape={execution_shape:?};reason={reason}",
            lane
        );
        Self {
            lane,
            status,
            execution_shape,
            reason,
            row_digest,
        }
    }
}

pub(super) fn current_head_edit_lane_support_rows() -> Vec<TopologyRuntimeEditLaneSupportRow> {
    TopologyQueryEditLane::ALL
        .into_iter()
        .map(current_head_edit_lane_support_row)
        .collect()
}

pub(super) fn snapshot_edit_lane_support_rows() -> Vec<TopologyRuntimeEditLaneSupportRow> {
    TopologyQueryEditLane::ALL
        .into_iter()
        .map(|lane| {
            let current_head_row = current_head_edit_lane_support_row(lane);
            TopologyRuntimeEditLaneSupportRow::new(
                lane,
                TopologyQueryEditLaneSupportStatus::Denied,
                current_head_row.execution_shape(),
                "snapshot read-only runtime does not admit authoritative topology edit execution",
            )
        })
        .collect()
}

pub(crate) const ALL_QUERY_EDIT_FAMILIES: [TopologyEditFamily; 10] = [
    TopologyEditFamily::CreateTopologyEntity,
    TopologyEditFamily::RetireTopologyEntity,
    TopologyEditFamily::AttachBoundaryMembership,
    TopologyEditFamily::AttachShellOrWireMembership,
    TopologyEditFamily::DetachBoundaryMembership,
    TopologyEditFamily::RewireLoopSuccessor,
    TopologyEditFamily::RewireLoopEndpoint,
    TopologyEditFamily::DetachShellOrWireMembership,
    TopologyEditFamily::SpliceRadialAdjacency,
    TopologyEditFamily::DetachRadialAdjacency,
];

pub(super) fn current_head_edit_family_support_rows() -> Vec<TopologyRuntimeEditFamilySupportRow> {
    ALL_QUERY_EDIT_FAMILIES
        .into_iter()
        .map(current_head_edit_family_support_row)
        .collect()
}

pub(super) fn snapshot_edit_family_support_rows() -> Vec<TopologyRuntimeEditFamilySupportRow> {
    ALL_QUERY_EDIT_FAMILIES
        .into_iter()
        .map(|family| {
            TopologyRuntimeEditFamilySupportRow::new(
                family,
                TopologyQueryEditFamilySupportStatus::Denied,
                Vec::new(),
                "snapshot read-only runtime does not admit authoritative topology edit execution",
            )
        })
        .collect()
}

fn current_head_edit_lane_support_row(
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

fn current_head_edit_family_support_row(
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
            "current-head runtime admits boundary-membership attachment only through the inner-loop graph-composed workflow",
        ),
        TopologyEditFamily::AttachShellOrWireMembership => (
            Status::PartiallyAdmittedByLane,
            vec![
                Lane::RehomeAllOwnedHalfEdgesToNewWire,
                Lane::SplitConnectedHalfEdgeSetIntoNewWire,
                Lane::SplitSingleFaceFromTwoFaceShellToNewShell,
                Lane::RehomeAllOwnedFacesToNewShell,
            ],
            "current-head runtime admits shell-or-wire membership attachment only through named graph-composed workflows",
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
            "current-head runtime admits loop-successor rewires only through graph-composed relocation workflows",
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
