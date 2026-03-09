mod adjacency;
mod bindings;
mod edge;
mod face;
mod hierarchy;
mod identity;
mod vertex;

use crate::projection::data::{
    ProjectedBodyId, ProjectedEdgeId, ProjectedEntityRef, ProjectedFaceId, ProjectedHalfEdgeId,
    ProjectedLoopId, ProjectedLumpId, ProjectedRegionId, ProjectedShellId, ProjectedTopology,
    ProjectedTopologyError, ProjectedVertexId,
};
use forge_spec::facade::{SpecNodeId, SpecShellKind};

pub trait ProjectedTopologyQueries {
    fn shell_kind(&self, shell: ProjectedShellId) -> SpecShellKind;
    fn body_spec_id(&self, body: ProjectedBodyId) -> SpecNodeId;
    fn lump_spec_id(&self, lump: ProjectedLumpId) -> SpecNodeId;
    fn region_spec_id(&self, region: ProjectedRegionId) -> SpecNodeId;
    fn shell_spec_id(&self, shell: ProjectedShellId) -> SpecNodeId;
    fn face_spec_id(&self, face: ProjectedFaceId) -> SpecNodeId;
    fn loop_spec_id(&self, loop_id: ProjectedLoopId) -> SpecNodeId;
    fn half_edge_spec_id(&self, half_edge: ProjectedHalfEdgeId) -> SpecNodeId;
    fn edge_spec_id(&self, edge: ProjectedEdgeId) -> SpecNodeId;
    fn vertex_spec_id(&self, vertex: ProjectedVertexId) -> SpecNodeId;
    fn resolve(&self, spec_id: SpecNodeId) -> Option<ProjectedEntityRef>;
    fn body_lumps(&self, body: ProjectedBodyId) -> Vec<ProjectedLumpId>;
    fn lump_body(&self, lump: ProjectedLumpId) -> ProjectedBodyId;
    fn lump_regions(&self, lump: ProjectedLumpId) -> Vec<ProjectedRegionId>;
    fn region_lump(&self, region: ProjectedRegionId) -> ProjectedLumpId;
    fn region_shells(&self, region: ProjectedRegionId) -> Vec<ProjectedShellId>;
    fn shell_region(&self, shell: ProjectedShellId) -> ProjectedRegionId;
    fn face_shell(&self, face: ProjectedFaceId) -> ProjectedShellId;
    fn face_outer_loop(&self, face: ProjectedFaceId) -> ProjectedLoopId;
    fn face_inner_loops(&self, face: ProjectedFaceId) -> Vec<ProjectedLoopId>;
    fn loop_face(&self, loop_id: ProjectedLoopId) -> ProjectedFaceId;
    fn half_edge_next(&self, half_edge: ProjectedHalfEdgeId) -> ProjectedHalfEdgeId;
    fn half_edge_prev(&self, half_edge: ProjectedHalfEdgeId) -> ProjectedHalfEdgeId;
    fn half_edge_radial_next(&self, half_edge: ProjectedHalfEdgeId) -> ProjectedHalfEdgeId;
    fn half_edge_face(&self, half_edge: ProjectedHalfEdgeId) -> ProjectedFaceId;
    fn half_edge_origin(&self, half_edge: ProjectedHalfEdgeId) -> ProjectedVertexId;
    fn half_edge_edge(&self, half_edge: ProjectedHalfEdgeId) -> ProjectedEdgeId;
    fn edge_representative_half_edge(&self, edge: ProjectedEdgeId) -> ProjectedHalfEdgeId;
    fn vertex_primary_half_edge(&self, vertex: ProjectedVertexId) -> Option<ProjectedHalfEdgeId>;
    fn face_surface_binding(&self, face: ProjectedFaceId) -> Option<SpecNodeId>;
    fn half_edge_coedge_binding(&self, half_edge: ProjectedHalfEdgeId) -> Option<SpecNodeId>;
    fn edge_curve_binding(&self, edge: ProjectedEdgeId) -> Option<SpecNodeId>;
    fn vertex_geometry_binding(&self, vertex: ProjectedVertexId) -> Option<SpecNodeId>;
    fn shell_faces(&self, shell: crate::projection::data::ProjectedShellId)
        -> Vec<ProjectedFaceId>;
    fn face_loops(&self, face: ProjectedFaceId) -> Vec<ProjectedLoopId>;
    fn loop_half_edges(
        &self,
        loop_id: ProjectedLoopId,
    ) -> Result<Vec<ProjectedHalfEdgeId>, ProjectedTopologyError>;
    fn face_half_edges(
        &self,
        face: ProjectedFaceId,
    ) -> Result<Vec<ProjectedHalfEdgeId>, ProjectedTopologyError>;
    fn face_edges(
        &self,
        face: ProjectedFaceId,
    ) -> Result<Vec<ProjectedEdgeId>, ProjectedTopologyError>;
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
    fn shell_kind(&self, shell: ProjectedShellId) -> SpecShellKind {
        identity::shell_kind(self, shell)
    }

    fn body_spec_id(&self, body: ProjectedBodyId) -> SpecNodeId {
        identity::body_spec_id(self, body)
    }

