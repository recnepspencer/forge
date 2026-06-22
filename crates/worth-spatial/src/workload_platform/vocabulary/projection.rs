use super::stage_contract::{
    certify_stage, SpatialWorkloadStage, WorkloadStageDenial, WorkloadStageEnvelope,
    WorkloadStageIdentity, WorkloadStageSupport,
};
use super::surface_support::SurfaceSupportWorkloadReceipt;

pub struct ProjectionWorkload<'a> {
    upstream: &'a SurfaceSupportWorkloadReceipt,
    declaration: String,
}

impl<'a> ProjectionWorkload<'a> {
    pub fn for_surface_support(upstream: &'a SurfaceSupportWorkloadReceipt) -> Self {
        Self {
            upstream,
            declaration: "projection workload".to_string(),
        }
    }

    pub fn declared(mut self, declaration: impl Into<String>) -> Self {
        self.declaration = declaration.into();
        self
    }

    pub fn admit(self) -> Result<ProjectionWorkloadReceipt, WorkloadStageDenial> {
        let envelope = certify_stage(
            SpatialWorkloadStage::Projection,
            self.declaration,
            self.upstream.identity().receipt_identity(),
            WorkloadStageSupport::Admitted,
            "projection workload is admitted",
        )?;
        Ok(ProjectionWorkloadReceipt { envelope })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionWorkloadReceipt {
    envelope: WorkloadStageEnvelope,
}

impl ProjectionWorkloadReceipt {
    pub fn identity(&self) -> &WorkloadStageIdentity {
        self.envelope.identity()
    }

    pub fn envelope(&self) -> &WorkloadStageEnvelope {
        &self.envelope
    }
}
