use forge_store_wal::{
    BlobWalRecordEnvelope, BlobWalRecordIdentity, DurablePublicationDeclaration,
};

use super::super::LsmPhysicalCompactionIntent;
use super::BaselineLsmCompactionPublicationReceipt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedLsmCompaction {
    pub(super) membership: forge_store_lsm_authority::LsmCompactionMembership,
    pub(super) replay_tail: super::LsmCompactionReplayTail,
    pub(super) output: forge_store_lsm_authority::AdmittedLsmReplacementOutput,
    pub(super) physical_intent: LsmPhysicalCompactionIntent,
}

#[derive(Debug, Clone)]
pub struct PublishedLsmCompaction {
    pub(super) maintenance_mode: crate::maintenance::IndexMaintenanceMode,
    pub(super) memtable_records: [BlobWalRecordIdentity; 1],
    pub(super) sorted_run_records: [BlobWalRecordIdentity; 2],
    pub(super) wal_publication: BlobWalRecordEnvelope,
    pub(super) manifest_publication: DurablePublicationDeclaration,
    pub(super) replay_tail: super::LsmCompactionReplayTail,
    pub(super) compaction: BaselineLsmCompactionPublicationReceipt,
    pub(super) physical_compaction: forge_store_physical_isolation::CompactionRewritePublication,
    pub(super) membership_replacement: forge_store_lsm_authority::PublishedLsmMembershipReplacement,
}

impl PublishedLsmCompaction {
    pub const fn replay_tail(&self) -> &super::LsmCompactionReplayTail {
        &self.replay_tail
    }
}
