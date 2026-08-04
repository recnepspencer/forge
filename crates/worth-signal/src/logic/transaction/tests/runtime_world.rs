use crate::data::checkpoint::CheckpointBarrier;
use crate::logic::transaction::SignalRuntime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::logic::transaction::tests) enum Domain {
    Cache,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::logic::transaction::tests) enum Impact {
    One,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::logic::transaction::tests) enum Ev {
    Tick,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::logic::transaction::tests) enum Tier {
    A,
}

pub(in crate::logic::transaction::tests) fn build_runtime(
    graph: crate::data::graph::SignalGraph,
) -> SignalRuntime<Domain, Impact, Ev, (), Tier> {
    SignalRuntime::builder(graph)
        .with_kernel_defaults()
        .with_domains::<Domain>()
        .with_impacts::<Impact>()
        .with_events::<Ev>()
        .with_tiers::<Tier>()
        .checkpoint_barrier(CheckpointBarrier::PerOperation)
        .build()
}
