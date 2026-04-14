use forge_relational::facade::identity::KindId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthGeometryRelationKind {
    FaceUsesSurfaceBinding,
    EdgeUsesCurveBinding,
    HalfEdgeUsesCoedgeBinding,
    VertexUsesGeometryBinding,
}

impl WorthGeometryRelationKind {
    pub const WRAPPED_ALL: [super::WorthRelationKind; 4] = [
        super::WorthRelationKind::Geometry(Self::FaceUsesSurfaceBinding),
        super::WorthRelationKind::Geometry(Self::EdgeUsesCurveBinding),
        super::WorthRelationKind::Geometry(Self::HalfEdgeUsesCoedgeBinding),
        super::WorthRelationKind::Geometry(Self::VertexUsesGeometryBinding),
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
            Self::FaceUsesSurfaceBinding => "worth.face_uses_surface_binding",
            Self::EdgeUsesCurveBinding => "worth.edge_uses_curve_binding",
            Self::HalfEdgeUsesCoedgeBinding => "worth.half_edge_uses_coedge_binding",
            Self::VertexUsesGeometryBinding => "worth.vertex_uses_geometry_binding",
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
