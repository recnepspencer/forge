use crate::boundary::errors::WORTHSignalJsError;
use crate::runtime::summaries::{HealthSummary, WebPerformanceSummary};
use worth_signal::facade::adapters::{FrontierExecutionSummary, InvalidationTraceRecord};
use worth_signal::facade::diagnostics::{FailureSummary, RollbackDiagnostic};

use super::WorkerRuntimeShell;

impl WorkerRuntimeShell {
    pub fn health(&self) -> Result<HealthSummary, WORTHSignalJsError> {
        self.core.health()
    }

    pub fn performance_summary(&self) -> WebPerformanceSummary {
        self.core.web_performance_summary()
    }

    pub fn latest_failure(&self) -> Result<Option<FailureSummary>, WORTHSignalJsError> {
        self.core.latest_failure()
    }

    pub fn latest_rollback(&self) -> Result<Option<RollbackDiagnostic>, WORTHSignalJsError> {
        self.core.latest_rollback()
    }

    pub fn latest_frontier_execution(
        &self,
    ) -> Result<Option<FrontierExecutionSummary>, WORTHSignalJsError> {
        self.core.latest_frontier_execution()
    }

    pub fn latest_invalidation_trace_records(
        &self,
    ) -> Result<Vec<InvalidationTraceRecord>, WORTHSignalJsError> {
        self.core.latest_invalidation_trace_records()
    }
}
