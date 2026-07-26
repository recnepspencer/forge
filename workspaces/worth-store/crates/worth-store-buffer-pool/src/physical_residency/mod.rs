mod denial;
mod frame_access;
mod lease;
mod limits;
mod observation;
mod operation_allocation;
mod pool;
mod pressure;
mod work_kind;

pub use denial::PhysicalResidencyDenial;
pub use frame_access::{
    PhysicalBoundedFrameAccess, PhysicalBoundedFrameFaultOwner, PhysicalBoundedFrameFaultWaiter,
    PhysicalFrameAccess, PhysicalFrameFaultError, PhysicalFrameFaultOwner,
    PhysicalFrameFaultWaiter, PhysicalFrameLoadTerminal, PhysicalFrameLoadTerminalKind,
    PhysicalFrameLoadingIdentity,
};
pub use lease::{
    DirtyPhysicalFrame, PhysicalCandidateBatchAdmission, PhysicalCandidateBatchReservation,
    PhysicalCandidateFrameReservation, PhysicalDirtyReplacementError,
    PhysicalDirtyReplacementReservation, PhysicalFrameLease, PhysicalWritebackClaim,
    SpeculativeResidencyGrant,
};
pub use limits::{
    PhysicalOperationAllocationScope, PhysicalResidencyDimension, PhysicalResidencyLimits,
    PhysicalResidencyLimitsAdmissionDenial,
};
pub(in crate::physical_residency) use observation::{
    PhysicalResidencyAccounting, PhysicalResidencyAllocationEventRecorder,
};
pub use observation::{
    PhysicalResidencyAllocationEventCounters, PhysicalResidencyAllocationEventObserver,
    PhysicalResidencyAllocationEventSnapshot, PhysicalResidencyCounters, PhysicalResidencyShutdown,
};
pub use operation_allocation::{OperationAllocationGrant, OperationAllocationObservation};
pub use pool::{
    PhysicalBoundedFrameKey, PhysicalCandidateFrameKey, PhysicalFrameKey,
    PhysicalResidencyIncarnation, PhysicalResidencyPool,
};
pub(in crate::physical_residency) use pressure::PhysicalResidencyPressureDemand;
pub use pressure::{PhysicalResidencyLimitsBuilder, PhysicalResidencyPressureDenial};
pub use work_kind::PhysicalSpeculativeWorkKind;

#[cfg(test)]
mod tests;
