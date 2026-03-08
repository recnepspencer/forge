use forge_spec::facade::SpecNodeId;

use crate::projection::data::{
    ProjectedEdgeId, ProjectedFaceId, ProjectedHalfEdgeId, ProjectedTopology, ProjectedVertexId,
};

pub fn face_surface_binding(
    topology: &ProjectedTopology,
    face: ProjectedFaceId,
) -> Option<SpecNodeId> {
    topology.face(face).surface_binding
}

pub fn half_edge_coedge_binding(
    topology: &ProjectedTopology,
    half_edge: ProjectedHalfEdgeId,
) -> Option<SpecNodeId> {
    topology.half_edge(half_edge).coedge_binding
}

pub fn edge_curve_binding(topology: &ProjectedTopology, edge: ProjectedEdgeId) -> Option<SpecNodeId> {
    topology.edge(edge).curve_binding
}

pub fn vertex_geometry_binding(
    topology: &ProjectedTopology,
    vertex: ProjectedVertexId,
) -> Option<SpecNodeId> {
    topology.vertex(vertex).geometry_binding
}
