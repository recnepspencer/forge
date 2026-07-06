use super::super::types::wal_types::{BlobPublicationWalCommit, BlobPublicationWalRecord};

pub(crate) fn append(wal_commit: BlobPublicationWalCommit) -> BlobPublicationWalRecord {
    let intent = wal_commit.intent().clone();
    let counters = intent.counters().with_wal_record();
    BlobPublicationWalRecord {
        intent: intent.with_counters(counters),
        wal_commit,
    }
}