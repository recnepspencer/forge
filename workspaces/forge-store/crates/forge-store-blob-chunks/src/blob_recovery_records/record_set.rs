use forge_store_physical_backend::BlobBackendResidueObservation;

use super::{
    BlobCheckpointFrontierRecord, BlobChunkAppendRecord, BlobGenerationPublicationRecord,
    BlobManifestAgreement, BlobRecoveryRecordCounterSnapshot, BlobRecoveryRecordDenial,
    BlobRecoveryRecordDenialKind, BlobResumeSessionCheckpointRecord, BlobRootCandidateRecord,
};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BlobAdmittedRecoveryRecords {
    chunk_append: Option<BlobChunkAppendRecord>,
    checkpoint_frontier: Option<BlobCheckpointFrontierRecord>,
    root_candidate: Option<BlobRootCandidateRecord>,
    publication: Option<BlobGenerationPublicationRecord>,
    resume_session: Option<BlobResumeSessionCheckpointRecord>,
    manifest: Option<BlobManifestAgreement>,
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

    pub fn with_chunk_append(mut self, chunk_append: BlobChunkAppendRecord) -> Self {
        self.chunk_append = Some(chunk_append);
        self
    }

    pub fn with_checkpoint_frontier(
        mut self,
        checkpoint_frontier: BlobCheckpointFrontierRecord,
    ) -> Self {
        self.checkpoint_frontier = Some(checkpoint_frontier);
        self
    }

    pub fn with_root_candidate(mut self, root_candidate: BlobRootCandidateRecord) -> Self {
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
    publication: BlobGenerationPublicationRecord,
    resume_session: BlobResumeSessionCheckpointRecord,
    manifest: BlobManifestAgreement,
    counters: BlobRecoveryRecordCounterSnapshot,
}

impl BlobRecoveryRecordSet {
    pub fn from_admitted_replay_records(
        admitted: BlobAdmittedRecoveryRecords,
    ) -> Result<Self, BlobRecoveryRecordDenial> {
        let BlobAdmittedRecoveryRecords {
            chunk_append,
            checkpoint_frontier,
            root_candidate,
            publication,
            resume_session,
            manifest,
        } = admitted;

        let chunk_append = chunk_append.ok_or_else(|| {
            BlobRecoveryRecordDenial::start(
                BlobRecoveryRecordDenialKind::ChunkBytesWithoutIntegrityAdmission,
            )
        })?;
        let checkpoint_frontier = checkpoint_frontier.ok_or_else(|| {
            BlobRecoveryRecordDenial::start(
                BlobRecoveryRecordDenialKind::IntegrityWithoutCheckpointFrontier,
            )
        })?;
        if &chunk_append != checkpoint_frontier.chunk_append() {
            return Err(BlobRecoveryRecordDenial::start(
                BlobRecoveryRecordDenialKind::IntegrityWithoutCheckpointFrontier,
            ));
        }

        let root_candidate = root_candidate.ok_or_else(|| {
            BlobRecoveryRecordDenial::start(
                BlobRecoveryRecordDenialKind::CheckpointFrontierWithoutRootCandidate,
            )
        })?;
        if &checkpoint_frontier != root_candidate.frontier() {
            return Err(BlobRecoveryRecordDenial::start(
                BlobRecoveryRecordDenialKind::CheckpointFrontierWithoutRootCandidate,
            ));
        }

        let publication = publication.ok_or_else(|| {
            BlobRecoveryRecordDenial::start(
                BlobRecoveryRecordDenialKind::RootCandidateWithoutPublication,
            )
        })?;
        if &root_candidate != publication.root_candidate() {
            return Err(BlobRecoveryRecordDenial::start(
                BlobRecoveryRecordDenialKind::RootCandidateWithoutPublication,
            ));
        }

        let resume_session = resume_session.ok_or_else(|| {
            BlobRecoveryRecordDenial::start(
                BlobRecoveryRecordDenialKind::PublicationWithoutClosedResumeSession,
            )
        })?;
        let manifest = manifest.ok_or_else(|| {
            BlobRecoveryRecordDenial::start(
                BlobRecoveryRecordDenialKind::PublicationWithoutManifestAgreement,
            )
        })?;

        let counters = BlobRecoveryRecordCounterSnapshot::start()
            .merge(chunk_append.counters())
            .merge(checkpoint_frontier.counters())
            .merge(root_candidate.counters())
            .merge(publication.counters())
            .merge(resume_session.counters())
            .merge(manifest.counters());

        Self::from_admitted_sources(publication, resume_session, manifest, counters)
    }

    fn from_admitted_sources(
        publication: BlobGenerationPublicationRecord,
        resume_session: BlobResumeSessionCheckpointRecord,
        manifest: BlobManifestAgreement,
        counters: BlobRecoveryRecordCounterSnapshot,
    ) -> Result<Self, BlobRecoveryRecordDenial> {
        if publication.published().object_id() != manifest.placement().observation().object_id()
            || publication.published().object_id() != resume_session.session().object_id()
            || publication.published().generation()
                != manifest.placement().observation().generation()
            || publication.published().generation() != resume_session.session().generation()
            || publication.published().logical_content_digest()
                != manifest.placement().observation().logical_content_digest()
            || publication.published().logical_content_digest()
                != resume_session.session().logical_content_digest()
        {
            return Err(BlobRecoveryRecordDenial::start(
                BlobRecoveryRecordDenialKind::PublicationWithoutManifestAgreement,
            ));
        }
        Ok(Self {
            publication,
            resume_session,
            manifest,
            counters,
        })
    }

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
