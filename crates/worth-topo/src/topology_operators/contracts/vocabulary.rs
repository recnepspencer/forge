use schema::facade::platform::relations::TopologyRelationKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum TopologyEditFamily {
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

impl TopologyEditFamily {
    pub const ALL: [Self; 10] = [
        Self::CreateTopologyEntity,
        Self::RetireTopologyEntity,
        Self::AttachBoundaryMembership,
        Self::DetachBoundaryMembership,
        Self::RewireLoopSuccessor,
        Self::RewireLoopEndpoint,
        Self::AttachShellOrWireMembership,
        Self::DetachShellOrWireMembership,
        Self::SpliceRadialAdjacency,
        Self::DetachRadialAdjacency,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum TopologyEditChangedScope {
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
pub enum TopologyEditNamingScope {
    EditedEntityNames,
    AdjacentEntityNames,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum TopologyDerivedRegion {
    LoopRegion,
    WireRegion,
    ShellRegion,
    RadialNeighborhoodRegion,
    EditLocalNeighborhoodRegion,
    NamingContinuityRegion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum TopologyEditDerivedFallbackPolicy {
    AllowExplicitFallback,
    RejectAnyFallback,
}

impl TopologyEditDerivedFallbackPolicy {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllowExplicitFallback => "allow_explicit_fallback",
            Self::RejectAnyFallback => "reject_any_fallback",
        }
    }

    pub const fn is_exceeded_by(self, fallback_count: usize) -> bool {
        match self {
            Self::AllowExplicitFallback => false,
            Self::RejectAnyFallback => fallback_count > 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum BoundaryMembershipKind {
    FaceOuterLoop,
    FaceInnerLoop,
    LoopOwnsHalfEdge,
}

impl BoundaryMembershipKind {
    pub const fn relation_kind(self) -> TopologyRelationKind {
        match self {
            Self::FaceOuterLoop => TopologyRelationKind::FaceOuterLoop,
            Self::FaceInnerLoop => TopologyRelationKind::FaceInnerLoop,
            Self::LoopOwnsHalfEdge => TopologyRelationKind::LoopOwnsHalfEdge,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum LoopSuccessorKind {
    Next,
    Prev,
}

impl LoopSuccessorKind {
    pub const fn relation_kind(self) -> TopologyRelationKind {
        match self {
            Self::Next => TopologyRelationKind::HalfEdgeNext,
            Self::Prev => TopologyRelationKind::HalfEdgePrev,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum LoopEndpointKind {
    Start,
    End,
}

impl LoopEndpointKind {
    pub const fn relation_kind(self) -> TopologyRelationKind {
        match self {
            Self::Start => TopologyRelationKind::HalfEdgeStartsAtVertex,
            Self::End => TopologyRelationKind::HalfEdgeEndsAtVertex,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ShellOrWireMembershipKind {
    RegionOwnsShell,
    ShellOwnsFace,
    WireOwnsHalfEdge,
}

impl ShellOrWireMembershipKind {
    pub const fn relation_kind(self) -> TopologyRelationKind {
        match self {
            Self::RegionOwnsShell => TopologyRelationKind::RegionOwnsShell,
            Self::ShellOwnsFace => TopologyRelationKind::ShellOwnsFace,
            Self::WireOwnsHalfEdge => TopologyRelationKind::WireOwnsHalfEdge,
        }
    }
}
