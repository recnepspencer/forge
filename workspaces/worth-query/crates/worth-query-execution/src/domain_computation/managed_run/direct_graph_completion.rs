use super::WorthQueryRunningDirectRun;
use crate::domain_computation::WorthQueryBoundGraphExecutionReceipt;

pub struct WorthQueryCompletedDirectGraphExecution {
    running: WorthQueryRunningDirectRun,
    receipt: WorthQueryBoundGraphExecutionReceipt,
}

impl WorthQueryCompletedDirectGraphExecution {
    pub(super) fn new(
        running: WorthQueryRunningDirectRun,
        receipt: WorthQueryBoundGraphExecutionReceipt,
    ) -> Self {
        Self { running, receipt }
    }

    pub fn run_identity(&self) -> &str {
        self.running.identity()
    }

    pub fn receipt(&self) -> &WorthQueryBoundGraphExecutionReceipt {
        &self.receipt
    }

    pub fn into_running(self) -> WorthQueryRunningDirectRun {
        self.running
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        WorthQueryRunningDirectRun,
        WorthQueryBoundGraphExecutionReceipt,
    ) {
        (self.running, self.receipt)
    }

    pub(crate) fn bind_convergence_candidate_evidence(
        &self,
        output_occurrence_identity: &str,
    ) -> Result<
        crate::domain_computation::WorthQueryDomainEvidenceExecutionBinding,
        crate::domain_computation::WorthQueryDomainEvidenceBindingDenial,
    > {
        self.running
            .bind_convergence_candidate_evidence(output_occurrence_identity)
    }
}
