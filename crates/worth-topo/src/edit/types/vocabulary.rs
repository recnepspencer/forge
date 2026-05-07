use serde::{Deserialize, Serialize};
use worth_schema::facade::WorthTopologyRelationKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthTopologyEditFamily {
    CreateTopologyEntity,
    RetireTopologyEntity,
    AttachBoundaryMembership,
    DetachBoundaryMembership,
    RewireLoopSuccessor,
    RewireLoopEndpoint,
    AttachShellOrWireMembership,
    DetachShellOrWireMembership,
    SpliceRadialAdjacency,
    DetachRadialAdjacency,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthTopologyEditChangedScope {
    Entity,
    Relation,
    LocalNeighborhood,
    Loop,
    Wire,
    Shell,
    RadialNeighborhood,
    Naming,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthTopologyEditNamingScope {
    EditedEntityNames,
    AdjacentEntityNames,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthTopologyDerivedRegion {
    LoopRegion,
    WireRegion,
    ShellRegion,
    RadialNeighborhoodRegion,
    EditLocalNeighborhoodRegion,
    NamingContinuityRegion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthBoundaryMembershipKind {
    FaceOuterLoop,
    FaceInnerLoop,
    LoopOwnsHalfEdge,
}

impl WorthBoundaryMembershipKind {
    pub const fn relation_kind(self) -> WorthTopologyRelationKind {
        match self {
            Self::FaceOuterLoop => WorthTopologyRelationKind::FaceOuterLoop,
            Self::FaceInnerLoop => WorthTopologyRelationKind::FaceInnerLoop,
            Self::LoopOwnsHalfEdge => WorthTopologyRelationKind::LoopOwnsHalfEdge,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthLoopSuccessorKind {
    Next,
    Prev,
}

impl WorthLoopSuccessorKind {
    pub const fn relation_kind(self) -> WorthTopologyRelationKind {
        match self {
            Self::Next => WorthTopologyRelationKind::HalfEdgeNext,
            Self::Prev => WorthTopologyRelationKind::HalfEdgePrev,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthLoopEndpointKind {
    Start,
    End,
}

impl WorthLoopEndpointKind {
    pub const fn relation_kind(self) -> WorthTopologyRelationKind {
        match self {
            Self::Start => WorthTopologyRelationKind::HalfEdgeStartsAtVertex,
            Self::End => WorthTopologyRelationKind::HalfEdgeEndsAtVertex,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthShellOrWireMembershipKind {
    RegionOwnsShell,
    ShellOwnsFace,
    WireOwnsHalfEdge,
}

impl WorthShellOrWireMembershipKind {
    pub const fn relation_kind(self) -> WorthTopologyRelationKind {
        match self {
            Self::RegionOwnsShell => WorthTopologyRelationKind::RegionOwnsShell,
            Self::ShellOwnsFace => WorthTopologyRelationKind::ShellOwnsFace,
            Self::WireOwnsHalfEdge => WorthTopologyRelationKind::WireOwnsHalfEdge,
        }
    }
}