    fn lump_spec_id(&self, lump: ProjectedLumpId) -> SpecNodeId {
        identity::lump_spec_id(self, lump)
    }

    fn region_spec_id(&self, region: ProjectedRegionId) -> SpecNodeId {
        identity::region_spec_id(self, region)
    }

    fn shell_spec_id(&self, shell: ProjectedShellId) -> SpecNodeId {
        identity::shell_spec_id(self, shell)
    }

    fn face_spec_id(&self, face: ProjectedFaceId) -> SpecNodeId {
        identity::face_spec_id(self, face)
    }

    fn loop_spec_id(&self, loop_id: ProjectedLoopId) -> SpecNodeId {
        identity::loop_spec_id(self, loop_id)
    }

    fn half_edge_spec_id(&self, half_edge: ProjectedHalfEdgeId) -> SpecNodeId {
        identity::half_edge_spec_id(self, half_edge)
    }

    fn edge_spec_id(&self, edge: ProjectedEdgeId) -> SpecNodeId {
        identity::edge_spec_id(self, edge)
    }

    fn vertex_spec_id(&self, vertex: ProjectedVertexId) -> SpecNodeId {
        identity::vertex_spec_id(self, vertex)
    }

    fn resolve(&self, spec_id: SpecNodeId) -> Option<ProjectedEntityRef> {
        identity::resolve(self, spec_id)
    }

    fn body_lumps(&self, body: ProjectedBodyId) -> Vec<ProjectedLumpId> {
        hierarchy::body_lumps(self, body)
    }

    fn lump_body(&self, lump: ProjectedLumpId) -> ProjectedBodyId {
        hierarchy::lump_body(self, lump)
    }

    fn lump_regions(&self, lump: ProjectedLumpId) -> Vec<ProjectedRegionId> {
        hierarchy::lump_regions(self, lump)
    }

    fn region_lump(&self, region: ProjectedRegionId) -> ProjectedLumpId {
        hierarchy::region_lump(self, region)
    }

    fn region_shells(&self, region: ProjectedRegionId) -> Vec<ProjectedShellId> {
        hierarchy::region_shells(self, region)
    }

    fn shell_region(&self, shell: ProjectedShellId) -> ProjectedRegionId {
        hierarchy::shell_region(self, shell)
    }

    fn face_shell(&self, face: ProjectedFaceId) -> ProjectedShellId {
        hierarchy::face_shell(self, face)
    }

    fn face_outer_loop(&self, face: ProjectedFaceId) -> ProjectedLoopId {
        hierarchy::face_outer_loop(self, face)
    }

    fn face_inner_loops(&self, face: ProjectedFaceId) -> Vec<ProjectedLoopId> {
        hierarchy::face_inner_loops(self, face)
    }

    fn loop_face(&self, loop_id: ProjectedLoopId) -> ProjectedFaceId {
        hierarchy::loop_face(self, loop_id)
    }

    fn half_edge_next(&self, half_edge: ProjectedHalfEdgeId) -> ProjectedHalfEdgeId {
        adjacency::half_edge_next(self, half_edge)
    }

    fn half_edge_prev(&self, half_edge: ProjectedHalfEdgeId) -> ProjectedHalfEdgeId {
        adjacency::half_edge_prev(self, half_edge)
    }

    fn half_edge_radial_next(&self, half_edge: ProjectedHalfEdgeId) -> ProjectedHalfEdgeId {
        adjacency::half_edge_radial_next(self, half_edge)
    }

    fn half_edge_face(&self, half_edge: ProjectedHalfEdgeId) -> ProjectedFaceId {
        adjacency::half_edge_face(self, half_edge)
    }

    fn half_edge_origin(&self, half_edge: ProjectedHalfEdgeId) -> ProjectedVertexId {
        adjacency::half_edge_origin(self, half_edge)
    }

    fn half_edge_edge(&self, half_edge: ProjectedHalfEdgeId) -> ProjectedEdgeId {
        adjacency::half_edge_edge(self, half_edge)
    }

    fn edge_representative_half_edge(&self, edge: ProjectedEdgeId) -> ProjectedHalfEdgeId {
        adjacency::edge_representative_half_edge(self, edge)
    }

    fn vertex_primary_half_edge(&self, vertex: ProjectedVertexId) -> Option<ProjectedHalfEdgeId> {
        adjacency::vertex_primary_half_edge(self, vertex)
    }

    fn face_surface_binding(&self, face: ProjectedFaceId) -> Option<SpecNodeId> {
        bindings::face_surface_binding(self, face)
    }

    fn half_edge_coedge_binding(&self, half_edge: ProjectedHalfEdgeId) -> Option<SpecNodeId> {
        bindings::half_edge_coedge_binding(self, half_edge)
    }

    fn edge_curve_binding(&self, edge: ProjectedEdgeId) -> Option<SpecNodeId> {
        bindings::edge_curve_binding(self, edge)
    }

    fn vertex_geometry_binding(&self, vertex: ProjectedVertexId) -> Option<SpecNodeId> {
        bindings::vertex_geometry_binding(self, vertex)
    }

    fn shell_faces(
        &self,
        shell: crate::projection::data::ProjectedShellId,
    ) -> Vec<ProjectedFaceId> {
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
