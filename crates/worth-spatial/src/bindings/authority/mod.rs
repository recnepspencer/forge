mod binding_kind;
mod coedge_pcurve;
mod completeness;
mod edge_curve;
mod face_surface;
mod vertex_geometry;

pub use binding_kind::SpatialBindingKind;
pub use coedge_pcurve::{AdmittedCoedgePCurveBinding, CoedgeBindingSite, CoedgePCurveBindingSpec};
pub(crate) use completeness::{
    evaluate_coedge_pcurve_completeness, evaluate_edge_curve_completeness,
    evaluate_face_surface_completeness, evaluate_vertex_geometry_completeness,
};
pub use completeness::{
    AdmittedPartialBindingPosture, SpatialBindingCompleteness, SpatialBindingIllegalityReason,
    SpatialBindingIncompleteness, SpatialBindingUnsupportedReason,
};
pub use edge_curve::{AdmittedEdgeCurveBinding, EdgeBindingSite, EdgeCurveBindingSpec};
pub use face_surface::{AdmittedFaceSurfaceBinding, FaceBindingSite, FaceSurfaceBindingSpec};
pub use vertex_geometry::{
    AdmittedVertexGeometryBinding, VertexBindingSite, VertexGeometryBindingSpec,
    VertexGeometryProvenanceKind, VertexToleranceRegime,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SpatialBindingAuthorityError {
    Unsupported(SpatialBindingUnsupportedReason),
    Illegal(SpatialBindingIllegalityReason),
}

pub fn attach_surface_to_face(
    spec: FaceSurfaceBindingSpec,
) -> Result<AdmittedFaceSurfaceBinding, SpatialBindingAuthorityError> {
    AdmittedFaceSurfaceBinding::admit(spec)
}

pub fn attach_curve_to_edge(
    spec: EdgeCurveBindingSpec,
) -> Result<AdmittedEdgeCurveBinding, SpatialBindingAuthorityError> {
    AdmittedEdgeCurveBinding::admit(spec)
}

pub fn attach_pcurve_to_coedge(
    spec: CoedgePCurveBindingSpec,
) -> Result<AdmittedCoedgePCurveBinding, SpatialBindingAuthorityError> {
    AdmittedCoedgePCurveBinding::admit(spec)
}

pub fn attach_vertex_geometry(
    spec: VertexGeometryBindingSpec,
) -> Result<AdmittedVertexGeometryBinding, SpatialBindingAuthorityError> {
    AdmittedVertexGeometryBinding::admit(spec)
}
