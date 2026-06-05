mod binding_kind;
mod coedge_pcurve;
mod completeness;
mod edge_curve;
mod face_surface;
mod vertex_geometry;

pub use binding_kind::SpatialBindingKind;
pub use coedge_pcurve::{AdmittedCoedgePCurveBinding, CoedgeBindingSite, CoedgePCurveBindingSpec};
pub use completeness::{SpatialBindingCompleteness, SpatialBindingIncompleteness};
pub use edge_curve::{AdmittedEdgeCurveBinding, EdgeBindingSite, EdgeCurveBindingSpec};
pub use face_surface::{AdmittedFaceSurfaceBinding, FaceBindingSite, FaceSurfaceBindingSpec};
pub use vertex_geometry::{
    AdmittedVertexGeometryBinding, VertexBindingSite, VertexGeometryBindingSpec,
    VertexGeometryProvenanceKind, VertexToleranceRegime,
};

use crate::bindings::anchors::{
    AdmittedCoedgePCurveDirectionAnchorBinding, AdmittedCoedgePCurvePointAnchorBinding,
    AdmittedEdgeCurveDirectionAnchorBinding, AdmittedEdgeCurvePointAnchorBinding,
    AdmittedFaceSurfaceDirectionAnchorBinding, AdmittedFaceSurfacePointAnchorBinding,
};
use crate::bindings::identity::SpatialBindingIdentity;

#[derive(Clone, Debug, PartialEq)]
pub enum SpatialAdmittedPrimitiveBinding {
    FaceSurface(AdmittedFaceSurfaceBinding),
    EdgeCurve(AdmittedEdgeCurveBinding),
    CoedgePCurve(AdmittedCoedgePCurveBinding),
    VertexGeometry(AdmittedVertexGeometryBinding),
    FaceSurfacePointAnchor(AdmittedFaceSurfacePointAnchorBinding),
    EdgeCurvePointAnchor(AdmittedEdgeCurvePointAnchorBinding),
    CoedgePCurvePointAnchor(AdmittedCoedgePCurvePointAnchorBinding),
    FaceSurfaceDirectionAnchor(AdmittedFaceSurfaceDirectionAnchorBinding),
    EdgeCurveDirectionAnchor(AdmittedEdgeCurveDirectionAnchorBinding),
    CoedgePCurveDirectionAnchor(AdmittedCoedgePCurveDirectionAnchorBinding),
}

impl SpatialAdmittedPrimitiveBinding {
    pub fn kind(&self) -> SpatialBindingKind {
        match self {
            Self::FaceSurface(binding) => binding.kind(),
            Self::EdgeCurve(binding) => binding.kind(),
            Self::CoedgePCurve(binding) => binding.kind(),
            Self::VertexGeometry(binding) => binding.kind(),
            Self::FaceSurfacePointAnchor(binding) => binding.kind(),
            Self::EdgeCurvePointAnchor(binding) => binding.kind(),
            Self::CoedgePCurvePointAnchor(binding) => binding.kind(),
            Self::FaceSurfaceDirectionAnchor(binding) => binding.kind(),
            Self::EdgeCurveDirectionAnchor(binding) => binding.kind(),
            Self::CoedgePCurveDirectionAnchor(binding) => binding.kind(),
        }
    }

    pub fn identity(&self) -> &SpatialBindingIdentity {
        match self {
            Self::FaceSurface(binding) => binding.identity(),
            Self::EdgeCurve(binding) => binding.identity(),
            Self::CoedgePCurve(binding) => binding.identity(),
            Self::VertexGeometry(binding) => binding.identity(),
            Self::FaceSurfacePointAnchor(binding) => binding.identity(),
            Self::EdgeCurvePointAnchor(binding) => binding.identity(),
            Self::CoedgePCurvePointAnchor(binding) => binding.identity(),
            Self::FaceSurfaceDirectionAnchor(binding) => binding.identity(),
            Self::EdgeCurveDirectionAnchor(binding) => binding.identity(),
            Self::CoedgePCurveDirectionAnchor(binding) => binding.identity(),
        }
    }

    pub fn completeness(&self) -> &SpatialBindingCompleteness {
        match self {
            Self::FaceSurface(binding) => binding.completeness(),
            Self::EdgeCurve(binding) => binding.completeness(),
            Self::CoedgePCurve(binding) => binding.completeness(),
            Self::VertexGeometry(binding) => binding.completeness(),
            Self::FaceSurfacePointAnchor(binding) => binding.completeness(),
            Self::EdgeCurvePointAnchor(binding) => binding.completeness(),
            Self::CoedgePCurvePointAnchor(binding) => binding.completeness(),
            Self::FaceSurfaceDirectionAnchor(binding) => binding.completeness(),
            Self::EdgeCurveDirectionAnchor(binding) => binding.completeness(),
            Self::CoedgePCurveDirectionAnchor(binding) => binding.completeness(),
        }
    }

    pub fn topology_site_identity(&self) -> &str {
        match self {
            Self::FaceSurface(binding) => binding.site().topology_face_identity(),
            Self::EdgeCurve(binding) => binding.site().topology_edge_identity(),
            Self::CoedgePCurve(binding) => binding.site().topology_coedge_identity(),
            Self::VertexGeometry(binding) => binding.site().topology_vertex_identity(),
            Self::FaceSurfacePointAnchor(binding) => {
                binding.binding().site().topology_face_identity()
            }
            Self::EdgeCurvePointAnchor(binding) => {
                binding.binding().site().topology_edge_identity()
            }
            Self::CoedgePCurvePointAnchor(binding) => {
                binding.binding().site().topology_coedge_identity()
            }
            Self::FaceSurfaceDirectionAnchor(binding) => {
                binding.binding().site().topology_face_identity()
            }
            Self::EdgeCurveDirectionAnchor(binding) => {
                binding.binding().site().topology_edge_identity()
            }
            Self::CoedgePCurveDirectionAnchor(binding) => {
                binding.binding().site().topology_coedge_identity()
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SpatialBindingAuthorityError {
    MissingTopologyIdentity(SpatialBindingKind),
    UnsupportedTopologyBirthClass {
        binding_kind: SpatialBindingKind,
        topology_birth_class: &'static str,
    },
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
