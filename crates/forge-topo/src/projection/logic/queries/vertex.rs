use std::collections::{BTreeSet, VecDeque};

use crate::projection::data::{
    ProjectedFaceId, ProjectedHalfEdgeId, ProjectedTopology, ProjectedTopologyError,
    ProjectedVertexId,
};

use super::edge::radial_half_edges;

pub fn vertex_outgoing_half_edges(
    topology: &ProjectedTopology,
    vertex: ProjectedVertexId,
) -> Vec<ProjectedHalfEdgeId> {
    let mut result = Vec::new();
    for (index, half_edge) in topology.half_edges().iter().enumerate() {
        if half_edge.origin == vertex {
            result.push(ProjectedHalfEdgeId::new(index as u32));
        }
    }
    result
}

pub fn vertex_faces(
    topology: &ProjectedTopology,
    vertex: ProjectedVertexId,
) -> Vec<ProjectedFaceId> {
    let mut faces = vertex_outgoing_half_edges(topology, vertex)
        .into_iter()
        .map(|half_edge| topology.half_edge(half_edge).face)
        .collect::<Vec<_>>();
    faces.sort_unstable();
    faces.dedup();
    faces
}

pub fn vertex_disk_components(
    topology: &ProjectedTopology,
    vertex: ProjectedVertexId,
) -> Result<Vec<Vec<ProjectedHalfEdgeId>>, ProjectedTopologyError> {
    let outgoing = vertex_outgoing_half_edges(topology, vertex);
    let outgoing_set = outgoing
        .iter()
        .copied()
        .map(|he| he.raw())
        .collect::<BTreeSet<_>>();
    let mut visited = BTreeSet::new();
    let mut components = Vec::new();

    for seed in outgoing {
        if visited.contains(&seed.raw()) {
            continue;
        }
        let component = collect_vertex_disk_component(topology, vertex, seed, &outgoing_set)?;
        for half_edge in &component {
            visited.insert(half_edge.raw());
        }
        components.push(component);
    }

    Ok(components)
}

fn collect_vertex_disk_component(
    topology: &ProjectedTopology,
    vertex: ProjectedVertexId,
    seed: ProjectedHalfEdgeId,
    outgoing_set: &BTreeSet<u32>,
) -> Result<Vec<ProjectedHalfEdgeId>, ProjectedTopologyError> {
    if topology.half_edge(seed).origin != vertex {
        return Err(ProjectedTopologyError::new(format!(
            "seed halfedge {} does not originate at vertex {}",
            seed.raw(),
            vertex.raw()
        )));
    }

    let mut queued = BTreeSet::new();
    let mut queue = VecDeque::new();
    let mut component = Vec::new();
    queued.insert(seed.raw());
    queue.push_back(seed);

    while let Some(current) = queue.pop_front() {
        component.push(current);
        enqueue_vertex_disk_neighbors(
            topology,
            vertex,
            current,
            outgoing_set,
            &mut queued,
            &mut queue,
        );
        let incoming = topology.half_edge(current).prev;
        enqueue_vertex_disk_neighbors(
            topology,
            vertex,
            incoming,
            outgoing_set,
            &mut queued,
            &mut queue,
        );
    }

    component.sort_unstable();
    Ok(component)
}

fn enqueue_vertex_disk_neighbors(
    topology: &ProjectedTopology,
    vertex: ProjectedVertexId,
    seed: ProjectedHalfEdgeId,
    outgoing_set: &BTreeSet<u32>,
    queued: &mut BTreeSet<u32>,
    queue: &mut VecDeque<ProjectedHalfEdgeId>,
) {
    for radial in radial_half_edges(topology, seed) {
        enqueue_outgoing(topology, vertex, radial, outgoing_set, queued, queue);
        enqueue_outgoing(
            topology,
            vertex,
            topology.half_edge(radial).next,
            outgoing_set,
            queued,
            queue,
        );
    }
}

fn enqueue_outgoing(
    topology: &ProjectedTopology,
    vertex: ProjectedVertexId,
    candidate: ProjectedHalfEdgeId,
    outgoing_set: &BTreeSet<u32>,
    queued: &mut BTreeSet<u32>,
    queue: &mut VecDeque<ProjectedHalfEdgeId>,
) {
    if topology.half_edge(candidate).origin == vertex
        && outgoing_set.contains(&candidate.raw())
        && queued.insert(candidate.raw())
    {
        queue.push_back(candidate);
    }
}
