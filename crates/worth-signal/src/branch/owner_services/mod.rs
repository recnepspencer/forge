//! Permanent assembly for independently borrowable Signal owner services.
//!
//! This module exposes the frozen public vocabulary and assembles the private
//! synchronization and capacity kernel consumed by later service phases.

mod basis_port;
mod branch_cell_state;
mod branch_execution_cell;
mod branch_registry;
mod cancellation;
mod cell_incarnation;
mod counters;
mod lifecycle_observation;
mod lifecycle_port;
mod lifecycle_state;
mod mutation_port;
#[cfg(feature = "test-operation-control")]
mod operation_control;
mod owner;
mod owner_metadata;
mod unavailable;

pub(crate) use cancellation::SignalOwnerMovementPermit;

pub(crate) use basis_port::SignalBranchBasisPort;
pub(crate) use branch_cell_state::SignalBranchCellState;
pub(crate) use branch_execution_cell::{
    SignalBranchCellAdmissionDenial, SignalBranchCellPoisonRecovery, SignalBranchCellWork,
    SignalBranchExecutionCell,
};
pub(crate) use branch_registry::{
    SignalBranchRegistry, SignalBranchRegistryDenial, SignalBranchRegistryPoisonRecovery,
    SignalBranchRetirement,
};
pub(crate) use counters::SignalOwnerServiceCounters;
pub(crate) use lifecycle_port::SignalBranchLifecyclePort;
pub(crate) use lifecycle_state::{
    SignalOwnerAdmissionDenial, SignalOwnerLifecyclePoisonRecovery, SignalOwnerLifecycleState,
    SignalOwnerOperationAdmission,
};
pub(crate) use mutation_port::SignalBranchMutationPort;
pub(crate) use owner::{
    SignalOwner, SignalOwnerRoot, SignalOwnerServiceIssuanceDenial,
    DEFAULT_MAXIMUM_LIVE_SIGNAL_BRANCHES,
};

pub use cancellation::{SignalOwnerCancellationSource, SignalOwnerCancellationToken};
pub use counters::SignalOwnerServiceCostSnapshot;
pub use lifecycle_observation::SignalOwnerLifecycleObservation;
#[cfg(feature = "test-operation-control")]
pub use operation_control::SignalOwnerOperationBoundary;
pub use unavailable::SignalOwnerUnavailable;

#[cfg(test)]
mod tests;
