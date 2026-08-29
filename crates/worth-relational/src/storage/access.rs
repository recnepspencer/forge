use crate::query::data::{PlannedQueryPacket, ReadPacketPlan};
use crate::runtime::RelationalRuntime;
use crate::snapshots::data::SnapshotHandle;
use crate::storage::data::{
    ChunkDiagnostics, ChunkedStorageSummary, PartitionStorageStats, RecordLifecycleState,
    StorageStats,
};
use crate::storage::overlay::BorrowedWorkingState;
use crate::storage::substrate::RecordKind;

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

    pub(crate) fn current_state(&self) -> BorrowedWorkingState {
        BorrowedWorkingState::new(self.runtime.partitions.read())
    }

    pub(crate) fn entity_slot_count(&self) -> usize {
        self.runtime
            .partitions
            .read()
            .values()
            .map(|partition| partition.entity_arena.slot_count())
            .sum()
    }

    pub(crate) fn relation_slot_count(&self) -> usize {
        self.runtime
            .partitions
            .read()
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

    pub(crate) fn record_slots<K: RecordKind>(
        &self,
        partition_id: crate::identity::data::PartitionId,
    ) -> Vec<usize> {
        self.runtime
            .partitions
            .read()
            .get(&partition_id)
            .map(|partition| K::arena(partition).occupied_slots())
            .unwrap_or_default()
    }

    pub(crate) fn record_slot_surface<K: RecordKind>(
        &self,
        partition_id: crate::identity::data::PartitionId,
        slot: usize,
    ) -> Option<RecordSlotSurface> {
        let partitions = self.runtime.partitions.read();
        let partition = partitions.get(&partition_id)?;
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

    pub fn plan_read_explicit_query_packet(
        &self,
        handle: &SnapshotHandle,
        packet: &PlannedQueryPacket,
    ) -> Option<ReadPacketPlan> {
        crate::storage::partition::chunks::plan_read_explicit_query_packet(
            self.runtime,
            handle,
            packet,
        )
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
