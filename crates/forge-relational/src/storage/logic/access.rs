use crate::logic::runtime::RelationalRuntime;
use crate::query::data::{QueryWorkPacket, ReadPacketPlan};
use crate::snapshots::data::SnapshotHandle;
use crate::storage::data::{
    ChunkDiagnostics, ChunkedStorageSummary, PartitionStorageStats, RecordLifecycleState,
    StorageStats,
};
use crate::storage::overlay::{
    BorrowedWorkingState, OverlayStateView, PartitionState, WorkingState,
};
use crate::storage::partition::DenseSlotBitSet;
use crate::storage::substrate::RecordKind;
use std::collections::BTreeMap;

pub struct StorageAccess<'runtime> {
    runtime: &'runtime RelationalRuntime,
}

impl RelationalRuntime {
    pub fn storage_access(&self) -> StorageAccess<'_> {
        StorageAccess::new(self)
    }
}

impl<'runtime> StorageAccess<'runtime> {
    pub(crate) fn new(runtime: &'runtime RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub(crate) fn current_state(&self) -> BorrowedWorkingState<'runtime> {
        BorrowedWorkingState::new(&self.runtime.partitions)
    }

    pub(crate) fn partition_state(
        &self,
        partition_id: crate::identity::data::PartitionId,
    ) -> Option<&'runtime PartitionState> {
        self.runtime.partitions.get(&partition_id)
    }

    pub(crate) fn overlay_state_view<'overlay>(
        &'overlay self,
        staged: &'overlay WorkingState,
    ) -> OverlayStateView<'overlay, WorkingState> {
        OverlayStateView::new(&self.runtime.partitions, staged)
    }

    pub(crate) fn entity_slot_count(&self) -> usize {
        self.runtime
            .partitions
            .values()
            .map(|partition| partition.entity_arena.slot_count())
            .sum()
    }

    pub(crate) fn relation_slot_count(&self) -> usize {
        self.runtime
            .partitions
            .values()
            .map(|partition| partition.relation_arena.slot_count())
            .sum()
    }

    pub(crate) fn entity_chunk_size(&self) -> usize {
        self.runtime.config.storage.layout.entity_chunk_size.max(1)
    }

    pub(crate) fn relation_chunk_size(&self) -> usize {
        self.runtime
            .config
            .storage
            .layout
            .relation_chunk_size
            .max(1)
    }

    pub(crate) fn current_version_partition_pins(
        &self,
    ) -> BTreeMap<crate::identity::data::PartitionId, crate::storage::overlay::SnapshotPartitionPins>
    {
        let mut pinned_partitions = BTreeMap::new();
        for (partition_id, partition) in &self.runtime.partitions {
            let mut entity_slots =
                DenseSlotBitSet::with_capacity(partition.entity_arena.slot_count());
            for slot in partition.entity_arena.live_bitset.iter_set_slots() {
                entity_slots.set(slot, true);
            }
            let mut relation_slots =
                DenseSlotBitSet::with_capacity(partition.relation_arena.slot_count());
            let mut retained_relation_slots =
                DenseSlotBitSet::with_capacity(partition.relation_arena.slot_count());
            for slot in partition.relation_arena.live_bitset.iter_set_slots() {
                relation_slots.set(slot, true);
                if partition
                    .relation_arena
                    .get_slot(slot)
                    .is_some_and(|slot_view| {
                        slot_view.lifecycle()
                            == crate::storage::data::RecordLifecycleState::RetainedDanglingForAudit
                    })
                {
                    retained_relation_slots.set(slot, true);
                }
            }
            if entity_slots.count_ones() > 0 || relation_slots.count_ones() > 0 {
                pinned_partitions.insert(
                    *partition_id,
                    crate::storage::overlay::SnapshotPartitionPins {
                        entity_slots,
                        relation_slots,
                        retained_relation_slots,
                    },
                );
            }
        }
        pinned_partitions
    }

    pub(crate) fn record_slot_count<K: RecordKind>(
        &self,
        partition_id: crate::identity::data::PartitionId,
    ) -> usize {
        self.runtime
            .partitions
            .get(&partition_id)
            .map(|partition| K::arena(partition).slot_count())
            .unwrap_or(0)
    }

    pub(crate) fn record_slot_surface<K: RecordKind>(
        &self,
        partition_id: crate::identity::data::PartitionId,
        slot: usize,
    ) -> Option<RecordSlotSurface> {
        let partition = self.runtime.partitions.get(&partition_id)?;
        let arena = K::arena(partition);
        let slot_view = arena.get_slot(slot)?;
        Some(RecordSlotSurface {
            retired_at: arena.retired_at_for_slot(slot),
            snapshot_pins: arena.snapshot_pin_count(slot).unwrap_or(0),
            branch_pins: arena.branch_pin_count(slot).unwrap_or(0),
            replay_pins: arena.replay_pin_count(slot).unwrap_or(0),
            lifecycle: slot_view.lifecycle(),
        })
    }

    pub fn partition_ids(&self) -> Vec<crate::identity::data::PartitionId> {
        crate::storage::partition::storage_stats::partition_ids(self.runtime)
    }

    pub fn partition_storage_stats(&self) -> Vec<PartitionStorageStats> {
        crate::storage::partition::storage_stats::partition_storage_stats(self.runtime)
    }

    pub fn storage_stats(&self) -> StorageStats {
        crate::storage::partition::storage_stats::storage_stats(self.runtime)
    }

    pub fn chunked_storage_summary(
        &self,
        version_id: crate::identity::data::VersionId,
    ) -> ChunkedStorageSummary {
        crate::storage::partition::chunks::chunked_storage_summary(self.runtime, version_id)
    }

    pub fn chunk_diagnostics(
        &self,
        version_id: crate::identity::data::VersionId,
    ) -> ChunkDiagnostics {
        crate::storage::partition::chunks::chunk_diagnostics(self.runtime, version_id)
    }

    pub fn plan_read_packet(
        &self,
        handle: &SnapshotHandle,
        packet: &QueryWorkPacket,
    ) -> Option<ReadPacketPlan> {
        crate::storage::partition::chunks::plan_read_packet(self.runtime, handle, packet)
    }

    pub fn outgoing_relations_for_entity(
        &self,
        entity_id: crate::identity::data::EntityId,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<crate::identity::data::RelationId> {
        crate::storage::partition::adjacency_queries::outgoing_relations_for_entity(
            self.runtime,
            entity_id,
            version_id,
        )
    }

    pub fn incoming_relations_for_entity(
        &self,
        entity_id: crate::identity::data::EntityId,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<crate::identity::data::RelationId> {
        crate::storage::partition::adjacency_queries::incoming_relations_for_entity(
            self.runtime,
            entity_id,
            version_id,
        )
    }

    pub fn all_relations_for_entity(
        &self,
        entity_id: crate::identity::data::EntityId,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<crate::identity::data::RelationId> {
        crate::storage::partition::adjacency_queries::all_relations_for_entity(
            self.runtime,
            entity_id,
            version_id,
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct RecordSlotSurface {
    pub(crate) retired_at: Option<crate::identity::data::VersionId>,
    pub(crate) snapshot_pins: u32,
    pub(crate) branch_pins: u32,
    pub(crate) replay_pins: u32,
    pub(crate) lifecycle: RecordLifecycleState,
}
