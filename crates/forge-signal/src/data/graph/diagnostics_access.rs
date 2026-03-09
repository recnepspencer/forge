use crate::data::aspect::Aspect;
use crate::data::dependency::DependencyEdge;
use crate::data::error::SignalError;
use crate::data::graph::signal_graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::diagnostics::access::GraphDiagnostics;
use crate::diagnostics::facts::{ExplanationFact, ProvenanceFact};
use crate::diagnostics::history::ExecutionInspector;
use crate::diagnostics::policy::{ArtifactMaterializationMode, SignalRuntimePolicy};
use crate::diagnostics::profile::DiagnosticsProfile;
use crate::diagnostics::replay::ReplayEvent;
use crate::diagnostics::summary::{ExecutionHistorySummary, GraphSummary};
use crate::diagnostics::{FailureSummary, FlowSummary, RollbackDiagnostic};
use crate::logic::explain::{dependency_chain_to, explain, NodeExplanation};
use crate::presentation::dot::to_dot;
use crate::presentation::metrics::GraphMetrics;

impl SignalGraph {
    pub fn telemetry(&self) -> &crate::data::telemetry::RuntimeTelemetry {
        &self.telemetry
    }

    pub fn telemetry_mut(&mut self) -> &mut crate::data::telemetry::RuntimeTelemetry {
        &mut self.telemetry
    }

    pub fn reset_telemetry(&mut self) {
        self.telemetry = crate::data::telemetry::RuntimeTelemetry::default();
    }

    pub fn explain(&self, node: NodeId) -> Result<NodeExplanation, SignalError> {
        explain(self, node)
    }

    pub fn dependencies_of(&self, node: NodeId) -> Result<&[DependencyEdge], SignalError> {
        let entry = self.get_entry(node)?;
        Ok(self.dependency_edges.get(entry.get_dependencies_id()))
    }

    pub fn subscribers_of(&self, node: NodeId) -> Result<&[NodeId], SignalError> {
        let entry = self.get_entry(node)?;
        Ok(self.subscriber_edges.get(entry.get_subscribers_id()))
    }

    pub fn depends_on(
        &self,
        node: NodeId,
        upstream: NodeId,
        aspect: Aspect,
    ) -> Result<bool, SignalError> {
        Ok(self
            .dependencies_of(node)?
            .iter()
            .any(|dependency| dependency.source() == upstream && dependency.aspect() == aspect))
    }

    pub fn dependency_chain_to(
        &self,
        root: NodeId,
        target: NodeId,
    ) -> Result<Option<Vec<NodeId>>, SignalError> {
        dependency_chain_to(self, root, target)
    }

    pub fn metrics(&self) -> GraphMetrics {
        GraphMetrics::from_runtime_telemetry(
            self.telemetry(),
            self.partition_interner.partition_count(),
        )
    }

    pub fn diagnostics_profile(&self) -> DiagnosticsProfile {
        self.diagnostics.profile()
    }

    pub fn runtime_policy(&self) -> SignalRuntimePolicy {
        self.diagnostics.policy()
    }

    pub fn set_diagnostics_profile(&mut self, profile: DiagnosticsProfile) {
        self.diagnostics.set_profile(profile);
    }

    pub fn set_runtime_policy(&mut self, policy: SignalRuntimePolicy) {
        self.diagnostics.set_policy(policy);
    }

    pub fn diagnostics_summary(&self, profile: DiagnosticsProfile) -> GraphSummary {
        GraphSummary::from_graph(self, profile)
    }

