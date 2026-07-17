mod control;
mod fault;
mod reach;
mod trace;

#[cfg(feature = "certification-test-authority")]
pub use control::ProcessCrashStorageBoundaryControl;
pub use control::{
    ProductionStorageBoundaryControl, ScriptedStorageBoundaryControl,
    StorageBoundaryExecutionIdentity, UninterruptedStorageBoundaryControl,
};
pub use fault::{StorageBoundaryFault, StorageBoundaryRegion};
pub use reach::reach_storage_boundary;
pub use trace::StorageBoundaryTrace;
