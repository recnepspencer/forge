mod admission;
mod denial;
mod grant;
mod scope;

pub use admission::PhysicalScopedAllocationAdmission;
pub use denial::PhysicalScopedAllocationFailure;
pub use grant::{
    BlobPhysicalAllocation, MaintenancePhysicalAllocation, RecoveryPhysicalAllocation,
    ScrubPhysicalAllocation, VerificationPhysicalAllocation,
};
