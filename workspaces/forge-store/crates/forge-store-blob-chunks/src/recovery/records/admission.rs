use super::{
    BlobAdmittedRecoveryRecords, BlobRecoveryRecordDenial, BlobRecoveryRecordDenialKind,
    BlobRecoveryRecordSet,
};
use super::receipt_construction::merge_admitted_records;
use super::verification::{
    assemble_record_set, require_checkpoint_frontier, require_chunk_append, require_publication,
    require_root_candidate, verify_chunk_matches_frontier, verify_frontier_matches_root,
    verify_root_matches_publication, verify_publication_session_manifest,
};

impl BlobRecoveryRecordSet {
    pub fn from_admitted_replay_records(
        admitted: BlobAdmittedRecoveryRecords,
    ) -> Result<Self, BlobRecoveryRecordDenial> {
        admit_replay_records(admitted)
    }
}

pub(crate) fn admit_replay_records(
    admitted: BlobAdmittedRecoveryRecords,
) -> Result<BlobRecoveryRecordSet, BlobRecoveryRecordDenial> {
    let BlobAdmittedRecoveryRecords {
        chunk_append,
        checkpoint_frontier,
        root_candidate,
        publication,
        resume_session,
        manifest,
    } = admitted;

    let chunk_append = require_chunk_append(chunk_append)?;
    let checkpoint_frontier = require_checkpoint_frontier(checkpoint_frontier)?;
    verify_chunk_matches_frontier(&chunk_append, &checkpoint_frontier)?;

    let root_candidate = require_root_candidate(root_candidate)?;
    verify_frontier_matches_root(&checkpoint_frontier, &root_candidate)?;

    let publication = require_publication(publication)?;
    verify_root_matches_publication(&root_candidate, &publication)?;

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

    verify_publication_session_manifest(&publication, &resume_session, &manifest)?;

    let counters = merge_admitted_records(
        &chunk_append,
        &checkpoint_frontier,
        &root_candidate,
        &publication,
        &resume_session,
        &manifest,
    );

    Ok(assemble_record_set(publication, resume_session, manifest, counters))
}