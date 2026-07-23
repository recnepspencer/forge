use worth_store_physical_backend::{MediaCounterSnapshot, QualifiedFilesystemMedia};

use super::{
    super::{residency::frame_ports::StoreCandidateFramePublicationSession, RecordAppendError},
    orchestration::{
        unpublished_candidate_frame_contract, PublicationPlan, RecordPublicationStage,
    },
};

pub(super) fn validate_frame_set(
    media: &QualifiedFilesystemMedia,
    plan: &PublicationPlan,
    residency: &StoreCandidateFramePublicationSession,
    before: MediaCounterSnapshot,
) -> Result<(), RecordAppendError> {
    residency.require_complete().map_err(|violation| {
        unpublished_candidate_frame_contract(
            media,
            plan,
            before,
            RecordPublicationStage::CatalogCandidateSynchronization,
            violation,
        )
    })
}

pub(super) fn prepare_residency(
    media: &QualifiedFilesystemMedia,
    plan: &PublicationPlan,
    residency: &mut StoreCandidateFramePublicationSession,
    before: MediaCounterSnapshot,
) -> Result<(), RecordAppendError> {
    residency.prepare_catalog_cutover().map_err(|violation| {
        unpublished_candidate_frame_contract(
            media,
            plan,
            before,
            RecordPublicationStage::CatalogReplacement,
            violation,
        )
    })
}
