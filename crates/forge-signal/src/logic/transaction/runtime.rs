use std::collections::{BTreeMap, BTreeSet};

use crate::data::aspect::Aspect;
use crate::data::checkpoint::CheckpointBarrier;
use crate::data::checkpoint_policy::CheckpointPolicy;
use crate::data::comparator::{TierPolicyResolver, VersionComparatorPolicy, VersionComparatorResolver};
use crate::data::effect_mapping::EffectMapping;
use crate::data::error::SignalError;
use crate::data::evaluator::CheckpointEvaluator;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::telemetry::RuntimeTelemetry;
use crate::data::tier::TierPolicy;
use crate::logic::checkpoint::CheckpointRuntime;
use crate::logic::evaluation::evaluate_with_policy_resolver;
use crate::logic::events::EventBus;
use crate::logic::invalidation::mark_dirty;
use super::patch_buffer::SparsePatchBuffer;

/// Transaction runtime that owns committed signal components.
pub struct SignalTransactionRuntime<D, I, E, Ctx, T = ()>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    graph: SignalGraph,
    checkpoint: CheckpointRuntime<D, I>,
    event_bus: EventBus<E, D, Ctx>,
    node_tiers: BTreeMap<NodeId, T>,
    tier_policies: BTreeMap<T, TierPolicy<T>>,
    fallback_comparator: VersionComparatorPolicy,
    telemetry: RuntimeTelemetry,
}

