use forge_store_security::{StoreKeyScope, StoreTenantScope};
use forge_store_wal::layout_access::baseline_lsm_counter_observation::{
    BaselineLsmCompactionPublicationReceipt, BaselineLsmCompactionRecordKind,
};

use crate::strategy::S8StrategyDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8LsmTombstoneLaw {
    newer_tombstone_shadows_older_value: bool,
}

impl S8LsmTombstoneLaw {
    pub(crate) const fn baseline() -> Self {
        Self {
            newer_tombstone_shadows_older_value: true,
        }
    }

    /// Verifies the sealed, fixed-shape receipt emitted by the WAL compaction
    /// and manifest-publication operation. This is O(1) and allocation-free;
    /// layout grammar never reconstructs source rows or executes compaction.
    pub fn verify_owner_receipt(
        self,
        receipt: &BaselineLsmCompactionPublicationReceipt,
    ) -> Result<(), S8StrategyDenial> {
        let tombstone = receipt.tombstone_record();
        let retired_value = receipt.retired_value_record();
        let scope_is_canonical = receipt.key().tenant_scope()
            == StoreTenantScope::TenantPhysicalBoundary
            && receipt.key().key_scope() == StoreKeyScope::WalCheckpointEnvelope;
        let identity_is_bound = tombstone.key() == receipt.key()
            && retired_value.key() == receipt.key()
            && tombstone.run() == receipt.input_runs()[2]
            && retired_value.run() == receipt.input_runs()[0];
        let replay_is_bound = receipt.replay_binding().contains(&tombstone.wal_record())
            && receipt
                .replay_binding()
                .contains(&retired_value.wal_record());
        let tombstone_is_preserved = self.newer_tombstone_shadows_older_value
            && tombstone.kind() == BaselineLsmCompactionRecordKind::Tombstone
            && retired_value.kind() == BaselineLsmCompactionRecordKind::Value
            && receipt.tombstone_blocks_older()
            && receipt.tombstone_newer_sequence() > receipt.tombstone_older_sequence();

        if scope_is_canonical
            && identity_is_bound
            && replay_is_bound
            && tombstone_is_preserved
            && receipt.publication_is_bound()
        {
            return Ok(());
        }
        Err(S8StrategyDenial::TombstonePreservationViolation)
    }
}
