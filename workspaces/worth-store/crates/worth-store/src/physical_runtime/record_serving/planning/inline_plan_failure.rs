use worth_store_physical_backend::ArtifactTreeFailureKind;
use worth_store_physical_format::PhysicalGeneration;

use super::super::{RecordAppendDenial, RecordAppendError};

pub(in crate::physical_runtime::record_serving) fn admitted_generation(
    value: Option<u64>,
) -> Result<PhysicalGeneration, RecordAppendError> {
    value
        .and_then(|value| PhysicalGeneration::from_raw(value).ok())
        .ok_or(RecordAppendError::Denied(
            RecordAppendDenial::PhysicalIdentityExhausted,
        ))
}

pub(in crate::physical_runtime::record_serving) fn layout_failure(
    failure: super::super::residency::frame_loading::FrameLoadFailure,
) -> RecordAppendError {
    match failure {
        super::super::residency::frame_loading::FrameLoadFailure::Backend(failure)
            if failure.kind() == ArtifactTreeFailureKind::DeniedBeforeEffect =>
        {
            RecordAppendError::Denied(RecordAppendDenial::BackendUnavailable(failure))
        }
        super::super::residency::frame_loading::FrameLoadFailure::Residency(reason) => {
            RecordAppendError::Denied(RecordAppendDenial::ResidencyUnavailable(reason))
        }
        _ => RecordAppendError::Denied(RecordAppendDenial::PublishedLayoutDamaged),
    }
}

pub(in crate::physical_runtime::record_serving) fn manifest_lookup_failure(
    failure: super::super::access::manifest_routing::ManifestLookupFailure,
) -> RecordAppendError {
    match failure {
        super::super::access::manifest_routing::ManifestLookupFailure::Backend(failure) => {
            layout_failure(
                super::super::residency::frame_loading::FrameLoadFailure::Backend(failure),
            )
        }
        super::super::access::manifest_routing::ManifestLookupFailure::Damaged => {
            RecordAppendError::Denied(RecordAppendDenial::PublishedLayoutDamaged)
        }
        super::super::access::manifest_routing::ManifestLookupFailure::Residency(reason) => {
            RecordAppendError::Denied(RecordAppendDenial::ResidencyUnavailable(reason))
        }
    }
}
