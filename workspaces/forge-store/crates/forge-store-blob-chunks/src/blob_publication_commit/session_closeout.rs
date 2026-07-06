use crate::{BlobLifecycleCounterSnapshot, BlobResumabilityReceipt, LogicalContentDigest};

use super::{
    BlobPublicationCounterSnapshot, BlobPublicationDenial, BlobPublicationIntent,
    BlobPublicationWalCommit, BlobPublicationWalRecord,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobPublicationSessionCloseout {
    intent: BlobPublicationIntent,
    wal_commit: BlobPublicationWalCommit,
    resumability_digest: LogicalContentDigest,
    resumability_counters: BlobLifecycleCounterSnapshot,
}

impl BlobPublicationSessionCloseout {
    pub fn close(
        wal_record: BlobPublicationWalRecord,
        resumability_receipt: BlobResumabilityReceipt,
    ) -> Result<Self, BlobPublicationDenial> {
        let (intent, wal_commit) = wal_record.into_parts();
        let counters = intent.counters().with_session_closeout();
        if intent.logical_content_digest() != resumability_receipt.logical_content_digest() {
            return Err(BlobPublicationDenial::ReachabilityDigestMismatch { counters });
        }
        Ok(Self {
            resumability_digest: resumability_receipt.logical_content_digest().clone(),
            resumability_counters: resumability_receipt.counters(),
            intent: intent.with_counters(counters),
            wal_commit,
        })
    }

    pub const fn intent(&self) -> &BlobPublicationIntent {
        &self.intent
    }

    pub const fn wal_commit(&self) -> &BlobPublicationWalCommit {
        &self.wal_commit
    }

    pub const fn resumability_digest(&self) -> &LogicalContentDigest {
        &self.resumability_digest
    }

    pub const fn resumability_counters(&self) -> BlobLifecycleCounterSnapshot {
        self.resumability_counters
    }

    pub const fn counters(&self) -> BlobPublicationCounterSnapshot {
        self.intent.counters()
    }

    pub(crate) fn into_parts(self) -> (BlobPublicationIntent, BlobPublicationWalCommit) {
        (self.intent, self.wal_commit)
    }
}
