//! Permanent assembly for independently borrowable Signal owner services.
//!
//! This gate owns shared service vocabulary only. Owner state, execution cells,
//! and callable ports enter as subordinate responsibilities in their assigned
//! implementation phases.

mod counters;
mod lifecycle_observation;
#[cfg(feature = "test-operation-control")]
mod operation_control;
mod unavailable;

pub use counters::SignalOwnerServiceCostSnapshot;
pub use lifecycle_observation::SignalOwnerLifecycleObservation;
#[cfg(feature = "test-operation-control")]
pub use operation_control::SignalOwnerOperationBoundary;
pub use unavailable::SignalOwnerUnavailable;
