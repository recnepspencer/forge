use crate::BlobResumabilityReceipt;

use super::super::types::session_closeout::BlobPublicationSessionCloseout;
use super::super::types::wal_types::BlobPublicationWalRecord;
use super::super::verification::resumability_match;
use super::super::BlobPublicationDenial;

pub(crate) fn close(
    wal_record: BlobPublicationWalRecord,
    resumability_receipt: BlobResumabilityReceipt,
) -> Result<BlobPublicationSessionCloseout, BlobPublicationDenial> {
    let (intent, wal_commit) = wal_record.into_parts();
    let counters = intent.counters().with_session_closeout();
    resumability_match::verify_resumability_digest(&intent, &resumability_receipt, counters)?;
    Ok(BlobPublicationSessionCloseout {
        resumability_digest: resumability_receipt.logical_content_digest().clone(),
        resumability_counters: resumability_receipt.counters(),
        intent: intent.with_counters(counters),
        wal_commit,
    })
}