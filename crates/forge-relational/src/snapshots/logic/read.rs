use crate::logic::runtime::RelationalRuntime;
use crate::snapshots::data::{SnapshotHandle, SnapshotInspectionSummary};
use crate::storage::data::{EntityReadRecord, RecordLifecycleState, RelationReadRecord};
use crate::storage::logic::state::{
    EntityArena, PartitionAccess, PartitionState, RelationArena, VersionedValue,
};

impl RelationalRuntime {
    pub fn inspect_snapshot(&self, handle: &SnapshotHandle) -> Option<SnapshotInspectionSummary> {
        if let Some(binding) = self.snapshots.active.get(&handle.snapshot_id) {
            let state = self.read_or_reconstruct_visibility_state(
                binding.version_id,
                !self.config.visibility_cache_policy.protect_active_snapshots,
            )?;
            return Some(SnapshotInspectionSummary {
                version_id: binding.version_id,
                entity_count: state.pinned_entity_count,
                relation_count: state.pinned_relation_count,
                pinned_entity_count: state.pinned_entity_count,
                pinned_relation_count: state.pinned_relation_count,
            });
        }
        let version_id = *self.snapshots.published_handles.get(&handle.snapshot_id)?;
        let read_view = self.read_version(version_id);
        Some(SnapshotInspectionSummary {
            version_id,
            entity_count: read_view.entities.len(),
            relation_count: read_view.relations.len(),
            pinned_entity_count: 0,
            pinned_relation_count: 0,
        })
    }

    pub(crate) fn entity_record_for_id_at_version(
        &self,
        state: &impl PartitionAccess,
        entity_id: crate::identity::data::EntityId,
        version_id: crate::identity::data::VersionId,
    ) -> Option<EntityReadRecord> {
        let partition = state.get_partition(entity_id.partition_id)?;
        let slot = entity_id.local_slot.0 as usize;
        if version_id == self.current_version_id() {
            materialize_current_entity_record(self, partition, entity_id.partition_id, slot)
        } else {
            materialize_entity_record_at_version(
                self,
                partition,
                entity_id.partition_id,
                slot,
                version_id,
            )
        }
    }

    pub(crate) fn relation_record_for_id_at_version(
        &self,
        state: &impl PartitionAccess,
        relation_id: crate::identity::data::RelationId,
        version_id: crate::identity::data::VersionId,
    ) -> Option<RelationReadRecord> {
        let partition = state.get_partition(relation_id.partition_id)?;
        let slot = relation_id.local_slot.0 as usize;
        if version_id == self.current_version_id() {
            materialize_current_relation_record(self, partition, relation_id.partition_id, slot)
        } else {
            materialize_relation_record_at_version(
                self,
                partition,
                relation_id.partition_id,
                slot,
                version_id,
            )
        }
    }

