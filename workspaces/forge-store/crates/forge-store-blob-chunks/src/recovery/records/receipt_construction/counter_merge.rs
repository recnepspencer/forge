use super::super::{
    BlobCheckpointFrontierRecord, BlobChunkAppendRecord, BlobGenerationPublicationRecord,
    BlobManifestAgreement, BlobRecoveryRecordCounterSnapshot, BlobResumeSessionCheckpointRecord,
    BlobRootCandidateRecord,
};

pub(crate) fn merge_admitted_records(
    chunk_append: &BlobChunkAppendRecord,
    checkpoint_frontier: &BlobCheckpointFrontierRecord,
    root_candidate: &BlobRootCandidateRecord,
    publication: &BlobGenerationPublicationRecord,
    resume_session: &BlobResumeSessionCheckpointRecord,
    manifest: &BlobManifestAgreement,
) -> BlobRecoveryRecordCounterSnapshot {
    BlobRecoveryRecordCounterSnapshot::start()
        .merge(chunk_append.counters())
        .merge(checkpoint_frontier.counters())
        .merge(root_candidate.counters())
        .merge(publication.counters())
        .merge(resume_session.counters())
        .merge(manifest.counters())
}