use crate::diagnostics::failure::{ExecutionFailureContext, FailureSummary};
use crate::diagnostics::policy::SignalRuntimePolicy;

use super::DiagnosticsRecorder;

impl<'a> DiagnosticsRecorder<'a> {
    fn policy(&self) -> SignalRuntimePolicy {
        SignalRuntimePolicy::for_tier(self.graph.diagnostics_profile())
    }

    pub(crate) fn record_failure(&mut self, context: ExecutionFailureContext) -> FailureSummary {
        let policy = self.policy();
        let summary = context.summarize(
            self.graph.observe().latest_rollback_diagnostics(),
            policy.tier,
        );
        self.record_failure_summary(summary.clone());
        self.graph.clear_pending_diagnostics_input();
        summary
    }

    pub(crate) fn record_failure_summary(&mut self, summary: FailureSummary) {
        self.graph.diagnostics_state_mut().record_failure(summary);
    }
}
