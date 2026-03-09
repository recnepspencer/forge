use std::collections::{BTreeMap, BTreeSet};
use std::marker::PhantomData;
use std::time::Instant;

use crate::data::aspect::{Aspect, AspectVersion};
use crate::data::checkpoint::CheckpointBarrier;
use crate::data::checkpoint_policy::CheckpointPolicy;
use crate::data::comparator::{
    TierPolicyResolver, VersionComparatorPolicy,
};
use crate::data::dirty_set::{BatchedDirtySet, DomainImpact};
use crate::data::effect_mapping::EffectMapping;
use crate::data::error::SignalError;
use crate::data::evaluator::CheckpointEvaluator;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::node_meta::NodeMetaStore;
use crate::data::output::ChangedRegion;
use crate::data::output::{
    ComputationFamily, ComputationKey, KeyedComputation, NodeEvaluationResult,
    StructuralMemoKey,
};
use crate::data::telemetry::RuntimeTelemetry;
use crate::data::tier::TierPolicy;
use crate::data::tier_policy_table::TierPolicyTable;
use crate::diagnostics::state::DiagnosticsState;
use crate::diagnostics::access::RuntimeDiagnostics;
use crate::diagnostics::history::ExecutionInspector;
use crate::diagnostics::profile::DiagnosticsProfile;
use crate::diagnostics::recorder::DiagnosticsRecorder;
use crate::diagnostics::summary::{ExecutionHistorySummary, GraphSummary};
use crate::diagnostics::{
    ExecutionFailureContext, ExecutionFailurePhase, FailureSummary, FlowSummary,
    RollbackDiagnostic,
};
use crate::logic::checkpoint::CheckpointRuntime;
use crate::logic::evaluation::EvaluationRequestMode;
use crate::logic::events::EventBus;
use crate::logic::explain::{explain_with_policy_resolver, NodeExplanation};
use crate::logic::invalidation::{mark_dirty, mark_dirty_with_regions};
use crate::logic::planner::{
    build_evaluation_plan_with_policy_resolver, execute_prepared_plan_with_policy,
    EvaluationPlan, ExecutionReport, StageExecutor,
};
use crate::logic::prepared::{
    ExecutionReadView, PreparedEvaluation, PreparedEvaluationOrigin, PreparedKeyedContext,
    PreparedMemoDecision,
};
use crate::presentation::metrics::RuntimeMetrics;

use super::patch_buffer::SparsePatchBuffer;

/// Builder for the productized runtime surface.
pub struct SignalRuntimeBuilder<D = (), I = (), E = (), Ctx = (), T = ()>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    graph: SignalGraph,
    checkpoint_policy: CheckpointPolicy<D>,
    fallback_comparator: VersionComparatorPolicy,
    _marker: PhantomData<fn(I, E, Ctx, T)>,
}

impl<D, I, E, Ctx, T> SignalRuntimeBuilder<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    fn new(graph: SignalGraph) -> Self {
        Self {
            graph,
            checkpoint_policy: CheckpointPolicy::new(CheckpointBarrier::PerOperation),
            fallback_comparator: VersionComparatorPolicy::Exact,
            _marker: PhantomData,
        }
    }

    /// Set a default checkpoint barrier for the runtime policy.
    pub fn checkpoint_barrier(mut self, barrier: CheckpointBarrier) -> Self {
        self.checkpoint_policy = CheckpointPolicy::new(barrier);
        self
    }

    /// Replace the checkpoint policy directly.
    pub fn checkpoint_policy(mut self, policy: CheckpointPolicy<D>) -> Self {
        self.checkpoint_policy = policy;
        self
    }

    /// Set the fallback comparator policy.
    pub fn fallback_comparator(mut self, comparator: VersionComparatorPolicy) -> Self {
        self.fallback_comparator = comparator;
        self
    }

    /// Change the runtime event type.
    pub fn with_events<E2>(self) -> SignalRuntimeBuilder<D, I, E2, Ctx, T> {
        SignalRuntimeBuilder {
            graph: self.graph,
            checkpoint_policy: self.checkpoint_policy,
            fallback_comparator: self.fallback_comparator,
            _marker: PhantomData,
        }
    }

    /// Change the checkpoint policy domain type.
    pub fn with_domains<D2>(self) -> SignalRuntimeBuilder<D2, I, E, Ctx, T>
    where
        D2: Copy + Ord + std::fmt::Debug + 'static,
    {
        SignalRuntimeBuilder {
            graph: self.graph,
            checkpoint_policy: CheckpointPolicy::new(self.checkpoint_policy.barrier_for_default()),
            fallback_comparator: self.fallback_comparator,
            _marker: PhantomData,
        }
    }

    /// Change the checkpoint evaluator impact type.
    pub fn with_impacts<I2>(self) -> SignalRuntimeBuilder<D, I2, E, Ctx, T>
    where
        I2: Copy + Ord,
    {
        SignalRuntimeBuilder {
            graph: self.graph,
            checkpoint_policy: self.checkpoint_policy,
            fallback_comparator: self.fallback_comparator,
            _marker: PhantomData,
        }
    }

    /// Change the runtime tier type.
    pub fn with_tiers<T2>(self) -> SignalRuntimeBuilder<D, I, E, Ctx, T2>
    where
        T2: Copy + Ord,
    {
        SignalRuntimeBuilder {
            graph: self.graph,
            checkpoint_policy: self.checkpoint_policy,
            fallback_comparator: self.fallback_comparator,
            _marker: PhantomData,
        }
    }

    /// Change the runtime context type.
    pub fn with_context<Ctx2>(self) -> SignalRuntimeBuilder<D, I, E, Ctx2, T> {
        SignalRuntimeBuilder {
            graph: self.graph,
            checkpoint_policy: self.checkpoint_policy,
            fallback_comparator: self.fallback_comparator,
            _marker: PhantomData,
        }
    }

    /// Build the runtime with safe defaults.
    pub fn build(self) -> SignalRuntime<D, I, E, Ctx, T> {
        let mut runtime = SignalRuntime::with_policy(self.graph, self.checkpoint_policy);
        runtime.set_fallback_comparator(self.fallback_comparator);
        runtime
    }
}

