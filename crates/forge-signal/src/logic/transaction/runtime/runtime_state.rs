use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};

use crate::data::checkpoint_policy::CheckpointPolicy;
use crate::data::comparator::{TierPolicyResolver, VersionComparatorPolicy};
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::output::{ComputationFamily, ComputationKey};
use crate::data::telemetry::RuntimeTelemetry;
use crate::data::tier::TierPolicy;
use crate::diagnostics::access::RuntimeDiagnostics;
use crate::diagnostics::facts::ProvenanceFact;
use crate::diagnostics::history::ExecutionInspector;
use crate::diagnostics::policy::SignalRuntimePolicy;
use crate::diagnostics::profile::DiagnosticsProfile;
use crate::diagnostics::summary::{ExecutionHistorySummary, GraphSummary};
use crate::diagnostics::{FailureSummary, FlowSummary, RollbackDiagnostic};
use crate::logic::checkpoint::CheckpointRuntime;
use crate::logic::events::EventBus;
use crate::logic::explain::{explain_with_policy_resolver, NodeExplanation};
use crate::logic::transaction::patch_buffer::SparsePatchBuffer;
use crate::presentation::metrics::RuntimeMetrics;
use crate::state::{SignalBranchHandle, SignalBranchId, SignalSnapshotV1};

use super::builder::SignalRuntimeBuilder;
use super::config::SignalRuntimeConfig;
use super::transaction_types::SignalTransaction;

#[derive(Debug, Clone)]
struct RuntimeBranchState<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    graph: SignalGraph,
    config: SignalRuntimeConfig<T>,
    checkpoint: CheckpointRuntime<D, I>,
    telemetry: RuntimeTelemetry,
}

/// Full runtime surface for transactional evaluation, diagnostics, replay, and
/// keyed or tier-aware execution.
pub struct SignalRuntime<D, I, E, Ctx, T = ()>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(super) config: SignalRuntimeConfig<T>,
    pub(super) graph: SignalGraph,
    pub(super) checkpoint: CheckpointRuntime<D, I>,
    pub(super) event_bus: EventBus<E, D, Ctx>,
    pub(super) telemetry: RuntimeTelemetry,
    branches: BTreeMap<SignalBranchId, RuntimeBranchState<D, I, T>>,
}

pub struct SignalGraphMut<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    runtime: &'a mut SignalRuntime<D, I, E, Ctx, T>,
}

impl<D, I, E, Ctx, T> Deref for SignalGraphMut<'_, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    type Target = SignalGraph;

    fn deref(&self) -> &Self::Target {
        &self.runtime.graph
    }
}

impl<D, I, E, Ctx, T> DerefMut for SignalGraphMut<'_, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.runtime.graph
    }
}

impl<D, I, E, Ctx, T> Drop for SignalGraphMut<'_, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    fn drop(&mut self) {
        self.runtime
            .config
            .prune_stale_node_meta(&self.runtime.graph);
    }
}

