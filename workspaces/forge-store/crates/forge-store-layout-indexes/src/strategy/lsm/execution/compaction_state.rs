use forge_store_wal::{
    BlobWalRecordEnvelope, BlobWalRecordIdentity, DurablePublicationDeclaration,
};

use super::{BaselineLsmCompactionPublicationReceipt, LsmPhysicalCompactionIntent};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedLsmCompaction {
    pub(crate) membership: forge_store_lsm_authority::LsmCompactionMembership,
    pub(crate) replay_tail: [BlobWalRecordEnvelope; 3],
    pub(crate) output: forge_store_lsm_authority::AdmittedLsmReplacementOutput,
    pub(crate) physical_intent: LsmPhysicalCompactionIntent,
}

#[derive(Debug, Clone)]
pub struct PublishedLsmCompaction {
    pub(crate) memtable_records: [BlobWalRecordIdentity; 1],
    pub(crate) sorted_run_records: [BlobWalRecordIdentity; 2],
    pub(crate) wal_publication: BlobWalRecordEnvelope,
    pub(crate) manifest_publication: DurablePublicationDeclaration,
    pub(crate) replay_tail: [BlobWalRecordEnvelope; 3],
    pub(crate) compaction: BaselineLsmCompactionPublicationReceipt,
    pub(crate) physical_compaction: forge_store_physical_isolation::CompactionRewritePublication,
    pub(crate) membership_replacement: forge_store_lsm_authority::PublishedLsmMembershipReplacement,
}

impl PublishedLsmCompaction {
    pub fn replay_tail(&self) -> [&BlobWalRecordEnvelope; 3] {
        std::array::from_fn(|index| &self.replay_tail[index])
    }
}
