use worth_store_recovery_physics::CrashBoundaryLayoutReport;
use worth_store_wal::DurablePublicationDeclaration;

use super::super::types::reachability_staging::BlobReachabilityStaging;
use super::super::types::wal_types::{BlobPublicationWalCommit, BlobPublicationWalPayload};
use super::super::verification::{replayable_wal, wal_replay_identity};
use super::super::BlobPublicationDenial;

pub(crate) fn from_replayable_wal_record(
    staged: BlobReachabilityStaging,
    payload: BlobPublicationWalPayload,
    durable_publication: DurablePublicationDeclaration,
    replay_report: &CrashBoundaryLayoutReport,
) -> Result<BlobPublicationWalCommit, BlobPublicationDenial> {
    let (intent, staging_identity, security_metadata) = staged.into_parts();
    let counters = intent.counters();
    wal_replay_identity::verify_staging_payload_match(&payload, &staging_identity, counters)?;
    let wal_scope =
        wal_replay_identity::verify_wal_frame_scope(&durable_publication, intent.counters())?;
    let durable_wal = replayable_wal::replayable_durable_wal(replay_report).ok_or(
        BlobPublicationDenial::WalReplayEvidenceRequired {
            counters: intent.counters(),
        },
    )?;
    wal_replay_identity::require_matching_replay_identity(
        &wal_scope,
        durable_wal,
        &payload,
        intent.counters(),
    )?;
    Ok(BlobPublicationWalCommit {
        intent,
        durable_publication,
        replay_classification_digest: replay_report.classification_digest().to_owned(),
        replay_counters: replay_report.counters(),
        staging_identity,
        security_metadata,
    })
}
