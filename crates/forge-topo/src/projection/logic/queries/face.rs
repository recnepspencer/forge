use crate::projection::data::{
    ProjectedEdgeId, ProjectedFaceId, ProjectedHalfEdgeId, ProjectedLoopId, ProjectedTopology,
    ProjectedTopologyError,
};

pub fn shell_faces(
    topology: &ProjectedTopology,
    shell: crate::projection::data::ProjectedShellId,
) -> Vec<ProjectedFaceId> {
    topology.shell(shell).faces.clone()
}

pub fn face_loops(topology: &ProjectedTopology, face: ProjectedFaceId) -> Vec<ProjectedLoopId> {
    let face_data = topology.face(face);
    let mut loops = Vec::with_capacity(1 + face_data.inner_loops.len());
    loops.push(face_data.outer_loop);
    loops.extend(face_data.inner_loops.iter().copied());
    loops
}

pub fn loop_half_edges(
    topology: &ProjectedTopology,
    loop_id: ProjectedLoopId,
) -> Result<Vec<ProjectedHalfEdgeId>, ProjectedTopologyError> {
    let loop_data = topology.loop_data(loop_id);
    let start = loop_data.half_edge;
    let mut result = Vec::new();
    let mut current = start;
    let max_steps = topology.half_edge_count().max(1);

    for _ in 0..max_steps {
        result.push(current);
        let next = topology.half_edge(current).next;
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

pub fn face_half_edges(
    topology: &ProjectedTopology,
    face: ProjectedFaceId,
) -> Result<Vec<ProjectedHalfEdgeId>, ProjectedTopologyError> {
    let mut half_edges = Vec::new();
    for loop_id in face_loops(topology, face) {
        half_edges.extend(loop_half_edges(topology, loop_id)?);
    }
    Ok(half_edges)
}

pub fn face_edges(
    topology: &ProjectedTopology,
    face: ProjectedFaceId,
) -> Result<Vec<ProjectedEdgeId>, ProjectedTopologyError> {
    let mut edges = face_half_edges(topology, face)?
        .into_iter()
        .map(|half_edge| topology.half_edge(half_edge).edge)
        .collect::<Vec<_>>();
    edges.sort_unstable();
    edges.dedup();
    Ok(edges)
}
