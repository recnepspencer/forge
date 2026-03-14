use crate::data::comparator::TierPolicyResolver;
use crate::data::error::SignalError;
use crate::data::graph::{EvaluationStrategy, GraphObserver};
use crate::data::handle::NodeId;
use crate::diagnostics::access::RuntimeDiagnostics;
use crate::diagnostics::facts::ProvenanceFact;
use crate::diagnostics::history::ExecutionInspector;
use crate::diagnostics::lineage::{LineageArtifactId, LineageRecord};
use crate::diagnostics::policy::SignalRuntimePolicy;
use crate::diagnostics::profile::DiagnosticsProfile;
use crate::diagnostics::summary::{ExecutionHistorySummary, GraphSummary};
use crate::diagnostics::{FailureSummary, FlowSummary, ReplaySlice, RollbackDiagnostic};
use crate::logic::explain::{explain_with_policy_resolver, NodeExplanation};
use crate::presentation::metrics::RuntimeMetrics;
use crate::state::{SignalBranchHandle, SignalBranchId, SignalSnapshotId};

use super::runtime_state::SignalRuntime;
use super::CheckpointRecord;

pub struct RuntimeObserver<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    runtime: &'a SignalRuntime<D, I, E, Ctx, T>,
}

impl<'a, D, I, E, Ctx, T> RuntimeObserver<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(crate) fn new(runtime: &'a SignalRuntime<D, I, E, Ctx, T>) -> Self {
        Self { runtime }
    }

    pub fn graph(&self) -> GraphObserver<'a> {
        self.runtime.graph.observe()
    }

    pub fn explain(&self, node: NodeId) -> Result<NodeExplanation, SignalError> {
        let resolver = TierPolicyResolver::new(
            self.runtime.config.node_meta(),
            self.runtime.config.tier_policies(),
            self.runtime.config.fallback_comparator(),
        );
        explain_with_policy_resolver(&self.runtime.graph, node, &resolver)
    }

    pub fn retained_explanation_artifact(&self, node: NodeId) -> Option<NodeExplanation> {
        self.graph().retained_explanation_artifact(node)
    }

    pub fn reconstruct_explanation_artifact(
        &self,
        node: NodeId,
    ) -> Result<NodeExplanation, SignalError> {
        self.graph().reconstruct_explanation_artifact(node)
    }

    pub fn retained_provenance_artifact(&self, node: NodeId) -> Option<ProvenanceFact> {
        self.graph().retained_provenance_artifact(node)
    }

    pub fn reconstruct_provenance_artifact(
        &self,
        node: NodeId,
    ) -> Result<ProvenanceFact, SignalError> {
        self.graph().reconstruct_provenance_artifact(node)
    }

    pub fn metrics(&self) -> RuntimeMetrics {
        RuntimeMetrics {
            evaluation: self.runtime.telemetry.evaluation,
            invalidation: self.runtime.telemetry.invalidation,
            transaction: self.runtime.telemetry.transaction,
            planner: self.runtime.telemetry.planner,
            execution: self.runtime.telemetry.execution,
            storage: self.runtime.telemetry.storage,
            checkpoint: self.composed_checkpoint_telemetry(),
        }
    }

    pub fn checkpoint_record(&self) -> CheckpointRecord {
        CheckpointRecord::from_checkpoint_telemetry(self.composed_checkpoint_telemetry())
    }

    pub fn diagnostics_summary(&self, profile: DiagnosticsProfile) -> GraphSummary {
        self.graph().diagnostics_summary(profile)
    }

    pub fn diagnostics(&self) -> RuntimeDiagnostics<'a> {
        crate::diagnostics::access::diagnostics_for_runtime(self.runtime)
    }

    pub fn diagnostics_profile(&self) -> DiagnosticsProfile {
        self.graph().diagnostics_profile()
    }

    fn composed_checkpoint_telemetry(&self) -> crate::data::telemetry::CheckpointTelemetry {
        crate::data::telemetry::CheckpointTelemetry {
            event_flushes: self.runtime.event_bus.telemetry().checkpoint.event_flushes,
            event_flush_nanos: self
                .runtime
                .event_bus
                .telemetry()
                .checkpoint
                .event_flush_nanos,
            checkpoint_flushes: self
                .runtime
                .checkpoint
                .telemetry()
                .checkpoint
                .checkpoint_flushes,
            checkpoint_flush_nanos: self
                .runtime
                .checkpoint
                .telemetry()
                .checkpoint
                .checkpoint_flush_nanos,
            rollback_count: self.runtime.event_bus.telemetry().checkpoint.rollback_count,
            checkpoint_size: self.runtime.telemetry.checkpoint.checkpoint_size,
            journal_replay_span: self.runtime.telemetry.checkpoint.journal_replay_span,
        }
    }

    pub fn runtime_policy(&self) -> SignalRuntimePolicy {
        self.graph().runtime_policy()
    }

    pub fn evaluation_strategy(&self) -> EvaluationStrategy {
        self.graph().evaluation_strategy()
    }

    pub fn replay_for_node(&self, node: NodeId) -> ReplaySlice {
        self.graph().replay_for_node(node)
    }

    pub fn replay_for_artifact(&self, artifact_id: LineageArtifactId) -> ReplaySlice {
        self.graph().replay_for_artifact(artifact_id)
    }

    pub fn replay_from_cursor(&self, start: crate::diagnostics::ReplayCursor) -> ReplaySlice {
        self.graph().replay_from_cursor(start)
    }

    pub fn replay_between(
        &self,
        start: crate::diagnostics::ReplayCursor,
        end: crate::diagnostics::ReplayCursor,
    ) -> ReplaySlice {
        self.graph().replay_between(start, end)
    }

    pub fn replay_around_snapshot(&self, snapshot_id: SignalSnapshotId) -> ReplaySlice {
        self.graph().replay_around_snapshot(snapshot_id)
    }

    pub fn replay_for_branch(&self, branch_id: SignalBranchId) -> ReplaySlice {
        self.runtime
            .branches
            .replay_graph(
                branch_id,
                self.runtime.graph.current_branch().id,
                &self.runtime.graph,
            )
            .map(|graph| graph.observe().replay_for_branch(branch_id))
            .unwrap_or_default()
    }

    pub fn current_lineage_artifact(&self, node: NodeId) -> Option<LineageArtifactId> {
        self.graph().current_lineage_artifact(node)
    }

    pub fn lineage_chain_for_node(&self, node: NodeId) -> Vec<LineageRecord> {
        self.graph().lineage_chain_for_node(node)
    }

    pub fn lineage_chain_for_artifact(&self, artifact_id: LineageArtifactId) -> Vec<LineageRecord> {
        self.graph().lineage_chain_for_artifact(artifact_id)
    }

    pub fn execution_history_summary(
        &self,
        profile: DiagnosticsProfile,
    ) -> ExecutionHistorySummary {
        self.graph().execution_history_summary(profile)
    }

    pub fn inspect_execution(&self) -> ExecutionInspector<'a> {
        self.graph().inspect_execution()
    }

    pub fn latest_flow_diagnostics(&self) -> Option<&'a FlowSummary> {
        self.graph().latest_flow_diagnostics()
    }

    pub fn latest_failure_diagnostics(&self) -> Option<&'a FailureSummary> {
        self.graph().latest_failure_diagnostics()
    }

    pub fn latest_rollback_diagnostics(&self) -> Option<&'a RollbackDiagnostic> {
        self.graph().latest_rollback_diagnostics()
    }

    pub fn recent_execution_history_diagnostics(
        &self,
    ) -> &'a std::collections::VecDeque<ExecutionHistorySummary> {
        self.graph().recent_execution_history_diagnostics()
    }

    pub fn to_dot(&self) -> String {
        self.graph().to_dot()
    }

    pub fn current_branch(&self) -> SignalBranchHandle {
        self.graph().current_branch()
    }

    pub fn known_branches(&self) -> Vec<SignalBranchHandle> {
        self.graph().known_branches()
    }

    pub fn branch_handle(&self, branch_id: SignalBranchId) -> Option<SignalBranchHandle> {
        self.graph()
            .branch_handle(branch_id)
            .or_else(|| self.runtime.branches.branch_handle(branch_id))
    }

    pub fn branch_ancestry(&self, branch_id: SignalBranchId) -> Vec<SignalBranchHandle> {
        if self.graph().branch_handle(branch_id).is_some() {
            self.graph().branch_ancestry(branch_id)
        } else {
            self.runtime.branches.branch_ancestry(branch_id)
        }
    }

    pub fn branch_head_snapshot_id(&self, branch_id: SignalBranchId) -> Option<SignalSnapshotId> {
        self.graph()
            .branch_head_snapshot_id(branch_id)
            .or_else(|| self.runtime.branches.branch_head_snapshot_id(branch_id))
    }
}
