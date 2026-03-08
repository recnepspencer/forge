use forge_spec::facade::{SpecNodeId, SpecShellKind};

use crate::projection::data::{
    ProjectedBodyId, ProjectedEdgeId, ProjectedEntityRef, ProjectedFaceId, ProjectedHalfEdgeId,
    ProjectedLoopId, ProjectedLumpId, ProjectedRegionId, ProjectedShellId, ProjectedTopology,
    ProjectedVertexId,
};

pub fn shell_kind(topology: &ProjectedTopology, shell: ProjectedShellId) -> SpecShellKind {
    topology.shell(shell).kind
}

pub fn body_spec_id(topology: &ProjectedTopology, body: ProjectedBodyId) -> SpecNodeId {
    topology.body(body).spec_id
}

pub fn lump_spec_id(topology: &ProjectedTopology, lump: ProjectedLumpId) -> SpecNodeId {
    topology.lump(lump).spec_id
}

pub fn region_spec_id(topology: &ProjectedTopology, region: ProjectedRegionId) -> SpecNodeId {
    topology.region(region).spec_id
}

pub fn shell_spec_id(topology: &ProjectedTopology, shell: ProjectedShellId) -> SpecNodeId {
    topology.shell(shell).spec_id
}

pub fn face_spec_id(topology: &ProjectedTopology, face: ProjectedFaceId) -> SpecNodeId {
    topology.face(face).spec_id
}

pub fn loop_spec_id(topology: &ProjectedTopology, loop_id: ProjectedLoopId) -> SpecNodeId {
    topology.loop_data(loop_id).spec_id
}

pub fn half_edge_spec_id(
    topology: &ProjectedTopology,
    half_edge: ProjectedHalfEdgeId,
) -> SpecNodeId {
    topology.half_edge(half_edge).spec_id
}

pub fn edge_spec_id(topology: &ProjectedTopology, edge: ProjectedEdgeId) -> SpecNodeId {
    topology.edge(edge).spec_id
}

pub fn vertex_spec_id(topology: &ProjectedTopology, vertex: ProjectedVertexId) -> SpecNodeId {
    topology.vertex(vertex).spec_id
}

pub fn resolve(topology: &ProjectedTopology, spec_id: SpecNodeId) -> Option<ProjectedEntityRef> {
    topology.resolve(spec_id)
}
