use crate::application_aftermath::{
    WorthQueryPublishedApplicationCommitBoundaryEvidence, WorthQueryPublishedApplicationCommitKind,
    WorthQueryPublishedMutationWork,
};

use super::WorthQueryPublishedApplicationCommitAttemptReleasePosture;

#[derive(Clone, Copy)]
pub struct WorthQueryApplicationCommitPublicationInspection<'receipt> {
    evidence: &'receipt WorthQueryPublishedApplicationCommitBoundaryEvidence,
}

impl<'receipt> WorthQueryApplicationCommitPublicationInspection<'receipt> {
    pub(super) const fn new(
        evidence: &'receipt WorthQueryPublishedApplicationCommitBoundaryEvidence,
    ) -> Self {
        Self { evidence }
    }

    pub const fn kind(&self) -> WorthQueryPublishedApplicationCommitKind {
        self.evidence.kind()
    }

    pub const fn mutation_work(&self) -> Option<&WorthQueryPublishedMutationWork> {
        self.evidence.mutation_work()
    }

    pub const fn changed_record_count(&self) -> usize {
        self.evidence.changed_record_count()
    }

    pub const fn emitted_effect_count(&self) -> usize {
        self.evidence.emitted_effect_count()
    }

    pub const fn publication_canonical_entries(&self) -> u32 {
        self.evidence.publication_work().canonical_entries()
    }

    pub const fn publication_sha256_compression_blocks(&self) -> usize {
        self.evidence.publication_work().sha256_compression_blocks()
    }

    pub const fn publication_identity_text_materializations(&self) -> u32 {
        self.evidence
            .publication_work()
            .digest_text_materializations()
    }

    pub const fn attempt_release(
        &self,
    ) -> WorthQueryPublishedApplicationCommitAttemptReleasePosture {
        self.evidence.attempt_release()
    }
}
