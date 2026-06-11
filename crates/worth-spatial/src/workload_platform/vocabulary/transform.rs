use super::projection::ProjectionWorkloadReceipt;
use super::stage_contract::{
    certify_stage, SpatialWorkloadStage, WorkloadStageDenial, WorkloadStageEnvelope,
    WorkloadStageIdentity, WorkloadStageSupport,
};

pub struct TransformWorkload<'a> {
    upstream: &'a ProjectionWorkloadReceipt,
    declaration: String,
}

impl<'a> TransformWorkload<'a> {
    pub fn for_projection(upstream: &'a ProjectionWorkloadReceipt) -> Self {
        Self {
            upstream,
            declaration: "transform workload".to_string(),
        }
    }

    pub fn declared(mut self, declaration: impl Into<String>) -> Self {
        self.declaration = declaration.into();
        self
    }

    pub fn admit(self) -> Result<TransformWorkloadReceipt, WorkloadStageDenial> {
        let envelope = certify_stage(
            SpatialWorkloadStage::Transform,
            self.declaration,
            self.upstream.identity().receipt_identity(),
            WorkloadStageSupport::Admitted,
            "transform workload is admitted",
        )?;
        Ok(TransformWorkloadReceipt { envelope })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransformWorkloadReceipt {
    envelope: WorkloadStageEnvelope,
}

impl TransformWorkloadReceipt {
    pub fn identity(&self) -> &WorkloadStageIdentity {
        self.envelope.identity()
    }

    pub fn envelope(&self) -> &WorkloadStageEnvelope {
        &self.envelope
    }
}
