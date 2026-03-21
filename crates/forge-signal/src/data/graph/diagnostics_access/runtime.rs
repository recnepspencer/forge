use crate::data::aspect::Aspect;
use crate::data::graph::signal_graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::proof::{FrontierExecutionSummary, InvalidationTraceRecord};
use crate::diagnostics::policy::SignalRuntimePolicy;
use crate::diagnostics::profile::DiagnosticsProfile;
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

    pub(crate) fn diagnostics_profile(&self) -> DiagnosticsProfile {
        self.observation.diagnostics.profile()
    }

    pub(crate) fn runtime_policy(&self) -> SignalRuntimePolicy {
        self.observation.diagnostics.policy()
    }

    pub fn set_diagnostics_profile(&mut self, profile: DiagnosticsProfile) {
        self.observation.diagnostics.set_profile(profile);
    }

    pub fn set_runtime_policy(&mut self, policy: SignalRuntimePolicy) {
        self.observation.diagnostics.set_policy(policy);
    }

    pub(crate) fn diagnostics_summary(&self, profile: DiagnosticsProfile) -> GraphSummary {
        GraphSummary::from_graph(self, profile)
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
            .get_entry(node)
            .ok()
            .and_then(|entry| entry.get_causality())
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
        self.observation
            .diagnostics
            .record_frontier_execution(summary, trace_records);
    }
}
