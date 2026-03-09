use forge_core::KernelError;
use forge_topo::projection::{
    ProjectedEdgeId, ProjectedFaceId, ProjectedHalfEdgeId, ProjectedLoopId,
    ProjectedTopologyQueries, ProjectedVertexId,
};

use super::super::{projected_topology_error_to_kernel_owned, SpecEnvelope};

impl SpecEnvelope {
    pub fn half_edge_next(
        &self,
        half_edge: ProjectedHalfEdgeId,
    ) -> Result<ProjectedHalfEdgeId, KernelError> {
        Ok(self.projection()?.half_edge_next(half_edge))
    }

    pub fn half_edge_prev(
        &self,
        half_edge: ProjectedHalfEdgeId,
    ) -> Result<ProjectedHalfEdgeId, KernelError> {
        Ok(self.projection()?.half_edge_prev(half_edge))
    }

    pub fn half_edge_radial_next(
        &self,
        half_edge: ProjectedHalfEdgeId,
    ) -> Result<ProjectedHalfEdgeId, KernelError> {
        Ok(self.projection()?.half_edge_radial_next(half_edge))
    }

    pub fn half_edge_face(
        &self,
        half_edge: ProjectedHalfEdgeId,
    ) -> Result<ProjectedFaceId, KernelError> {
        Ok(self.projection()?.half_edge_face(half_edge))
    }

    pub fn half_edge_origin(
        &self,
        half_edge: ProjectedHalfEdgeId,
    ) -> Result<ProjectedVertexId, KernelError> {
        Ok(self.projection()?.half_edge_origin(half_edge))
    }

    pub fn half_edge_edge(
        &self,
        half_edge: ProjectedHalfEdgeId,
    ) -> Result<ProjectedEdgeId, KernelError> {
        Ok(self.projection()?.half_edge_edge(half_edge))
    }

    pub fn edge_representative_half_edge(
        &self,
        edge: ProjectedEdgeId,
    ) -> Result<ProjectedHalfEdgeId, KernelError> {
        Ok(self.projection()?.edge_representative_half_edge(edge))
    }

    pub fn vertex_primary_half_edge(
        &self,
        vertex: ProjectedVertexId,
    ) -> Result<Option<ProjectedHalfEdgeId>, KernelError> {
        Ok(self.projection()?.vertex_primary_half_edge(vertex))
    }

    pub fn face_loops(&self, face: ProjectedFaceId) -> Result<Vec<ProjectedLoopId>, KernelError> {
        Ok(self.projection()?.face_loops(face))
    }

    pub fn shell_faces(
        &self,
        shell: forge_topo::projection::ProjectedShellId,
    ) -> Result<Vec<ProjectedFaceId>, KernelError> {
        Ok(self.projection()?.shell_faces(shell))
    }

    pub fn loop_half_edges(
        &self,
        loop_id: ProjectedLoopId,
    ) -> Result<Vec<ProjectedHalfEdgeId>, KernelError> {
        self.projection()?
            .loop_half_edges(loop_id)
            .map_err(projected_topology_error_to_kernel_owned)
    }

    pub fn face_half_edges(
        &self,
        face: ProjectedFaceId,
    ) -> Result<Vec<ProjectedHalfEdgeId>, KernelError> {
        self.projection()?
            .face_half_edges(face)
            .map_err(projected_topology_error_to_kernel_owned)
    }

    pub fn face_edges(&self, face: ProjectedFaceId) -> Result<Vec<ProjectedEdgeId>, KernelError> {
        self.projection()?
            .face_edges(face)
            .map_err(projected_topology_error_to_kernel_owned)
    }

    pub fn radial_half_edges(
        &self,
        half_edge: ProjectedHalfEdgeId,
    ) -> Result<Vec<ProjectedHalfEdgeId>, KernelError> {
        Ok(self.projection()?.radial_half_edges(half_edge))
    }

    pub fn edge_half_edges(
        &self,
        edge: ProjectedEdgeId,
    ) -> Result<Vec<ProjectedHalfEdgeId>, KernelError> {
        Ok(self.projection()?.edge_half_edges(edge))
    }

    pub fn edge_faces(&self, edge: ProjectedEdgeId) -> Result<Vec<ProjectedFaceId>, KernelError> {
        Ok(self.projection()?.edge_faces(edge))
    }

    pub fn radial_valence(&self, edge: ProjectedEdgeId) -> Result<usize, KernelError> {
        Ok(self.projection()?.radial_valence(edge))
    }

    pub fn is_boundary_edge(&self, edge: ProjectedEdgeId) -> Result<bool, KernelError> {
        Ok(self.projection()?.is_boundary_edge(edge))
    }

    pub fn vertex_outgoing_half_edges(
        &self,
        vertex: ProjectedVertexId,
    ) -> Result<Vec<ProjectedHalfEdgeId>, KernelError> {
        Ok(self.projection()?.vertex_outgoing_half_edges(vertex))
    }

    pub fn vertex_faces(
        &self,
        vertex: ProjectedVertexId,
    ) -> Result<Vec<ProjectedFaceId>, KernelError> {
        Ok(self.projection()?.vertex_faces(vertex))
    }

    pub fn vertex_disk_components(
        &self,
        vertex: ProjectedVertexId,
    ) -> Result<Vec<Vec<ProjectedHalfEdgeId>>, KernelError> {
        self.projection()?
            .vertex_disk_components(vertex)
            .map_err(projected_topology_error_to_kernel_owned)
    }
}
