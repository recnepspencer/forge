use worth_store_physical_backend::{MediaCounterSnapshot, QualifiedFilesystemMedia};

use super::{
    super::{residency::frame_ports::StoreCandidateFramePublicationSession, RecordAppendError},
    unpublished_candidate_frame_contract, PublicationPlan, RecordPublicationStage,
};

pub(super) struct ValidatedCatalogFrameSet {
    candidate: worth_store_physical_format::RecordArtifactFile,
}

pub(super) struct PreparedCatalogResidency {
    candidate: worth_store_physical_format::RecordArtifactFile,
}

pub(super) fn validate_frame_set(
    media: &QualifiedFilesystemMedia,
    plan: &PublicationPlan,
    residency: &StoreCandidateFramePublicationSession,
    before: MediaCounterSnapshot,
) -> Result<ValidatedCatalogFrameSet, RecordAppendError> {
    residency.require_complete().map_err(|violation| {
        unpublished_candidate_frame_contract(
            media,
            plan,
            before,
            RecordPublicationStage::CatalogCandidateSynchronization,
            violation,
        )
    })?;
    Ok(ValidatedCatalogFrameSet {
        candidate: plan.candidate,
    })
}

pub(super) fn prepare_residency(
    media: &QualifiedFilesystemMedia,
    plan: &PublicationPlan,
    residency: &mut StoreCandidateFramePublicationSession,
    before: MediaCounterSnapshot,
) -> Result<PreparedCatalogResidency, RecordAppendError> {
    residency.prepare_catalog_cutover().map_err(|violation| {
        unpublished_candidate_frame_contract(
            media,
            plan,
            before,
            RecordPublicationStage::CatalogReplacement,
            violation,
        )
    })?;
    Ok(PreparedCatalogResidency {
        candidate: plan.candidate,
    })
}

impl ValidatedCatalogFrameSet {
    pub(super) const fn candidate(&self) -> worth_store_physical_format::RecordArtifactFile {
        self.candidate
    }

    #[cfg(feature = "certification-test-authority")]
    pub(super) fn certification_mismatched(mut self) -> Self {
        self.candidate = worth_store_physical_format::RecordArtifactFile::BootstrapCatalog;
        self
    }
}

impl PreparedCatalogResidency {
    pub(super) const fn candidate(&self) -> worth_store_physical_format::RecordArtifactFile {
        self.candidate
    }
}
