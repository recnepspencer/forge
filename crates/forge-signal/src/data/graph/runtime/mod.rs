pub(crate) mod effect;
pub(crate) mod execution;
pub(crate) mod graph;
pub(crate) mod observer;
pub(crate) mod scratch;
pub(crate) mod strategy;

pub use graph::SignalGraph;
pub use observer::GraphObserver;
pub use scratch::ScratchLeaseKind;
pub use strategy::{EvaluationStrategy, GcPressure, ObservationLevel, ParallelismHint};
pub(crate) use scratch::TraversalScratch;
