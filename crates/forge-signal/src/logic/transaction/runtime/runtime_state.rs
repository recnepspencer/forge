use crate::data::checkpoint_policy::CheckpointPolicy;
use crate::data::comparator::{TierPolicyResolver, VersionComparatorPolicy};
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::output::{ComputationFamily, ComputationKey};
use crate::data::telemetry::RuntimeTelemetry;
use crate::data::tier::TierPolicy;
use crate::diagnostics::access::RuntimeDiagnostics;
use crate::diagnostics::history::ExecutionInspector;
use crate::diagnostics::profile::DiagnosticsProfile;
use crate::diagnostics::summary::{ExecutionHistorySummary, GraphSummary};
use crate::diagnostics::{FailureSummary, FlowSummary, RollbackDiagnostic};
use crate::logic::checkpoint::CheckpointRuntime;
use crate::logic::events::EventBus;
use crate::logic::explain::{explain_with_policy_resolver, NodeExplanation};
use crate::logic::transaction::patch_buffer::SparsePatchBuffer;
use crate::presentation::metrics::RuntimeMetrics;

use super::builder::SignalRuntimeBuilder;
use super::config::SignalRuntimeConfig;
use super::transaction_types::SignalTransaction;

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
}

impl SignalRuntime<(), (), (), (), ()> {
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

    pub fn graph_mut(&mut self) -> &mut SignalGraph {
        self.config.sync_graph_capacity(&self.graph);
        &mut self.graph
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

    pub fn explain(&self, node: NodeId) -> Result<NodeExplanation, SignalError> {
        let resolver = TierPolicyResolver::new(
            self.config.node_meta(),
            self.config.tier_policies(),
            self.config.fallback_comparator(),
        );
        explain_with_policy_resolver(&self.graph, node, &resolver)
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
            partition_scoped_invalidation_checks: self.telemetry.partition_scoped_invalidation_checks,
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

    pub fn set_diagnostics_profile(&mut self, profile: DiagnosticsProfile) {
        self.graph.set_diagnostics_profile(profile);
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
        let diagnostics_snapshot = self.graph.diagnostics_state().clone();
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
            diagnostics_snapshot,
            pending_failure_summary: None,
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
