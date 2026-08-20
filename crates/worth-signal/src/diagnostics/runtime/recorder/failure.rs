use super::DiagnosticsRecorder;
use crate::diagnostics::failure::{ExecutionFailureContext, FailureSummary};

impl<'a> DiagnosticsRecorder<'a> {
    fn policy_tier(&self) -> crate::diagnostics::profile::DiagnosticsTier {
        self.graph.installed_runtime_policy().tier()
    }

    pub(crate) fn record_failure(&mut self, context: ExecutionFailureContext) -> FailureSummary {
        if !self.graph.captures_failure_diagnostics() {
            self.graph.clear_pending_diagnostics_input();
            return FailureSummary::suppressed(self.policy_tier(), context.phase);
        }
        let summary = context.summarize(
            self.graph.observe().latest_rollback_diagnostics(),
            self.policy_tier(),
        );
        self.record_failure_summary(summary.clone());
        self.graph.clear_pending_diagnostics_input();
        summary
    }

    pub(crate) fn record_failure_summary(&mut self, summary: FailureSummary) {
        if !self.graph.captures_failure_diagnostics() {
            return;
        }
        self.graph.diagnostics_state_mut().record_failure(summary);
    }
}
