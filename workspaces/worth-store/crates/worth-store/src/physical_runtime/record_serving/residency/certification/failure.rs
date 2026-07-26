use crate::physical_runtime::{
    record_serving::{
        residency::frame_load_failure::{
            FrameLoadFailure, FrameLoadFailureKind, FrameLoadFaultCause,
        },
        CanonicalRecordReadFailure,
    },
    PhysicalWorkPreEffectDenial,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CertificationFrameReadFailure {
    PhysicalWork(CertificationFrameWorkFailure),
    Backend(worth_store_physical_backend::ArtifactTreeFailure),
    Residency(worth_store_buffer_pool::PhysicalResidencyDenial),
    FaultTerminated {
        terminal: worth_store_buffer_pool::PhysicalFrameLoadTerminal,
        cause: CertificationFrameFaultCause,
    },
    CoalescedFault(worth_store_buffer_pool::PhysicalFrameLoadTerminal),
    AccessLimitExceeded,
    InvalidCoordinate,
    ReturnedFrameIdentityMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CertificationFrameFaultCause {
    PhysicalWork(CertificationFrameWorkFailure),
    Backend(worth_store_physical_backend::ArtifactTreeFailure),
    Residency(worth_store_buffer_pool::PhysicalResidencyDenial),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CertificationFrameWorkFailure {
    RuntimeReleased,
    InvalidCoordinate,
    SubmissionRejected,
    PreEffect(PhysicalWorkPreEffectDenial),
    DependencyBlocked,
    SchedulerReservationRejected,
    SchedulerRejected,
    CommandRejected,
    Backend(worth_store_physical_backend::ArtifactTreeFailure),
    Terminal(crate::physical_runtime::PhysicalWorkTerminalCause),
    SchedulerSettlementRejected,
    SettlementMismatch,
    ProjectionFailureUnavailable,
}

impl From<FrameLoadFailure> for CertificationFrameReadFailure {
    fn from(failure: FrameLoadFailure) -> Self {
        match failure.kind() {
            FrameLoadFailureKind::Backend(failure) => Self::Backend(failure),
            FrameLoadFailureKind::Work(failure) => Self::PhysicalWork(failure.into()),
            FrameLoadFailureKind::Residency(failure) => Self::Residency(failure),
            FrameLoadFailureKind::FaultTerminated { terminal, cause } => {
                let cause = match cause {
                    FrameLoadFaultCause::Backend(failure) => {
                        CertificationFrameFaultCause::Backend(failure)
                    }
                    FrameLoadFaultCause::Work(failure) => {
                        CertificationFrameFaultCause::PhysicalWork(failure.into())
                    }
                    FrameLoadFaultCause::Residency(failure) => {
                        CertificationFrameFaultCause::Residency(failure)
                    }
                };
                Self::FaultTerminated { terminal, cause }
            }
            FrameLoadFailureKind::AccessLimitExceeded => Self::AccessLimitExceeded,
            FrameLoadFailureKind::InvalidCoordinate => Self::InvalidCoordinate,
            FrameLoadFailureKind::ReturnedFrameIdentityMismatch => {
                Self::ReturnedFrameIdentityMismatch
            }
            FrameLoadFailureKind::CoalescedFault(terminal) => Self::CoalescedFault(terminal),
        }
    }
}

impl From<CanonicalRecordReadFailure> for CertificationFrameWorkFailure {
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
            CanonicalRecordReadFailure::Scheduler(_) => Self::SchedulerRejected,
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
