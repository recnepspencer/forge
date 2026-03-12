use crate::logic::runtime::RelationalRuntime;
use crate::query::data::{QueryWorkPacket, ReadPacketPlan};
use crate::snapshots::data::SnapshotHandle;
use crate::storage::data::{
    ChunkDiagnostics, ChunkedStorageSummary, PartitionStorageStats, StorageStats,
};

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
}
