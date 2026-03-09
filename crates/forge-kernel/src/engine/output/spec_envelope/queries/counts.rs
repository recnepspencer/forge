use forge_core::KernelError;
use forge_topo::projection::{
    ProjectedBodyId, ProjectedEdgeId, ProjectedFaceId, ProjectedHalfEdgeId, ProjectedLoopId,
    ProjectedLumpId, ProjectedRegionId, ProjectedShellId, ProjectedVertexId,
};

use super::super::{projected_ids, SpecEnvelope};

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
        Ok(self.face_count()?
            + self.half_edges()?.len()
            + self.vertex_count()?
            + self.loops()?.len())
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
}