    pub(crate) fn visible_entities_of_kind_in_partition_from_state(
        &self,
        state: &impl PartitionAccess,
        partition_id: crate::identity::data::PartitionId,
        kind_id: crate::identity::data::KindId,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<EntityReadRecord> {
        let mut records = Vec::new();
        let current_version = self.current_version_id();
        let Some(partition) = state.get_partition(partition_id) else {
            return records;
        };
        if version_id == current_version {
            for slot in partition.entity_arena.live_bitset.iter_set_slots() {
                if partition.entity_arena.kind_ids.get(slot).copied().flatten() != Some(kind_id) {
                    continue;
                }
                if let Some(record) =
                    materialize_current_entity_record(self, partition, partition_id, slot)
                {
                    records.push(record);
                }
            }
        } else {
            self.instrumentation
                .complexity_counters
                .borrow_mut()
                .visibility_entity_slot_scans += partition.entity_arena.generations.len();
            for slot in 0..partition.entity_arena.generations.len() {
                if partition.entity_arena.kind_ids.get(slot).copied().flatten() != Some(kind_id) {
                    continue;
                }
                if let Some(record) = materialize_entity_record_at_version(
                    self,
                    partition,
                    partition_id,
                    slot,
                    version_id,
                ) {
                    records.push(record);
                }
            }
        }
        self.instrumentation
            .complexity_counters
            .borrow_mut()
            .visible_entity_records_materialized += records.len();
        records
    }

    pub(crate) fn visible_relations_of_kind_in_partition_from_state(
        &self,
        state: &impl PartitionAccess,
        partition_id: crate::identity::data::PartitionId,
        kind_id: crate::identity::data::KindId,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<RelationReadRecord> {
        let mut records = Vec::new();
        let current_version = self.current_version_id();
        let Some(partition) = state.get_partition(partition_id) else {
            return records;
        };
        if version_id == current_version {
            for slot in partition.relation_arena.live_bitset.iter_set_slots() {
                if partition
                    .relation_arena
                    .kind_ids
                    .get(slot)
                    .copied()
                    .flatten()
                    != Some(kind_id)
                {
                    continue;
                }
                if let Some(record) =
                    materialize_current_relation_record(self, partition, partition_id, slot)
                {
                    records.push(record);
                }
            }
        } else {
            self.instrumentation
                .complexity_counters
                .borrow_mut()
                .visibility_relation_slot_scans += partition.relation_arena.generations.len();
            for slot in 0..partition.relation_arena.generations.len() {
                if partition
                    .relation_arena
                    .kind_ids
                    .get(slot)
                    .copied()
                    .flatten()
                    != Some(kind_id)
                {
                    continue;
                }
                if let Some(record) = materialize_relation_record_at_version(
                    self,
                    partition,
                    partition_id,
                    slot,
                    version_id,
                ) {
                    records.push(record);
                }
            }
        }
        self.instrumentation
            .complexity_counters
            .borrow_mut()
            .visible_relation_records_materialized += records.len();
        records
    }

    pub(crate) fn visible_entities_from_state(
        &self,
        state: &impl PartitionAccess,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<EntityReadRecord> {
        let mut records = Vec::new();
        for partition_id in state.partition_ids() {
            records.extend(self.visible_entities_in_partition_from_state(
                state,
                partition_id,
                version_id,
            ));
        }
        self.instrumentation
            .complexity_counters
            .borrow_mut()
            .visible_entity_records_materialized += records.len();
        records
    }

    pub(crate) fn visible_entities_in_partition_from_state(
        &self,
        state: &impl PartitionAccess,
        partition_id: crate::identity::data::PartitionId,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<EntityReadRecord> {
        let mut records = Vec::new();
        let current_version = self.current_version_id();
        let Some(partition) = state.get_partition(partition_id) else {
            return records;
        };
        if version_id == current_version {
            for slot in partition.entity_arena.live_bitset.iter_set_slots() {
                if let Some(record) =
                    materialize_current_entity_record(self, partition, partition_id, slot)
                {
                    records.push(record);
                }
            }
        } else {
            self.instrumentation
                .complexity_counters
                .borrow_mut()
                .visibility_entity_slot_scans += partition.entity_arena.generations.len();
            for slot in 0..partition.entity_arena.generations.len() {
                if let Some(record) = materialize_entity_record_at_version(
                    self,
                    partition,
                    partition_id,
                    slot,
                    version_id,
                ) {
                    records.push(record);
                }
            }
        }
        records
    }

    pub(crate) fn visible_relations_from_state(
        &self,
        state: &impl PartitionAccess,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<RelationReadRecord> {
        let mut records = Vec::new();
        for partition_id in state.partition_ids() {
            records.extend(self.visible_relations_in_partition_from_state(
                state,
                partition_id,
                version_id,
            ));
        }
        self.instrumentation
            .complexity_counters
            .borrow_mut()
            .visible_relation_records_materialized += records.len();
        records
    }

    pub(crate) fn visible_relations_in_partition_from_state(
        &self,
        state: &impl PartitionAccess,
        partition_id: crate::identity::data::PartitionId,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<RelationReadRecord> {
        let mut records = Vec::new();
        let current_version = self.current_version_id();
        let Some(partition) = state.get_partition(partition_id) else {
            return records;
        };
        if version_id == current_version {
            for slot in partition.relation_arena.live_bitset.iter_set_slots() {
                if let Some(record) =
                    materialize_current_relation_record(self, partition, partition_id, slot)
                {
                    records.push(record);
                }
            }
        } else {
            self.instrumentation
                .complexity_counters
                .borrow_mut()
                .visibility_relation_slot_scans += partition.relation_arena.generations.len();
            for slot in 0..partition.relation_arena.generations.len() {
                if let Some(record) = materialize_relation_record_at_version(
                    self,
                    partition,
                    partition_id,
                    slot,
                    version_id,
                ) {
                    records.push(record);
                }
            }
        }
        records
    }

    pub(crate) fn relation_visible_at_version(
        &self,
        relation_id: crate::identity::data::RelationId,
        version_id: crate::identity::data::VersionId,
    ) -> bool {
        let Some(partition) = self.partition(relation_id.partition_id) else {
            return false;
        };
        let slot = relation_id.local_slot.0 as usize;
        if slot >= partition.relation_arena.generations.len() {
            return false;
        }
        relation_visible_in_arena_at_version(&partition.relation_arena, slot, version_id)
    }
}

fn materialize_current_entity_record(
    runtime: &RelationalRuntime,
    partition: &PartitionState,
    partition_id: crate::identity::data::PartitionId,
    slot: usize,
) -> Option<EntityReadRecord> {
    if partition.entity_arena.lifecycle.get(slot) != Some(&RecordLifecycleState::Live) {
        return None;
    }
    let kind_id = partition
        .entity_arena
        .kind_ids
        .get(slot)
        .copied()
        .flatten()?;
    let kind = runtime
        .config
        .schema_registry
        .resolve_entity(kind_id)
        .ok()?;
    let payload = partition
        .entity_arena
        .payload_history
        .get(slot)?
        .last()?
        .value
        .clone();
    Some(EntityReadRecord {
        entity_id: crate::identity::data::EntityId::new(
            partition_id,
            slot as u64,
            partition.entity_arena.generations[slot],
        ),
        kind,
        lifecycle: partition.entity_arena.lifecycle[slot],
        created_at_version: partition.entity_arena.created_at[slot],
        retired_at_version: partition.entity_arena.retired_at[slot],
        payload,
    })
}

fn materialize_entity_record_at_version(
    runtime: &RelationalRuntime,
    partition: &PartitionState,
    partition_id: crate::identity::data::PartitionId,
    slot: usize,
    version_id: crate::identity::data::VersionId,
) -> Option<EntityReadRecord> {
    if !entity_visible_in_partition_at_version(partition, slot, version_id) {
        return None;
    }
    let kind_id = partition.entity_arena.kind_ids[slot]?;
    let kind = runtime
        .config
        .schema_registry
        .resolve_entity(kind_id)
        .ok()?;
    let payload =
        visible_payload(&partition.entity_arena.payload_history[slot], version_id)?.clone();
    Some(EntityReadRecord {
        entity_id: crate::identity::data::EntityId::new(
            partition_id,
            slot as u64,
            partition.entity_arena.generations[slot],
        ),
        kind,
        lifecycle: partition.entity_arena.lifecycle[slot],
        created_at_version: partition.entity_arena.created_at[slot],
        retired_at_version: partition.entity_arena.retired_at[slot],
        payload,
    })
}

fn materialize_current_relation_record(
    runtime: &RelationalRuntime,
    partition: &PartitionState,
    partition_id: crate::identity::data::PartitionId,
    slot: usize,
) -> Option<RelationReadRecord> {
    if partition.relation_arena.lifecycle.get(slot) != Some(&RecordLifecycleState::Live) {
        return None;
    }
    let kind_id = partition
        .relation_arena
        .kind_ids
        .get(slot)
        .copied()
        .flatten()?;
    let kind = runtime
        .config
        .schema_registry
        .resolve_relation(kind_id)
        .ok()?;
    let endpoints = partition.relation_arena.endpoints.get(slot)?.as_ref()?;
    let payload = partition
        .relation_arena
        .payload_history
        .get(&slot)
        .and_then(|history| history.last())
        .map(|entry| entry.value.clone());
    Some(RelationReadRecord {
        relation_id: crate::identity::data::RelationId::new(
            partition_id,
            slot as u64,
            partition.relation_arena.generations[slot],
        ),
        kind,
        lifecycle: partition.relation_arena.lifecycle[slot],
        created_at_version: partition.relation_arena.created_at[slot],
        retired_at_version: partition.relation_arena.retired_at[slot],
        source: endpoints.source,
        target: endpoints.target,
        payload,
    })
}

fn materialize_relation_record_at_version(
    runtime: &RelationalRuntime,
    partition: &PartitionState,
    partition_id: crate::identity::data::PartitionId,
    slot: usize,
    version_id: crate::identity::data::VersionId,
) -> Option<RelationReadRecord> {
    if !relation_visible_in_partition_at_version(partition, slot, version_id) {
        return None;
    }
    let kind_id = partition.relation_arena.kind_ids[slot]?;
    let kind = runtime
        .config
        .schema_registry
        .resolve_relation(kind_id)
        .ok()?;
    let payload = partition
        .relation_arena
        .payload_history
        .get(&slot)
        .and_then(|history| visible_payload(history, version_id))
        .cloned();
    let endpoints = partition.relation_arena.endpoints[slot].as_ref()?;
    Some(RelationReadRecord {
        relation_id: crate::identity::data::RelationId::new(
            partition_id,
            slot as u64,
            partition.relation_arena.generations[slot],
        ),
        kind,
        lifecycle: partition.relation_arena.lifecycle[slot],
        created_at_version: partition.relation_arena.created_at[slot],
        retired_at_version: partition.relation_arena.retired_at[slot],
        source: endpoints.source,
        target: endpoints.target,
        payload,
    })
}

fn visible_payload(
    history: &[VersionedValue],
    version_id: crate::identity::data::VersionId,
) -> Option<&crate::payloads::data::RecordPayload> {
    history
        .iter()
        .find(|entry| {
            entry.effective_at <= version_id
                && entry.retired_at.is_none_or(|retired| version_id < retired)
        })
        .map(|entry| &entry.value)
}

fn entity_visible_in_partition_at_version(
    partition: &PartitionState,
    slot: usize,
    version_id: crate::identity::data::VersionId,
) -> bool {
    lifecycle_storage_visible(partition.entity_arena.lifecycle[slot])
        && partition.entity_arena.created_at[slot] <= version_id
        && partition.entity_arena.retired_at[slot].is_none_or(|retired| version_id < retired)
}

fn relation_visible_in_partition_at_version(
    partition: &PartitionState,
    slot: usize,
    version_id: crate::identity::data::VersionId,
) -> bool {
    relation_visible_in_arena_at_version(&partition.relation_arena, slot, version_id)
}

fn relation_visible_in_arena_at_version(
    arena: &RelationArena,
    slot: usize,
    version_id: crate::identity::data::VersionId,
) -> bool {
    lifecycle_storage_visible(arena.lifecycle[slot])
        && arena.created_at[slot] <= version_id
        && arena.retired_at[slot].is_none_or(|retired| version_id < retired)
}

fn lifecycle_storage_visible(lifecycle: RecordLifecycleState) -> bool {
    lifecycle != RecordLifecycleState::Reusable
}

#[allow(dead_code)]
fn _entity_payload_visible_in_arena_at_version(
    arena: &EntityArena,
    slot: usize,
    version_id: crate::identity::data::VersionId,
) -> bool {
    lifecycle_storage_visible(arena.lifecycle[slot])
        && arena.created_at[slot] <= version_id
        && arena.retired_at[slot].is_none_or(|retired| version_id < retired)
}
