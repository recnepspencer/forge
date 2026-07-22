mod denial;
mod lease;
mod limits;
mod observation;
mod pool;
mod work_kind;

pub use denial::{PhysicalFrameLoadError, PhysicalResidencyDenial};
pub use lease::{
    DirtyPhysicalFrame, OperationAllocationGrant, PhysicalCandidateBatchReservation,
    PhysicalCandidateFrameReservation, PhysicalFrameLease, PhysicalWritebackClaim,
    SpeculativeResidencyGrant,
};
pub use limits::{OperationAllocationScope, PhysicalResidencyLimits};
pub use observation::{PhysicalResidencyCounters, PhysicalResidencyShutdown};
pub use pool::{PhysicalFrameKey, PhysicalResidencyIncarnation, PhysicalResidencyPool};
pub use work_kind::SpeculativePhysicalWorkKind;

#[cfg(test)]
mod tests;
