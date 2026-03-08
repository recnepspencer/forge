use forge_core::KernelError;
use forge_spec::facade::{SpecNodeId, SpecShellKind};
use forge_topo::projection::{
    ProjectedBodyId, ProjectedEdgeId, ProjectedEntityRef, ProjectedFaceId, ProjectedHalfEdgeId,
    ProjectedLoopId, ProjectedLumpId, ProjectedRegionId, ProjectedShellId, ProjectedTopologyQueries,
    ProjectedVertexId,
};

use super::super::SpecEnvelope;

impl SpecEnvelope {
    pub fn shell_kind(&self, shell: ProjectedShellId) -> Result<SpecShellKind, KernelError> {
        Ok(self.projection()?.shell_kind(shell))
    }

    pub fn body_spec_id(&self, body: ProjectedBodyId) -> Result<SpecNodeId, KernelError> {
        Ok(self.projection()?.body_spec_id(body))
    }

    pub fn lump_spec_id(&self, lump: ProjectedLumpId) -> Result<SpecNodeId, KernelError> {
        Ok(self.projection()?.lump_spec_id(lump))
    }

    pub fn region_spec_id(&self, region: ProjectedRegionId) -> Result<SpecNodeId, KernelError> {
        Ok(self.projection()?.region_spec_id(region))
    }

    pub fn shell_spec_id(&self, shell: ProjectedShellId) -> Result<SpecNodeId, KernelError> {
        Ok(self.projection()?.shell_spec_id(shell))
    }

    pub fn face_spec_id(&self, face: ProjectedFaceId) -> Result<SpecNodeId, KernelError> {
        Ok(self.projection()?.face_spec_id(face))
    }

    pub fn loop_spec_id(&self, loop_id: ProjectedLoopId) -> Result<SpecNodeId, KernelError> {
        Ok(self.projection()?.loop_spec_id(loop_id))
    }

    pub fn half_edge_spec_id(
        &self,
        half_edge: ProjectedHalfEdgeId,
    ) -> Result<SpecNodeId, KernelError> {
        Ok(self.projection()?.half_edge_spec_id(half_edge))
    }

    pub fn edge_spec_id(&self, edge: ProjectedEdgeId) -> Result<SpecNodeId, KernelError> {
        Ok(self.projection()?.edge_spec_id(edge))
    }

    pub fn vertex_spec_id(&self, vertex: ProjectedVertexId) -> Result<SpecNodeId, KernelError> {
        Ok(self.projection()?.vertex_spec_id(vertex))
    }

    pub fn resolve(&self, spec_id: SpecNodeId) -> Result<Option<ProjectedEntityRef>, KernelError> {
        Ok(self.projection()?.resolve(spec_id))
    }
}