/// Immutable or near-immutable runtime configuration shared across transactions.
#[derive(Debug, Clone)]
pub struct SignalRuntimeConfig<T: Copy + Ord> {
    node_meta: NodeMetaStore<T>,
    tier_policies: TierPolicyTable<T>,
    fallback_comparator: VersionComparatorPolicy,
    keyed_nodes: BTreeMap<(ComputationFamily, ComputationKey), NodeId>,
    memo_cache: BTreeMap<(ComputationFamily, StructuralMemoKey), NodeEvaluationResult>,
}

impl<T: Copy + Ord> Default for SignalRuntimeConfig<T> {
    fn default() -> Self {
        Self {
            node_meta: NodeMetaStore::default(),
            tier_policies: TierPolicyTable::default(),
            fallback_comparator: VersionComparatorPolicy::Exact,
            keyed_nodes: BTreeMap::new(),
            memo_cache: BTreeMap::new(),
        }
    }
}

impl<T: Copy + Ord> SignalRuntimeConfig<T> {
    /// Create an empty runtime config.
    pub fn new() -> Self {
        Self::default()
    }

    fn sync_graph_capacity(&mut self, graph: &SignalGraph) {
        self.node_meta.ensure_capacity(graph.arena_capacity());
    }

    /// Assign one node to a comparator tier.
    pub fn set_node_tier(&mut self, graph: &SignalGraph, node: NodeId, tier: T) {
        self.sync_graph_capacity(graph);
        self.node_meta.set_tier(node, tier);
    }

    /// Register/update one tier policy.
    pub fn set_tier_policy(&mut self, policy: TierPolicy<T>) {
        self.tier_policies.set(policy);
    }

    /// Set global comparator fallback.
    pub fn set_fallback_comparator(&mut self, policy: VersionComparatorPolicy) {
        self.fallback_comparator = policy;
    }

    /// Read-only metadata storage.
    pub fn node_meta(&self) -> &NodeMetaStore<T> {
        &self.node_meta
    }

    /// Read-only tier policy table.
    pub fn tier_policies(&self) -> &TierPolicyTable<T> {
        &self.tier_policies
    }

    /// Read-only fallback comparator policy.
    pub fn fallback_comparator(&self) -> &VersionComparatorPolicy {
        &self.fallback_comparator
    }

    pub fn register_computation_family(
        &mut self,
        family: impl Into<ComputationFamily>,
    ) -> ComputationFamily {
        family.into()
    }

    pub fn keyed_node(
        &mut self,
        graph: &mut SignalGraph,
        family: &ComputationFamily,
        key: impl Into<ComputationKey>,
    ) -> NodeId {
        let key = key.into();
        let registry_key = (family.clone(), key.clone());
        if let Some(node) = self.keyed_nodes.get(&registry_key).copied() {
            return node;
        }
        let node = graph.node().build();
        self.sync_graph_capacity(graph);
        self.keyed_nodes.insert(registry_key, node);
        node
    }

    fn lookup_memoized_result(
        &self,
        family: &ComputationFamily,
        memo_key: &StructuralMemoKey,
    ) -> Option<NodeEvaluationResult> {
        self.memo_cache
            .get(&(family.clone(), memo_key.clone()))
            .cloned()
    }

    fn store_memoized_result(
        &mut self,
        family: &ComputationFamily,
        memo_key: &StructuralMemoKey,
        result: NodeEvaluationResult,
    ) {
        self.memo_cache
            .insert((family.clone(), memo_key.clone()), result);
    }
}

/// Transaction runtime that owns committed signal components.
pub struct SignalRuntime<D, I, E, Ctx, T = ()>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    config: SignalRuntimeConfig<T>,
    graph: SignalGraph,
    checkpoint: CheckpointRuntime<D, I>,
    event_bus: EventBus<E, D, Ctx>,
    telemetry: RuntimeTelemetry,
}

impl SignalRuntime<(), (), (), (), ()> {
    /// Productized runtime entrypoint with sensible defaults.
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
    /// Build a runtime from committed components.
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

    #[doc(hidden)]
    /// Low-level constructor with fresh checkpoint/event runtimes.
    pub fn with_policy(graph: SignalGraph, checkpoint_policy: CheckpointPolicy<D>) -> Self {
        Self::new(
            graph,
            CheckpointRuntime::new(checkpoint_policy),
            EventBus::new(),
        )
    }

    /// Immutable access to runtime config.
    pub fn config(&self) -> &SignalRuntimeConfig<T> {
        &self.config
    }

    /// Mutable access to runtime config.
    pub fn config_mut(&mut self) -> &mut SignalRuntimeConfig<T> {
        self.config.sync_graph_capacity(&self.graph);
        &mut self.config
    }

    /// Immutable access to committed signal graph.
    pub fn graph(&self) -> &SignalGraph {
        &self.graph
    }

    /// Mutable access to committed signal graph for host-owned structural rewiring.
    ///
    /// Embeddings use this for node allocation and dependency edits. Evaluation,
    /// invalidation, and rollback-sensitive work should still flow through
    /// `SignalTransaction`.
    pub fn graph_mut(&mut self) -> &mut SignalGraph {
        self.config.sync_graph_capacity(&self.graph);
        &mut self.graph
    }

    /// Immutable access to committed checkpoint runtime.
    pub fn checkpoint(&self) -> &CheckpointRuntime<D, I> {
        &self.checkpoint
    }

    /// Immutable access to committed event bus.
    pub fn event_bus(&self) -> &EventBus<E, D, Ctx> {
        &self.event_bus
    }

    /// Mutable access to committed event bus for subscriber registration.
    pub fn event_bus_mut(&mut self) -> &mut EventBus<E, D, Ctx> {
        &mut self.event_bus
    }

    /// Runtime telemetry snapshot.
    pub fn telemetry(&self) -> &RuntimeTelemetry {
        &self.telemetry
    }

    /// Structured explanation for one node using runtime comparator policy.
    pub fn explain(&self, node: NodeId) -> Result<NodeExplanation, SignalError> {
        let resolver = TierPolicyResolver::new(
            self.config.node_meta(),
            self.config.tier_policies(),
            self.config.fallback_comparator(),
        );
        explain_with_policy_resolver(&self.graph, node, &resolver)
    }

