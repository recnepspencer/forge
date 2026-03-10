use crate::logic::runtime::RelationalRuntime;
use crate::publication::data::diff::AspectKey;
use crate::storage::data::{PartitionStorageStats, StorageStats};
use crate::storage::logic::state::{LifecycleCounts, PartitionAccess};
use crate::symbols::data::InternedString;
impl RelationalRuntime {
    pub fn visible_entities_of_kind(
        &self,
        kind_id: crate::identity::data::KindId,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<crate::storage::data::EntityReadRecord> {
        let state = self.current_state();
        let mut records = Vec::new();
        for partition_id in state.partition_ids() {
            records.extend(
                self.visible_entities_of_kind_in_partition_from_state(
                    &state,
                    partition_id,
                    kind_id,
                    version_id,
                ),
            );
        }
        sort_entity_records(&mut records);
        records
    }

    pub fn visible_entities_of_kind_in_partition(
        &self,
        partition_id: crate::identity::data::PartitionId,
        kind_id: crate::identity::data::KindId,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<crate::storage::data::EntityReadRecord> {
        let state = self.current_state();
        let mut records = self.visible_entities_of_kind_in_partition_from_state(
            &state,
            partition_id,
            kind_id,
            version_id,
        );
        sort_entity_records(&mut records);
        records
    }

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

    pub fn visible_relations_of_kind(
        &self,
        kind_id: crate::identity::data::KindId,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<crate::storage::data::RelationReadRecord> {
        let state = self.current_state();
        let mut records = Vec::new();
        for partition_id in state.partition_ids() {
            records.extend(
                self.visible_relations_of_kind_in_partition_from_state(
                    &state,
                    partition_id,
                    kind_id,
                    version_id,
                ),
            );
        }
        records.sort_by_key(|record| {
            (
                record.source.partition_id.0,
                record.source.local_slot.0,
                record.target.partition_id.0,
                record.target.local_slot.0,
                record.relation_id.partition_id.0,
                record.relation_id.local_slot.0,
            )
        });
        records
    }

    pub fn visible_relations_of_kind_in_partition(
        &self,
        partition_id: crate::identity::data::PartitionId,
        kind_id: crate::identity::data::KindId,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<crate::storage::data::RelationReadRecord> {
        let state = self.current_state();
        let mut records = self.visible_relations_of_kind_in_partition_from_state(
            &state,
            partition_id,
            kind_id,
            version_id,
        );
        records.sort_by_key(|record| {
            (
                record.source.partition_id.0,
                record.source.local_slot.0,
                record.target.partition_id.0,
                record.target.local_slot.0,
                record.relation_id.partition_id.0,
                record.relation_id.local_slot.0,
            )
        });
        records
    }

    pub fn entity_aspect_versions(
        &self,
        entity_id: crate::identity::data::EntityId,
    ) -> Option<Vec<(AspectKey, u64)>> {
        let partition = self.partition(entity_id.partition_id)?;
        let slot = entity_id.local_slot.0 as usize;
        let versions = partition.entity_arena.aspect_versions.get(slot)?;
        Some(
            versions
                .iter()
                .filter_map(|(symbol, version)| {
                    self.symbols.resolve(*symbol).map(|_| {
                        (AspectKey(InternedString::Symbol(*symbol)), *version)
                    })
                })
                .collect(),
        )
    }

    pub fn relation_aspect_versions(
        &self,
        relation_id: crate::identity::data::RelationId,
    ) -> Option<Vec<(AspectKey, u64)>> {
        let partition = self.partition(relation_id.partition_id)?;
        let slot = relation_id.local_slot.0 as usize;
        let versions = partition.relation_arena.aspect_versions.get(slot)?;
        Some(
            versions
                .iter()
                .filter_map(|(symbol, version)| {
                    self.symbols.resolve(*symbol).map(|_| {
                        (AspectKey(InternedString::Symbol(*symbol)), *version)
                    })
                })
                .collect(),
        )
    }

    pub fn entity_aspects_at_version(
        &self,
        entity_id: crate::identity::data::EntityId,
        version_id: crate::identity::data::VersionId,
    ) -> Option<Vec<AspectKey>> {
        let state = self.current_state();
        let record = self.entity_record_for_id_at_version(&state, entity_id, version_id)?;
        Some(aspect_keys_for_payload(&record.payload, &mut self.symbols.clone()))
    }

    pub fn relation_aspects_at_version(
        &self,
        relation_id: crate::identity::data::RelationId,
        version_id: crate::identity::data::VersionId,
    ) -> Option<Vec<AspectKey>> {
        let state = self.current_state();
        let record = self.relation_record_for_id_at_version(&state, relation_id, version_id)?;
        record
            .payload
            .as_ref()
            .map(|payload| aspect_keys_for_payload(payload, &mut self.symbols.clone()))
    }
}

fn sort_entity_records(records: &mut [crate::storage::data::EntityReadRecord]) {
    records.sort_by_key(|record| {
        (
            record.entity_id.partition_id.0,
            record.entity_id.local_slot.0,
            record.entity_id.generation,
        )
    });
}

fn aspect_keys_for_payload(
    payload: &crate::payloads::data::RecordPayload,
    _symbols: &mut crate::symbols::data::StringInterner,
) -> Vec<AspectKey> {
    let mut aspects = Vec::new();
    match payload {
        crate::payloads::data::RecordPayload::StructuredJson(value) => {
            if let Some(object) = value.as_object() {
                for key in object.keys() {
                    aspects.push(AspectKey(InternedString::Raw(key.clone())));
                }
            }
        }
        crate::payloads::data::RecordPayload::OpaqueBytes(_) => {
            aspects.push(AspectKey(InternedString::Raw("opaque_payload".to_string())));
        }
    }
    aspects.sort_by(|left, right| format!("{left:?}").cmp(&format!("{right:?}")));
    aspects.dedup();
    aspects
}
