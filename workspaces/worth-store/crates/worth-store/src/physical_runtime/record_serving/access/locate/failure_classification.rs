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
        FrameLoadFailureKind::Backend(failure)
            if failure.kind() == ArtifactTreeFailureKind::Absent =>
        {
            RecordReadDenial::ArtifactUnavailable
        }
        FrameLoadFailureKind::Residency(reason) => RecordReadDenial::ResidencyUnavailable(reason),
        FrameLoadFailureKind::Work(failure) => work_denial(failure),
        _ => RecordReadDenial::ArtifactDamaged,
    }
}

fn work_denial(
    failure: crate::physical_runtime::record_serving::CanonicalRecordReadFailure,
) -> RecordReadDenial {
    use crate::physical_runtime::record_serving::CanonicalRecordReadFailure;
    let work = match failure {
        CanonicalRecordReadFailure::RuntimeReleased => RecordReadWorkDenial::RuntimeReleased,
        CanonicalRecordReadFailure::InvalidCoordinate => RecordReadWorkDenial::InvalidCoordinate,
        CanonicalRecordReadFailure::SubmissionRejected => RecordReadWorkDenial::SubmissionRejected,
        CanonicalRecordReadFailure::PreEffect(_) => RecordReadWorkDenial::AdmissionRejected,
        CanonicalRecordReadFailure::DependencyBlocked => RecordReadWorkDenial::DependencyBlocked,
        CanonicalRecordReadFailure::SchedulerReservation(_) => {
            RecordReadWorkDenial::SchedulerReservationRejected
        }
        CanonicalRecordReadFailure::Scheduler(_) => RecordReadWorkDenial::SchedulerRejected,
        CanonicalRecordReadFailure::Command(_) => RecordReadWorkDenial::CommandRejected,
        CanonicalRecordReadFailure::Backend(failure) => {
            return if failure.kind() == ArtifactTreeFailureKind::Absent {
                RecordReadDenial::ArtifactUnavailable
            } else {
                RecordReadDenial::ArtifactDamaged
            };
        }
        CanonicalRecordReadFailure::SchedulerSettlementRejected => {
            RecordReadWorkDenial::SchedulerSettlementRejected
        }
        CanonicalRecordReadFailure::SettlementMismatch => RecordReadWorkDenial::SettlementMismatch,
    };
    RecordReadDenial::PhysicalWork(work)
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
