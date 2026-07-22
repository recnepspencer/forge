use worth_store_physical_backend::{
    ArtifactTreeFailure, ArtifactTreeFailureKind, MediaCounterSnapshot, QualifiedFilesystemMedia,
};

use super::super::{RecordAppendDenial, RecordAppendError, UnpublishedRecordEffectFate};
use super::orchestration::{
    indeterminate, unpublished_backend, CandidateDataWriteFailure, PublicationPlan,
    RecordPublicationStage,
};

pub(in crate::physical_runtime::record_serving) fn classify_first_write(
    failure: ArtifactTreeFailure,
) -> CandidateDataWriteFailure {
    match failure.kind() {
        ArtifactTreeFailureKind::DeniedBeforeEffect => CandidateDataWriteFailure::PreEffectDenied(
            RecordAppendDenial::BackendUnavailable(failure),
        ),
        kind => CandidateDataWriteFailure::Backend {
            failure,
            effect_fate: if matches!(
                kind,
                ArtifactTreeFailureKind::PartialWrite { .. }
                    | ArtifactTreeFailureKind::IndeterminateEffect
            ) {
                UnpublishedRecordEffectFate::EffectPossible
            } else {
                UnpublishedRecordEffectFate::DeniedBeforeEffect
            },
        },
    }
}

pub(in crate::physical_runtime::record_serving) fn classify_catalog_replacement_failure(
    media: &QualifiedFilesystemMedia,
    plan: &PublicationPlan,
    before: MediaCounterSnapshot,
    failure: ArtifactTreeFailure,
) -> RecordAppendError {
    if failure.kind() == ArtifactTreeFailureKind::DeniedBeforeEffect {
        unpublished_backend(
            media,
            plan,
            before,
            RecordPublicationStage::CatalogReplacement,
            failure,
            UnpublishedRecordEffectFate::EffectPossible,
        )
    } else {
        indeterminate(
            media,
            plan,
            before,
            RecordPublicationStage::CatalogReplacement,
            failure,
        )
    }
}
