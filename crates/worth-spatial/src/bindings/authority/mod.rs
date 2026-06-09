mod binding_kind;
mod coedge_pcurve;
mod completeness;
mod edge_curve;
mod face_surface;
mod vertex_geometry;

pub use binding_kind::SpatialBindingKind;
pub use coedge_pcurve::{CoedgeBindingSite, CoedgePCurveBindingSpec};
pub use completeness::{
    SpatialBindingCompleteness, SpatialBindingIllegalityReason, SpatialBindingIncompleteness,
    SpatialBindingUnsupportedReason,
};
pub use edge_curve::{EdgeBindingSite, EdgeCurveBindingSpec};
pub use face_surface::{FaceBindingSite, FaceSurfaceBindingSpec};
pub use vertex_geometry::{
    VertexBindingSite, VertexGeometryBindingSpec, VertexGeometryProvenanceKind,
    VertexToleranceRegime,
};

#[cfg(test)]
pub(crate) use completeness::AdmittedPartialBindingPosture;

pub(crate) use completeness::{
    evaluate_coedge_pcurve_completeness, evaluate_edge_curve_completeness,
    evaluate_face_surface_completeness, evaluate_vertex_geometry_completeness,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SpatialBindingAuthorityError {
    Unsupported(SpatialBindingUnsupportedReason),
    Illegal(SpatialBindingIllegalityReason),
}
