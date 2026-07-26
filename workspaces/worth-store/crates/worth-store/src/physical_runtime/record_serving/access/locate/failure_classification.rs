use worth_store_physical_backend::ArtifactTreeFailureKind;

use super::super::super::{RecordReadDenial, RecordReadWorkDenial};
use super::super::{
    super::residency::frame_loading::{
        FrameLoadFailure, FrameLoadFailureKind, FrameLoadFaultCause,
    },
    manifest_routing::ManifestLookupFailure,
};

pub(in crate::physical_runtime::record_serving) fn read_failure(
    failure: FrameLoadFailure,
) -> RecordReadDenial {
    match failure.kind() {
        FrameLoadFailureKind::Backend(failure) => backend_denial(failure),
        FrameLoadFailureKind::Residency(reason) => RecordReadDenial::from_residency(reason),
        FrameLoadFailureKind::Work(failure) => work_denial(failure),
        FrameLoadFailureKind::FaultTerminated { cause, .. } => fault_denial(cause),
        FrameLoadFailureKind::CoalescedFault(terminal) => RecordReadDenial::from_residency(
            worth_store_buffer_pool::PhysicalResidencyDenial::FrameLoadTerminated(terminal),
        ),
        _ => RecordReadDenial::ArtifactDamaged,
    }
}

fn fault_denial(cause: FrameLoadFaultCause) -> RecordReadDenial {
    match cause {
        FrameLoadFaultCause::Backend(failure) => backend_denial(failure),
        FrameLoadFaultCause::Work(failure) => work_denial(failure),
        FrameLoadFaultCause::Residency(reason) => RecordReadDenial::from_residency(reason),
    }
}

fn work_denial(
    failure: crate::physical_runtime::record_serving::CanonicalRecordReadFailure,
) -> RecordReadDenial {
    match failure {
        crate::physical_runtime::record_serving::CanonicalRecordReadFailure::Backend(failure) => {
            backend_denial(failure)
        }
        crate::physical_runtime::record_serving::CanonicalRecordReadFailure::Terminal(cause) => {
            terminal_denial(cause)
        }
        _ => RecordReadDenial::PhysicalWork(
            failure
                .work_denial()
                .expect("non-backend canonical failures have work denials"),
        ),
    }
}

fn terminal_denial(cause: crate::physical_runtime::PhysicalWorkTerminalCause) -> RecordReadDenial {
    match cause {
        crate::physical_runtime::PhysicalWorkTerminalCause::Backend(failure) => {
            backend_denial(failure)
        }
        crate::physical_runtime::PhysicalWorkTerminalCause::IncompleteRead { .. } => {
            RecordReadDenial::ArtifactDamaged
        }
        crate::physical_runtime::PhysicalWorkTerminalCause::SchedulerRejectedAfterEffect => {
            RecordReadDenial::PhysicalWork(RecordReadWorkDenial::SchedulerSettlementRejected)
        }
        crate::physical_runtime::PhysicalWorkTerminalCause::ResidencyRejectedAfterEffect(
            reason,
        ) => RecordReadDenial::from_residency(reason),
    }
}

fn backend_denial(failure: worth_store_physical_backend::ArtifactTreeFailure) -> RecordReadDenial {
    match failure.kind() {
        ArtifactTreeFailureKind::Absent => RecordReadDenial::ArtifactUnavailable,
        ArtifactTreeFailureKind::AccessLimitExceeded => RecordReadDenial::AccessLimitExceeded,
        ArtifactTreeFailureKind::DeniedBeforeEffect => {
            RecordReadDenial::BackendUnavailable(failure)
        }
        _ => RecordReadDenial::ArtifactDamaged,
    }
}

pub(in crate::physical_runtime::record_serving) fn manifest_failure(
    failure: ManifestLookupFailure,
) -> RecordReadDenial {
    match failure {
        ManifestLookupFailure::Backend(failure) => read_failure(FrameLoadFailure::new(
            FrameLoadFailureKind::Backend(failure),
        )),
        ManifestLookupFailure::Frame(kind) => read_failure(FrameLoadFailure::new(kind)),
        ManifestLookupFailure::Damaged => RecordReadDenial::ArtifactDamaged,
        ManifestLookupFailure::Residency(reason) => RecordReadDenial::from_residency(reason),
    }
}
