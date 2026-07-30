mod candidate;
pub(super) mod candidate_allocation;
mod dirty;
pub(super) mod dirty_replacement_allocation;
mod frame;
mod writeback;

pub use candidate::{
    PhysicalCandidateBatchAdmission, PhysicalCandidateBatchReservation,
    PhysicalCandidateFrameReservation,
};
pub use dirty::{
    DirtyPhysicalFrame, PhysicalDirtyReplacementError, PhysicalDirtyReplacementReservation,
};
pub use frame::PhysicalFrameLease;
pub use writeback::PhysicalWritebackClaim;
