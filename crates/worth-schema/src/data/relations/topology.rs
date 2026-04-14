use forge_relational::facade::identity::KindId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthTopologyRelationKind {
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

impl WorthTopologyRelationKind {
    pub const WRAPPED_ALL: [super::WorthRelationKind; 15] = [
        super::WorthRelationKind::Topology(Self::ModelOwnsBody),
        super::WorthRelationKind::Topology(Self::BodyOwnsLump),
        super::WorthRelationKind::Topology(Self::LumpOwnsRegion),
        super::WorthRelationKind::Topology(Self::RegionOwnsShell),
        super::WorthRelationKind::Topology(Self::ShellOwnsFace),
        super::WorthRelationKind::Topology(Self::FaceOuterLoop),
        super::WorthRelationKind::Topology(Self::FaceInnerLoop),
        super::WorthRelationKind::Topology(Self::LoopOwnsHalfEdge),
        super::WorthRelationKind::Topology(Self::WireOwnsHalfEdge),
        super::WorthRelationKind::Topology(Self::HalfEdgeNext),
        super::WorthRelationKind::Topology(Self::HalfEdgePrev),
        super::WorthRelationKind::Topology(Self::HalfEdgeRadialNext),
        super::WorthRelationKind::Topology(Self::HalfEdgeUsesEdge),
        super::WorthRelationKind::Topology(Self::HalfEdgeStartsAtVertex),
        super::WorthRelationKind::Topology(Self::HalfEdgeEndsAtVertex),
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
            Self::ModelOwnsBody => "worth.model_owns_body",
            Self::BodyOwnsLump => "worth.body_owns_lump",
            Self::LumpOwnsRegion => "worth.lump_owns_region",
            Self::RegionOwnsShell => "worth.region_owns_shell",
            Self::ShellOwnsFace => "worth.shell_owns_face",
            Self::FaceOuterLoop => "worth.face_outer_loop",
            Self::FaceInnerLoop => "worth.face_inner_loop",
            Self::LoopOwnsHalfEdge => "worth.loop_owns_half_edge",
            Self::WireOwnsHalfEdge => "worth.wire_owns_half_edge",
            Self::HalfEdgeNext => "worth.half_edge_next",
            Self::HalfEdgePrev => "worth.half_edge_prev",
            Self::HalfEdgeRadialNext => "worth.half_edge_radial_next",
            Self::HalfEdgeUsesEdge => "worth.half_edge_uses_edge",
            Self::HalfEdgeStartsAtVertex => "worth.half_edge_starts_at_vertex",
            Self::HalfEdgeEndsAtVertex => "worth.half_edge_ends_at_vertex",
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
