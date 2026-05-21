use crate::boundary::errors::ForgeSignalJsError;
use crate::runtime::summaries::{HealthSummary, WebPerformanceSummary};
use forge_signal::facade::adapters::{FrontierExecutionSummary, InvalidationTraceRecord};
use forge_signal::facade::diagnostics::{FailureSummary, RollbackDiagnostic};

use super::WorkerRuntimeShell;

impl WorkerRuntimeShell {
    pub fn health(&self) -> Result<HealthSummary, ForgeSignalJsError> {
        self.core.health()
    }

    pub fn performance_summary(&self) -> WebPerformanceSummary {
        self.core.web_performance_summary()
    }

    pub fn latest_failure(&self) -> Result<Option<FailureSummary>, ForgeSignalJsError> {
        self.core.latest_failure()
    }

    pub fn latest_rollback(&self) -> Result<Option<RollbackDiagnostic>, ForgeSignalJsError> {
        self.core.latest_rollback()
    }

    pub fn latest_frontier_execution(
        &self,
    ) -> Result<Option<FrontierExecutionSummary>, ForgeSignalJsError> {
        self.core.latest_frontier_execution()
    }

    pub fn latest_invalidation_trace_records(
        &self,
    ) -> Result<Vec<InvalidationTraceRecord>, ForgeSignalJsError> {
        self.core.latest_invalidation_trace_records()
    }
}
