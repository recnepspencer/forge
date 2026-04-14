mod diagnostics;
mod geometry;
mod naming;
mod topology;

use forge_relational::facade::identity::KindId;
use serde::{Deserialize, Serialize};

pub use diagnostics::WorthDiagnosticsRelationKind;
pub use geometry::WorthGeometryRelationKind;
pub use naming::WorthNamingRelationKind;
pub use topology::WorthTopologyRelationKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthRelationKind {
    Topology(WorthTopologyRelationKind),
    Geometry(WorthGeometryRelationKind),
    Naming(WorthNamingRelationKind),
    Diagnostics(WorthDiagnosticsRelationKind),
}

impl WorthRelationKind {
    pub const ALL: [Self; 22] = [
        Self::Topology(WorthTopologyRelationKind::ModelOwnsBody),
        Self::Topology(WorthTopologyRelationKind::BodyOwnsLump),
        Self::Topology(WorthTopologyRelationKind::LumpOwnsRegion),
        Self::Topology(WorthTopologyRelationKind::RegionOwnsShell),
        Self::Topology(WorthTopologyRelationKind::ShellOwnsFace),
        Self::Topology(WorthTopologyRelationKind::FaceOuterLoop),
        Self::Topology(WorthTopologyRelationKind::FaceInnerLoop),
        Self::Topology(WorthTopologyRelationKind::LoopOwnsHalfEdge),
        Self::Topology(WorthTopologyRelationKind::WireOwnsHalfEdge),
        Self::Topology(WorthTopologyRelationKind::HalfEdgeNext),
        Self::Topology(WorthTopologyRelationKind::HalfEdgePrev),
        Self::Topology(WorthTopologyRelationKind::HalfEdgeRadialNext),
        Self::Topology(WorthTopologyRelationKind::HalfEdgeUsesEdge),
        Self::Topology(WorthTopologyRelationKind::HalfEdgeStartsAtVertex),
        Self::Topology(WorthTopologyRelationKind::HalfEdgeEndsAtVertex),
        Self::Geometry(WorthGeometryRelationKind::FaceUsesSurfaceBinding),
        Self::Geometry(WorthGeometryRelationKind::EdgeUsesCurveBinding),
        Self::Geometry(WorthGeometryRelationKind::HalfEdgeUsesCoedgeBinding),
        Self::Geometry(WorthGeometryRelationKind::VertexUsesGeometryBinding),
        Self::Naming(WorthNamingRelationKind::PersistentNameTargetsEntity),
        Self::Diagnostics(WorthDiagnosticsRelationKind::WireHasInterpretation),
        Self::Diagnostics(WorthDiagnosticsRelationKind::ShellHasInterpretation),
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
        if let Some(kind) = WorthTopologyRelationKind::from_kind_id(kind_id) {
            return Some(Self::Topology(kind));
        }
        if let Some(kind) = WorthGeometryRelationKind::from_kind_id(kind_id) {
            return Some(Self::Geometry(kind));
        }
        if let Some(kind) = WorthNamingRelationKind::from_kind_id(kind_id) {
            return Some(Self::Naming(kind));
        }
        WorthDiagnosticsRelationKind::from_kind_id(kind_id).map(Self::Diagnostics)
    }
}
