use forge_relational::facade::identity::KindId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthTopologyEntityKind {
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

impl WorthTopologyEntityKind {
    pub const WRAPPED_ALL: [super::WorthEntityKind; 11] = [
        super::WorthEntityKind::Topology(Self::Model),
        super::WorthEntityKind::Topology(Self::Body),
        super::WorthEntityKind::Topology(Self::Lump),
        super::WorthEntityKind::Topology(Self::Region),
        super::WorthEntityKind::Topology(Self::Shell),
        super::WorthEntityKind::Topology(Self::Face),
        super::WorthEntityKind::Topology(Self::Loop),
        super::WorthEntityKind::Topology(Self::Wire),
        super::WorthEntityKind::Topology(Self::HalfEdge),
        super::WorthEntityKind::Topology(Self::Edge),
        super::WorthEntityKind::Topology(Self::Vertex),
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
            Self::Model => "worth.model",
            Self::Body => "worth.body",
            Self::Lump => "worth.lump",
            Self::Region => "worth.region",
            Self::Shell => "worth.shell",
            Self::Face => "worth.face",
            Self::Loop => "worth.loop",
            Self::Wire => "worth.wire",
            Self::HalfEdge => "worth.half_edge",
            Self::Edge => "worth.edge",
            Self::Vertex => "worth.vertex",
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
