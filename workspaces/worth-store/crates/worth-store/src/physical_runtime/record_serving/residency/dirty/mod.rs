mod admitted_frame;
mod failure;
mod outcome;
mod writeback;

#[cfg(not(feature = "certification-test-authority"))]
pub(in crate::physical_runtime::record_serving) use admitted_frame::AdmittedDirtyFrame;
#[cfg(feature = "certification-test-authority")]
pub use admitted_frame::{AdmittedDirtyFrame, PhysicalDirtyTransitionFailure};
#[cfg(feature = "certification-test-authority")]
pub use failure::PhysicalWritebackTransitionFailure;
#[cfg(not(feature = "certification-test-authority"))]
pub(in crate::physical_runtime::record_serving) use failure::PhysicalWritebackTransitionFailure;
pub use failure::{
    PhysicalRecordWritebackFailureCause, PhysicalRecordWritebackFailureEvidence,
    PhysicalWritebackFailureCause,
};
#[cfg(feature = "certification-test-authority")]
pub use outcome::{
    PhysicalWritebackExecution, PhysicalWritebackInspectionRequired, PhysicalWritebackSettlement,
    RetryablePhysicalWriteback,
};
#[cfg(not(feature = "certification-test-authority"))]
pub(in crate::physical_runtime::record_serving) use outcome::{
    PhysicalWritebackExecution, PhysicalWritebackSettlement,
};
pub(in crate::physical_runtime::record_serving) use writeback::FrameWritebackPort;
#[cfg(feature = "certification-test-authority")]
pub use writeback::{AdmittedPhysicalWriteback, PreparedPhysicalWriteback, ReadyPhysicalWriteback};
