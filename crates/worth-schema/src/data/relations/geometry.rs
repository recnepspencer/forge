use forge_relational::facade::identity::KindId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum GeometryRelationKind {
    FaceUsesSurfaceBinding,
    EdgeUsesCurveBinding,
    HalfEdgeUsesCoedgeBinding,
    VertexUsesGeometryBinding,
}

impl GeometryRelationKind {
    pub const WRAPPED_ALL: [super::RelationKind; 4] = [
        super::RelationKind::Geometry(Self::FaceUsesSurfaceBinding),
        super::RelationKind::Geometry(Self::EdgeUsesCurveBinding),
        super::RelationKind::Geometry(Self::HalfEdgeUsesCoedgeBinding),
        super::RelationKind::Geometry(Self::VertexUsesGeometryBinding),
    ];

    pub const ALL: [Self; 4] = [
        Self::FaceUsesSurfaceBinding,
        Self::EdgeUsesCurveBinding,
        Self::HalfEdgeUsesCoedgeBinding,
        Self::VertexUsesGeometryBinding,
    ];

    pub const fn kind_id(self) -> KindId {
        match self {
            Self::FaceUsesSurfaceBinding => KindId(114),
            Self::EdgeUsesCurveBinding => KindId(115),
            Self::HalfEdgeUsesCoedgeBinding => KindId(116),
            Self::VertexUsesGeometryBinding => KindId(117),
        }
    }

    pub const fn kind_name(self) -> &'static str {
        match self {
            Self::FaceUsesSurfaceBinding => ".face_uses_surface_binding",
            Self::EdgeUsesCurveBinding => ".edge_uses_curve_binding",
            Self::HalfEdgeUsesCoedgeBinding => ".half_edge_uses_coedge_binding",
            Self::VertexUsesGeometryBinding => ".vertex_uses_geometry_binding",
        }
    }

    pub fn from_kind_id(kind_id: KindId) -> Option<Self> {
        Some(match kind_id {
            KindId(114) => Self::FaceUsesSurfaceBinding,
            KindId(115) => Self::EdgeUsesCurveBinding,
            KindId(116) => Self::HalfEdgeUsesCoedgeBinding,
            KindId(117) => Self::VertexUsesGeometryBinding,
            _ => return None,
        })
    }
}
