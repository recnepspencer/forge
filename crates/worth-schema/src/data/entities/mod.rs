//! Worth entity catalogs for platform bootstrap and descriptor assembly.
//!
//! This module is the authority for Worth-specific entity identity and lower
//! relational contract data. It is not the ordinary Query lifecycle entry
//! surface; downstream runtime work should enter through `forge-query`.

mod diagnostics;
mod geometry;
mod naming;
mod topology;

use forge_relational::facade::identity::KindId;
use serde::{Deserialize, Serialize};

pub use diagnostics::DiagnosticsEntityKind;
pub use geometry::GeometryEntityKind;
pub use naming::NamingEntityKind;
pub use topology::TopologyEntityKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum EntityKind {
    Topology(TopologyEntityKind),
    Geometry(GeometryEntityKind),
    Naming(NamingEntityKind),
    Diagnostics(DiagnosticsEntityKind),
}

impl EntityKind {
    pub const ALL: [Self; 18] = [
        Self::Topology(TopologyEntityKind::Model),
        Self::Topology(TopologyEntityKind::Body),
        Self::Topology(TopologyEntityKind::Lump),
        Self::Topology(TopologyEntityKind::Region),
        Self::Topology(TopologyEntityKind::Shell),
        Self::Topology(TopologyEntityKind::Face),
        Self::Topology(TopologyEntityKind::Loop),
        Self::Topology(TopologyEntityKind::Wire),
        Self::Topology(TopologyEntityKind::HalfEdge),
        Self::Topology(TopologyEntityKind::Edge),
        Self::Topology(TopologyEntityKind::Vertex),
        Self::Geometry(GeometryEntityKind::SurfaceBinding),
        Self::Geometry(GeometryEntityKind::CurveBinding),
        Self::Geometry(GeometryEntityKind::CoedgeBinding),
        Self::Geometry(GeometryEntityKind::VertexGeometryBinding),
        Self::Naming(NamingEntityKind::PersistentName),
        Self::Diagnostics(DiagnosticsEntityKind::WireInterpretation),
        Self::Diagnostics(DiagnosticsEntityKind::ShellInterpretation),
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
        if let Some(kind) = TopologyEntityKind::from_kind_id(kind_id) {
            return Some(Self::Topology(kind));
        }
        if let Some(kind) = GeometryEntityKind::from_kind_id(kind_id) {
            return Some(Self::Geometry(kind));
        }
        if let Some(kind) = NamingEntityKind::from_kind_id(kind_id) {
            return Some(Self::Naming(kind));
        }
        DiagnosticsEntityKind::from_kind_id(kind_id).map(Self::Diagnostics)
    }

    pub const fn is_topological(self) -> bool {
        matches!(self, Self::Topology(_))
    }
}
