use super::super::{
    BlobGenerationPublicationRecord, BlobManifestAgreement, BlobRecoveryRecordCounterSnapshot,
    BlobRecoveryRecordDenial, BlobRecoveryRecordDenialKind, BlobRecoveryRecordSet,
    BlobResumeSessionCheckpointRecord,
};

pub(crate) fn verify_publication_session_manifest(
    publication: &BlobGenerationPublicationRecord,
    resume_session: &BlobResumeSessionCheckpointRecord,
    manifest: &BlobManifestAgreement,
) -> Result<(), BlobRecoveryRecordDenial> {
    if publication.published().object_id() != manifest.placement().observation().object_id()
        || publication.published().object_id() != resume_session.session().object_id()
        || publication.published().generation() != manifest.placement().observation().generation()
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
    Ok(())
}

pub(crate) fn assemble_record_set(
    publication: BlobGenerationPublicationRecord,
    resume_session: BlobResumeSessionCheckpointRecord,
    manifest: BlobManifestAgreement,
    counters: BlobRecoveryRecordCounterSnapshot,
) -> BlobRecoveryRecordSet {
    BlobRecoveryRecordSet {
        publication,
        resume_session,
        manifest,
        counters,
    }
}