impl SignalRuntime<(), (), (), (), ()> {
    /// Create a runtime builder from a graph.
    ///
    /// This is the recommended entrypoint for most applications.
    pub fn builder(graph: SignalGraph) -> SignalRuntimeBuilder<(), (), (), (), ()> {
        SignalRuntimeBuilder::new(graph)
    }
}

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn new(
        graph: SignalGraph,
        checkpoint: CheckpointRuntime<D, I>,
        event_bus: EventBus<E, D, Ctx>,
    ) -> Self {
        let mut config = SignalRuntimeConfig::default();
        config.sync_graph_capacity(&graph);
        Self {
            config,
            graph,
            checkpoint,
            event_bus,
            telemetry: RuntimeTelemetry::default(),
            branches: BTreeMap::new(),
        }
    }

    pub fn with_policy(graph: SignalGraph, checkpoint_policy: CheckpointPolicy<D>) -> Self {
        Self::new(
            graph,
            CheckpointRuntime::new(checkpoint_policy),
            EventBus::new(),
        )
    }

    pub fn config(&self) -> &SignalRuntimeConfig<T> {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut SignalRuntimeConfig<T> {
        self.config.sync_graph_capacity(&self.graph);
        &mut self.config
    }

    pub fn graph(&self) -> &SignalGraph {
        &self.graph
    }

    pub fn graph_mut(&mut self) -> SignalGraphMut<'_, D, I, E, Ctx, T> {
        self.config.sync_graph_capacity(&self.graph);
        SignalGraphMut { runtime: self }
    }

    pub fn checkpoint(&self) -> &CheckpointRuntime<D, I> {
        &self.checkpoint
    }

    pub fn event_bus(&self) -> &EventBus<E, D, Ctx> {
        &self.event_bus
    }

    pub fn event_bus_mut(&mut self) -> &mut EventBus<E, D, Ctx> {
        &mut self.event_bus
    }

    pub fn telemetry(&self) -> &RuntimeTelemetry {
        &self.telemetry
    }

    fn capture_branch_state(&self) -> RuntimeBranchState<D, I, T> {
        RuntimeBranchState {
            graph: self.graph.clone_stateful(),
            config: self.config.clone(),
            checkpoint: self.checkpoint.clone(),
            telemetry: self.telemetry.clone(),
        }
    }

    fn load_branch_state(&mut self, state: RuntimeBranchState<D, I, T>) {
        self.graph = state.graph;
        self.config = state.config;
        self.checkpoint = state.checkpoint;
        self.telemetry = state.telemetry;
    }

    /// Explain the current node state using the best available artifact path.
    ///
    /// Depending on runtime policy this may use retained artifacts or
    /// deterministic reconstruction.
    pub fn explain(&self, node: NodeId) -> Result<NodeExplanation, SignalError> {
        let resolver = TierPolicyResolver::new(
            self.config.node_meta(),
            self.config.tier_policies(),
            self.config.fallback_comparator(),
        );
        explain_with_policy_resolver(&self.graph, node, &resolver)
    }

    /// Return the eagerly retained explanation artifact if one exists.
    ///
    /// Use this in hot paths when you do not want reconstruction work.
    pub fn retained_explanation_artifact(&self, node: NodeId) -> Option<NodeExplanation> {
        self.graph.retained_explanation_artifact(node)
    }

    /// Reconstruct the explanation artifact deterministically on demand.
    pub fn reconstruct_explanation_artifact(
        &self,
        node: NodeId,
    ) -> Result<NodeExplanation, SignalError> {
        self.graph.reconstruct_explanation_artifact(node)
    }

    /// Return the eagerly retained provenance artifact if one exists.
    pub fn retained_provenance_artifact(&self, node: NodeId) -> Option<ProvenanceFact> {
        self.graph.retained_provenance_artifact(node)
    }

    /// Reconstruct the provenance artifact deterministically on demand.
    pub fn reconstruct_provenance_artifact(
        &self,
        node: NodeId,
    ) -> Result<ProvenanceFact, SignalError> {
        self.graph.reconstruct_provenance_artifact(node)
    }

    pub fn metrics(&self) -> RuntimeMetrics {
        RuntimeMetrics {
            transaction_begin_count: self.telemetry.transaction_begin_count,
            transaction_commit_count: self.telemetry.transaction_commit_count,
            transaction_rollback_count: self.telemetry.transaction_rollback_count,
            transaction_poison_count: self.telemetry.transaction_poison_count,
            checkpoint_flushes: self.checkpoint.telemetry().checkpoint_flushes,
            checkpoint_flush_nanos: self.checkpoint.telemetry().checkpoint_flush_nanos,
            event_flushes: self.event_bus.telemetry().event_flushes,
            rollback_count: self.event_bus.telemetry().rollback_count,
            staged_node_patch_count: self.telemetry.staged_node_patch_count,
            max_touched_nodes_in_txn: self.telemetry.max_touched_nodes_in_txn,
            keyed_evaluation_count: self.telemetry.keyed_evaluation_count,
            memoization_hits: self.telemetry.memoization_hits,
            memoization_misses: self.telemetry.memoization_misses,
            suppressed_downstream_propagations: self.telemetry.suppressed_downstream_propagations,
            partition_scoped_invalidation_checks: self
                .telemetry
                .partition_scoped_invalidation_checks,
            partition_match_dirty_count: self.telemetry.partition_match_dirty_count,
            detail_match_dirty_count: self.telemetry.detail_match_dirty_count,
            partition_scope_revert_clean_count: self.telemetry.partition_scope_revert_clean_count,
            plans_built: self.telemetry.plans_built,
            stages_built: self.telemetry.stages_built,
            tasks_scheduled: self.telemetry.tasks_scheduled,
            tasks_pruned_before_execution: self.telemetry.tasks_pruned_before_execution,
            maybe_stale_validation_tasks: self.telemetry.maybe_stale_validation_tasks,
            stage_execution_count: self.telemetry.stage_execution_count,
            stage_execution_nanos: self.telemetry.stage_execution_nanos,
            parallel_stage_dispatch_count: self.telemetry.parallel_stage_dispatch_count,
            max_tasks_in_stage: self.telemetry.max_tasks_in_stage,
            serial_executor_usage_count: self.telemetry.serial_executor_usage_count,
            parallel_executor_usage_count: self.telemetry.parallel_executor_usage_count,
            execution_snapshots_built: self.telemetry.execution_snapshots_built,
            prepared_evaluations_produced: self.telemetry.prepared_evaluations_produced,
            prepared_evaluations_applied: self.telemetry.prepared_evaluations_applied,
            dependency_capture_updates: self.telemetry.dependency_capture_updates,
            serial_precompute_task_count: self.telemetry.serial_precompute_task_count,
            parallel_precompute_task_count: self.telemetry.parallel_precompute_task_count,
            execution_snapshot_nanos: self.telemetry.execution_snapshot_nanos,
            stage_precompute_nanos: self.telemetry.stage_precompute_nanos,
            stage_apply_nanos: self.telemetry.stage_apply_nanos,
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

    pub fn runtime_policy(&self) -> SignalRuntimePolicy {
        self.graph.runtime_policy()
    }

    pub fn set_diagnostics_profile(&mut self, profile: DiagnosticsProfile) {
        self.graph.set_diagnostics_profile(profile);
    }

    /// Replace the active runtime policy.
    ///
    /// This changes artifact retention/materialization behavior for subsequent
    /// work. Replay and stable semantic IDs remain the authoritative truth.
    pub fn set_runtime_policy(&mut self, policy: SignalRuntimePolicy) {
        self.graph.set_runtime_policy(policy);
    }

    pub fn capture_snapshot(&mut self) -> SignalSnapshotV1 {
        let mut snapshot = self.graph.capture_snapshot();
        snapshot.runtime_telemetry = Some(self.telemetry.clone());
        snapshot
    }

    pub fn restore_snapshot(&mut self, snapshot: &SignalSnapshotV1) -> Result<(), SignalError> {
        self.graph.restore_snapshot(snapshot)?;
        if let Some(telemetry) = &snapshot.runtime_telemetry {
            self.telemetry = telemetry.clone();
        }
        Ok(())
    }

    pub fn create_branch(
        &mut self,
        name: impl Into<String>,
    ) -> Result<SignalBranchHandle, SignalError> {
        let current_branch_name = self.graph.current_branch().name;
        let handle = self.graph.diagnostics_state_mut().create_branch(name);
        let mut branch_state = self.capture_branch_state();
        branch_state
            .graph
            .diagnostics_state_mut()
            .set_active_branch(handle.id);
        self.branches.insert(handle.id, branch_state);
        let branch_catalog = self.graph.diagnostics_state().branch_catalog().clone();
        for state in self.branches.values_mut() {
            let active_branch = state.graph.current_branch().id;
            state
                .graph
                .diagnostics_state_mut()
                .synchronize_branch_catalog(branch_catalog.clone(), active_branch);
        }
        crate::diagnostics::recorder::record_snapshot_event(
            &mut self.graph,
            crate::diagnostics::replay::ReplayEventKind::BranchCreated,
            None,
            format!("created branch `{}`", handle.name),
        );
        crate::diagnostics::recorder::record_branch_lineage_event(
            &mut self.graph,
            crate::diagnostics::lineage::LineageEvent::BranchedFrom,
            format!(
                "created branch `{}` from {}",
                handle.name, current_branch_name
            ),
        );
        Ok(handle)
    }

    pub fn switch_branch(&mut self, branch: SignalBranchHandle) -> Result<(), SignalError> {
        let current = self.graph.current_branch();
        self.branches
            .insert(current.id, self.capture_branch_state());
        let Some(state) = self.branches.get(&branch.id).cloned() else {
            return Err(SignalError::invalid_input(format!(
                "unknown branch `{}`",
                branch.name
            )));
        };
        self.load_branch_state(state);
        self.graph
            .diagnostics_state_mut()
            .set_active_branch(branch.id);
        crate::diagnostics::recorder::record_snapshot_event(
            &mut self.graph,
            crate::diagnostics::replay::ReplayEventKind::BranchSwitched,
            None,
            format!("switched from `{}` to `{}`", current.name, branch.name),
        );
        crate::diagnostics::recorder::record_branch_lineage_event(
            &mut self.graph,
            crate::diagnostics::lineage::LineageEvent::BranchedFrom,
            format!("switched from `{}` to `{}`", current.name, branch.name),
        );
        Ok(())
    }

    pub fn capture_branch_snapshot(
        &mut self,
        branch: SignalBranchHandle,
    ) -> Result<SignalSnapshotV1, SignalError> {
        if branch.id == self.graph.current_branch().id {
            return Ok(self.capture_snapshot());
        }
        let Some(state) = self.branches.get(&branch.id) else {
            return Err(SignalError::invalid_input(format!(
                "unknown branch `{}`",
                branch.name
            )));
        };
        let policy = state.graph.runtime_policy();
        let mut graph = state.graph.clone_stateful();
        let meta = graph.diagnostics_state_mut().allocate_snapshot_meta(policy);
        let diagnostics = graph.diagnostics_state().snapshot_payload();
        let graph_telemetry = graph.telemetry().clone();
        Ok(SignalSnapshotV1 {
            meta,
            graph,
            diagnostics,
            graph_telemetry,
            runtime_telemetry: Some(state.telemetry.clone()),
        })
    }

    pub fn restore_branch_snapshot(
        &mut self,
        branch: SignalBranchHandle,
        snapshot: &SignalSnapshotV1,
    ) -> Result<(), SignalError> {
        self.graph.validate_snapshot_compatibility(snapshot)?;
        if branch.id == self.graph.current_branch().id {
            return self.restore_snapshot(snapshot);
        }
        let mut graph = snapshot.graph.clone();
        *graph.telemetry_mut() = snapshot.graph_telemetry.clone();
        graph
            .diagnostics_state_mut()
            .restore_snapshot_payload(snapshot.diagnostics.clone());
        let state = RuntimeBranchState {
            graph,
            config: self.config.clone(),
            checkpoint: self.checkpoint.clone(),
            telemetry: snapshot
                .runtime_telemetry
                .clone()
                .unwrap_or_else(|| self.telemetry.clone()),
        };
        let mut state = state;
        crate::diagnostics::recorder::record_snapshot_restore_lineage(
            &mut state.graph,
            snapshot.meta.snapshot_id,
        );
        self.branches.insert(branch.id, state);
        Ok(())
    }

    pub fn current_branch(&self) -> SignalBranchHandle {
        self.graph.current_branch()
    }

    pub fn known_branches(&self) -> Vec<SignalBranchHandle> {
        self.graph.known_branches()
    }

    pub fn branch_handle(&self, branch_id: SignalBranchId) -> Option<SignalBranchHandle> {
        self.graph.branch_handle(branch_id)
    }

    pub fn branch_ancestry(&self, branch_id: SignalBranchId) -> Vec<SignalBranchHandle> {
        self.graph.branch_ancestry(branch_id)
    }

    pub fn replay_for_branch(&self, branch_id: SignalBranchId) -> crate::diagnostics::ReplaySlice {
        self.graph.replay_for_branch(branch_id)
    }

    pub fn replay_for_node(&self, node: NodeId) -> crate::diagnostics::ReplaySlice {
        self.graph.replay_for_node(node)
    }

    pub fn replay_from_cursor(
        &self,
        start: crate::diagnostics::ReplayCursor,
    ) -> crate::diagnostics::ReplaySlice {
        self.graph.replay_from_cursor(start)
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

    /// Register a named computation family for keyed runtime usage.
    pub fn register_computation_family(
        &mut self,
        family: impl Into<ComputationFamily>,
    ) -> ComputationFamily {
        self.config.register_computation_family(family)
    }

    pub fn keyed_node(
        &mut self,
        family: &ComputationFamily,
        key: impl Into<ComputationKey>,
    ) -> NodeId {
        self.config.keyed_node(&mut self.graph, family, key)
    }

    pub fn begin<'a>(&'a mut self) -> SignalTransaction<'a, D, I, E, Ctx, T> {
        self.telemetry.transaction_begin_count += 1;
        self.config.sync_graph_capacity(&self.graph);
        let baseline_config = self.config.clone();
        let baseline_diagnostics_state = self.graph.diagnostics_state().clone();
        SignalTransaction {
            config: &mut self.config,
            graph: &mut self.graph,
            checkpoint: &mut self.checkpoint,
            event_bus: &mut self.event_bus,
            telemetry: &mut self.telemetry,
            staged_dirty: crate::data::dirty_set::BatchedDirtySet::new(),
            staged_checkpoint_flushes: 0,
            staged_checkpoint_flush_nanos: 0,
            staged_events: Vec::new(),
            staged_event_flushes: Vec::new(),
            staged_memo_writes: std::collections::BTreeMap::new(),
            graph_patches: SparsePatchBuffer::new(),
            baseline_config,
            baseline_diagnostics_state,
            semantic_delta: super::transaction_types::TransactionSemanticDelta::default(),
            poisoned: false,
            finished: false,
            staged_patch_count: 0,
        }
    }

    pub fn transaction<F>(
        &mut self,
        runtime_ctx: &mut Ctx,
        apply: F,
    ) -> Result<super::transaction_types::TransactionOutcome, SignalError>
    where
        F: FnOnce(&mut SignalTransaction<'_, D, I, E, Ctx, T>) -> Result<(), SignalError>,
    {
        let mut transaction = self.begin();
        match apply(&mut transaction) {
            Ok(()) => transaction.commit(runtime_ctx),
            Err(err) => {
                let rollback_result = transaction.rollback(runtime_ctx);
                match rollback_result {
                    Ok(_) => Err(err),
                    Err(rollback_err) => Err(rollback_err),
                }
            }
        }
    }
}
