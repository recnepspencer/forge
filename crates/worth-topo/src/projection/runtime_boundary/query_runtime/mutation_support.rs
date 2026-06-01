use crate::topology_operators::TopologyMutationFamily;

use super::contracts::TopologyRuntimeSupport;
use super::mutation_support_rows::{
    current_head_mutation_family_support_row, current_head_mutation_lane_support_row,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopologyQueryMutationFamilySupportStatus {
    Denied,
    PartiallyAdmittedByLane,
    Admitted,
}

impl TopologyQueryMutationFamilySupportStatus {
    pub fn is_supported(self) -> bool {
        !matches!(self, Self::Denied)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TopologyQueryMutationLane {
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

impl TopologyQueryMutationLane {
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
pub enum TopologyQueryMutationLaneSupportStatus {
    Denied,
    Admitted,
}

impl TopologyQueryMutationLaneSupportStatus {
    pub fn is_admitted(self) -> bool {
        matches!(self, Self::Admitted)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopologyQueryMutationLaneExecutionShape {
    ScalarMutation,
    GraphComposition,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyRuntimeMutationFamilySupportRow {
    family: TopologyMutationFamily,
    status: TopologyQueryMutationFamilySupportStatus,
    admitted_lanes: Vec<TopologyQueryMutationLane>,
    reason: String,
    row_digest: String,
}

impl TopologyRuntimeMutationFamilySupportRow {
    pub fn family(&self) -> TopologyMutationFamily {
        self.family
    }

    pub fn status(&self) -> TopologyQueryMutationFamilySupportStatus {
        self.status
    }

    pub fn admitted_lanes(&self) -> &[TopologyQueryMutationLane] {
        &self.admitted_lanes
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }

    pub(super) fn new(
        family: TopologyMutationFamily,
        status: TopologyQueryMutationFamilySupportStatus,
        admitted_lanes: Vec<TopologyQueryMutationLane>,
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
pub struct TopologyRuntimeMutationLaneSupportRow {
    lane: TopologyQueryMutationLane,
    status: TopologyQueryMutationLaneSupportStatus,
    execution_shape: TopologyQueryMutationLaneExecutionShape,
    reason: String,
    row_digest: String,
}

impl TopologyRuntimeSupport {
    pub fn query_mutation_family_support_rows(&self) -> &[TopologyRuntimeMutationFamilySupportRow] {
        &self.query_mutation_family_support_rows
    }

    pub fn query_mutation_lane_support_rows(&self) -> &[TopologyRuntimeMutationLaneSupportRow] {
        &self.query_mutation_lane_support_rows
    }

    pub fn query_mutation_lane_support_status(
        &self,
        lane: TopologyQueryMutationLane,
    ) -> TopologyQueryMutationLaneSupportStatus {
        self.query_mutation_lane_support_rows
            .iter()
            .find(|row| row.lane == lane)
            .map(TopologyRuntimeMutationLaneSupportRow::status)
            .unwrap_or_else(|| {
                panic!(" runtime mutation-lane support rows should cover every declared lane")
            })
    }
}

impl TopologyRuntimeMutationLaneSupportRow {
    pub fn lane(&self) -> TopologyQueryMutationLane {
        self.lane
    }

    pub fn status(&self) -> TopologyQueryMutationLaneSupportStatus {
        self.status
    }

    pub fn execution_shape(&self) -> TopologyQueryMutationLaneExecutionShape {
        self.execution_shape
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }

    pub(super) fn new(
        lane: TopologyQueryMutationLane,
        status: TopologyQueryMutationLaneSupportStatus,
        execution_shape: TopologyQueryMutationLaneExecutionShape,
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

pub(super) fn current_head_mutation_lane_support_rows() -> Vec<TopologyRuntimeMutationLaneSupportRow>
{
    TopologyQueryMutationLane::ALL
        .into_iter()
        .map(current_head_mutation_lane_support_row)
        .collect()
}

pub(super) fn snapshot_mutation_lane_support_rows() -> Vec<TopologyRuntimeMutationLaneSupportRow> {
    TopologyQueryMutationLane::ALL
        .into_iter()
        .map(|lane| {
            let current_head_row = current_head_mutation_lane_support_row(lane);
            TopologyRuntimeMutationLaneSupportRow::new(
                lane,
                TopologyQueryMutationLaneSupportStatus::Denied,
                current_head_row.execution_shape(),
                "snapshot read-only runtime does not admit authoritative topology mutation application",
            )
        })
        .collect()
}

pub(crate) const ALL_QUERY_MUTATION_FAMILIES: [TopologyMutationFamily; 10] = [
    TopologyMutationFamily::CreateTopologyEntity,
    TopologyMutationFamily::RetireTopologyEntity,
    TopologyMutationFamily::AttachBoundaryMembership,
    TopologyMutationFamily::AttachShellOrWireMembership,
    TopologyMutationFamily::DetachBoundaryMembership,
    TopologyMutationFamily::RewireLoopSuccessor,
    TopologyMutationFamily::RewireLoopEndpoint,
    TopologyMutationFamily::DetachShellOrWireMembership,
    TopologyMutationFamily::SpliceRadialAdjacency,
    TopologyMutationFamily::DetachRadialAdjacency,
];

pub(super) fn current_head_mutation_family_support_rows(
) -> Vec<TopologyRuntimeMutationFamilySupportRow> {
    ALL_QUERY_MUTATION_FAMILIES
        .into_iter()
        .map(current_head_mutation_family_support_row)
        .collect()
}

pub(super) fn snapshot_mutation_family_support_rows() -> Vec<TopologyRuntimeMutationFamilySupportRow>
{
    ALL_QUERY_MUTATION_FAMILIES
        .into_iter()
        .map(|family| {
            TopologyRuntimeMutationFamilySupportRow::new(
                family,
                TopologyQueryMutationFamilySupportStatus::Denied,
                Vec::new(),
                "snapshot read-only runtime does not admit authoritative topology mutation application",
            )
        })
        .collect()
}
