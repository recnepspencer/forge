use worth_proof::NonEmpty;
use worth_store_wal::LogSequenceNumber;

use super::{PhysicalWalSegmentInventory, PhysicalWalSegmentInventoryEntry};

/// Immutable physical WAL facts observed together under the WAL owner's lock.
pub(in crate::physical_runtime::durability) struct PhysicalWalInventorySnapshot {
    durable_lsn_end: LogSequenceNumber,
    segments: NonEmpty<PhysicalWalSegmentInventoryEntry>,
}

impl PhysicalWalSegmentInventory {
    pub(in crate::physical_runtime::durability::wal) fn snapshot(
        &self,
        durable_lsn_end: LogSequenceNumber,
    ) -> Option<PhysicalWalInventorySnapshot> {
        let segments = NonEmpty::try_from_vec(self.entries.clone()).ok()?;
        Some(PhysicalWalInventorySnapshot {
            durable_lsn_end,
            segments,
        })
    }
}

impl PhysicalWalInventorySnapshot {
    pub(in crate::physical_runtime::durability) const fn durable_lsn_end(
        &self,
    ) -> LogSequenceNumber {
        self.durable_lsn_end
    }

    pub(in crate::physical_runtime::durability) fn segments(
        &self,
    ) -> &NonEmpty<PhysicalWalSegmentInventoryEntry> {
        &self.segments
    }
}
