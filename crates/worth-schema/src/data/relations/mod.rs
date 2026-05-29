mod diagnostics;
mod geometry;
mod naming;
mod topology;

use forge_relational::facade::identity::KindId;
use serde::{Deserialize, Serialize};

pub use diagnostics::DiagnosticsRelationKind;
pub use geometry::GeometryRelationKind;
pub use naming::NamingRelationKind;
pub use topology::TopologyRelationKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum RelationKind {
    Topology(TopologyRelationKind),
    Geometry(GeometryRelationKind),
    Naming(NamingRelationKind),
    Diagnostics(DiagnosticsRelationKind),
}

impl RelationKind {
    pub const ALL: [Self; 22] = [
        Self::Topology(TopologyRelationKind::ModelOwnsBody),
        Self::Topology(TopologyRelationKind::BodyOwnsLump),
        Self::Topology(TopologyRelationKind::LumpOwnsRegion),
        Self::Topology(TopologyRelationKind::RegionOwnsShell),
        Self::Topology(TopologyRelationKind::ShellOwnsFace),
        Self::Topology(TopologyRelationKind::FaceOuterLoop),
        Self::Topology(TopologyRelationKind::FaceInnerLoop),
        Self::Topology(TopologyRelationKind::LoopOwnsHalfEdge),
        Self::Topology(TopologyRelationKind::WireOwnsHalfEdge),
        Self::Topology(TopologyRelationKind::HalfEdgeNext),
        Self::Topology(TopologyRelationKind::HalfEdgePrev),
        Self::Topology(TopologyRelationKind::HalfEdgeRadialNext),
        Self::Topology(TopologyRelationKind::HalfEdgeUsesEdge),
        Self::Topology(TopologyRelationKind::HalfEdgeStartsAtVertex),
        Self::Topology(TopologyRelationKind::HalfEdgeEndsAtVertex),
        Self::Geometry(GeometryRelationKind::FaceUsesSurfaceBinding),
        Self::Geometry(GeometryRelationKind::EdgeUsesCurveBinding),
        Self::Geometry(GeometryRelationKind::HalfEdgeUsesCoedgeBinding),
        Self::Geometry(GeometryRelationKind::VertexUsesGeometryBinding),
        Self::Naming(NamingRelationKind::PersistentNameTargetsEntity),
        Self::Diagnostics(DiagnosticsRelationKind::WireHasInterpretation),
        Self::Diagnostics(DiagnosticsRelationKind::ShellHasInterpretation),
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
        if let Some(kind) = TopologyRelationKind::from_kind_id(kind_id) {
            return Some(Self::Topology(kind));
        }
        if let Some(kind) = GeometryRelationKind::from_kind_id(kind_id) {
            return Some(Self::Geometry(kind));
        }
        if let Some(kind) = NamingRelationKind::from_kind_id(kind_id) {
            return Some(Self::Naming(kind));
        }
        DiagnosticsRelationKind::from_kind_id(kind_id).map(Self::Diagnostics)
    }
}
