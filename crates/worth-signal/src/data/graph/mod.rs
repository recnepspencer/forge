mod compaction;
mod construction;
mod diagnostics_access;
mod lifecycle;
mod runtime;
pub(crate) mod storage;
mod topology;

pub(crate) use construction::node_builder;
pub use construction::NodeBuilder;
pub(crate) use runtime::graph as signal_graph;
pub(crate) use runtime::scratch;
pub use runtime::ScratchLeaseKind;
pub(crate) use runtime::TraversalScratch;
#[cfg_attr(not(feature = "parallel"), allow(unused_imports))]
pub(crate) use runtime::{ApplyCommitPacket, PreparedParallelApplyCommitPacket};
#[allow(unused_imports)]
pub(crate) use runtime::{BranchMutationRecord, BranchStructuralDelta};
pub use runtime::{
    EvaluationStrategy, GcPressure, GraphMaterializer, GraphObserver, ObservationLevel,
    ParallelismHint,
};
pub use runtime::{SignalGraph, SignalGraphLifecycleProbe};
#[cfg(test)]
pub(crate) use storage::checked_segment_component_for_test;
pub(crate) use storage::{
    DependencyEdgeStore, DependencySetId, SubscriberEdgeStore, SubscriberSetId,
};
