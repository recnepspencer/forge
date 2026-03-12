use std::marker::PhantomData;

use crate::data::checkpoint::CheckpointBarrier;
use crate::data::checkpoint_policy::CheckpointPolicy;
use crate::data::comparator::VersionComparatorPolicy;
use crate::data::graph::SignalGraph;
use crate::data::tier::TierPolicy;
use crate::diagnostics::policy::SignalRuntimePolicy;
use crate::logic::checkpoint::CheckpointRuntime;
use crate::logic::events::EventBus;

use super::runtime_state::SignalRuntime;

/// Builder for `SignalRuntime`.
///
/// Start here if you want the full runtime surface with transactions,
/// checkpoint control, runtime policy, keyed nodes, and diagnostics.
pub struct SignalRuntimeBuilder<D = (), I = (), E = (), Ctx = (), T = ()>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    graph: SignalGraph,
    checkpoint_policy: CheckpointPolicy<D>,
    fallback_comparator: VersionComparatorPolicy,
    runtime_policy: SignalRuntimePolicy,
    tier_policies: Vec<TierPolicy<T>>,
    _marker: PhantomData<fn(I, E, Ctx, T)>,
}

impl<D, I, E, Ctx, T> SignalRuntimeBuilder<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(super) fn new(graph: SignalGraph) -> Self {
        Self {
            graph,
            checkpoint_policy: CheckpointPolicy::new(CheckpointBarrier::PerOperation),
            fallback_comparator: VersionComparatorPolicy::Exact,
            runtime_policy: SignalRuntimePolicy::default(),
            tier_policies: Vec::new(),
            _marker: PhantomData,
        }
    }

    /// Set a simple checkpoint barrier policy.
    ///
    /// This is the shortest path when you want standard checkpoint behavior
    /// without constructing a full `CheckpointPolicy`.
    pub fn checkpoint_barrier(mut self, barrier: CheckpointBarrier) -> Self {
        self.checkpoint_policy = CheckpointPolicy::new(barrier);
        self
    }

    /// Set the full checkpoint policy.
    pub fn checkpoint_policy(mut self, policy: CheckpointPolicy<D>) -> Self {
        self.checkpoint_policy = policy;
        self
    }

    /// Set the fallback comparator used when a node or tier does not provide one.
    pub fn fallback_comparator(mut self, comparator: VersionComparatorPolicy) -> Self {
        self.fallback_comparator = comparator;
        self
    }

    /// Set runtime observability and semantic retention policy.
    ///
    /// Use one of the named presets like `SignalRuntimePolicy::operational()`
    /// or `SignalRuntimePolicy::fintech()` unless you need a custom mix.
    pub fn runtime_policy(mut self, runtime_policy: SignalRuntimePolicy) -> Self {
        self.runtime_policy = runtime_policy;
        self
    }

    /// Seed an initial tier policy into runtime config before build completes.
    pub fn tier_policy(mut self, policy: TierPolicy<T>) -> Self {
        self.tier_policies.push(policy);
        self
    }

    /// Switch the runtime to a typed event payload.
    ///
    /// This is usually only needed once you start integrating an event bus.
    pub fn with_events<E2>(self) -> SignalRuntimeBuilder<D, I, E2, Ctx, T> {
        SignalRuntimeBuilder {
            graph: self.graph,
            checkpoint_policy: self.checkpoint_policy,
            fallback_comparator: self.fallback_comparator,
            runtime_policy: self.runtime_policy,
            tier_policies: self.tier_policies,
            _marker: PhantomData,
        }
    }

    /// Switch the runtime to a typed checkpoint domain key.
    pub fn with_domains<D2>(self) -> SignalRuntimeBuilder<D2, I, E, Ctx, T>
    where
        D2: Copy + Ord + std::fmt::Debug + 'static,
    {
        SignalRuntimeBuilder {
            graph: self.graph,
            checkpoint_policy: CheckpointPolicy::new(self.checkpoint_policy.barrier_for_default()),
            fallback_comparator: self.fallback_comparator,
            runtime_policy: self.runtime_policy,
            tier_policies: Vec::new(),
            _marker: PhantomData,
        }
    }

    /// Switch the runtime to a typed checkpoint impact key.
    pub fn with_impacts<I2>(self) -> SignalRuntimeBuilder<D, I2, E, Ctx, T>
    where
        I2: Copy + Ord,
    {
        SignalRuntimeBuilder {
            graph: self.graph,
            checkpoint_policy: self.checkpoint_policy,
            fallback_comparator: self.fallback_comparator,
            runtime_policy: self.runtime_policy,
            tier_policies: self.tier_policies,
            _marker: PhantomData,
        }
    }

    /// Enable typed node tiers for tier policy configuration.
    pub fn with_tiers<T2>(self) -> SignalRuntimeBuilder<D, I, E, Ctx, T2>
    where
        T2: Copy + Ord,
    {
        SignalRuntimeBuilder {
            graph: self.graph,
            checkpoint_policy: self.checkpoint_policy,
            fallback_comparator: self.fallback_comparator,
            runtime_policy: self.runtime_policy,
            tier_policies: Vec::new(),
            _marker: PhantomData,
        }
    }

    /// Switch the runtime to a typed external transaction/event context.
    pub fn with_context<Ctx2>(self) -> SignalRuntimeBuilder<D, I, E, Ctx2, T> {
        SignalRuntimeBuilder {
            graph: self.graph,
            checkpoint_policy: self.checkpoint_policy,
            fallback_comparator: self.fallback_comparator,
            runtime_policy: self.runtime_policy,
            tier_policies: self.tier_policies,
            _marker: PhantomData,
        }
    }

    /// Build the runtime.
    pub fn build(self) -> SignalRuntime<D, I, E, Ctx, T> {
        let checkpoint = CheckpointRuntime::new(self.checkpoint_policy);
        let event_bus = EventBus::new();
        let mut runtime = SignalRuntime::new(self.graph, checkpoint, event_bus);
        runtime.set_fallback_comparator(self.fallback_comparator);
        runtime.set_runtime_policy(self.runtime_policy);
        for policy in self.tier_policies {
            runtime.set_tier_policy(policy);
        }
        runtime
    }
}
