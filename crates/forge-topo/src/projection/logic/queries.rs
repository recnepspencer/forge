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
}

impl ProjectedTopologyQueries for ProjectedTopology {
    fn shell_faces(&self, shell: crate::projection::data::ProjectedShellId) -> Vec<ProjectedFaceId> {
        self.shell(shell).faces.clone()
    }

    fn face_loops(&self, face: ProjectedFaceId) -> Vec<ProjectedLoopId> {
        let face_data = self.face(face);
        let mut loops = Vec::with_capacity(1 + face_data.inner_loops.len());
        loops.push(face_data.outer_loop);
        loops.extend(face_data.inner_loops.iter().copied());
        loops
    }

    fn loop_half_edges(
        &self,
        loop_id: ProjectedLoopId,
    ) -> Result<Vec<ProjectedHalfEdgeId>, ProjectedTopologyError> {
        let loop_data = self.loop_data(loop_id);
        let start = loop_data.half_edge;
        let mut result = Vec::new();
        let mut current = start;
        let max_steps = self.half_edge_count().max(1);

        for _ in 0..max_steps {
            result.push(current);
            let next = self.half_edge(current).next;
            if next == start {
                return Ok(result);
            }
            current = next;
        }

        Err(ProjectedTopologyError::new(format!(
            "loop {} does not close within {} halfedges",
            loop_id.raw(),
            max_steps
        )))
    }

    fn face_half_edges(
        &self,
        face: ProjectedFaceId,
    ) -> Result<Vec<ProjectedHalfEdgeId>, ProjectedTopologyError> {
        let mut half_edges = Vec::new();
        for loop_id in self.face_loops(face) {
            half_edges.extend(self.loop_half_edges(loop_id)?);
        }
        Ok(half_edges)
    }

    fn face_edges(&self, face: ProjectedFaceId) -> Result<Vec<ProjectedEdgeId>, ProjectedTopologyError> {
        let mut edges = self
            .face_half_edges(face)?
            .into_iter()
            .map(|half_edge| self.half_edge(half_edge).edge)
            .collect::<Vec<_>>();
        edges.sort_unstable();
        edges.dedup();
        Ok(edges)
    }

    fn radial_half_edges(&self, half_edge: ProjectedHalfEdgeId) -> Vec<ProjectedHalfEdgeId> {
        let mut result = Vec::new();
        let mut current = half_edge;
        let max_steps = self.half_edge_count().max(1);

        for _ in 0..max_steps {
            result.push(current);
            let next = self.half_edge(current).radial_next;
            if next == half_edge {
                break;
            }
            current = next;
        }

        result
    }

    fn edge_half_edges(&self, edge: ProjectedEdgeId) -> Vec<ProjectedHalfEdgeId> {
        let representative = self.edge(edge).half_edge;
        self.radial_half_edges(representative)
            .into_iter()
            .filter(|half_edge| self.half_edge(*half_edge).edge == edge)
            .collect()
    }

    fn edge_faces(&self, edge: ProjectedEdgeId) -> Vec<ProjectedFaceId> {
        let mut faces = self
            .edge_half_edges(edge)
            .into_iter()
            .map(|half_edge| self.half_edge(half_edge).face)
            .collect::<Vec<_>>();
        faces.sort_unstable();
        faces.dedup();
        faces
    }

    fn radial_valence(&self, edge: ProjectedEdgeId) -> usize {
        self.edge_half_edges(edge).len()
    }

    fn is_boundary_edge(&self, edge: ProjectedEdgeId) -> bool {
        self.radial_valence(edge) == 1
    }

    fn vertex_outgoing_half_edges(&self, vertex: ProjectedVertexId) -> Vec<ProjectedHalfEdgeId> {
        let mut result = Vec::new();
        for (index, half_edge) in self.half_edges().iter().enumerate() {
            if half_edge.origin == vertex {
                result.push(ProjectedHalfEdgeId::new(index as u32));
            }
        }
        result
    }

    fn vertex_faces(&self, vertex: ProjectedVertexId) -> Vec<ProjectedFaceId> {
        let mut faces = self
            .vertex_outgoing_half_edges(vertex)
            .into_iter()
            .map(|half_edge| self.half_edge(half_edge).face)
            .collect::<Vec<_>>();
        faces.sort_unstable();
        faces.dedup();
        faces
    }
}
