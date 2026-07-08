use forge_store_physical_backend::BlobBackendResidueObservation;

use super::{
    BlobGenerationPublicationRecord, BlobManifestAgreement, BlobRecoveryRecordCounterSnapshot,
    BlobRecoveryRecordDenial, BlobRecoveryRecordDenialKind, BlobResumeSessionCheckpointRecord,
};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BlobAdmittedRecoveryRecords {
    pub(crate) chunk_append: Option<super::BlobChunkAppendRecord>,
    pub(crate) checkpoint_frontier: Option<super::BlobCheckpointFrontierRecord>,
    pub(crate) root_candidate: Option<super::BlobRootCandidateRecord>,
    pub(crate) publication: Option<BlobGenerationPublicationRecord>,
    pub(crate) resume_session: Option<BlobResumeSessionCheckpointRecord>,
    pub(crate) manifest: Option<BlobManifestAgreement>,
}

impl BlobAdmittedRecoveryRecords {
    pub const fn new() -> Self {
        Self {
            chunk_append: None,
            checkpoint_frontier: None,
            root_candidate: None,
            publication: None,
            resume_session: None,
            manifest: None,
        }
    }

    pub fn with_chunk_append(mut self, chunk_append: super::BlobChunkAppendRecord) -> Self {
        self.chunk_append = Some(chunk_append);
        self
    }

    pub fn with_checkpoint_frontier(
        mut self,
        checkpoint_frontier: super::BlobCheckpointFrontierRecord,
    ) -> Self {
        self.checkpoint_frontier = Some(checkpoint_frontier);
        self
    }

    pub fn with_root_candidate(mut self, root_candidate: super::BlobRootCandidateRecord) -> Self {
        self.root_candidate = Some(root_candidate);
        self
    }

    pub fn with_publication(mut self, publication: BlobGenerationPublicationRecord) -> Self {
        self.publication = Some(publication);
        self
    }

    pub fn with_resume_session(
        mut self,
        resume_session: BlobResumeSessionCheckpointRecord,
    ) -> Self {
        self.resume_session = Some(resume_session);
        self
    }

    pub fn with_manifest(mut self, manifest: BlobManifestAgreement) -> Self {
        self.manifest = Some(manifest);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobRecoveryRecordSet {
    pub(crate) publication: BlobGenerationPublicationRecord,
    pub(crate) resume_session: BlobResumeSessionCheckpointRecord,
    pub(crate) manifest: BlobManifestAgreement,
    pub(crate) counters: BlobRecoveryRecordCounterSnapshot,
}

impl BlobRecoveryRecordSet {
    pub fn reject_backend_residue(
        _observation: &BlobBackendResidueObservation,
    ) -> BlobRecoveryRecordDenial {
        BlobRecoveryRecordDenial::start(BlobRecoveryRecordDenialKind::BackendResidueRejected)
    }

    pub const fn publication(&self) -> &BlobGenerationPublicationRecord {
        &self.publication
    }

    pub const fn resume_session(&self) -> &BlobResumeSessionCheckpointRecord {
        &self.resume_session
    }

    pub const fn manifest(&self) -> &BlobManifestAgreement {
        &self.manifest
    }

    pub const fn counters(&self) -> BlobRecoveryRecordCounterSnapshot {
        self.counters
    }
}
