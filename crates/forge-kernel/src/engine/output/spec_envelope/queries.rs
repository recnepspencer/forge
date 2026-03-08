use forge_core::KernelError;
use forge_spec::facade::{SpecNodeId, SpecShellKind};
use forge_topo::projection::{
    ProjectedBodyId, ProjectedEdgeId, ProjectedEntityRef, ProjectedFaceId, ProjectedHalfEdgeId,
    ProjectedLoopId, ProjectedLumpId, ProjectedRegionId, ProjectedShellId, ProjectedTopologyQueries,
    ProjectedVertexId,
};

use super::{projected_ids, projected_topology_error_to_kernel_owned, SpecEnvelope};

impl SpecEnvelope {
    pub fn bodies(&self) -> Result<&[ProjectedBodyId], KernelError> {
        projected_ids(
            &self.bodies,
            self.projection(),
            |projection| projection.body_count(),
            ProjectedBodyId::new,
        )
    }

    pub fn lumps(&self) -> Result<&[ProjectedLumpId], KernelError> {
        projected_ids(
            &self.lumps,
            self.projection(),
            |projection| projection.lump_count(),
            ProjectedLumpId::new,
        )
    }

    pub fn regions(&self) -> Result<&[ProjectedRegionId], KernelError> {
        projected_ids(
            &self.regions,
            self.projection(),
            |projection| projection.region_count(),
            ProjectedRegionId::new,
        )
    }

    pub fn shells(&self) -> Result<&[ProjectedShellId], KernelError> {
        projected_ids(
            &self.shells,
            self.projection(),
            |projection| projection.shell_count(),
            ProjectedShellId::new,
        )
    }

    pub fn faces(&self) -> Result<&[ProjectedFaceId], KernelError> {
        projected_ids(
            &self.faces,
            self.projection(),
            |projection| projection.face_count(),
            ProjectedFaceId::new,
        )
    }

    pub fn loops(&self) -> Result<&[ProjectedLoopId], KernelError> {
        projected_ids(
            &self.loops,
            self.projection(),
            |projection| projection.loop_count(),
            ProjectedLoopId::new,
        )
    }

    pub fn half_edges(&self) -> Result<&[ProjectedHalfEdgeId], KernelError> {
        projected_ids(
            &self.half_edges,
            self.projection(),
            |projection| projection.half_edge_count(),
            ProjectedHalfEdgeId::new,
        )
    }

    pub fn edges(&self) -> Result<&[ProjectedEdgeId], KernelError> {
        projected_ids(
            &self.edges,
            self.projection(),
            |projection| projection.edge_count(),
            ProjectedEdgeId::new,
        )
    }

    pub fn vertices(&self) -> Result<&[ProjectedVertexId], KernelError> {
        projected_ids(
            &self.vertices,
            self.projection(),
            |projection| projection.vertex_count(),
            ProjectedVertexId::new,
        )
    }

    pub fn body_count(&self) -> Result<usize, KernelError> {
        Ok(self.bodies()?.len())
    }

    pub fn shell_count(&self) -> Result<usize, KernelError> {
        Ok(self.shells()?.len())
    }

    pub fn face_count(&self) -> Result<usize, KernelError> {
        Ok(self.faces()?.len())
    }

    pub fn vertex_count(&self) -> Result<usize, KernelError> {
        Ok(self.vertices()?.len())
    }

    pub fn edge_count(&self) -> Result<usize, KernelError> {
        Ok(self.edges()?.len())
    }

    pub fn entity_count(&self) -> Result<usize, KernelError> {
        Ok(self.face_count()? + self.half_edges()?.len() + self.vertex_count()? + self.loops()?.len())
    }

    pub fn body(&self) -> Result<ProjectedBodyId, KernelError> {
        let bodies = self.bodies()?;
        if bodies.len() != 1 {
            return Err(KernelError::InvalidInput {
                message: format!(
                    "SpecEnvelope::body() requires exactly 1 body, found {}",
                    bodies.len()
                ),
                context: None,
            });
        }
        Ok(bodies[0])
    }

