use forge_core::KernelError;
use forge_topo::projection::{
    ProjectedFaceId, ProjectedLoopId, ProjectedLumpId, ProjectedRegionId, ProjectedShellId,
    ProjectedTopologyQueries, ProjectedBodyId,
};

use super::super::SpecEnvelope;

impl SpecEnvelope {
    pub fn body_lumps(&self, body: ProjectedBodyId) -> Result<Vec<ProjectedLumpId>, KernelError> {
        Ok(self.projection()?.body_lumps(body))
    }

    pub fn lump_body(&self, lump: ProjectedLumpId) -> Result<ProjectedBodyId, KernelError> {
        Ok(self.projection()?.lump_body(lump))
    }

    pub fn lump_regions(&self, lump: ProjectedLumpId) -> Result<Vec<ProjectedRegionId>, KernelError> {
        Ok(self.projection()?.lump_regions(lump))
    }

    pub fn region_lump(&self, region: ProjectedRegionId) -> Result<ProjectedLumpId, KernelError> {
        Ok(self.projection()?.region_lump(region))
    }

    pub fn region_shells(&self, region: ProjectedRegionId) -> Result<Vec<ProjectedShellId>, KernelError> {
        Ok(self.projection()?.region_shells(region))
    }

    pub fn shell_region(&self, shell: ProjectedShellId) -> Result<ProjectedRegionId, KernelError> {
        Ok(self.projection()?.shell_region(shell))
    }

    pub fn face_shell(&self, face: ProjectedFaceId) -> Result<ProjectedShellId, KernelError> {
        Ok(self.projection()?.face_shell(face))
    }

    pub fn face_outer_loop(&self, face: ProjectedFaceId) -> Result<ProjectedLoopId, KernelError> {
        Ok(self.projection()?.face_outer_loop(face))
    }

    pub fn face_inner_loops(&self, face: ProjectedFaceId) -> Result<Vec<ProjectedLoopId>, KernelError> {
        Ok(self.projection()?.face_inner_loops(face))
    }

    pub fn loop_face(&self, loop_id: ProjectedLoopId) -> Result<ProjectedFaceId, KernelError> {
        Ok(self.projection()?.loop_face(loop_id))
    }
}
