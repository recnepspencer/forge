use worth_store_physical_backend::ArtifactTreeFailureKind;

use super::super::super::RecordReadDenial;
use super::super::{
    super::residency::frame_loading::{FrameLoadFailure, FrameLoadFailure::Backend},
    manifest_routing::ManifestLookupFailure,
};

pub(super) fn read_failure(failure: FrameLoadFailure) -> RecordReadDenial {
    match failure {
        Backend(failure) if failure.kind() == ArtifactTreeFailureKind::Absent => {
            RecordReadDenial::ArtifactUnavailable
        }
        FrameLoadFailure::Residency(reason) => RecordReadDenial::ResidencyUnavailable(reason),
        _ => RecordReadDenial::ArtifactDamaged,
    }
}

pub(super) fn manifest_failure(failure: ManifestLookupFailure) -> RecordReadDenial {
    match failure {
        ManifestLookupFailure::Backend(failure) => read_failure(Backend(failure)),
        ManifestLookupFailure::Damaged => RecordReadDenial::ArtifactDamaged,
        ManifestLookupFailure::Residency(reason) => RecordReadDenial::ResidencyUnavailable(reason),
    }
}
