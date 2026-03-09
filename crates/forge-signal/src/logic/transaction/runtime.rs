use std::collections::{BTreeMap, BTreeSet};
use std::marker::PhantomData;
use std::time::Instant;

use crate::data::aspect::{Aspect, AspectVersion};
use crate::data::checkpoint::CheckpointBarrier;
use crate::data::checkpoint_policy::CheckpointPolicy;
use crate::data::comparator::{
    DefaultComparatorResolver, TierPolicyResolver, VersionComparatorPolicy,
    VersionComparatorResolver,
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
    ComputationFamily, ComputationKey, IntoNodeEvaluationResult, KeyedComputation,
    NodeEvaluationResult, StructuralMemoKey,
};
use crate::data::telemetry::RuntimeTelemetry;
use crate::data::tier::TierPolicy;
use crate::data::tier_policy_table::TierPolicyTable;
use crate::logic::checkpoint::CheckpointRuntime;
use crate::logic::evaluation::{
    apply_evaluation_result_with_policy_and_condition, DefaultConditionResolver,
    EvaluationExecutionMetadata, EvaluationRequestMode,
};
use crate::logic::events::EventBus;
use crate::logic::explain::{explain_with_policy_resolver, NodeExplanation};
use crate::logic::invalidation::{mark_dirty, mark_dirty_with_regions};
use crate::logic::planner::{
    build_evaluation_plan_with_policy_resolver, execute_plan_with_policy_and_condition,
    execute_prepared_plan_with_policy, EvaluationPlan, ExecutionReport, StageExecutor,
};
use crate::logic::prepared::{ExecutionReadView, PreparedEvaluation};
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

    /// Execute one pre-built plan with the runtime's comparator policy.
    pub fn execute_plan<F, O>(
        &mut self,
        plan: &EvaluationPlan,
        compute: &mut F,
    ) -> Result<ExecutionReport, SignalError>
    where
        F: FnMut(NodeId, &SignalGraph) -> Result<O, SignalError>,
        O: IntoNodeEvaluationResult,
    {
        self.execute_plan_with_executor(plan, compute, StageExecutor::Serial)
    }

    /// Execute one pre-built plan with an explicit stage executor.
    pub fn execute_plan_with_executor<F, O>(
        &mut self,
        plan: &EvaluationPlan,
        compute: &mut F,
        executor: StageExecutor,
    ) -> Result<ExecutionReport, SignalError>
    where
        F: FnMut(NodeId, &SignalGraph) -> Result<O, SignalError>,
        O: IntoNodeEvaluationResult,
    {
        let mut resolver = TierPolicyResolver::new(
            self.config.node_meta(),
            self.config.tier_policies(),
            self.config.fallback_comparator(),
        );
        let mut condition_resolver = DefaultConditionResolver;
        let report = execute_plan_with_policy_and_condition(
            &mut self.graph,
            plan,
            compute,
            &mut resolver,
            &mut condition_resolver,
            executor,
            None,
        )?;
        self.absorb_execution_report_telemetry(&report);
        Ok(report)
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

    /// Convenience evaluation path that builds and executes a plan for one target.
    pub fn evaluate_with_plan<F, O>(
        &mut self,
        node: NodeId,
        compute: &mut F,
        request_mode: EvaluationRequestMode,
    ) -> Result<ExecutionReport, SignalError>
    where
        F: FnMut(NodeId, &SignalGraph) -> Result<O, SignalError>,
        O: IntoNodeEvaluationResult,
    {
        let plan = self.build_evaluation_plan(&[node], request_mode)?;
        self.execute_plan(&plan, compute)
    }

    /// Convenience evaluation path that builds and executes a plan with an explicit executor.
    pub fn evaluate_with_plan_and_executor<F, O>(
        &mut self,
        node: NodeId,
        compute: &mut F,
        request_mode: EvaluationRequestMode,
        executor: StageExecutor,
    ) -> Result<ExecutionReport, SignalError>
    where
        F: FnMut(NodeId, &SignalGraph) -> Result<O, SignalError>,
        O: IntoNodeEvaluationResult,
    {
        let plan = self.build_evaluation_plan(&[node], request_mode)?;
        self.execute_plan_with_executor(&plan, compute, executor)
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

    /// Evaluate one node in staged graph with the default comparator resolver.
    pub fn evaluate<F, O>(&mut self, node: NodeId, compute: &mut F) -> Result<(), SignalError>
    where
        F: FnMut(NodeId, &SignalGraph) -> Result<O, SignalError>,
        O: IntoNodeEvaluationResult,
    {
        self.evaluate_with_mode(node, compute, EvaluationRequestMode::Default)
    }

    /// Evaluate one node in staged graph with tier-aware comparator inheritance.
    pub fn evaluate_with_resolver<F, O, R>(
        &mut self,
        node: NodeId,
        compute: &mut F,
        custom_resolver: R,
    ) -> Result<(), SignalError>
    where
        F: FnMut(NodeId, &SignalGraph) -> Result<O, SignalError>,
        O: IntoNodeEvaluationResult,
        R: VersionComparatorResolver,
    {
        self.evaluate_with_mode_and_resolver(
            node,
            compute,
            custom_resolver,
            EvaluationRequestMode::Default,
        )
    }

    /// Evaluate one node in staged graph with explicit request mode and the default comparator resolver.
    pub fn evaluate_with_mode<F, O>(
        &mut self,
        node: NodeId,
        compute: &mut F,
        request_mode: EvaluationRequestMode,
    ) -> Result<(), SignalError>
    where
        F: FnMut(NodeId, &SignalGraph) -> Result<O, SignalError>,
        O: IntoNodeEvaluationResult,
    {
        self.evaluate_with_mode_and_resolver(node, compute, DefaultComparatorResolver, request_mode)
    }

    /// Evaluate one node in staged graph with explicit request mode.
    pub fn evaluate_with_mode_and_resolver<F, O, R>(
        &mut self,
        node: NodeId,
        compute: &mut F,
        custom_resolver: R,
        request_mode: EvaluationRequestMode,
    ) -> Result<(), SignalError>
    where
        F: FnMut(NodeId, &SignalGraph) -> Result<O, SignalError>,
        O: IntoNodeEvaluationResult,
        R: VersionComparatorResolver,
    {
        self.stage_evaluate_candidates(node)?;
        let mut resolver = TierPolicyResolver::new(
            self.config.node_meta(),
            self.config.tier_policies(),
            self.config.fallback_comparator(),
        )
        .with_custom_resolver(custom_resolver);
        let plan = build_evaluation_plan_with_policy_resolver(
            self.graph,
            &[node],
            request_mode,
            &mut resolver,
        )?;
        let mut condition_resolver = DefaultConditionResolver;
        let report = execute_plan_with_policy_and_condition(
            self.graph,
            &plan,
            compute,
            &mut resolver,
            &mut condition_resolver,
            StageExecutor::Serial,
            None,
        )?;
        self.absorb_execution_report_telemetry(&report);
        Ok(())
    }

    /// Convenience evaluation path that builds and executes a plan for one target.
    pub fn evaluate_with_plan<F, O>(
        &mut self,
        node: NodeId,
        compute: &mut F,
        request_mode: EvaluationRequestMode,
    ) -> Result<ExecutionReport, SignalError>
    where
        F: FnMut(NodeId, &SignalGraph) -> Result<O, SignalError>,
        O: IntoNodeEvaluationResult,
    {
        self.evaluate_with_plan_and_executor(node, compute, request_mode, StageExecutor::Serial)
    }

    /// Convenience evaluation path that builds and executes a plan with an explicit executor.
    pub fn evaluate_with_plan_and_executor<F, O>(
        &mut self,
        node: NodeId,
        compute: &mut F,
        request_mode: EvaluationRequestMode,
        executor: StageExecutor,
    ) -> Result<ExecutionReport, SignalError>
    where
        F: FnMut(NodeId, &SignalGraph) -> Result<O, SignalError>,
        O: IntoNodeEvaluationResult,
    {
        self.stage_evaluate_candidates(node)?;
        let mut resolver = TierPolicyResolver::new(
            self.config.node_meta(),
            self.config.tier_policies(),
            self.config.fallback_comparator(),
        );
        let plan = build_evaluation_plan_with_policy_resolver(
            self.graph,
            &[node],
            request_mode,
            &mut resolver,
        )?;
        let mut condition_resolver = DefaultConditionResolver;
        let report = execute_plan_with_policy_and_condition(
            self.graph,
            &plan,
            compute,
            &mut resolver,
            &mut condition_resolver,
            executor,
            None,
        )?;
        self.absorb_execution_report_telemetry(&report);
        Ok(report)
    }

    /// Convenience prepared-evaluation path that builds and executes a plan for one target.
    pub fn evaluate_prepared_with_plan<F>(
        &mut self,
        node: NodeId,
        precompute: &F,
        request_mode: EvaluationRequestMode,
    ) -> Result<ExecutionReport, SignalError>
    where
        F: Fn(NodeId, &ExecutionReadView<'_>) -> Result<PreparedEvaluation, SignalError> + Sync,
    {
        self.evaluate_prepared_with_plan_and_executor(
            node,
            precompute,
            request_mode,
            StageExecutor::Serial,
        )
    }

    /// Convenience prepared-evaluation path with an explicit executor.
    pub fn evaluate_prepared_with_plan_and_executor<F>(
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
        let plan = build_evaluation_plan_with_policy_resolver(
            self.graph,
            &[node],
            request_mode,
            &mut resolver,
        )?;
        self.execute_prepared_plan_with_executor(&plan, precompute, executor)
    }

    /// Execute one pre-built plan against the staged graph with an explicit executor.
    pub fn execute_plan_with_executor<F, O>(
        &mut self,
        plan: &EvaluationPlan,
        compute: &mut F,
        executor: StageExecutor,
    ) -> Result<ExecutionReport, SignalError>
    where
        F: FnMut(NodeId, &SignalGraph) -> Result<O, SignalError>,
        O: IntoNodeEvaluationResult,
    {
        let mut resolver = TierPolicyResolver::new(
            self.config.node_meta(),
            self.config.tier_policies(),
            self.config.fallback_comparator(),
        );
        let mut condition_resolver = DefaultConditionResolver;
        let report = execute_plan_with_policy_and_condition(
            self.graph,
            plan,
            compute,
            &mut resolver,
            &mut condition_resolver,
            executor,
            None,
        )?;
        self.absorb_execution_report_telemetry(&report);
        Ok(report)
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
        let report = execute_prepared_plan_with_policy(
            self.graph,
            plan,
            precompute,
            &mut resolver,
            executor,
        )?;
        self.absorb_execution_report_telemetry(&report);
        Ok(report)
    }

    /// Evaluate one keyed computation with optional structural memoization.
    pub fn evaluate_keyed<F, O>(
        &mut self,
        node: NodeId,
        computation: &KeyedComputation,
        compute: &mut F,
    ) -> Result<(), SignalError>
    where
        F: FnMut(NodeId, &SignalGraph) -> Result<O, SignalError>,
        O: IntoNodeEvaluationResult,
    {
        self.evaluate_keyed_with_mode(node, computation, compute, EvaluationRequestMode::Default)
    }

    /// Evaluate one keyed computation with explicit request mode.
    pub fn evaluate_keyed_with_mode<F, O>(
        &mut self,
        node: NodeId,
        computation: &KeyedComputation,
        compute: &mut F,
        request_mode: EvaluationRequestMode,
    ) -> Result<(), SignalError>
    where
        F: FnMut(NodeId, &SignalGraph) -> Result<O, SignalError>,
        O: IntoNodeEvaluationResult,
    {
        self.telemetry.keyed_evaluation_count += 1;
        self.stage_evaluate_candidates(node)?;
        let mut resolver = TierPolicyResolver::new(
            self.config.node_meta(),
            self.config.tier_policies(),
            self.config.fallback_comparator(),
        );
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
                let metadata = EvaluationExecutionMetadata::from_keyed(
                    computation,
                    crate::data::output::MemoizedResultOrigin::MemoizedFromCache,
                );
                let mut condition_resolver = DefaultConditionResolver;
                let result = apply_evaluation_result_with_policy_and_condition(
                    self.graph,
                    node,
                    cached,
                    &mut resolver,
                    &mut condition_resolver,
                    &metadata,
                    false,
                );
                return self.apply_result(result);
            }
            self.telemetry.memoization_misses += 1;
        }

        let metadata = EvaluationExecutionMetadata::from_keyed(
            computation,
            crate::data::output::MemoizedResultOrigin::DirectCompute,
        );
        let mut last_result = None;
        let mut wrapped = |current: NodeId, graph: &SignalGraph| {
            let result = compute(current, graph)?.into_evaluation_result();
            last_result = Some(result.clone());
            Ok(result)
        };
        let plan = build_evaluation_plan_with_policy_resolver(
            self.graph,
            &[node],
            request_mode,
            &mut resolver,
        )?;
        let mut condition_resolver = DefaultConditionResolver;
        let result = execute_plan_with_policy_and_condition(
            self.graph,
            &plan,
            &mut wrapped,
            &mut resolver,
            &mut condition_resolver,
            StageExecutor::Serial,
            Some(&metadata),
        );
        let result = match result {
            Ok(report) => {
                self.absorb_execution_report_telemetry(&report);
                self.apply_result(Ok(()))
            }
            Err(err) => self.apply_result(Err(err)),
        };
        if result.is_ok() {
            if let (Some(memo_key), Some(last_result)) =
                (computation.memo_key.as_ref(), last_result)
            {
                self.staged_memo_writes
                    .insert((computation.family.clone(), memo_key.clone()), last_result);
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

    /// Commit transaction atomically into parent committed runtime.
    pub fn commit(mut self, runtime_ctx: &mut Ctx) -> Result<TransactionOutcome, SignalError> {
        if self.finished {
            return Err(SignalError::internal("transaction already finished"));
        }
        self.finished = true;

        if self.poisoned {
            self.event_bus.rollback(runtime_ctx);
            self.graph_patches.rollback_and_clear(self.graph)?;
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
        self.telemetry.transaction_rollback_count += 1;
        if self.poisoned {
            self.telemetry.transaction_poison_count += 1;
            return Ok(TransactionOutcome::Poisoned);
        }
        Ok(TransactionOutcome::RolledBack)
    }
}

/// Transaction-gated evaluate helper.
pub fn evaluate_in_txn<'a, D, I, E, Ctx, T, F, R>(
    txn: &mut SignalTransaction<'a, D, I, E, Ctx, T>,
    node: NodeId,
    compute: &mut F,
    custom_resolver: R,
) -> Result<(), SignalError>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
    F: FnMut(NodeId, &SignalGraph) -> Result<AspectVersion, SignalError>,
    R: VersionComparatorResolver,
{
    txn.evaluate_with_resolver(node, compute, custom_resolver)
}

/// Transaction-gated evaluate helper with explicit request mode.
pub fn evaluate_in_txn_with_mode<'a, D, I, E, Ctx, T, F, R>(
    txn: &mut SignalTransaction<'a, D, I, E, Ctx, T>,
    node: NodeId,
    compute: &mut F,
    custom_resolver: R,
    request_mode: EvaluationRequestMode,
) -> Result<(), SignalError>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
    F: FnMut(NodeId, &SignalGraph) -> Result<AspectVersion, SignalError>,
    R: VersionComparatorResolver,
{
    txn.evaluate_with_mode_and_resolver(node, compute, custom_resolver, request_mode)
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
