use crate::physical_runtime::{
    record_serving::{
        residency::frame_load_failure::{
            FrameLoadFailure, FrameLoadFailureKind, FrameLoadFaultCause,
        },
        CanonicalRecordReadFailure,
    },
    PhysicalSchedulerDenial, PhysicalWorkPreEffectDenial,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalFrameReadFailure {
    PhysicalWork(PhysicalFrameWorkFailure),
    Backend(worth_store_physical_backend::ArtifactTreeFailure),
    Residency(worth_store_buffer_pool::PhysicalResidencyDenial),
    FaultTerminated {
        terminal: worth_store_buffer_pool::PhysicalFrameLoadTerminal,
        cause: PhysicalFrameFaultCause,
    },
    CoalescedFault(worth_store_buffer_pool::PhysicalFrameLoadTerminal),
    AccessLimitExceeded,
    ArtifactLengthMismatch,
    InvalidCoordinate,
    ReturnedFrameIdentityMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalFrameFaultCause {
    PhysicalWork(PhysicalFrameWorkFailure),
    Backend(worth_store_physical_backend::ArtifactTreeFailure),
    Residency(worth_store_buffer_pool::PhysicalResidencyDenial),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalFrameWorkFailure {
    RuntimeReleased,
    InvalidCoordinate,
    SubmissionRejected,
    PreEffect(PhysicalWorkPreEffectDenial),
    DependencyBlocked,
    SchedulerReservationRejected,
    SecureIo(worth_store_io_scheduler::SecureIoPreservationDenial),
    Scheduler(PhysicalSchedulerDenial),
    CommandRejected,
    Backend(worth_store_physical_backend::ArtifactTreeFailure),
    Terminal(crate::physical_runtime::PhysicalWorkTerminalCause),
    SchedulerSettlementRejected,
    SettlementMismatch,
    ProjectionFailureUnavailable,
}

impl From<FrameLoadFailure> for PhysicalFrameReadFailure {
    fn from(failure: FrameLoadFailure) -> Self {
        match failure.kind() {
            FrameLoadFailureKind::Backend(failure) => Self::Backend(failure),
            FrameLoadFailureKind::Work(failure) => Self::PhysicalWork(failure.into()),
            FrameLoadFailureKind::Residency(failure) => Self::Residency(failure),
            FrameLoadFailureKind::FaultTerminated { terminal, cause } => {
                let cause = match cause {
                    FrameLoadFaultCause::Backend(failure) => {
                        PhysicalFrameFaultCause::Backend(failure)
                    }
                    FrameLoadFaultCause::Work(failure) => {
                        PhysicalFrameFaultCause::PhysicalWork(failure.into())
                    }
                    FrameLoadFaultCause::Residency(failure) => {
                        PhysicalFrameFaultCause::Residency(failure)
                    }
                };
                Self::FaultTerminated { terminal, cause }
            }
            FrameLoadFailureKind::AccessLimitExceeded => Self::AccessLimitExceeded,
            FrameLoadFailureKind::ArtifactLengthMismatch => Self::ArtifactLengthMismatch,
            FrameLoadFailureKind::InvalidCoordinate => Self::InvalidCoordinate,
            FrameLoadFailureKind::ReturnedFrameIdentityMismatch => {
                Self::ReturnedFrameIdentityMismatch
            }
            FrameLoadFailureKind::CoalescedFault(terminal) => Self::CoalescedFault(terminal),
        }
    }
}

impl From<CanonicalRecordReadFailure> for PhysicalFrameWorkFailure {
    fn from(failure: CanonicalRecordReadFailure) -> Self {
        match failure {
            CanonicalRecordReadFailure::RuntimeReleased => Self::RuntimeReleased,
            CanonicalRecordReadFailure::InvalidCoordinate => Self::InvalidCoordinate,
            CanonicalRecordReadFailure::SubmissionRejected => Self::SubmissionRejected,
            CanonicalRecordReadFailure::PreEffect(failure) => Self::PreEffect(failure),
            CanonicalRecordReadFailure::DependencyBlocked => Self::DependencyBlocked,
            CanonicalRecordReadFailure::SchedulerReservation(_) => {
                Self::SchedulerReservationRejected
            }
            #[cfg(feature = "certification-test-authority")]
            CanonicalRecordReadFailure::SecureIo(failure) => Self::SecureIo(failure),
            CanonicalRecordReadFailure::Scheduler(failure) => Self::Scheduler(failure),
            CanonicalRecordReadFailure::Command(_) => Self::CommandRejected,
            CanonicalRecordReadFailure::Backend(failure) => Self::Backend(failure),
            CanonicalRecordReadFailure::Terminal(cause) => Self::Terminal(cause),
            CanonicalRecordReadFailure::SchedulerSettlementRejected => {
                Self::SchedulerSettlementRejected
            }
            CanonicalRecordReadFailure::SettlementMismatch => Self::SettlementMismatch,
            CanonicalRecordReadFailure::ProjectionFailureUnavailable => {
                Self::ProjectionFailureUnavailable
            }
        }
    }
}
