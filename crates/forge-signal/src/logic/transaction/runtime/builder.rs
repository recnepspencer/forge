use std::marker::PhantomData;

use crate::data::checkpoint::CheckpointBarrier;
use crate::data::checkpoint_policy::CheckpointPolicy;
use crate::data::comparator::VersionComparatorPolicy;
use crate::data::graph::SignalGraph;

use super::runtime_state::SignalRuntime;

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
    pub(super) fn new(graph: SignalGraph) -> Self {
        Self {
            graph,
            checkpoint_policy: CheckpointPolicy::new(CheckpointBarrier::PerOperation),
            fallback_comparator: VersionComparatorPolicy::Exact,
            _marker: PhantomData,
        }
    }

    pub fn checkpoint_barrier(mut self, barrier: CheckpointBarrier) -> Self {
        self.checkpoint_policy = CheckpointPolicy::new(barrier);
        self
    }

    pub fn checkpoint_policy(mut self, policy: CheckpointPolicy<D>) -> Self {
        self.checkpoint_policy = policy;
        self
    }

    pub fn fallback_comparator(mut self, comparator: VersionComparatorPolicy) -> Self {
        self.fallback_comparator = comparator;
        self
    }

    pub fn with_events<E2>(self) -> SignalRuntimeBuilder<D, I, E2, Ctx, T> {
        SignalRuntimeBuilder {
            graph: self.graph,
            checkpoint_policy: self.checkpoint_policy,
            fallback_comparator: self.fallback_comparator,
            _marker: PhantomData,
        }
    }

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

    pub fn with_context<Ctx2>(self) -> SignalRuntimeBuilder<D, I, E, Ctx2, T> {
        SignalRuntimeBuilder {
            graph: self.graph,
            checkpoint_policy: self.checkpoint_policy,
            fallback_comparator: self.fallback_comparator,
            _marker: PhantomData,
        }
    }

    pub fn build(self) -> SignalRuntime<D, I, E, Ctx, T> {
        let mut runtime = SignalRuntime::with_policy(self.graph, self.checkpoint_policy);
        runtime.set_fallback_comparator(self.fallback_comparator);
        runtime
    }
}