    /// Structured runtime metrics snapshot.
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

    /// Production diagnostics summary for the committed runtime graph state.
    pub fn diagnostics_summary(&self, profile: DiagnosticsProfile) -> GraphSummary {
        self.graph.diagnostics_summary(profile)
    }

    /// Central diagnostics facade for this runtime.
    pub fn diagnostics(&self) -> RuntimeDiagnostics<'_> {
        crate::diagnostics::access::diagnostics_for_runtime(self)
    }

    pub fn diagnostics_profile(&self) -> DiagnosticsProfile {
        self.graph.diagnostics_profile()
    }

    pub fn set_diagnostics_profile(&mut self, profile: DiagnosticsProfile) {
        self.graph.set_diagnostics_profile(profile);
    }

    /// Production diagnostics summary for execution/trace history visible on the committed graph.
    pub fn execution_history_summary(
        &self,
        profile: DiagnosticsProfile,
    ) -> ExecutionHistorySummary {
        self.graph.execution_history_summary(profile)
    }

    /// Structured execution-history inspector for the committed graph.
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

    /// Graphviz DOT export for the committed graph.
    pub fn to_dot(&self) -> String {
        self.graph.to_dot()
    }

    /// Build a deterministic evaluation plan using tier-aware comparator policy.
    pub fn build_evaluation_plan(
        &self,
        targets: &[NodeId],
        request_mode: EvaluationRequestMode,
    ) -> Result<EvaluationPlan, SignalError> {
        let mut resolver = TierPolicyResolver::new(
            self.config.node_meta(),
            self.config.tier_policies(),
            self.config.fallback_comparator(),
        );
        build_evaluation_plan_with_policy_resolver(
            &self.graph,
            targets,
            request_mode,
            &mut resolver,
        )
    }

    /// Execute one pre-built plan with the prepared-evaluation contract.
    pub fn execute_prepared_plan<F>(
        &mut self,
        plan: &EvaluationPlan,
        precompute: &F,
    ) -> Result<ExecutionReport, SignalError>
    where
        F: Fn(NodeId, &ExecutionReadView<'_>) -> Result<PreparedEvaluation, SignalError> + Sync,
    {
        self.execute_prepared_plan_with_executor(plan, precompute, StageExecutor::Serial)
    }

    /// Execute one pre-built prepared-evaluation plan with an explicit executor.
    pub fn execute_prepared_plan_with_executor<F>(
        &mut self,
        plan: &EvaluationPlan,
        precompute: &F,
        executor: StageExecutor,
    ) -> Result<ExecutionReport, SignalError>
    where
        F: Fn(NodeId, &ExecutionReadView<'_>) -> Result<PreparedEvaluation, SignalError> + Sync,
    {
        let mut resolver = TierPolicyResolver::new(
            self.config.node_meta(),
            self.config.tier_policies(),
            self.config.fallback_comparator(),
        );
        let report = execute_prepared_plan_with_policy(
            &mut self.graph,
            plan,
            precompute,
            &mut resolver,
            executor,
        )?;
        self.absorb_execution_report_telemetry(&report);
        Ok(report)
    }

    /// Convenience evaluation path that builds and executes a prepared plan for one target.
    pub fn evaluate_with_plan<F>(
        &mut self,
        node: NodeId,
        precompute: &F,
        request_mode: EvaluationRequestMode,
    ) -> Result<ExecutionReport, SignalError>
    where
        F: Fn(NodeId, &ExecutionReadView<'_>) -> Result<PreparedEvaluation, SignalError> + Sync,
    {
        let plan = self.build_evaluation_plan(&[node], request_mode)?;
        self.execute_prepared_plan(&plan, precompute)
    }

    /// Convenience evaluation path that builds and executes a prepared plan with an explicit executor.
    pub fn evaluate_with_plan_and_executor<F>(
        &mut self,
        node: NodeId,
        precompute: &F,
        request_mode: EvaluationRequestMode,
        executor: StageExecutor,
    ) -> Result<ExecutionReport, SignalError>
    where
        F: Fn(NodeId, &ExecutionReadView<'_>) -> Result<PreparedEvaluation, SignalError> + Sync,
    {
        let plan = self.build_evaluation_plan(&[node], request_mode)?;
        self.execute_prepared_plan_with_executor(&plan, precompute, executor)
    }

    /// Read one node through the planner-backed prepared path and return its current version.
    pub fn read<F>(
        &mut self,
        node: NodeId,
        precompute: &F,
    ) -> Result<AspectVersion, SignalError>
    where
        F: Fn(NodeId, &ExecutionReadView<'_>) -> Result<PreparedEvaluation, SignalError> + Sync,
    {
        self.read_with_executor(node, precompute, StageExecutor::Serial)
    }

    /// Alias for `read(...)` using more familiar signal vocabulary.
    pub fn get<F>(
        &mut self,
        node: NodeId,
        precompute: &F,
    ) -> Result<AspectVersion, SignalError>
    where
        F: Fn(NodeId, &ExecutionReadView<'_>) -> Result<PreparedEvaluation, SignalError> + Sync,
    {
        self.read(node, precompute)
    }

    /// Read one node with an explicit stage executor.
    pub fn read_with_executor<F>(
        &mut self,
        node: NodeId,
        precompute: &F,
        executor: StageExecutor,
    ) -> Result<AspectVersion, SignalError>
    where
        F: Fn(NodeId, &ExecutionReadView<'_>) -> Result<PreparedEvaluation, SignalError> + Sync,
    {
        self.evaluate_with_plan_and_executor(node, precompute, EvaluationRequestMode::Default, executor)?;
        Ok(self.graph.get_entry(node)?.get_aspect_version())
    }

    /// Evaluate the current dirty or maybe-stale frontier in one planner pass.
    pub fn evaluate_dirty<F>(&mut self, precompute: &F) -> Result<ExecutionReport, SignalError>
    where
        F: Fn(NodeId, &ExecutionReadView<'_>) -> Result<PreparedEvaluation, SignalError> + Sync,
    {
        self.evaluate_dirty_with_executor(precompute, StageExecutor::Serial)
    }

    /// Evaluate the current dirty or maybe-stale frontier with an explicit executor.
    pub fn evaluate_dirty_with_executor<F>(
        &mut self,
        precompute: &F,
        executor: StageExecutor,
    ) -> Result<ExecutionReport, SignalError>
    where
        F: Fn(NodeId, &ExecutionReadView<'_>) -> Result<PreparedEvaluation, SignalError> + Sync,
    {
        let targets = collect_dirty_targets(&self.graph);
        if targets.is_empty() {
            return Ok(empty_execution_report());
        }
        let plan = self.build_evaluation_plan(&targets, EvaluationRequestMode::Default)?;
        self.execute_prepared_plan_with_executor(&plan, precompute, executor)
    }

    fn absorb_execution_report_telemetry(&mut self, report: &ExecutionReport) {
        self.telemetry.plans_built += 1;
        self.telemetry.stages_built += report.stage_count as u64;
        self.telemetry.tasks_scheduled += report.task_count as u64;
        self.telemetry.tasks_pruned_before_execution += report.tasks_pruned as u64;
        self.telemetry.maybe_stale_validation_tasks += report
            .stages
            .iter()
            .flat_map(|stage| &stage.task_records)
            .filter(|record| {
                matches!(
                    record.scheduled_reason,
                    crate::logic::planner::TaskReason::MaybeStaleValidation
                )
            })
            .count() as u64;
        self.telemetry.stage_execution_count += report.stage_count as u64;
        self.telemetry.stage_execution_nanos += report
            .stages
            .iter()
            .map(|stage| stage.duration_nanos)
            .sum::<u128>();
        self.telemetry.execution_snapshots_built += report.execution_snapshots_built as u64;
        self.telemetry.prepared_evaluations_produced += report.prepared_evaluations_produced as u64;
        self.telemetry.prepared_evaluations_applied += report.prepared_evaluations_applied as u64;
        self.telemetry.dependency_capture_updates += report.dependency_capture_updates as u64;
        self.telemetry.execution_snapshot_nanos += report.execution_snapshot_nanos;
        self.telemetry.stage_precompute_nanos += report.stage_precompute_nanos;
        self.telemetry.stage_apply_nanos += report.stage_apply_nanos;
        #[cfg(feature = "parallel")]
        let parallel_stages = report
            .stages
            .iter()
            .filter(|stage| {
                matches!(
                    stage.outcome,
                    crate::logic::planner::StageExecutionOutcome::CompletedParallel
                )
            })
            .count() as u64;
        #[cfg(not(feature = "parallel"))]
        let parallel_stages = 0_u64;
        if parallel_stages > 0 {
            self.telemetry.parallel_executor_usage_count += 1;
            self.telemetry.parallel_stage_dispatch_count += parallel_stages;
            self.telemetry.parallel_precompute_task_count += report.task_count as u64;
        } else {
            self.telemetry.serial_executor_usage_count += 1;
            self.telemetry.serial_precompute_task_count += report.task_count as u64;
        }
        self.telemetry.max_tasks_in_stage = self.telemetry.max_tasks_in_stage.max(
            report
                .stages
                .iter()
                .map(|stage| stage.task_records.len() as u64)
                .max()
                .unwrap_or(0),
        );
    }

    /// Assign one node to a comparator tier.
    pub fn set_node_tier(&mut self, node: NodeId, tier: T) {
        self.config.set_node_tier(&self.graph, node, tier);
    }

    /// Register/update one tier policy.
    pub fn set_tier_policy(&mut self, policy: TierPolicy<T>) {
        self.config.set_tier_policy(policy);
    }

    /// Set global comparator fallback.
    pub fn set_fallback_comparator(&mut self, policy: VersionComparatorPolicy) {
        self.config.set_fallback_comparator(policy);
    }

    /// Register or normalize one computation family namespace.
    pub fn register_computation_family(
        &mut self,
        family: impl Into<ComputationFamily>,
    ) -> ComputationFamily {
        self.config.register_computation_family(family)
    }

    /// Look up or allocate one keyed node inside a computation family.
    pub fn keyed_node(
        &mut self,
        family: &ComputationFamily,
        key: impl Into<ComputationKey>,
    ) -> NodeId {
        self.config.keyed_node(&mut self.graph, family, key)
    }

    /// Begin a transaction scope over committed runtime state.
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
            staged_dirty: BatchedDirtySet::new(),
            staged_checkpoint_flushes: 0,
            staged_checkpoint_flush_nanos: 0,
            staged_events: Vec::new(),
            staged_event_flushes: Vec::new(),
            staged_memo_writes: BTreeMap::new(),
            graph_patches: SparsePatchBuffer::new(),
            diagnostics_snapshot,
            pending_failure_summary: None,
            poisoned: false,
            finished: false,
            staged_patch_count: 0,
        }
    }

    /// Run one transaction with automatic commit/rollback behavior.
    pub fn transaction<F>(
        &mut self,
        runtime_ctx: &mut Ctx,
        apply: F,
    ) -> Result<TransactionOutcome, SignalError>
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

/// Outcome of closing a transaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionOutcome {
    Committed,
    RolledBack,
    Poisoned,
}

/// Active transaction scope for signal runtime.
pub struct SignalTransaction<'a, D, I, E, Ctx, T = ()>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    config: &'a mut SignalRuntimeConfig<T>,
    graph: &'a mut SignalGraph,
    checkpoint: &'a mut CheckpointRuntime<D, I>,
    event_bus: &'a mut EventBus<E, D, Ctx>,
    telemetry: &'a mut RuntimeTelemetry,
    staged_dirty: BatchedDirtySet<D, I>,
    staged_checkpoint_flushes: u64,
    staged_checkpoint_flush_nanos: u128,
    staged_events: Vec<E>,
    staged_event_flushes: Vec<CheckpointBarrier>,
    staged_memo_writes: BTreeMap<(ComputationFamily, StructuralMemoKey), NodeEvaluationResult>,
    graph_patches: SparsePatchBuffer,
    diagnostics_snapshot: DiagnosticsState,
    pending_failure_summary: Option<FailureSummary>,
    poisoned: bool,
    finished: bool,
    staged_patch_count: u64,
}

impl<'a, D, I, E, Ctx, T> SignalTransaction<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    /// Immutable view of staged graph.
    pub fn staged_graph(&self) -> &SignalGraph {
        self.graph
    }

    /// Emit one event into staged event bus.
    pub fn emit_event(&mut self, event: E) {
        self.staged_events.push(event);
    }

    /// Route one effect into staged checkpoint dirty set.
    pub fn record_effect<M>(&mut self, effect: &M::Effect)
    where
        M: EffectMapping<Domain = D, Impact = I>,
    {
        M::route(effect, &mut self.staged_dirty);
    }

    /// Mark one source dirty in staged graph.
    pub fn mark_dirty(
        &mut self,
        source: NodeId,
        changed_aspect: Aspect,
    ) -> Result<(), SignalError> {
        self.stage_mark_dirty_candidates(source)?;
        let result = mark_dirty(self.graph, source, changed_aspect);
        self.apply_result(result)
    }

    /// Mark one source dirty with changed partition/region metadata.
    pub fn mark_dirty_with_regions(
        &mut self,
        source: NodeId,
        changed_aspect: Aspect,
        changed_regions: &[ChangedRegion],
    ) -> Result<(), SignalError> {
        self.stage_mark_dirty_candidates(source)?;
        let result = mark_dirty_with_regions(self.graph, source, changed_aspect, changed_regions);
        self.apply_result(result)
    }

    /// Convenience prepared-evaluation path that builds and executes a plan for one target.
    pub fn evaluate_with_plan<F>(
        &mut self,
        node: NodeId,
        precompute: &F,
        request_mode: EvaluationRequestMode,
    ) -> Result<ExecutionReport, SignalError>
    where
        F: Fn(NodeId, &ExecutionReadView<'_>) -> Result<PreparedEvaluation, SignalError> + Sync,
    {
        self.evaluate_with_plan_and_executor(node, precompute, request_mode, StageExecutor::Serial)
    }

    /// Convenience prepared-evaluation path with an explicit executor.
    pub fn evaluate_with_plan_and_executor<F>(
        &mut self,
        node: NodeId,
        precompute: &F,
        request_mode: EvaluationRequestMode,
        executor: StageExecutor,
    ) -> Result<ExecutionReport, SignalError>
    where
        F: Fn(NodeId, &ExecutionReadView<'_>) -> Result<PreparedEvaluation, SignalError> + Sync,
    {
        self.stage_evaluate_candidates(node)?;
        let mut resolver = TierPolicyResolver::new(
            self.config.node_meta(),
            self.config.tier_policies(),
            self.config.fallback_comparator(),
        );
        let plan = match build_evaluation_plan_with_policy_resolver(
            self.graph,
            &[node],
            request_mode,
            &mut resolver,
        ) {
            Ok(plan) => plan,
            Err(err) => {
                self.record_failure_from_error(ExecutionFailurePhase::Planning, &err, None);
                return Err(err);
            }
        };
        self.execute_prepared_plan_with_executor(&plan, precompute, executor)
    }

    /// Execute one pre-built prepared-evaluation plan against the staged graph.
    pub fn execute_prepared_plan_with_executor<F>(
        &mut self,
        plan: &EvaluationPlan,
        precompute: &F,
        executor: StageExecutor,
    ) -> Result<ExecutionReport, SignalError>
    where
        F: Fn(NodeId, &ExecutionReadView<'_>) -> Result<PreparedEvaluation, SignalError> + Sync,
    {
        let mut resolver = TierPolicyResolver::new(
            self.config.node_meta(),
            self.config.tier_policies(),
            self.config.fallback_comparator(),
        );
        let report = match execute_prepared_plan_with_policy(
            self.graph,
            plan,
            precompute,
            &mut resolver,
            executor,
        ) {
            Ok(report) => report,
            Err(err) => {
                if let Some(summary) = self.graph.latest_failure_diagnostics().cloned() {
                    self.pending_failure_summary = Some(summary);
                } else {
                    self.record_failure_from_error(
                        ExecutionFailurePhase::Apply,
                        &err,
                        Some(plan.summary.clone()),
                    );
                }
                return Err(err);
            }
        };
        self.absorb_execution_report_telemetry(&report);
        Ok(report)
    }

    /// Read one node through the planner-backed prepared path and return its current version.
    pub fn read<F>(
        &mut self,
        node: NodeId,
        precompute: &F,
    ) -> Result<AspectVersion, SignalError>
    where
        F: Fn(NodeId, &ExecutionReadView<'_>) -> Result<PreparedEvaluation, SignalError> + Sync,
    {
        self.read_with_executor(node, precompute, StageExecutor::Serial)
    }

    /// Alias for `read(...)` using more familiar signal vocabulary.
    pub fn get<F>(
        &mut self,
        node: NodeId,
        precompute: &F,
    ) -> Result<AspectVersion, SignalError>
    where
        F: Fn(NodeId, &ExecutionReadView<'_>) -> Result<PreparedEvaluation, SignalError> + Sync,
    {
        self.read(node, precompute)
    }

    /// Read one node with an explicit stage executor.
    pub fn read_with_executor<F>(
        &mut self,
        node: NodeId,
        precompute: &F,
        executor: StageExecutor,
    ) -> Result<AspectVersion, SignalError>
    where
        F: Fn(NodeId, &ExecutionReadView<'_>) -> Result<PreparedEvaluation, SignalError> + Sync,
    {
        self.evaluate_with_plan_and_executor(node, precompute, EvaluationRequestMode::Default, executor)?;
        Ok(self.graph.get_entry(node)?.get_aspect_version())
    }

    /// Evaluate the currently dirty or maybe-stale frontier in one planner pass.
    pub fn evaluate_dirty<F>(&mut self, precompute: &F) -> Result<ExecutionReport, SignalError>
    where
        F: Fn(NodeId, &ExecutionReadView<'_>) -> Result<PreparedEvaluation, SignalError> + Sync,
    {
        self.evaluate_dirty_with_executor(precompute, StageExecutor::Serial)
    }

    /// Evaluate the currently dirty or maybe-stale frontier with an explicit executor.
    pub fn evaluate_dirty_with_executor<F>(
        &mut self,
        precompute: &F,
        executor: StageExecutor,
    ) -> Result<ExecutionReport, SignalError>
    where
        F: Fn(NodeId, &ExecutionReadView<'_>) -> Result<PreparedEvaluation, SignalError> + Sync,
    {
        let targets = self.collect_dirty_targets();
        if targets.is_empty() {
            return Ok(empty_execution_report());
        }
        let mut resolver = TierPolicyResolver::new(
            self.config.node_meta(),
            self.config.tier_policies(),
            self.config.fallback_comparator(),
        );
        let plan = match build_evaluation_plan_with_policy_resolver(
            self.graph,
            &targets,
            EvaluationRequestMode::Default,
            &mut resolver,
        ) {
            Ok(plan) => plan,
            Err(err) => {
                self.record_failure_from_error(ExecutionFailurePhase::Planning, &err, None);
                return Err(err);
            }
        };
        self.execute_prepared_plan_with_executor(&plan, precompute, executor)
    }

    /// Evaluate one keyed computation with optional structural memoization.
    pub fn evaluate_keyed<F>(
        &mut self,
        node: NodeId,
        computation: &KeyedComputation,
        precompute: &F,
    ) -> Result<(), SignalError>
    where
        F: Fn(NodeId, &ExecutionReadView<'_>) -> Result<PreparedEvaluation, SignalError> + Sync,
    {
        self.evaluate_keyed_with_mode(node, computation, precompute, EvaluationRequestMode::Default)
    }

    /// Evaluate one keyed computation with explicit request mode.
    pub fn evaluate_keyed_with_mode<F>(
        &mut self,
        node: NodeId,
        computation: &KeyedComputation,
        precompute: &F,
        request_mode: EvaluationRequestMode,
    ) -> Result<(), SignalError>
    where
        F: Fn(NodeId, &ExecutionReadView<'_>) -> Result<PreparedEvaluation, SignalError> + Sync,
    {
        self.telemetry.keyed_evaluation_count += 1;
        self.stage_evaluate_candidates(node)?;
        let mut resolver = TierPolicyResolver::new(
            self.config.node_meta(),
            self.config.tier_policies(),
            self.config.fallback_comparator(),
        );
        let plan = match build_evaluation_plan_with_policy_resolver(
            self.graph,
            &[node],
            request_mode,
            &mut resolver,
        ) {
            Ok(plan) => plan,
            Err(err) => {
                self.record_failure_from_error(ExecutionFailurePhase::Planning, &err, None);
                return Err(err);
            }
        };
        let base_keyed_context = PreparedKeyedContext {
            family: Some(computation.family.clone()),
            key: Some(computation.key.clone()),
            memo_key: computation.memo_key.clone(),
            memoized_origin: crate::data::output::MemoizedResultOrigin::DirectCompute,
        };
        if let Some(memo_key) = computation.memo_key.as_ref() {
            if let Some(cached) = self
                .staged_memo_writes
                .get(&(computation.family.clone(), memo_key.clone()))
                .cloned()
                .or_else(|| {
                    self.config
                        .lookup_memoized_result(&computation.family, memo_key)
                })
            {
                self.telemetry.memoization_hits += 1;
                let cached_result = cached.clone();
                let report = match execute_prepared_plan_with_policy(
                    self.graph,
                    &plan,
                    &|_current, _view| {
                        Ok(PreparedEvaluation::from_result(cached_result.clone())
                            .with_origin(PreparedEvaluationOrigin::MemoizedReuse)
                            .with_memo_decision(PreparedMemoDecision::Hit)
                            .with_keyed(PreparedKeyedContext {
                                memoized_origin: crate::data::output::MemoizedResultOrigin::MemoizedFromCache,
                                ..base_keyed_context.clone()
                            }))
                    },
                    &mut resolver,
                    StageExecutor::Serial,
                ) {
                    Ok(report) => report,
                    Err(err) => {
                        self.record_failure_from_error(
                            ExecutionFailurePhase::Apply,
                            &err,
                            Some(plan.summary.clone()),
                        );
                        return Err(err);
                    }
                };
                self.absorb_execution_report_telemetry(&report);
                return self.apply_result(Ok(()));
            }
            self.telemetry.memoization_misses += 1;
        }

        let last_result = std::sync::Mutex::new(None);
        let result = match execute_prepared_plan_with_policy(
            self.graph,
            &plan,
            &|current, view| {
                let prepared = precompute(current, view)?
                    .with_memo_decision(PreparedMemoDecision::Miss)
                    .with_keyed(base_keyed_context.clone());
                if current == node {
                    let mut guard = last_result
                        .lock()
                        .map_err(|_| SignalError::internal("memo capture mutex poisoned"))?;
                    *guard = Some(prepared.result.clone());
                }
                Ok(prepared)
            },
            &mut resolver,
            StageExecutor::Serial,
        ) {
            Ok(report) => Ok(report),
            Err(err) => {
                self.record_failure_from_error(
                    ExecutionFailurePhase::Apply,
                    &err,
                    Some(plan.summary.clone()),
                );
                Err(err)
            }
        };
        let result = match result {
            Ok(report) => {
                self.absorb_execution_report_telemetry(&report);
                self.apply_result(Ok(()))
            }
            Err(err) => self.apply_result(Err(err)),
        };
        if result.is_ok() {
            if let (Some(memo_key), Ok(mut guard)) =
                (computation.memo_key.as_ref(), last_result.lock())
            {
                if let Some(last_result) = guard.take() {
                    self.staged_memo_writes
                        .insert((computation.family.clone(), memo_key.clone()), last_result);
                }
            }
        }
        result
    }

    /// Flush staged checkpoint runtime at the specified barrier.
    pub fn flush_checkpoint<Ev>(
        &mut self,
        barrier: CheckpointBarrier,
        evaluator: &mut Ev,
        ctx: &mut Ev::Context,
    ) -> Result<usize, SignalError>
    where
        Ev: CheckpointEvaluator<Domain = D, Impact = I>,
    {
        let flush_start = Instant::now();
        let domains: Vec<D> = self
            .staged_dirty
            .dirty_domains()
            .filter(|domain| self.checkpoint.policy().barrier_for(*domain) == barrier)
            .collect();

        for domain in &domains {
            let impact = self
                .staged_dirty
                .take_domain_impact(*domain)
                .unwrap_or_else(DomainImpact::empty);
            evaluator.refresh(*domain, impact, ctx)?;
        }

        self.staged_checkpoint_flushes += 1;
        self.staged_checkpoint_flush_nanos += flush_start.elapsed().as_nanos();
        Ok(domains.len())
    }

    /// Flush staged event bus at the specified barrier.
    pub fn flush_events(&mut self, barrier: CheckpointBarrier) -> Result<(), SignalError> {
        self.staged_event_flushes.push(barrier);
        Ok(())
    }

    fn apply_result<R>(&mut self, result: Result<R, SignalError>) -> Result<R, SignalError> {
        match result {
            Ok(value) => Ok(value),
            Err(err) => {
                self.poisoned = true;
                Err(err)
            }
        }
    }

    fn absorb_execution_report_telemetry(&mut self, report: &ExecutionReport) {
        self.telemetry.plans_built += 1;
        self.telemetry.stages_built += report.stage_count as u64;
        self.telemetry.tasks_scheduled += report.task_count as u64;
        self.telemetry.tasks_pruned_before_execution += report.tasks_pruned as u64;
        self.telemetry.maybe_stale_validation_tasks += report
            .stages
            .iter()
            .flat_map(|stage| &stage.task_records)
            .filter(|record| {
                matches!(
                    record.scheduled_reason,
                    crate::logic::planner::TaskReason::MaybeStaleValidation
                )
            })
            .count() as u64;
        self.telemetry.stage_execution_count += report.stage_count as u64;
        self.telemetry.serial_executor_usage_count += 1;
        self.telemetry.max_tasks_in_stage = self.telemetry.max_tasks_in_stage.max(
            report
                .stages
                .iter()
                .map(|stage| stage.task_records.len() as u64)
                .max()
                .unwrap_or(0),
        );
    }

    fn stage_mark_dirty_candidates(&mut self, source: NodeId) -> Result<(), SignalError> {
        let mut stack = vec![source];
        let mut seen: BTreeSet<NodeId> = BTreeSet::new();
        while let Some(node) = stack.pop() {
            if !seen.insert(node) {
                continue;
            }
            if !self.graph.is_alive(node) {
                continue;
            }
            self.graph_patches.stage_original(self.graph, node)?;
            for &subscriber in self.graph.get_entry(node)?.get_subscribers() {
                stack.push(subscriber);
            }
        }
        Ok(())
    }

    fn stage_evaluate_candidates(&mut self, node: NodeId) -> Result<(), SignalError> {
        let mut stack = vec![node];
        let mut seen: BTreeSet<NodeId> = BTreeSet::new();
        while let Some(current) = stack.pop() {
            if !seen.insert(current) {
                continue;
            }
            if !self.graph.is_alive(current) {
                continue;
            }
            self.graph_patches.stage_original(self.graph, current)?;
            for dependency in self.graph.get_entry(current)?.get_dependencies() {
                stack.push(dependency.source());
            }
        }
        Ok(())
    }

    fn collect_dirty_targets(&self) -> Vec<NodeId> {
        let mut targets = Vec::new();
        for index in 0..self.graph.arena_capacity() {
            let Some(node) = self.graph.live_node_id_at(index) else {
                continue;
            };
            let Ok(entry) = self.graph.get_entry(node) else {
                continue;
            };
            if !matches!(entry.get_state(), crate::data::node::NodeState::Clean) {
                targets.push(node);
            }
        }
        targets
    }

    fn record_failure_from_error(
        &mut self,
        phase: ExecutionFailurePhase,
        err: &SignalError,
        plan_summary: Option<crate::logic::planner::PlanSummary>,
    ) {
        let summary = ExecutionFailureContext::from_error(phase, err, plan_summary)
            .summarize(None, self.graph.diagnostics_profile());
        self.pending_failure_summary = Some(summary.clone());
        DiagnosticsRecorder::new(self.graph).record_failure_summary(summary);
    }

    /// Commit transaction atomically into parent committed runtime.
    pub fn commit(mut self, runtime_ctx: &mut Ctx) -> Result<TransactionOutcome, SignalError> {
        if self.finished {
            return Err(SignalError::internal("transaction already finished"));
        }
        self.finished = true;

        if self.poisoned {
            self.event_bus.rollback(runtime_ctx);
            self.graph_patches.rollback_and_clear(self.graph)?;
            DiagnosticsRecorder::new(self.graph).restore_snapshot(self.diagnostics_snapshot.clone());
            let rollback = RollbackDiagnostic::new(
                true,
                self.graph_patches.touched_count() as u64,
                self.telemetry.max_touched_nodes_in_txn,
                Some("poisoned transaction rollback".to_string()),
            );
            DiagnosticsRecorder::new(self.graph).record_rollback(rollback.clone());
            let profile = self.graph.diagnostics_profile();
            DiagnosticsRecorder::new(self.graph).record_failure_summary(
                ExecutionFailureContext::new(
                    ExecutionFailurePhase::Rollback,
                    None,
                    None,
                    None,
                    None,
                    None,
                    "transaction rolled back because it was poisoned",
                )
                .summarize(Some(&rollback), profile),
            );
            self.telemetry.transaction_poison_count += 1;
            return Ok(TransactionOutcome::Poisoned);
        }

        self.staged_patch_count = self.graph_patches.touched_count() as u64;

        if let Err(err) = self
            .event_bus
            .begin(runtime_ctx)
            .map_err(|e| SignalError::invalid_input(format!("event bus begin failed: {e:?}")))
        {
            self.event_bus.rollback(runtime_ctx);
            self.graph_patches.rollback_and_clear(self.graph)?;
            DiagnosticsRecorder::new(self.graph).restore_snapshot(self.diagnostics_snapshot.clone());
            let rollback = RollbackDiagnostic::new(
                true,
                self.graph_patches.touched_count() as u64,
                self.telemetry.max_touched_nodes_in_txn,
                Some("event bus begin failed".to_string()),
            );
            DiagnosticsRecorder::new(self.graph).record_rollback(rollback.clone());
            let profile = self.graph.diagnostics_profile();
            DiagnosticsRecorder::new(self.graph).record_failure_summary(
                ExecutionFailureContext::from_error(
                    ExecutionFailurePhase::CommitPromotion,
                    &err,
                    None,
                )
                .summarize(Some(&rollback), profile),
            );
            self.telemetry.transaction_poison_count += 1;
            return Err(err);
        }
        for event in self.staged_events {
            self.event_bus.emit(event);
        }
        for barrier in self.staged_event_flushes {
            if let Err(err) = self
                .event_bus
                .flush(barrier, runtime_ctx)
                .map_err(|e| SignalError::invalid_input(format!("event bus flush failed: {e:?}")))
            {
                self.event_bus.rollback(runtime_ctx);
                self.graph_patches.rollback_and_clear(self.graph)?;
                DiagnosticsRecorder::new(self.graph)
                    .restore_snapshot(self.diagnostics_snapshot.clone());
                let rollback = RollbackDiagnostic::new(
                    true,
                    self.graph_patches.touched_count() as u64,
                    self.telemetry.max_touched_nodes_in_txn,
                    Some("event bus flush failed".to_string()),
                );
                DiagnosticsRecorder::new(self.graph).record_rollback(rollback.clone());
                let profile = self.graph.diagnostics_profile();
                DiagnosticsRecorder::new(self.graph).record_failure_summary(
                    ExecutionFailureContext::from_error(
                        ExecutionFailurePhase::CommitPromotion,
                        &err,
                        None,
                    )
                    .summarize(Some(&rollback), profile),
                );
                self.telemetry.transaction_poison_count += 1;
                return Err(err);
            }
        }

        while let Some(domain) = self.staged_dirty.first_dirty_domain() {
            if let Some(impact) = self.staged_dirty.take_domain_impact(domain) {
                self.checkpoint
                    .dirty_mut()
                    .merge_domain_impact(domain, impact);
            }
        }
        self.checkpoint.telemetry_mut().checkpoint_flushes += self.staged_checkpoint_flushes;
        self.checkpoint.telemetry_mut().checkpoint_flush_nanos +=
            self.staged_checkpoint_flush_nanos;
        for ((family, memo_key), result) in self.staged_memo_writes {
            self.config
                .store_memoized_result(&family, &memo_key, result);
        }
        self.graph_patches.commit_and_clear();
        self.telemetry.transaction_commit_count += 1;
        self.telemetry.staged_node_patch_count += self.staged_patch_count;
        self.telemetry.max_touched_nodes_in_txn = self
            .telemetry
            .max_touched_nodes_in_txn
            .max(self.staged_patch_count);

        Ok(TransactionOutcome::Committed)
    }

    /// Roll back staged state and keep parent committed state untouched.
    pub fn rollback(mut self, runtime_ctx: &mut Ctx) -> Result<TransactionOutcome, SignalError> {
        if self.finished {
            return Err(SignalError::internal("transaction already finished"));
        }
        self.finished = true;
        self.event_bus.rollback(runtime_ctx);
        self.graph_patches.rollback_and_clear(self.graph)?;
        DiagnosticsRecorder::new(self.graph).restore_snapshot(self.diagnostics_snapshot);
        self.telemetry.transaction_rollback_count += 1;
        let rollback = RollbackDiagnostic::new(
            true,
            self.graph_patches.touched_count() as u64,
            self.telemetry.max_touched_nodes_in_txn,
            Some(if self.poisoned {
                "poisoned transaction rollback".to_string()
            } else {
                "explicit rollback".to_string()
            }),
        );
        DiagnosticsRecorder::new(self.graph).record_rollback(rollback);
        if let Some(failure) = self.pending_failure_summary {
            DiagnosticsRecorder::new(self.graph).record_failure_summary(failure);
        }
        if self.poisoned {
            self.telemetry.transaction_poison_count += 1;
            return Ok(TransactionOutcome::Poisoned);
        }
        Ok(TransactionOutcome::RolledBack)
    }
}

/// Transaction-gated checkpoint flush helper.
pub fn flush_checkpoint_in_txn<'a, D, I, E, Ctx, T, Ev>(
    txn: &mut SignalTransaction<'a, D, I, E, Ctx, T>,
    barrier: CheckpointBarrier,
    evaluator: &mut Ev,
    ctx: &mut Ev::Context,
) -> Result<usize, SignalError>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
    Ev: CheckpointEvaluator<Domain = D, Impact = I>,
{
    txn.flush_checkpoint(barrier, evaluator, ctx)
}

/// Transaction-gated event emission helper.
pub fn emit_event_in_txn<'a, D, I, E, Ctx, T>(
    txn: &mut SignalTransaction<'a, D, I, E, Ctx, T>,
    event: E,
) where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    txn.emit_event(event);
}

fn collect_dirty_targets(graph: &SignalGraph) -> Vec<NodeId> {
    let mut targets = Vec::new();
    for index in 0..graph.arena_capacity() {
        let Some(node) = graph.live_node_id_at(index) else {
            continue;
        };
        let Ok(entry) = graph.get_entry(node) else {
            continue;
        };
        if !matches!(entry.get_state(), crate::data::node::NodeState::Clean) {
            targets.push(node);
        }
    }
    targets
}

fn empty_execution_report() -> ExecutionReport {
    ExecutionReport {
        plan_summary: crate::logic::planner::PlanSummary::default(),
        stage_count: 0,
        task_count: 0,
        tasks_executed: 0,
        tasks_pruned: 0,
        tasks_validated_clean: 0,
        tasks_deferred_by_condition: 0,
        tasks_reverted_clean_by_condition: 0,
        tasks_satisfied_by_memoization: 0,
        tasks_with_suppressed_propagation: 0,
        execution_snapshots_built: 0,
        prepared_evaluations_produced: 0,
        prepared_evaluations_applied: 0,
        dependency_capture_updates: 0,
        execution_snapshot_nanos: 0,
        stage_precompute_nanos: 0,
        stage_apply_nanos: 0,
        stages: Vec::new(),
    }
}
