use std::collections::BTreeSet;
use std::time::Instant;

use crate::data::aspect::{Aspect, AspectVersion};
use crate::data::checkpoint::CheckpointBarrier;
use crate::data::checkpoint_policy::CheckpointPolicy;
use crate::data::comparator::{TierPolicyResolver, VersionComparatorPolicy, VersionComparatorResolver};
use crate::data::dirty_set::{BatchedDirtySet, DomainImpact};
use crate::data::effect_mapping::EffectMapping;
use crate::data::error::SignalError;
use crate::data::evaluator::CheckpointEvaluator;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::node_meta::NodeMetaStore;
use crate::data::telemetry::RuntimeTelemetry;
use crate::data::tier::TierPolicy;
use crate::data::tier_policy_table::TierPolicyTable;
use crate::logic::checkpoint::CheckpointRuntime;
use crate::logic::evaluation::{
    evaluate_with_policy_and_condition_resolvers, DefaultConditionResolver, EvaluationRequestMode,
};
use crate::logic::events::EventBus;
use crate::logic::invalidation::mark_dirty;

use super::patch_buffer::SparsePatchBuffer;

/// Immutable or near-immutable runtime configuration shared across transactions.
#[derive(Debug, Clone)]
pub struct SignalRuntimeConfig<T: Copy + Ord> {
    node_meta: NodeMetaStore<T>,
    tier_policies: TierPolicyTable<T>,
    fallback_comparator: VersionComparatorPolicy,
}

impl<T: Copy + Ord> Default for SignalRuntimeConfig<T> {
    fn default() -> Self {
        Self {
            node_meta: NodeMetaStore::default(),
            tier_policies: TierPolicyTable::default(),
            fallback_comparator: VersionComparatorPolicy::Exact,
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
}

/// Transaction runtime that owns committed signal components.
pub struct SignalRuntimeState<D, I, E, Ctx, T = ()>
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

impl<D, I, E, Ctx, T> SignalRuntimeState<D, I, E, Ctx, T>
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

    /// Convenience constructor with fresh checkpoint/event runtimes.
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

    /// Begin a transaction scope over committed runtime state.
    pub fn begin<'a>(&'a mut self) -> SignalTransaction<'a, D, I, E, Ctx, T> {
        self.telemetry.transaction_begin_count += 1;
        self.config.sync_graph_capacity(&self.graph);
        SignalTransaction {
            config: &self.config,
            graph: &mut self.graph,
            checkpoint: &mut self.checkpoint,
            event_bus: &mut self.event_bus,
            telemetry: &mut self.telemetry,
            staged_dirty: BatchedDirtySet::new(),
            staged_checkpoint_flushes: 0,
            staged_checkpoint_flush_nanos: 0,
            staged_events: Vec::new(),
            staged_event_flushes: Vec::new(),
            graph_patches: SparsePatchBuffer::new(),
            poisoned: false,
            finished: false,
            staged_patch_count: 0,
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
    config: &'a SignalRuntimeConfig<T>,
    graph: &'a mut SignalGraph,
    checkpoint: &'a mut CheckpointRuntime<D, I>,
    event_bus: &'a mut EventBus<E, D, Ctx>,
    telemetry: &'a mut RuntimeTelemetry,
    staged_dirty: BatchedDirtySet<D, I>,
    staged_checkpoint_flushes: u64,
    staged_checkpoint_flush_nanos: u128,
    staged_events: Vec<E>,
    staged_event_flushes: Vec<CheckpointBarrier>,
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
    pub fn mark_dirty(&mut self, source: NodeId, changed_aspect: Aspect) -> Result<(), SignalError> {
        self.stage_mark_dirty_candidates(source)?;
        let result = mark_dirty(self.graph, source, changed_aspect);
        self.apply_result(result)
    }

    /// Evaluate one node in staged graph with tier-aware comparator inheritance.
    pub fn evaluate<F, R>(
        &mut self,
        node: NodeId,
        compute: &mut F,
        custom_resolver: R,
    ) -> Result<(), SignalError>
    where
        F: FnMut(NodeId, &SignalGraph) -> Result<AspectVersion, SignalError>,
        R: VersionComparatorResolver,
    {
        self.evaluate_with_mode(
            node,
            compute,
            custom_resolver,
            EvaluationRequestMode::Default,
        )
    }

    /// Evaluate one node in staged graph with explicit request mode.
    pub fn evaluate_with_mode<F, R>(
        &mut self,
        node: NodeId,
        compute: &mut F,
        custom_resolver: R,
        request_mode: EvaluationRequestMode,
    ) -> Result<(), SignalError>
    where
        F: FnMut(NodeId, &SignalGraph) -> Result<AspectVersion, SignalError>,
        R: VersionComparatorResolver,
    {
        self.stage_evaluate_candidates(node)?;
        let mut resolver = TierPolicyResolver::new(
            self.config.node_meta(),
            self.config.tier_policies(),
            self.config.fallback_comparator(),
        )
        .with_custom_resolver(custom_resolver);
        let mut condition_resolver = DefaultConditionResolver;
        let result = evaluate_with_policy_and_condition_resolvers(
            self.graph,
            node,
            compute,
            &mut resolver,
            &mut condition_resolver,
            request_mode,
        );
        self.apply_result(result)
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
        self.checkpoint.telemetry_mut().checkpoint_flush_nanos += self.staged_checkpoint_flush_nanos;
        self.graph_patches.commit_and_clear();
        self.telemetry.transaction_commit_count += 1;
        self.telemetry.staged_node_patch_count += self.staged_patch_count;
        self.telemetry.max_touched_nodes_in_txn =
            self.telemetry.max_touched_nodes_in_txn.max(self.staged_patch_count);

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
    txn.evaluate(node, compute, custom_resolver)
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
    txn.evaluate_with_mode(node, compute, custom_resolver, request_mode)
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
