use super::super::{
    BlobCheckpointFrontierRecord, BlobChunkAppendRecord, BlobGenerationPublicationRecord,
    BlobRecoveryRecordDenial, BlobRecoveryRecordDenialKind, BlobRootCandidateRecord,
};

pub(crate) fn require_chunk_append(
    chunk_append: Option<BlobChunkAppendRecord>,
) -> Result<BlobChunkAppendRecord, BlobRecoveryRecordDenial> {
    chunk_append.ok_or_else(|| {
        BlobRecoveryRecordDenial::start(
            BlobRecoveryRecordDenialKind::ChunkBytesWithoutIntegrityAdmission,
        )
    })
}

pub(crate) fn require_checkpoint_frontier(
    checkpoint_frontier: Option<BlobCheckpointFrontierRecord>,
) -> Result<BlobCheckpointFrontierRecord, BlobRecoveryRecordDenial> {
    checkpoint_frontier.ok_or_else(|| {
        BlobRecoveryRecordDenial::start(
            BlobRecoveryRecordDenialKind::IntegrityWithoutCheckpointFrontier,
        )
    })
}

pub(crate) fn verify_chunk_matches_frontier(
    chunk_append: &BlobChunkAppendRecord,
    checkpoint_frontier: &BlobCheckpointFrontierRecord,
) -> Result<(), BlobRecoveryRecordDenial> {
    if chunk_append == checkpoint_frontier.chunk_append() {
        Ok(())
    } else {
        Err(BlobRecoveryRecordDenial::start(
            BlobRecoveryRecordDenialKind::IntegrityWithoutCheckpointFrontier,
        ))
    }
}

pub(crate) fn require_root_candidate(
    root_candidate: Option<BlobRootCandidateRecord>,
) -> Result<BlobRootCandidateRecord, BlobRecoveryRecordDenial> {
    root_candidate.ok_or_else(|| {
        BlobRecoveryRecordDenial::start(
            BlobRecoveryRecordDenialKind::CheckpointFrontierWithoutRootCandidate,
        )
    })
}

pub(crate) fn verify_frontier_matches_root(
    checkpoint_frontier: &BlobCheckpointFrontierRecord,
    root_candidate: &BlobRootCandidateRecord,
) -> Result<(), BlobRecoveryRecordDenial> {
    if checkpoint_frontier == root_candidate.frontier() {
        Ok(())
    } else {
        Err(BlobRecoveryRecordDenial::start(
            BlobRecoveryRecordDenialKind::CheckpointFrontierWithoutRootCandidate,
        ))
    }
}

pub(crate) fn require_publication(
    publication: Option<BlobGenerationPublicationRecord>,
) -> Result<BlobGenerationPublicationRecord, BlobRecoveryRecordDenial> {
    publication.ok_or_else(|| {
        BlobRecoveryRecordDenial::start(BlobRecoveryRecordDenialKind::RootCandidateWithoutPublication)
    })
}

pub(crate) fn verify_root_matches_publication(
    root_candidate: &BlobRootCandidateRecord,
    publication: &BlobGenerationPublicationRecord,
) -> Result<(), BlobRecoveryRecordDenial> {
    if root_candidate == publication.root_candidate() {
        Ok(())
    } else {
        Err(BlobRecoveryRecordDenial::start(
            BlobRecoveryRecordDenialKind::RootCandidateWithoutPublication,
        ))
    }
}