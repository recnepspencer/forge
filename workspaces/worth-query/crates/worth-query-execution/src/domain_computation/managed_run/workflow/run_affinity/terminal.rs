use std::sync::Arc;

use crate::domain_computation::{
    WorthQueryWorkflowExecutionAttemptReleaseReceipt, WorthQueryWorkflowExecutionResourceAttempt,
};

pub(in crate::domain_computation::managed_run::workflow) struct WorthQueryWorkflowRunTerminalAffinity
{
    logical: Arc<str>,
    attempt: WorthQueryWorkflowExecutionResourceAttempt,
}

impl WorthQueryWorkflowRunTerminalAffinity {
    pub(super) fn new(
        logical: Arc<str>,
        attempt: WorthQueryWorkflowExecutionResourceAttempt,
    ) -> Self {
        Self { logical, attempt }
    }

    pub(in crate::domain_computation::managed_run::workflow) fn logical_identity(&self) -> &str {
        &self.logical
    }

    pub(in crate::domain_computation::managed_run::workflow) fn attempt_identity(&self) -> &str {
        self.attempt.attempt_identity().as_str()
    }

    pub(in crate::domain_computation::managed_run::workflow) fn release(
        self,
    ) -> WorthQueryWorkflowExecutionAttemptReleaseReceipt {
        self.attempt.release()
    }
}
