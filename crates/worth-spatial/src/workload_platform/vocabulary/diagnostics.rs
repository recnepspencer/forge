use super::retained_replay::RetainedReplayWorkloadReceipt;
use super::stage_contract::{
    certify_stage, SpatialWorkloadStage, WorkloadStageDenial, WorkloadStageEnvelope,
    WorkloadStageIdentity, WorkloadStageSupport,
};

pub struct DiagnosticWorkload<'a> {
    upstream: &'a RetainedReplayWorkloadReceipt,
    declaration: String,
}

impl<'a> DiagnosticWorkload<'a> {
    pub fn for_retained_replay(upstream: &'a RetainedReplayWorkloadReceipt) -> Self {
        Self {
            upstream,
            declaration: "diagnostic workload".to_string(),
        }
    }

    pub fn declared(mut self, declaration: impl Into<String>) -> Self {
        self.declaration = declaration.into();
        self
    }

    pub fn admit(self) -> Result<DiagnosticWorkloadReceipt, WorkloadStageDenial> {
        let envelope = certify_stage(
            SpatialWorkloadStage::Diagnostics,
            self.declaration,
            self.upstream.identity().declaration().to_string(),
            WorkloadStageSupport::Admitted,
            "diagnostic workload is admitted",
        )?;
        Ok(DiagnosticWorkloadReceipt { envelope })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiagnosticWorkloadReceipt {
    envelope: WorkloadStageEnvelope,
}

impl DiagnosticWorkloadReceipt {
    pub fn identity(&self) -> &WorkloadStageIdentity {
        self.envelope.identity()
    }

    pub fn envelope(&self) -> &WorkloadStageEnvelope {
        &self.envelope
    }
}
