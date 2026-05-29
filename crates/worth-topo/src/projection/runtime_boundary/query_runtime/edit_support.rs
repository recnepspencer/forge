use crate::topology_operators::TopologyEditFamily;

use super::contracts::TopologyRuntimeSupport;
use super::edit_support_rows::{
    current_head_edit_family_support_row, current_head_edit_lane_support_row,
};

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




