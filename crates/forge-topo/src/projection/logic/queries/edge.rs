use crate::projection::data::{
    ProjectedEdgeId, ProjectedFaceId, ProjectedHalfEdgeId, ProjectedTopology,
};

pub fn radial_half_edges(
    topology: &ProjectedTopology,
    half_edge: ProjectedHalfEdgeId,
) -> Vec<ProjectedHalfEdgeId> {
    let mut result = Vec::new();
    let mut current = half_edge;
    let max_steps = topology.half_edge_count().max(1);

    for _ in 0..max_steps {
        result.push(current);
        let next = topology.half_edge(current).radial_next;
        if next == half_edge {
            break;
        }
        current = next;
    }

    result
}

pub fn edge_half_edges(topology: &ProjectedTopology, edge: ProjectedEdgeId) -> Vec<ProjectedHalfEdgeId> {
    let representative = topology.edge(edge).half_edge;
    radial_half_edges(topology, representative)
        .into_iter()
        .filter(|half_edge| topology.half_edge(*half_edge).edge == edge)
        .collect()
}

pub fn edge_faces(topology: &ProjectedTopology, edge: ProjectedEdgeId) -> Vec<ProjectedFaceId> {
    let mut faces = edge_half_edges(topology, edge)
        .into_iter()
        .map(|half_edge| topology.half_edge(half_edge).face)
        .collect::<Vec<_>>();
    faces.sort_unstable();
    faces.dedup();
    faces
}

pub fn radial_valence(topology: &ProjectedTopology, edge: ProjectedEdgeId) -> usize {
    edge_half_edges(topology, edge).len()
}

pub fn is_boundary_edge(topology: &ProjectedTopology, edge: ProjectedEdgeId) -> bool {
    radial_valence(topology, edge) == 1
}
