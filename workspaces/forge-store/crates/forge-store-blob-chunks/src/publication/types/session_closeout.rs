use crate::{BlobLifecycleCounterSnapshot, BlobResumabilityReceipt, LogicalContentDigest};

use super::super::{
    BlobPublicationCounterSnapshot, BlobPublicationDenial, BlobPublicationIntent,
    BlobPublicationWalCommit, BlobPublicationWalRecord,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobPublicationSessionCloseout {
    pub(crate) intent: BlobPublicationIntent,
    pub(crate) wal_commit: BlobPublicationWalCommit,
    pub(crate) resumability_digest: LogicalContentDigest,
    pub(crate) resumability_counters: BlobLifecycleCounterSnapshot,
}

impl BlobPublicationSessionCloseout {
    pub fn close(
        wal_record: BlobPublicationWalRecord,
        resumability_receipt: BlobResumabilityReceipt,
    ) -> Result<Self, BlobPublicationDenial> {
        super::super::transitions::session_closeout::close(wal_record, resumability_receipt)
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