    pub fn diagnostics(&self) -> GraphDiagnostics<'_> {
        GraphDiagnostics::new(self)
    }

    pub fn execution_history_summary(
        &self,
        profile: DiagnosticsProfile,
    ) -> ExecutionHistorySummary {
        ExecutionHistorySummary::from_graph(self, profile)
    }

    pub fn inspect_execution(&self) -> ExecutionInspector<'_> {
        ExecutionInspector { graph: self }
    }

    pub fn latest_flow_diagnostics(&self) -> Option<&FlowSummary> {
        self.diagnostics.latest_flow()
    }

    pub fn latest_failure_diagnostics(&self) -> Option<&FailureSummary> {
        self.diagnostics.latest_failure()
    }

    pub fn latest_rollback_diagnostics(&self) -> Option<&RollbackDiagnostic> {
        self.diagnostics.latest_rollback()
    }

    pub fn recent_execution_history_diagnostics(
        &self,
    ) -> &std::collections::VecDeque<ExecutionHistorySummary> {
        self.diagnostics.recent_history()
    }

    pub fn replay_events(&self) -> &std::collections::VecDeque<ReplayEvent> {
        self.diagnostics.replay_events()
    }

    pub fn explanation_fact(&self, node: NodeId) -> Option<&ExplanationFact> {
        self.diagnostics.explanation_facts().get(&node)
    }

    pub fn provenance_fact(&self, node: NodeId) -> Option<&ProvenanceFact> {
        self.diagnostics.provenance_facts().get(&node)
    }

    pub fn retained_explanation_artifact(&self, node: NodeId) -> Option<NodeExplanation> {
        self.explanation_fact(node)
            .map(|fact| fact.explanation.clone())
    }

    pub fn reconstruct_explanation_artifact(
        &self,
        node: NodeId,
    ) -> Result<NodeExplanation, SignalError> {
        explain(self, node)
    }

    pub fn retained_provenance_artifact(&self, node: NodeId) -> Option<ProvenanceFact> {
        self.provenance_fact(node).cloned()
    }

    pub fn reconstruct_provenance_artifact(
        &self,
        node: NodeId,
    ) -> Result<ProvenanceFact, SignalError> {
        Ok(ProvenanceFact::from_explanation(&explain(self, node)?))
    }

    pub fn explain_artifact(
        &self,
        node: NodeId,
    ) -> Result<(Option<NodeExplanation>, ArtifactMaterializationMode), SignalError> {
        if let Some(fact) = self.explanation_fact(node) {
            return Ok((
                Some(fact.explanation.clone()),
                ArtifactMaterializationMode::Retained,
            ));
        }
        if self.runtime_policy().can_reconstruct_explanation() {
            return Ok((
                Some(self.reconstruct_explanation_artifact(node)?),
                ArtifactMaterializationMode::Reconstructed,
            ));
        }
        Ok((None, ArtifactMaterializationMode::Unavailable))
    }

    pub fn provenance_artifact(
        &self,
        node: NodeId,
    ) -> Result<(Option<ProvenanceFact>, ArtifactMaterializationMode), SignalError> {
        if let Some(fact) = self.provenance_fact(node) {
            return Ok((Some(fact.clone()), ArtifactMaterializationMode::Retained));
        }
        if self.runtime_policy().can_reconstruct_provenance() {
            return Ok((
                Some(self.reconstruct_provenance_artifact(node)?),
                ArtifactMaterializationMode::Reconstructed,
            ));
        }
        Ok((None, ArtifactMaterializationMode::Unavailable))
    }

    pub fn to_dot(&self) -> String {
        to_dot(self)
    }

    pub(crate) fn diagnostics_state(&self) -> &crate::diagnostics::state::DiagnosticsState {
        &self.diagnostics
    }

    pub(crate) fn diagnostics_state_mut(
        &mut self,
    ) -> &mut crate::diagnostics::state::DiagnosticsState {
        &mut self.diagnostics
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
        self.diagnostics
            .note_change_input(node, aspect, changed_regions, causality_kind);
    }

    pub(crate) fn record_invalidation_diagnostics(
        &mut self,
        invalidated_direct_subscribers: u32,
        maybe_stale_direct_subscribers: u32,
        partition_scoped_checks: u32,
    ) {
        self.diagnostics.record_invalidation_result(
            invalidated_direct_subscribers,
            maybe_stale_direct_subscribers,
            partition_scoped_checks,
        );
    }

    pub(crate) fn clear_pending_diagnostics_input(&mut self) {
        self.diagnostics.clear_pending_input();
    }
}
