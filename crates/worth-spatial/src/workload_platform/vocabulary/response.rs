use super::diagnostics::DiagnosticWorkloadReceipt;
use super::stage_contract::{
    certify_stage, SpatialWorkloadStage, WorkloadStageDenial, WorkloadStageEnvelope,
    WorkloadStageIdentity, WorkloadStageSupport,
};

pub struct ResponseWorkload<'a> {
    upstream: &'a DiagnosticWorkloadReceipt,
    declaration: String,
}

impl<'a> ResponseWorkload<'a> {
    pub fn for_diagnostics(upstream: &'a DiagnosticWorkloadReceipt) -> Self {
        Self {
            upstream,
            declaration: "response workload".to_string(),
        }
    }

    pub fn declared(mut self, declaration: impl Into<String>) -> Self {
        self.declaration = declaration.into();
        self
    }

    pub fn admit(self) -> Result<ResponseWorkloadReceipt, WorkloadStageDenial> {
        let envelope = certify_stage(
            SpatialWorkloadStage::Response,
            self.declaration,
            self.upstream.identity().declaration().to_string(),
            WorkloadStageSupport::Admitted,
            "response workload is admitted",
        )?;
        Ok(ResponseWorkloadReceipt { envelope })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResponseWorkloadReceipt {
    envelope: WorkloadStageEnvelope,
}

impl ResponseWorkloadReceipt {
    pub(crate) fn from_envelope(envelope: WorkloadStageEnvelope) -> Self {
        Self { envelope }
    }

    pub fn identity(&self) -> &WorkloadStageIdentity {
        self.envelope.identity()
    }

    pub fn envelope(&self) -> &WorkloadStageEnvelope {
        &self.envelope
    }
}
