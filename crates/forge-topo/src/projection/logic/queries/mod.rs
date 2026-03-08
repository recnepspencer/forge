mod edge;
mod face;
mod vertex;

use crate::projection::data::{
    ProjectedEdgeId, ProjectedFaceId, ProjectedHalfEdgeId, ProjectedLoopId, ProjectedTopology,
    ProjectedTopologyError, ProjectedVertexId,
};

pub trait ProjectedTopologyQueries {
    fn shell_faces(&self, shell: crate::projection::data::ProjectedShellId) -> Vec<ProjectedFaceId>;
    fn face_loops(&self, face: ProjectedFaceId) -> Vec<ProjectedLoopId>;
    fn loop_half_edges(
        &self,
        loop_id: ProjectedLoopId,
    ) -> Result<Vec<ProjectedHalfEdgeId>, ProjectedTopologyError>;
    fn face_half_edges(
        &self,
        face: ProjectedFaceId,
    ) -> Result<Vec<ProjectedHalfEdgeId>, ProjectedTopologyError>;
    fn face_edges(&self, face: ProjectedFaceId) -> Result<Vec<ProjectedEdgeId>, ProjectedTopologyError>;
    fn radial_half_edges(&self, half_edge: ProjectedHalfEdgeId) -> Vec<ProjectedHalfEdgeId>;
    fn edge_half_edges(&self, edge: ProjectedEdgeId) -> Vec<ProjectedHalfEdgeId>;
    fn edge_faces(&self, edge: ProjectedEdgeId) -> Vec<ProjectedFaceId>;
    fn radial_valence(&self, edge: ProjectedEdgeId) -> usize;
    fn is_boundary_edge(&self, edge: ProjectedEdgeId) -> bool;
    fn vertex_outgoing_half_edges(&self, vertex: ProjectedVertexId) -> Vec<ProjectedHalfEdgeId>;
    fn vertex_faces(&self, vertex: ProjectedVertexId) -> Vec<ProjectedFaceId>;
    fn vertex_disk_components(
        &self,
        vertex: ProjectedVertexId,
    ) -> Result<Vec<Vec<ProjectedHalfEdgeId>>, ProjectedTopologyError>;
}

impl ProjectedTopologyQueries for ProjectedTopology {
    fn shell_faces(&self, shell: crate::projection::data::ProjectedShellId) -> Vec<ProjectedFaceId> {
        face::shell_faces(self, shell)
    }

    fn face_loops(&self, face: ProjectedFaceId) -> Vec<ProjectedLoopId> {
        face::face_loops(self, face)
    }

    fn loop_half_edges(
        &self,
        loop_id: ProjectedLoopId,
    ) -> Result<Vec<ProjectedHalfEdgeId>, ProjectedTopologyError> {
        face::loop_half_edges(self, loop_id)
    }

    fn face_half_edges(
        &self,
        face: ProjectedFaceId,
    ) -> Result<Vec<ProjectedHalfEdgeId>, ProjectedTopologyError> {
        face::face_half_edges(self, face)
    }

    fn face_edges(
        &self,
        face: ProjectedFaceId,
    ) -> Result<Vec<ProjectedEdgeId>, ProjectedTopologyError> {
        face::face_edges(self, face)
    }

    fn radial_half_edges(&self, half_edge: ProjectedHalfEdgeId) -> Vec<ProjectedHalfEdgeId> {
        edge::radial_half_edges(self, half_edge)
    }

    fn edge_half_edges(&self, edge: ProjectedEdgeId) -> Vec<ProjectedHalfEdgeId> {
        edge::edge_half_edges(self, edge)
    }

    fn edge_faces(&self, edge: ProjectedEdgeId) -> Vec<ProjectedFaceId> {
        edge::edge_faces(self, edge)
    }

    fn radial_valence(&self, edge: ProjectedEdgeId) -> usize {
        edge::radial_valence(self, edge)
    }

    fn is_boundary_edge(&self, edge: ProjectedEdgeId) -> bool {
        edge::is_boundary_edge(self, edge)
    }

    fn vertex_outgoing_half_edges(&self, vertex: ProjectedVertexId) -> Vec<ProjectedHalfEdgeId> {
        vertex::vertex_outgoing_half_edges(self, vertex)
    }

    fn vertex_faces(&self, vertex: ProjectedVertexId) -> Vec<ProjectedFaceId> {
        vertex::vertex_faces(self, vertex)
    }

    fn vertex_disk_components(
        &self,
        vertex: ProjectedVertexId,
    ) -> Result<Vec<Vec<ProjectedHalfEdgeId>>, ProjectedTopologyError> {
        vertex::vertex_disk_components(self, vertex)
    }
}
