use crate::data::comparator::{TierPolicyResolver, VersionComparatorPolicy};
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::data::tier::TierPolicy;
use crate::diagnostics::access::RuntimeDiagnostics;
use crate::diagnostics::facts::ProvenanceFact;
use crate::diagnostics::history::ExecutionInspector;
use crate::diagnostics::policy::SignalRuntimePolicy;
use crate::diagnostics::profile::DiagnosticsProfile;
use crate::diagnostics::summary::{ExecutionHistorySummary, GraphSummary};
use crate::diagnostics::{FailureSummary, FlowSummary, RollbackDiagnostic};
use crate::logic::explain::{explain_with_policy_resolver, NodeExplanation};
use crate::presentation::metrics::RuntimeMetrics;

use super::runtime_state::SignalRuntime;

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn explain(&self, node: NodeId) -> Result<NodeExplanation, SignalError> {
        let resolver = TierPolicyResolver::new(
            self.config.node_meta(),
            self.config.tier_policies(),
            self.config.fallback_comparator(),
        );
        explain_with_policy_resolver(&self.graph, node, &resolver)
    }

    pub fn retained_explanation_artifact(&self, node: NodeId) -> Option<NodeExplanation> {
        self.graph.retained_explanation_artifact(node)
    }

    pub fn reconstruct_explanation_artifact(
        &self,
        node: NodeId,
    ) -> Result<NodeExplanation, SignalError> {
        self.graph.reconstruct_explanation_artifact(node)
    }

    pub fn retained_provenance_artifact(&self, node: NodeId) -> Option<ProvenanceFact> {
        self.graph.retained_provenance_artifact(node)
    }

    pub fn reconstruct_provenance_artifact(
        &self,
        node: NodeId,
    ) -> Result<ProvenanceFact, SignalError> {
        self.graph.reconstruct_provenance_artifact(node)
    }

    pub fn metrics(&self) -> RuntimeMetrics {
        RuntimeMetrics {
            evaluation: self.telemetry.evaluation,
            invalidation: self.telemetry.invalidation,
            transaction: self.telemetry.transaction,
            planner: self.telemetry.planner,
            execution: self.telemetry.execution,
            storage: self.telemetry.storage,
            checkpoint: crate::data::telemetry::CheckpointTelemetry {
                event_flushes: self.event_bus.telemetry().checkpoint.event_flushes,
                event_flush_nanos: self.event_bus.telemetry().checkpoint.event_flush_nanos,
                checkpoint_flushes: self.checkpoint.telemetry().checkpoint.checkpoint_flushes,
                checkpoint_flush_nanos: self.checkpoint.telemetry().checkpoint.checkpoint_flush_nanos,
                rollback_count: self.event_bus.telemetry().checkpoint.rollback_count,
            },
        }
    }

    pub fn diagnostics_summary(&self, profile: DiagnosticsProfile) -> GraphSummary {
        self.graph.diagnostics_summary(profile)
    }

    pub fn diagnostics(&self) -> RuntimeDiagnostics<'_> {
        crate::diagnostics::access::diagnostics_for_runtime(self)
    }

    pub fn diagnostics_profile(&self) -> DiagnosticsProfile {
        self.graph.diagnostics_profile()
    }

    pub fn prepare_for_observation(&mut self) {
        self.graph.prepare_for_observation();
    }

    pub fn runtime_policy(&self) -> SignalRuntimePolicy {
        self.graph.runtime_policy()
    }

    pub fn set_diagnostics_profile(&mut self, profile: DiagnosticsProfile) {
        self.graph.set_diagnostics_profile(profile);
    }

    pub fn set_runtime_policy(&mut self, policy: SignalRuntimePolicy) {
        self.graph.set_runtime_policy(policy);
    }

    pub fn replay_for_node(&self, node: NodeId) -> crate::diagnostics::ReplaySlice {
        self.graph.replay_for_node(node)
    }

    pub fn replay_for_artifact(
        &self,
        artifact_id: crate::diagnostics::LineageArtifactId,
    ) -> crate::diagnostics::ReplaySlice {
        self.graph.replay_for_artifact(artifact_id)
    }

    pub fn replay_from_cursor(
        &self,
        start: crate::diagnostics::ReplayCursor,
    ) -> crate::diagnostics::ReplaySlice {
        self.graph.replay_from_cursor(start)
    }

    pub fn replay_between(
        &self,
        start: crate::diagnostics::ReplayCursor,
        end: crate::diagnostics::ReplayCursor,
    ) -> crate::diagnostics::ReplaySlice {
        self.graph.replay_between(start, end)
    }

    pub fn replay_around_snapshot(
        &self,
        snapshot_id: crate::state::SignalSnapshotId,
    ) -> crate::diagnostics::ReplaySlice {
        self.graph.replay_around_snapshot(snapshot_id)
    }

    pub fn current_lineage_artifact(
        &self,
        node: NodeId,
    ) -> Option<crate::diagnostics::LineageArtifactId> {
        self.graph.current_lineage_artifact(node)
    }

    pub fn lineage_chain_for_node(&self, node: NodeId) -> Vec<crate::diagnostics::LineageRecord> {
        self.graph.lineage_chain_for_node(node)
    }

    pub fn lineage_chain_for_artifact(
        &self,
        artifact_id: crate::diagnostics::LineageArtifactId,
    ) -> Vec<crate::diagnostics::LineageRecord> {
        self.graph.lineage_chain_for_artifact(artifact_id)
    }

    pub fn execution_history_summary(
        &self,
        profile: DiagnosticsProfile,
    ) -> ExecutionHistorySummary {
        self.graph.execution_history_summary(profile)
    }

    pub fn inspect_execution(&self) -> ExecutionInspector<'_> {
        self.graph.inspect_execution()
    }

    pub fn latest_flow_diagnostics(&self) -> Option<&FlowSummary> {
        self.graph.latest_flow_diagnostics()
    }

    pub fn latest_failure_diagnostics(&self) -> Option<&FailureSummary> {
        self.graph.latest_failure_diagnostics()
    }

    pub fn latest_rollback_diagnostics(&self) -> Option<&RollbackDiagnostic> {
        self.graph.latest_rollback_diagnostics()
    }

    pub fn recent_execution_history_diagnostics(
        &self,
    ) -> &std::collections::VecDeque<ExecutionHistorySummary> {
        self.graph.recent_execution_history_diagnostics()
    }

    pub fn to_dot(&self) -> String {
        self.graph.to_dot()
    }

    pub fn set_node_tier(&mut self, node: NodeId, tier: T) {
        self.config.set_node_tier(&self.graph, node, tier);
    }

    pub fn set_tier_policy(&mut self, policy: TierPolicy<T>) {
        self.config.set_tier_policy(policy);
    }

    pub fn set_fallback_comparator(&mut self, policy: VersionComparatorPolicy) {
        self.config.set_fallback_comparator(policy);
    }
}
