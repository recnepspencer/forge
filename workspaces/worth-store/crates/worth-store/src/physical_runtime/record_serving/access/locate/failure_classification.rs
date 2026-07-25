use worth_store_physical_backend::ArtifactTreeFailureKind;

use super::super::super::{RecordReadDenial, RecordReadWorkDenial};
use super::super::{
    super::residency::frame_loading::{FrameLoadFailure, FrameLoadFailureKind},
    manifest_routing::ManifestLookupFailure,
};

pub(in crate::physical_runtime::record_serving) fn read_failure(
    failure: FrameLoadFailure,
) -> RecordReadDenial {
    match failure.kind() {
        FrameLoadFailureKind::Backend(failure) => backend_denial(failure),
        FrameLoadFailureKind::Residency(reason) => RecordReadDenial::ResidencyUnavailable(reason),
        FrameLoadFailureKind::Work(failure) => work_denial(failure),
        _ => RecordReadDenial::ArtifactDamaged,
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
        ) => RecordReadDenial::ResidencyUnavailable(reason),
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
        ManifestLookupFailure::Residency(reason) => RecordReadDenial::ResidencyUnavailable(reason),
    }
}
