use forge_relational::facade::identity::KindId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum TopologyRelationKind {
    ModelOwnsBody,
    BodyOwnsLump,
    LumpOwnsRegion,
    RegionOwnsShell,
    ShellOwnsFace,
    FaceOuterLoop,
    FaceInnerLoop,
    LoopOwnsHalfEdge,
    WireOwnsHalfEdge,
    HalfEdgeNext,
    HalfEdgePrev,
    HalfEdgeRadialNext,
    HalfEdgeUsesEdge,
    HalfEdgeStartsAtVertex,
    HalfEdgeEndsAtVertex,
}

impl TopologyRelationKind {
    pub const WRAPPED_ALL: [super::RelationKind; 15] = [
        super::RelationKind::Topology(Self::ModelOwnsBody),
        super::RelationKind::Topology(Self::BodyOwnsLump),
        super::RelationKind::Topology(Self::LumpOwnsRegion),
        super::RelationKind::Topology(Self::RegionOwnsShell),
        super::RelationKind::Topology(Self::ShellOwnsFace),
        super::RelationKind::Topology(Self::FaceOuterLoop),
        super::RelationKind::Topology(Self::FaceInnerLoop),
        super::RelationKind::Topology(Self::LoopOwnsHalfEdge),
        super::RelationKind::Topology(Self::WireOwnsHalfEdge),
        super::RelationKind::Topology(Self::HalfEdgeNext),
        super::RelationKind::Topology(Self::HalfEdgePrev),
        super::RelationKind::Topology(Self::HalfEdgeRadialNext),
        super::RelationKind::Topology(Self::HalfEdgeUsesEdge),
        super::RelationKind::Topology(Self::HalfEdgeStartsAtVertex),
        super::RelationKind::Topology(Self::HalfEdgeEndsAtVertex),
    ];

    pub const ALL: [Self; 15] = [
        Self::ModelOwnsBody,
        Self::BodyOwnsLump,
        Self::LumpOwnsRegion,
        Self::RegionOwnsShell,
        Self::ShellOwnsFace,
        Self::FaceOuterLoop,
        Self::FaceInnerLoop,
        Self::LoopOwnsHalfEdge,
        Self::WireOwnsHalfEdge,
        Self::HalfEdgeNext,
        Self::HalfEdgePrev,
        Self::HalfEdgeRadialNext,
        Self::HalfEdgeUsesEdge,
        Self::HalfEdgeStartsAtVertex,
        Self::HalfEdgeEndsAtVertex,
    ];

    pub const fn kind_id(self) -> KindId {
        match self {
            Self::ModelOwnsBody => KindId(101),
            Self::BodyOwnsLump => KindId(102),
            Self::LumpOwnsRegion => KindId(103),
            Self::RegionOwnsShell => KindId(104),
            Self::ShellOwnsFace => KindId(105),
            Self::FaceOuterLoop => KindId(106),
            Self::FaceInnerLoop => KindId(107),
            Self::LoopOwnsHalfEdge => KindId(108),
            Self::HalfEdgeNext => KindId(109),
            Self::HalfEdgeRadialNext => KindId(110),
            Self::HalfEdgeUsesEdge => KindId(111),
            Self::HalfEdgeStartsAtVertex => KindId(112),
            Self::HalfEdgeEndsAtVertex => KindId(120),
            Self::WireOwnsHalfEdge => KindId(118),
            Self::HalfEdgePrev => KindId(119),
        }
    }

    pub const fn kind_name(self) -> &'static str {
        match self {
            Self::ModelOwnsBody => ".model_owns_body",
            Self::BodyOwnsLump => ".body_owns_lump",
            Self::LumpOwnsRegion => ".lump_owns_region",
            Self::RegionOwnsShell => ".region_owns_shell",
            Self::ShellOwnsFace => ".shell_owns_face",
            Self::FaceOuterLoop => ".face_outer_loop",
            Self::FaceInnerLoop => ".face_inner_loop",
            Self::LoopOwnsHalfEdge => ".loop_owns_half_edge",
            Self::WireOwnsHalfEdge => ".wire_owns_half_edge",
            Self::HalfEdgeNext => ".half_edge_next",
            Self::HalfEdgePrev => ".half_edge_prev",
            Self::HalfEdgeRadialNext => ".half_edge_radial_next",
            Self::HalfEdgeUsesEdge => ".half_edge_uses_edge",
            Self::HalfEdgeStartsAtVertex => ".half_edge_starts_at_vertex",
            Self::HalfEdgeEndsAtVertex => ".half_edge_ends_at_vertex",
        }
    }

    pub fn from_kind_id(kind_id: KindId) -> Option<Self> {
        Some(match kind_id {
            KindId(101) => Self::ModelOwnsBody,
            KindId(102) => Self::BodyOwnsLump,
            KindId(103) => Self::LumpOwnsRegion,
            KindId(104) => Self::RegionOwnsShell,
            KindId(105) => Self::ShellOwnsFace,
            KindId(106) => Self::FaceOuterLoop,
            KindId(107) => Self::FaceInnerLoop,
            KindId(108) => Self::LoopOwnsHalfEdge,
            KindId(109) => Self::HalfEdgeNext,
            KindId(110) => Self::HalfEdgeRadialNext,
            KindId(111) => Self::HalfEdgeUsesEdge,
            KindId(112) => Self::HalfEdgeStartsAtVertex,
            KindId(118) => Self::WireOwnsHalfEdge,
            KindId(119) => Self::HalfEdgePrev,
            KindId(120) => Self::HalfEdgeEndsAtVertex,
            _ => return None,
        })
    }
}
