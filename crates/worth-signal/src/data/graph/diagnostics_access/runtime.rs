use crate::data::aspect::Aspect;
use crate::data::graph::signal_graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::proof::{FrontierExecutionSummary, InvalidationTraceRecord};
use crate::diagnostics::policy::OrdinaryAccessLane;
use crate::diagnostics::policy::SignalRuntimePolicy;
use crate::diagnostics::profile::DiagnosticsTier;
use crate::diagnostics::summary::GraphSummary;

impl SignalGraph {
    /// Execute any bounded maintenance work required before entering a pure
    /// observation phase.
    pub fn prepare_for_observation(&mut self) {
        self.run_gc_epoch();
    }

    pub(crate) fn telemetry(&self) -> &crate::data::telemetry::RuntimeTelemetry {
        &self.observation.telemetry
    }

    pub fn telemetry_mut(&mut self) -> &mut crate::data::telemetry::RuntimeTelemetry {
        &mut self.observation.telemetry
    }

    pub fn reset_telemetry(&mut self) {
        self.observation.telemetry = crate::data::telemetry::RuntimeTelemetry::default();
    }

    pub(crate) fn diagnostics_profile(&self) -> DiagnosticsTier {
        self.observation.diagnostics.tier()
    }

    pub(crate) fn runtime_policy(&self) -> SignalRuntimePolicy {
        self.observation.diagnostics.policy()
    }

    /// Reset graph diagnostics to the stock policy for one diagnostics tier.
    ///
    /// This is a lower-level convenience reset. If the caller means to keep
    /// custom retention or replay overrides, they should apply a full
    /// `SignalRuntimePolicy` instead.
    pub fn reset_runtime_policy_to_tier(&mut self, profile: DiagnosticsTier) {
        self.observation.diagnostics.set_profile(profile);
    }

    #[deprecated(
        note = "use reset_runtime_policy_to_tier(...) for stock preset resets, or set_runtime_policy(...) for full policy control"
    )]
    pub fn set_diagnostics_profile(&mut self, profile: DiagnosticsTier) {
        self.reset_runtime_policy_to_tier(profile);
    }

    /// Apply the full runtime policy bundle to the graph diagnostics state.
    pub fn set_runtime_policy(&mut self, policy: SignalRuntimePolicy) {
        self.observation.diagnostics.set_policy(policy);
    }

    #[cfg(any(test, doctest))]
    pub(crate) fn diagnostics_summary(&self, profile: DiagnosticsTier) -> GraphSummary {
        if self.diagnostics_state().has_pending_change_input() {
            if let Some(summary) = self.diagnostics_state().pending_graph_summary() {
                return summary.with_profile(profile);
            }
        } else if let Some(summary) = self.diagnostics_state().latest_graph_summary() {
            return summary.with_profile(profile);
        }
        GraphSummary::from_graph(
            self,
            profile,
            self.runtime_policy().retention_budget.detail_limit,
            OrdinaryAccessLane,
        )
    }

    pub(crate) fn diagnostics_state(&self) -> &crate::diagnostics::state::DiagnosticsState {
        &self.observation.diagnostics
    }

    pub(crate) fn diagnostics_state_mut(
        &mut self,
    ) -> &mut crate::diagnostics::state::DiagnosticsState {
        &mut self.observation.diagnostics
    }

    pub(crate) fn note_change_input(
        &mut self,
        node: NodeId,
        aspect: Aspect,
        changed_regions: &[crate::data::output::ChangedRegion],
    ) {
        let causality_kind = self
            .causality_of(node)
            .ok()
            .flatten()
            .map(|causality| causality.kind.clone());
        self.observation.diagnostics.note_change_input(
            node,
            aspect,
            changed_regions,
            causality_kind,
        );
    }

    pub(crate) fn clear_pending_diagnostics_input(&mut self) {
        self.observation.diagnostics.clear_pending_input();
    }

    pub(crate) fn record_frontier_execution_diagnostics(
        &mut self,
        summary: FrontierExecutionSummary,
        trace_records: Vec<InvalidationTraceRecord>,
    ) {
        if let Some(summary) = self.diagnostics_state().latest_graph_summary().cloned() {
            self.observation
                .diagnostics
                .set_pending_graph_summary(summary.with_profile(self.diagnostics_profile()));
        } else {
            let retention_budget = self.runtime_policy().retention_budget;
            self.observation
                .diagnostics
                .set_pending_graph_summary(GraphSummary::from_graph(
                    self,
                    self.diagnostics_profile(),
                    retention_budget.detail_limit,
                    OrdinaryAccessLane,
                ));
        }
        self.observation
            .diagnostics
            .record_frontier_execution(summary, trace_records);
    }
}