    pub fn shell(&self) -> Result<ProjectedShellId, KernelError> {
        let shells = self.shells()?;
        if shells.len() != 1 {
            return Err(KernelError::InvalidInput {
                message: format!(
                    "SpecEnvelope::shell() requires exactly 1 shell, found {}",
                    shells.len()
                ),
                context: None,
            });
        }
        Ok(shells[0])
    }

    pub fn shell_kind(&self, shell: ProjectedShellId) -> Result<SpecShellKind, KernelError> {
        Ok(self.projection()?.shell(shell).kind)
    }

    pub fn body_spec_id(&self, body: ProjectedBodyId) -> Result<SpecNodeId, KernelError> {
        Ok(self.projection()?.body(body).spec_id)
    }

    pub fn lump_spec_id(&self, lump: ProjectedLumpId) -> Result<SpecNodeId, KernelError> {
        Ok(self.projection()?.lump(lump).spec_id)
    }

    pub fn region_spec_id(&self, region: ProjectedRegionId) -> Result<SpecNodeId, KernelError> {
        Ok(self.projection()?.region(region).spec_id)
    }

    pub fn shell_spec_id(&self, shell: ProjectedShellId) -> Result<SpecNodeId, KernelError> {
        Ok(self.projection()?.shell(shell).spec_id)
    }

    pub fn face_spec_id(&self, face: ProjectedFaceId) -> Result<SpecNodeId, KernelError> {
        Ok(self.projection()?.face(face).spec_id)
    }

    pub fn loop_spec_id(&self, loop_id: ProjectedLoopId) -> Result<SpecNodeId, KernelError> {
        Ok(self.projection()?.loop_data(loop_id).spec_id)
    }

    pub fn half_edge_spec_id(
        &self,
        half_edge: ProjectedHalfEdgeId,
    ) -> Result<SpecNodeId, KernelError> {
        Ok(self.projection()?.half_edge(half_edge).spec_id)
    }

    pub fn edge_spec_id(&self, edge: ProjectedEdgeId) -> Result<SpecNodeId, KernelError> {
        Ok(self.projection()?.edge(edge).spec_id)
    }

    pub fn vertex_spec_id(&self, vertex: ProjectedVertexId) -> Result<SpecNodeId, KernelError> {
        Ok(self.projection()?.vertex(vertex).spec_id)
    }

    pub fn resolve(&self, spec_id: SpecNodeId) -> Result<Option<ProjectedEntityRef>, KernelError> {
        Ok(self.projection()?.resolve(spec_id))
    }

    pub fn body_lumps(&self, body: ProjectedBodyId) -> Result<Vec<ProjectedLumpId>, KernelError> {
        Ok(self.projection()?.body(body).lumps.clone())
    }

    pub fn lump_body(&self, lump: ProjectedLumpId) -> Result<ProjectedBodyId, KernelError> {
        Ok(self.projection()?.lump(lump).body)
    }

    pub fn lump_regions(&self, lump: ProjectedLumpId) -> Result<Vec<ProjectedRegionId>, KernelError> {
        Ok(self.projection()?.lump(lump).regions.clone())
    }

    pub fn region_lump(&self, region: ProjectedRegionId) -> Result<ProjectedLumpId, KernelError> {
        Ok(self.projection()?.region(region).lump)
    }

    pub fn region_shells(&self, region: ProjectedRegionId) -> Result<Vec<ProjectedShellId>, KernelError> {
        Ok(self.projection()?.region(region).shells.clone())
    }

    pub fn shell_region(&self, shell: ProjectedShellId) -> Result<ProjectedRegionId, KernelError> {
        Ok(self.projection()?.shell(shell).region)
    }

