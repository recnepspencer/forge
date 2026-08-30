//! Permanent assembly for independently borrowable Signal owner services.
//!
//! This module exposes the frozen public vocabulary and assembles the private
//! synchronization and capacity kernel consumed by later service phases.

mod branch_execution_cell;
mod branch_registry;
mod counters;
mod lifecycle_observation;
mod lifecycle_state;
#[cfg(feature = "test-operation-control")]
mod operation_control;
mod unavailable;

pub(crate) use branch_execution_cell::{
    SignalBranchCellAdmissionDenial, SignalBranchCellWork, SignalBranchExecutionCell,
};
pub(crate) use branch_registry::{
    SignalBranchRegistry, SignalBranchRegistryDenial, SignalBranchReservation,
};
pub(crate) use counters::SignalOwnerServiceCounters;
pub(crate) use lifecycle_state::{
    SignalOwnerAdmissionDenial, SignalOwnerCloseDenial, SignalOwnerLifecycleIdentity,
    SignalOwnerLifecycleState, SignalOwnerOperationAdmission,
};

pub use counters::SignalOwnerServiceCostSnapshot;
pub use lifecycle_observation::SignalOwnerLifecycleObservation;
#[cfg(feature = "test-operation-control")]
pub use operation_control::SignalOwnerOperationBoundary;
pub use unavailable::SignalOwnerUnavailable;

#[cfg(test)]
mod tests;
