use crate::bindings::anchors::{
    AdmittedCoedgePCurveDirectionAnchorBinding, AdmittedCoedgePCurvePointAnchorBinding,
    AdmittedEdgeCurveDirectionAnchorBinding, AdmittedEdgeCurvePointAnchorBinding,
    AdmittedFaceSurfaceDirectionAnchorBinding, AdmittedFaceSurfacePointAnchorBinding,
};
use crate::bindings::authority::{
    AdmittedCoedgePCurveBinding, AdmittedEdgeCurveBinding, AdmittedFaceSurfaceBinding,
    AdmittedVertexGeometryBinding, SpatialBindingCompleteness, SpatialBindingKind,
};
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

    pub fn identity(&self) -> &str {
        match self {
            Self::FaceSurface(binding) => binding.identity().as_str(),
            Self::EdgeCurve(binding) => binding.identity().as_str(),
            Self::CoedgePCurve(binding) => binding.identity().as_str(),
            Self::VertexGeometry(binding) => binding.identity().as_str(),
            Self::FaceSurfacePointAnchor(binding) => binding.identity().as_str(),
            Self::EdgeCurvePointAnchor(binding) => binding.identity().as_str(),
            Self::CoedgePCurvePointAnchor(binding) => binding.identity().as_str(),
            Self::FaceSurfaceDirectionAnchor(binding) => binding.identity().as_str(),
            Self::EdgeCurveDirectionAnchor(binding) => binding.identity().as_str(),
            Self::CoedgePCurveDirectionAnchor(binding) => binding.identity().as_str(),
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