    pub fn face_shell(&self, face: ProjectedFaceId) -> Result<ProjectedShellId, KernelError> {
        Ok(self.projection()?.face(face).shell)
    }

    pub fn face_outer_loop(&self, face: ProjectedFaceId) -> Result<ProjectedLoopId, KernelError> {
        Ok(self.projection()?.face(face).outer_loop)
    }

    pub fn face_inner_loops(&self, face: ProjectedFaceId) -> Result<Vec<ProjectedLoopId>, KernelError> {
        Ok(self.projection()?.face(face).inner_loops.clone())
    }

    pub fn face_surface_binding(
        &self,
        face: ProjectedFaceId,
    ) -> Result<Option<SpecNodeId>, KernelError> {
        Ok(self.projection()?.face(face).surface_binding)
    }

    pub fn loop_face(&self, loop_id: ProjectedLoopId) -> Result<ProjectedFaceId, KernelError> {
        Ok(self.projection()?.loop_data(loop_id).face)
    }

    pub fn half_edge_next(
        &self,
        half_edge: ProjectedHalfEdgeId,
    ) -> Result<ProjectedHalfEdgeId, KernelError> {
        Ok(self.projection()?.half_edge(half_edge).next)
    }

    pub fn half_edge_prev(
        &self,
        half_edge: ProjectedHalfEdgeId,
    ) -> Result<ProjectedHalfEdgeId, KernelError> {
        Ok(self.projection()?.half_edge(half_edge).prev)
    }

    pub fn half_edge_radial_next(
        &self,
        half_edge: ProjectedHalfEdgeId,
    ) -> Result<ProjectedHalfEdgeId, KernelError> {
        Ok(self.projection()?.half_edge(half_edge).radial_next)
    }

    pub fn half_edge_face(
        &self,
        half_edge: ProjectedHalfEdgeId,
    ) -> Result<ProjectedFaceId, KernelError> {
        Ok(self.projection()?.half_edge(half_edge).face)
    }

    pub fn half_edge_origin(
        &self,
        half_edge: ProjectedHalfEdgeId,
    ) -> Result<ProjectedVertexId, KernelError> {
        Ok(self.projection()?.half_edge(half_edge).origin)
    }

    pub fn half_edge_edge(
        &self,
        half_edge: ProjectedHalfEdgeId,
    ) -> Result<ProjectedEdgeId, KernelError> {
        Ok(self.projection()?.half_edge(half_edge).edge)
    }

    pub fn half_edge_coedge_binding(
        &self,
        half_edge: ProjectedHalfEdgeId,
    ) -> Result<Option<SpecNodeId>, KernelError> {
        Ok(self.projection()?.half_edge(half_edge).coedge_binding)
    }

    pub fn edge_representative_half_edge(
        &self,
        edge: ProjectedEdgeId,
    ) -> Result<ProjectedHalfEdgeId, KernelError> {
        Ok(self.projection()?.edge(edge).half_edge)
    }

    pub fn edge_curve_binding(&self, edge: ProjectedEdgeId) -> Result<Option<SpecNodeId>, KernelError> {
        Ok(self.projection()?.edge(edge).curve_binding)
    }

    pub fn vertex_primary_half_edge(
        &self,
        vertex: ProjectedVertexId,
    ) -> Result<Option<ProjectedHalfEdgeId>, KernelError> {
        Ok(self.projection()?.vertex(vertex).primary_half_edge)
    }

    pub fn vertex_geometry_binding(
        &self,
        vertex: ProjectedVertexId,
    ) -> Result<Option<SpecNodeId>, KernelError> {
        Ok(self.projection()?.vertex(vertex).geometry_binding)
    }

    pub fn face_loops(&self, face: ProjectedFaceId) -> Result<Vec<ProjectedLoopId>, KernelError> {
        Ok(self.projection()?.face_loops(face))
    }

    pub fn shell_faces(&self, shell: ProjectedShellId) -> Result<Vec<ProjectedFaceId>, KernelError> {
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
