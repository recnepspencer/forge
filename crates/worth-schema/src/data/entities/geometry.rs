use forge_relational::facade::identity::KindId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthGeometryEntityKind {
    SurfaceBinding,
    CurveBinding,
    CoedgeBinding,
    VertexGeometryBinding,
}

impl WorthGeometryEntityKind {
    pub const WRAPPED_ALL: [super::WorthEntityKind; 4] = [
        super::WorthEntityKind::Geometry(Self::SurfaceBinding),
        super::WorthEntityKind::Geometry(Self::CurveBinding),
        super::WorthEntityKind::Geometry(Self::CoedgeBinding),
        super::WorthEntityKind::Geometry(Self::VertexGeometryBinding),
    ];

    pub const ALL: [Self; 4] = [
        Self::SurfaceBinding,
        Self::CurveBinding,
        Self::CoedgeBinding,
        Self::VertexGeometryBinding,
    ];

    pub const fn kind_id(self) -> KindId {
        match self {
            Self::SurfaceBinding => KindId(11),
            Self::CurveBinding => KindId(12),
            Self::CoedgeBinding => KindId(13),
            Self::VertexGeometryBinding => KindId(14),
        }
    }

    pub const fn kind_name(self) -> &'static str {
        match self {
            Self::SurfaceBinding => "worth.surface_binding",
            Self::CurveBinding => "worth.curve_binding",
            Self::CoedgeBinding => "worth.coedge_binding",
            Self::VertexGeometryBinding => "worth.vertex_geometry_binding",
        }
    }

    pub fn from_kind_id(kind_id: KindId) -> Option<Self> {
        Some(match kind_id {
            KindId(11) => Self::SurfaceBinding,
            KindId(12) => Self::CurveBinding,
            KindId(13) => Self::CoedgeBinding,
            KindId(14) => Self::VertexGeometryBinding,
            _ => return None,
        })
    }
}
