use forge_relational::facade::identity::KindId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum TopologyEntityKind {
    Model,
    Body,
    Lump,
    Region,
    Shell,
    Face,
    Loop,
    Wire,
    HalfEdge,
    Edge,
    Vertex,
}

impl TopologyEntityKind {
    pub const WRAPPED_ALL: [super::EntityKind; 11] = [
        super::EntityKind::Topology(Self::Model),
        super::EntityKind::Topology(Self::Body),
        super::EntityKind::Topology(Self::Lump),
        super::EntityKind::Topology(Self::Region),
        super::EntityKind::Topology(Self::Shell),
        super::EntityKind::Topology(Self::Face),
        super::EntityKind::Topology(Self::Loop),
        super::EntityKind::Topology(Self::Wire),
        super::EntityKind::Topology(Self::HalfEdge),
        super::EntityKind::Topology(Self::Edge),
        super::EntityKind::Topology(Self::Vertex),
    ];

    pub const ALL: [Self; 11] = [
        Self::Model,
        Self::Body,
        Self::Lump,
        Self::Region,
        Self::Shell,
        Self::Face,
        Self::Loop,
        Self::Wire,
        Self::HalfEdge,
        Self::Edge,
        Self::Vertex,
    ];

    pub const fn kind_id(self) -> KindId {
        match self {
            Self::Model => KindId(1),
            Self::Body => KindId(2),
            Self::Lump => KindId(3),
            Self::Region => KindId(4),
            Self::Shell => KindId(5),
            Self::Face => KindId(6),
            Self::Loop => KindId(7),
            Self::HalfEdge => KindId(8),
            Self::Edge => KindId(9),
            Self::Vertex => KindId(10),
            Self::Wire => KindId(15),
        }
    }

    pub const fn kind_name(self) -> &'static str {
        match self {
            Self::Model => ".model",
            Self::Body => ".body",
            Self::Lump => ".lump",
            Self::Region => ".region",
            Self::Shell => ".shell",
            Self::Face => ".face",
            Self::Loop => ".loop",
            Self::Wire => ".wire",
            Self::HalfEdge => ".half_edge",
            Self::Edge => ".edge",
            Self::Vertex => ".vertex",
        }
    }

    pub fn from_kind_id(kind_id: KindId) -> Option<Self> {
        Some(match kind_id {
            KindId(1) => Self::Model,
            KindId(2) => Self::Body,
            KindId(3) => Self::Lump,
            KindId(4) => Self::Region,
            KindId(5) => Self::Shell,
            KindId(6) => Self::Face,
            KindId(7) => Self::Loop,
            KindId(8) => Self::HalfEdge,
            KindId(9) => Self::Edge,
            KindId(10) => Self::Vertex,
            KindId(15) => Self::Wire,
            _ => return None,
        })
    }
}
