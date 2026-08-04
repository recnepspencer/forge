use crate::data::checkpoint::CheckpointBarrier;
use crate::facade::{SignalGraph, SignalRuntime};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::tests::observability) enum Domain {
    Cache,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::tests::observability) enum Impact {
    One,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::tests::observability) enum Ev {
    Tick,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::tests::observability) enum Tier {
    Slow,
}

pub(in crate::tests::observability) fn build_runtime(
    graph: SignalGraph,
) -> SignalRuntime<Domain, Impact, Ev, (), Tier> {
    let _ = Domain::Cache;
    let _ = Impact::One;
    SignalRuntime::builder(graph)
        .with_kernel_defaults()
        .with_domains::<Domain>()
        .with_impacts::<Impact>()
        .with_events::<Ev>()
        .with_tiers::<Tier>()
        .checkpoint_barrier(CheckpointBarrier::PerOperation)
        .build()
}
