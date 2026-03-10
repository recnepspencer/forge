use crate::logic::runtime::RelationalRuntime;
use crate::storage::data::{PartitionStorageStats, StorageStats};
use crate::storage::logic::state::LifecycleCounts;

impl RelationalRuntime {
    pub fn partition_ids(&self) -> Vec<crate::identity::data::PartitionId> {
        self.partitions.keys().copied().collect()
    }

    pub fn partition_storage_stats(&self) -> Vec<PartitionStorageStats> {
        self.partitions
            .iter()
            .map(|(partition_id, partition)| {
                let entity_counts = partition.entity_arena.lifecycle_counts();
                let relation_counts = partition.relation_arena.lifecycle_counts();
                PartitionStorageStats {
                    partition_id: *partition_id,
                    entity_slots: partition.entity_arena.generations.len(),
                    entity_chunks: partition
                        .entity_arena
                        .generations
                        .len()
                        .div_ceil(self.config.storage_layout.entity_chunk_size.max(1)),
                    live_entities: entity_counts.live,
                    deleted_entities: entity_counts.deleted,
                    reusable_entity_slots: entity_counts.reusable,
                    relation_slots: partition.relation_arena.generations.len(),
                    relation_chunks: partition
                        .relation_arena
                        .generations
                        .len()
                        .div_ceil(self.config.storage_layout.relation_chunk_size.max(1)),
                    live_relations: relation_counts.live,
                    deleted_relations: relation_counts.deleted,
                    reusable_relation_slots: relation_counts.reusable,
                }
            })
            .collect()
    }

    pub fn storage_stats(&self) -> StorageStats {
        let chunked_summary = self.chunked_storage_summary(self.current_version_id());
        let mut entity_counts = LifecycleCounts::default();
        let mut relation_counts = LifecycleCounts::default();
        for partition in self.partitions.values() {
            let counts = partition.entity_arena.lifecycle_counts();
            entity_counts.live += counts.live;
            entity_counts.deleted += counts.deleted;
            entity_counts.reusable += counts.reusable;
            let counts = partition.relation_arena.lifecycle_counts();
            relation_counts.live += counts.live;
            relation_counts.deleted += counts.deleted;
            relation_counts.reusable += counts.reusable;
        }
        StorageStats {
            entity_slots: self.entity_slot_count(),
            entity_chunks: chunked_summary.entity_chunks.len(),
            live_entities: entity_counts.live,
            deleted_entities: entity_counts.deleted,
            reusable_entity_slots: entity_counts.reusable,
            relation_slots: self.relation_slot_count(),
            relation_chunks: chunked_summary.relation_chunks.len(),
            live_relations: relation_counts.live,
            deleted_relations: relation_counts.deleted,
            reusable_relation_slots: relation_counts.reusable,
            snapshot_count: self.snapshots.active.len(),
        }
    }

    pub fn outgoing_relations_for_entity(
        &self,
        entity_id: crate::identity::data::EntityId,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<crate::identity::data::RelationId> {
        let slot = entity_id.local_slot.0 as usize;
        self.partition(entity_id.partition_id)
            .and_then(|partition| partition.adjacency.get(slot))
            .into_iter()
            .flat_map(|relations| relations.ids().into_iter())
            .filter(|relation_id| self.relation_visible_at_version(*relation_id, version_id))
            .collect()
    }

    pub fn incoming_relations_for_entity(
        &self,
        entity_id: crate::identity::data::EntityId,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<crate::identity::data::RelationId> {
        let slot = entity_id.local_slot.0 as usize;
        self.partition(entity_id.partition_id)
            .and_then(|partition| partition.reverse_adjacency.get(slot))
            .into_iter()
            .flat_map(|relations| relations.ids().into_iter())
            .filter(|relation_id| self.relation_visible_at_version(*relation_id, version_id))
            .collect()
    }
}
