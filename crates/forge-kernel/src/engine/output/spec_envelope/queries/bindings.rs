use forge_core::KernelError;
use forge_spec::facade::SpecNodeId;
use forge_topo::projection::{
    ProjectedEdgeId, ProjectedFaceId, ProjectedHalfEdgeId, ProjectedTopologyQueries,
    ProjectedVertexId,
};

use super::super::SpecEnvelope;

impl SpecEnvelope {
    pub fn face_surface_binding(
        &self,
        face: ProjectedFaceId,
    ) -> Result<Option<SpecNodeId>, KernelError> {
        Ok(self.projection()?.face_surface_binding(face))
    }

    pub fn half_edge_coedge_binding(
        &self,
        half_edge: ProjectedHalfEdgeId,
    ) -> Result<Option<SpecNodeId>, KernelError> {
        Ok(self.projection()?.half_edge_coedge_binding(half_edge))
    }

    pub fn edge_curve_binding(
        &self,
        edge: ProjectedEdgeId,
    ) -> Result<Option<SpecNodeId>, KernelError> {
        Ok(self.projection()?.edge_curve_binding(edge))
    }

    pub fn vertex_geometry_binding(
        &self,
        vertex: ProjectedVertexId,
    ) -> Result<Option<SpecNodeId>, KernelError> {
        Ok(self.projection()?.vertex_geometry_binding(vertex))
    }
}