impl<D, I, E, Ctx, T> SignalTransactionRuntime<D, I, E, Ctx, T>
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
        Self {
            graph,
            checkpoint,
            event_bus,
            node_tiers: BTreeMap::new(),
            tier_policies: BTreeMap::new(),
            fallback_comparator: VersionComparatorPolicy::Exact,
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

    /// Immutable access to committed signal graph.
    pub fn graph(&self) -> &SignalGraph {
        &self.graph
    }

    /// Mutable access to committed signal graph.
    ///
    /// Compatibility path during migration to fully transaction-gated writes.
    #[deprecated(note = "Use SignalTransactionRuntime::begin() and mutate through SignalTransaction")]
    pub fn graph_mut(&mut self) -> &mut SignalGraph {
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
        self.node_tiers.insert(node, tier);
    }

    /// Register/update one tier policy.
    pub fn set_tier_policy(&mut self, policy: TierPolicy<T>) {
        self.tier_policies.insert(policy.tier, policy);
    }

    /// Set global comparator fallback.
    pub fn set_fallback_comparator(&mut self, policy: VersionComparatorPolicy) {
        self.fallback_comparator = policy;
    }

    /// Begin a transaction scope over committed runtime state.
    pub fn begin<'a>(&'a mut self) -> SignalTransaction<'a, D, I, E, Ctx, T> {
        self.telemetry.transaction_begin_count += 1;
        let staged_checkpoint = self.checkpoint.clone();
        let node_tiers = self.node_tiers.clone();
        let tier_policies = self.tier_policies.clone();
        let fallback_comparator = self.fallback_comparator.clone();
        SignalTransaction {
            parent: self,
            staged_checkpoint,
            staged_events: Vec::new(),
            staged_event_flushes: Vec::new(),
            graph_patches: SparsePatchBuffer::new(),
            node_tiers,
            tier_policies,
            fallback_comparator,
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
    parent: &'a mut SignalTransactionRuntime<D, I, E, Ctx, T>,
    staged_checkpoint: CheckpointRuntime<D, I>,
    staged_events: Vec<E>,
    staged_event_flushes: Vec<CheckpointBarrier>,
    graph_patches: SparsePatchBuffer,
    node_tiers: BTreeMap<NodeId, T>,
    tier_policies: BTreeMap<T, TierPolicy<T>>,
    fallback_comparator: VersionComparatorPolicy,
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
        &self.parent.graph
    }

    /// Mutable view of staged graph.
    pub fn staged_graph_mut(&mut self) -> &mut SignalGraph {
        &mut self.parent.graph
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
        self.staged_checkpoint.record_effect::<M>(effect);
    }

    /// Mark one source dirty in staged graph.
    pub fn mark_dirty(&mut self, source: NodeId, changed_aspect: Aspect) -> Result<(), SignalError> {
        self.stage_mark_dirty_candidates(source)?;
        let result = mark_dirty(&mut self.parent.graph, source, changed_aspect);
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
        F: FnMut(NodeId, &SignalGraph) -> Result<crate::data::aspect::AspectVersion, SignalError>,
        R: VersionComparatorResolver,
    {
        self.stage_evaluate_candidates(node)?;
        let mut resolver = TierPolicyResolver::new(
            self.node_tiers.clone(),
            self.tier_policies.clone(),
            self.fallback_comparator.clone(),
        )
        .with_custom_resolver(custom_resolver);
        let result = evaluate_with_policy_resolver(
            &mut self.parent.graph,
            node,
            compute,
            &mut resolver,
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
        let result = self.staged_checkpoint.flush(barrier, evaluator, ctx);
        self.apply_result(result)
    }

    /// Flush staged event bus at the specified barrier.
    pub fn flush_events(
        &mut self,
        barrier: CheckpointBarrier,
    ) -> Result<(), SignalError> {
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
            if !self.parent.graph.is_alive(node) {
                continue;
            }
            self.graph_patches.stage_original(&self.parent.graph, node)?;
            let subs: Vec<NodeId> = self
                .parent
                .graph
                .get_entry(node)?
                .get_subscribers()
                .iter()
                .copied()
                .collect();
            stack.extend(subs);
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
            if !self.parent.graph.is_alive(current) {
                continue;
            }
            self.graph_patches.stage_original(&self.parent.graph, current)?;
            let deps: Vec<NodeId> = self
                .parent
                .graph
                .get_entry(current)?
                .get_dependencies()
                .iter()
                .map(|d| d.source())
                .collect();
            stack.extend(deps);
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
            self.parent.telemetry.transaction_poison_count += 1;
            return Ok(TransactionOutcome::Poisoned);
        }

        self.staged_patch_count = self.graph_patches.touched_count() as u64;

        // Apply staged event lifecycle first; if this fails, rewind event bus and
        // preserve committed graph/checkpoint state.
        if let Err(err) = self
            .parent
            .event_bus
            .begin(runtime_ctx)
            .map_err(|e| SignalError::invalid_input(format!("event bus begin failed: {e:?}")))
        {
            self.parent.event_bus.rollback(runtime_ctx);
            self.parent.telemetry.transaction_poison_count += 1;
            return Err(err);
        }
        for event in self.staged_events {
            self.parent.event_bus.emit(event);
        }
        for barrier in self.staged_event_flushes {
            if let Err(err) = self
                .parent
                .event_bus
                .flush(barrier, runtime_ctx)
                .map_err(|e| SignalError::invalid_input(format!("event bus flush failed: {e:?}")))
            {
                self.parent.event_bus.rollback(runtime_ctx);
                self.parent.telemetry.transaction_poison_count += 1;
                return Err(err);
            }
        }

        self.parent.checkpoint = self.staged_checkpoint;
        self.graph_patches.commit_and_clear();
        self.parent.telemetry.transaction_commit_count += 1;
        self.parent.telemetry.staged_node_patch_count += self.staged_patch_count;

        Ok(TransactionOutcome::Committed)
    }

    /// Roll back staged state and keep parent committed state untouched.
    pub fn rollback(mut self, runtime_ctx: &mut Ctx) -> Result<TransactionOutcome, SignalError> {
        if self.finished {
            return Err(SignalError::internal("transaction already finished"));
        }
        self.finished = true;
        self.parent.event_bus.rollback(runtime_ctx);
        self.graph_patches.rollback_and_clear(&mut self.parent.graph)?;
        self.parent.telemetry.transaction_rollback_count += 1;
        if self.poisoned {
            self.parent.telemetry.transaction_poison_count += 1;
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
    F: FnMut(NodeId, &SignalGraph) -> Result<crate::data::aspect::AspectVersion, SignalError>,
    R: VersionComparatorResolver,
{
    txn.evaluate(node, compute, custom_resolver)
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
