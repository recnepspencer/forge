mod diagnostics;
mod geometry;
mod naming;
mod topology;

use forge_relational::facade::identity::KindId;
use serde::{Deserialize, Serialize};

pub use diagnostics::WorthDiagnosticsEntityKind;
pub use geometry::WorthGeometryEntityKind;
pub use naming::WorthNamingEntityKind;
pub use topology::WorthTopologyEntityKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthEntityKind {
    Topology(WorthTopologyEntityKind),
    Geometry(WorthGeometryEntityKind),
    Naming(WorthNamingEntityKind),
    Diagnostics(WorthDiagnosticsEntityKind),
}

impl WorthEntityKind {
    pub const ALL: [Self; 18] = [
        Self::Topology(WorthTopologyEntityKind::Model),
        Self::Topology(WorthTopologyEntityKind::Body),
        Self::Topology(WorthTopologyEntityKind::Lump),
        Self::Topology(WorthTopologyEntityKind::Region),
        Self::Topology(WorthTopologyEntityKind::Shell),
        Self::Topology(WorthTopologyEntityKind::Face),
        Self::Topology(WorthTopologyEntityKind::Loop),
        Self::Topology(WorthTopologyEntityKind::Wire),
        Self::Topology(WorthTopologyEntityKind::HalfEdge),
        Self::Topology(WorthTopologyEntityKind::Edge),
        Self::Topology(WorthTopologyEntityKind::Vertex),
        Self::Geometry(WorthGeometryEntityKind::SurfaceBinding),
        Self::Geometry(WorthGeometryEntityKind::CurveBinding),
        Self::Geometry(WorthGeometryEntityKind::CoedgeBinding),
        Self::Geometry(WorthGeometryEntityKind::VertexGeometryBinding),
        Self::Naming(WorthNamingEntityKind::PersistentName),
        Self::Diagnostics(WorthDiagnosticsEntityKind::WireInterpretation),
        Self::Diagnostics(WorthDiagnosticsEntityKind::ShellInterpretation),
    ];

    pub const fn kind_id(self) -> KindId {
        match self {
            Self::Topology(kind) => kind.kind_id(),
            Self::Geometry(kind) => kind.kind_id(),
            Self::Naming(kind) => kind.kind_id(),
            Self::Diagnostics(kind) => kind.kind_id(),
        }
    }

    pub const fn kind_name(self) -> &'static str {
        match self {
            Self::Topology(kind) => kind.kind_name(),
            Self::Geometry(kind) => kind.kind_name(),
            Self::Naming(kind) => kind.kind_name(),
            Self::Diagnostics(kind) => kind.kind_name(),
        }
    }

    pub fn from_kind_id(kind_id: KindId) -> Option<Self> {
        if let Some(kind) = WorthTopologyEntityKind::from_kind_id(kind_id) {
            return Some(Self::Topology(kind));
        }
        if let Some(kind) = WorthGeometryEntityKind::from_kind_id(kind_id) {
            return Some(Self::Geometry(kind));
        }
        if let Some(kind) = WorthNamingEntityKind::from_kind_id(kind_id) {
            return Some(Self::Naming(kind));
        }
        WorthDiagnosticsEntityKind::from_kind_id(kind_id).map(Self::Diagnostics)
    }

    pub const fn is_topological(self) -> bool {
        matches!(self, Self::Topology(_))
    }
}
