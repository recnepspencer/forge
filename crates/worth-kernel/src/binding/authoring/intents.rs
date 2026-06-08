use worth_spatial::facade::bindings::{
    CoedgePCurveBindingSpec, EdgeCurveBindingSpec, FaceSurfaceBindingSpec,
    VertexGeometryBindingSpec,
};

#[derive(Clone, Debug, PartialEq)]
pub enum AuthorPrimitiveBindingIntent {
    AttachSurfaceToFace(FaceSurfaceBindingSpec),
    AttachCurveToEdge(EdgeCurveBindingSpec),
    AttachPCurveToCoedge(CoedgePCurveBindingSpec),
    AttachVertexGeometry(VertexGeometryBindingSpec),
}

impl AuthorPrimitiveBindingIntent {
    pub fn attach_surface_to_face(spec: FaceSurfaceBindingSpec) -> Self {
        Self::AttachSurfaceToFace(spec)
    }

    pub fn attach_curve_to_edge(spec: EdgeCurveBindingSpec) -> Self {
        Self::AttachCurveToEdge(spec)
    }

    pub fn attach_pcurve_to_coedge(spec: CoedgePCurveBindingSpec) -> Self {
        Self::AttachPCurveToCoedge(spec)
    }

    pub fn attach_vertex_geometry(spec: VertexGeometryBindingSpec) -> Self {
        Self::AttachVertexGeometry(spec)
    }
}
