#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialBindingKind {
    FaceSurface,
    EdgeCurve,
    CoedgePCurve,
    VertexGeometry,
}

impl SpatialBindingKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FaceSurface => "face-surface",
            Self::EdgeCurve => "edge-curve",
            Self::CoedgePCurve => "coedge-pcurve",
            Self::VertexGeometry => "vertex-geometry",
        }
    }
}
