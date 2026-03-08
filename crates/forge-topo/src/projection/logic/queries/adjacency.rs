use crate::projection::data::{
    ProjectedEdgeId, ProjectedFaceId, ProjectedHalfEdgeId, ProjectedTopology, ProjectedVertexId,
};

pub fn half_edge_next(
    topology: &ProjectedTopology,
    half_edge: ProjectedHalfEdgeId,
) -> ProjectedHalfEdgeId {
    topology.half_edge(half_edge).next
}

pub fn half_edge_prev(
    topology: &ProjectedTopology,
    half_edge: ProjectedHalfEdgeId,
) -> ProjectedHalfEdgeId {
    topology.half_edge(half_edge).prev
}

pub fn half_edge_radial_next(
    topology: &ProjectedTopology,
    half_edge: ProjectedHalfEdgeId,
) -> ProjectedHalfEdgeId {
    topology.half_edge(half_edge).radial_next
}

pub fn half_edge_face(topology: &ProjectedTopology, half_edge: ProjectedHalfEdgeId) -> ProjectedFaceId {
    topology.half_edge(half_edge).face
}

pub fn half_edge_origin(
    topology: &ProjectedTopology,
    half_edge: ProjectedHalfEdgeId,
) -> ProjectedVertexId {
    topology.half_edge(half_edge).origin
}

pub fn half_edge_edge(topology: &ProjectedTopology, half_edge: ProjectedHalfEdgeId) -> ProjectedEdgeId {
    topology.half_edge(half_edge).edge
}

pub fn edge_representative_half_edge(
    topology: &ProjectedTopology,
    edge: ProjectedEdgeId,
) -> ProjectedHalfEdgeId {
    topology.edge(edge).half_edge
}

pub fn vertex_primary_half_edge(
    topology: &ProjectedTopology,
    vertex: ProjectedVertexId,
) -> Option<ProjectedHalfEdgeId> {
    topology.vertex(vertex).primary_half_edge
}
