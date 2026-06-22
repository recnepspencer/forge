use super::stage_contract::{
    certify_stage, SpatialWorkloadStage, WorkloadStageDenial, WorkloadStageEnvelope,
    WorkloadStageIdentity, WorkloadStageSupport,
};
use super::transform::TransformWorkloadReceipt;

pub struct RetainedReplayWorkload<'a> {
    upstream: &'a TransformWorkloadReceipt,
    declaration: String,
}

impl<'a> RetainedReplayWorkload<'a> {
    pub fn for_transform(upstream: &'a TransformWorkloadReceipt) -> Self {
        Self {
            upstream,
            declaration: "retained replay workload".to_string(),
        }
    }

    pub fn declared(mut self, declaration: impl Into<String>) -> Self {
        self.declaration = declaration.into();
        self
    }

    pub fn admit(self) -> Result<RetainedReplayWorkloadReceipt, WorkloadStageDenial> {
        let envelope = certify_stage(
            SpatialWorkloadStage::RetainedReplay,
            self.declaration,
            self.upstream.identity().receipt_identity(),
            WorkloadStageSupport::Admitted,
            "retained replay workload is admitted",
        )?;
        Ok(RetainedReplayWorkloadReceipt { envelope })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RetainedReplayWorkloadReceipt {
    envelope: WorkloadStageEnvelope,
}

impl RetainedReplayWorkloadReceipt {
    pub fn identity(&self) -> &WorkloadStageIdentity {
        self.envelope.identity()
    }

    pub fn envelope(&self) -> &WorkloadStageEnvelope {
        &self.envelope
    }
}
