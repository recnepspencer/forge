use crate::identity::data::VersionBound;
use crate::logic::runtime::RelationalRuntime;
use crate::snapshots::data::{SnapshotHandle, SnapshotInspectionSummary};
use crate::storage::data::{EntityReadRecord, RecordLifecycleState, RelationReadRecord};
use crate::storage::logic::state::{
    DenseSlotBitSet, EntityArena, EntityRecordKind, HistoricalMetadata, PartitionAccess,
    PartitionState, RecordArena, RecordKind, RelationRecordKind, VersionedValue,
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
                .lock()
                .expect("complexity counter lock poisoned")
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
                .lock()
                .expect("complexity counter lock poisoned")
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
        records
    }

    pub(crate) fn visible_entity_slots_from_state(
        &self,
        state: &impl PartitionAccess,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<(crate::identity::data::PartitionId, DenseSlotBitSet)> {
        let mut partitions = Vec::new();
        for partition_id in state.partition_ids() {
            if let Some(entity_slots) =
                self.visible_entity_slots_in_partition_from_state(state, partition_id, version_id)
            {
                partitions.push((partition_id, entity_slots));
            }
        }
        partitions
    }

    pub(crate) fn visible_entity_slots_in_partition_from_state(
        &self,
        state: &impl PartitionAccess,
        partition_id: crate::identity::data::PartitionId,
        version_id: crate::identity::data::VersionId,
    ) -> Option<DenseSlotBitSet> {
        visible_slots_in_partition_from_state::<EntityRecordKind>(
            self,
            state,
            partition_id,
            version_id,
            |runtime, scanned| {
                runtime
                    .instrumentation
                    .complexity_counters
                    .lock()
                    .expect("complexity counter lock poisoned")
                    .visibility_entity_slot_scans += scanned;
            },
        )
    }

    pub(crate) fn visible_relation_slots_from_state(
        &self,
        state: &impl PartitionAccess,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<(crate::identity::data::PartitionId, DenseSlotBitSet)> {
        let mut partitions = Vec::new();
        for partition_id in state.partition_ids() {
            if let Some(relation_slots) =
                self.visible_relation_slots_in_partition_from_state(state, partition_id, version_id)
            {
                partitions.push((partition_id, relation_slots));
            }
        }
        partitions
    }

    pub(crate) fn visible_relation_slots_in_partition_from_state(
        &self,
        state: &impl PartitionAccess,
        partition_id: crate::identity::data::PartitionId,
        version_id: crate::identity::data::VersionId,
    ) -> Option<DenseSlotBitSet> {
        visible_slots_in_partition_from_state::<RelationRecordKind>(
            self,
            state,
            partition_id,
            version_id,
            |runtime, scanned| {
                runtime
                    .instrumentation
                    .complexity_counters
                    .lock()
                    .expect("complexity counter lock poisoned")
                    .visibility_relation_slot_scans += scanned;
            },
        )
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
        record_visible_in_arena_at_version(&partition.relation_arena, slot, version_id)
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
    let metadata =
        visible_metadata(&partition.entity_arena.metadata_history[slot], version_id)?;
    let kind = runtime
        .config
        .schema_registry
        .resolve_entity(metadata.kind_id)
        .ok()?;
    let payload = visible_payload_for_generation(
        &partition.entity_arena.payload_history[slot],
        version_id,
        metadata.generation,
    )?
    .clone();
    Some(EntityReadRecord {
        entity_id: crate::identity::data::EntityId::new(
            partition_id,
            slot as u64,
            metadata.generation,
        ),
        kind,
        lifecycle: historical_lifecycle(
            partition.entity_arena.retired_at[slot],
            partition.entity_arena.lifecycle[slot],
        ),
        created_at_version: metadata.effective_at,
        retired_at_version: metadata.retired_at,
        payload,
    })
}

fn materialize_current_relation_record(
    runtime: &RelationalRuntime,
    partition: &PartitionState,
    partition_id: crate::identity::data::PartitionId,
    slot: usize,
) -> Option<RelationReadRecord> {
    let lifecycle = *partition.relation_arena.lifecycle.get(slot)?;
    if !lifecycle_storage_visible(lifecycle) {
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
    let endpoints = partition.relation_arena.extra.get(slot)?.as_ref()?;
    let payload = partition
        .relation_arena
        .payload_history
        .get(slot)
        .and_then(|history| history.last())
        .map(|entry| entry.value.clone());
    Some(RelationReadRecord {
        relation_id: crate::identity::data::RelationId::new(
            partition_id,
            slot as u64,
            partition.relation_arena.generations[slot],
        ),
        kind,
        lifecycle,
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
    let metadata =
        visible_metadata(&partition.relation_arena.metadata_history[slot], version_id)?;
    let kind = runtime
        .config
        .schema_registry
        .resolve_relation(metadata.kind_id)
        .ok()?;
    let payload = visible_payload_for_generation(
        &partition.relation_arena.payload_history[slot],
        version_id,
        metadata.generation,
    )
    .cloned();
    Some(RelationReadRecord {
        relation_id: crate::identity::data::RelationId::new(
            partition_id,
            slot as u64,
            metadata.generation,
        ),
        kind,
        lifecycle: historical_lifecycle(
            partition.relation_arena.retired_at[slot],
            partition.relation_arena.lifecycle[slot],
        ),
        created_at_version: metadata.effective_at,
        retired_at_version: metadata.retired_at,
        source: metadata.endpoints.source,
        target: metadata.endpoints.target,
        payload,
    })
}

fn visible_payload_for_generation(
    history: &[VersionedValue],
    version_id: crate::identity::data::VersionId,
    generation: u32,
) -> Option<&crate::payloads::data::RecordPayload> {
    let bound = VersionBound::new(version_id);
    let end = history.partition_point(|entry| bound.includes_created(entry.effective_at));
    history[..end]
        .iter()
        .rev()
        .find(|entry| {
            entry.generation == generation
                && bound.includes_created(entry.effective_at)
                && entry.retired_at.is_none_or(|retired| bound.retains_retired(retired))
        })
        .map(|entry| &entry.value)
}

fn visible_metadata<M: HistoricalMetadata>(
    history: &[M],
    version_id: crate::identity::data::VersionId,
) -> Option<&M> {
    let bound = VersionBound::new(version_id);
    let end = history.partition_point(|entry| bound.includes_created(entry.effective_at()));
    history[..end].iter().rev().find(|entry| {
        bound.includes_created(entry.effective_at())
            && entry.retired_at().is_none_or(|retired| bound.retains_retired(retired))
    })
}

fn entity_visible_in_partition_at_version(
    partition: &PartitionState,
    slot: usize,
    version_id: crate::identity::data::VersionId,
) -> bool {
    partition
        .entity_arena
        .metadata_history
        .get(slot)
        .and_then(|history| visible_metadata(history, version_id))
        .is_some()
}

fn relation_visible_in_partition_at_version(
    partition: &PartitionState,
    slot: usize,
    version_id: crate::identity::data::VersionId,
) -> bool {
    record_visible_in_arena_at_version(&partition.relation_arena, slot, version_id)
}

fn record_visible_in_arena_at_version<K: RecordKind>(
    arena: &RecordArena<K>,
    slot: usize,
    version_id: crate::identity::data::VersionId,
) -> bool
where
    K::Meta: HistoricalMetadata,
{
    arena
        .metadata_history
        .get(slot)
        .and_then(|history| visible_metadata(history, version_id))
        .is_some()
}

fn visible_slots_in_partition_from_state<K: RecordKind>(
    runtime: &RelationalRuntime,
    state: &impl PartitionAccess,
    partition_id: crate::identity::data::PartitionId,
    version_id: crate::identity::data::VersionId,
    count_scans: impl Fn(&RelationalRuntime, usize),
) -> Option<DenseSlotBitSet>
where
    K::Meta: HistoricalMetadata,
{
    let current_version = runtime.current_version_id();
    let partition = state.get_partition(partition_id)?;
    let arena = K::arena(partition);
    let mut visible_slots = DenseSlotBitSet::with_capacity(arena.generations.len());
    if version_id == current_version {
        for slot in arena.live_bitset.iter_set_slots() {
            visible_slots.set(slot, true);
        }
    } else {
        count_scans(runtime, arena.generations.len());
        for slot in 0..arena.generations.len() {
            if record_visible_in_arena_at_version(arena, slot, version_id) {
                visible_slots.set(slot, true);
            }
        }
    }
    (visible_slots.count_ones() > 0).then_some(visible_slots)
}

fn lifecycle_storage_visible(lifecycle: RecordLifecycleState) -> bool {
    lifecycle != RecordLifecycleState::Reusable
}

fn historical_lifecycle(
    retired_at: Option<crate::identity::data::VersionId>,
    current_lifecycle: RecordLifecycleState,
) -> RecordLifecycleState {
    if retired_at.is_some() {
        RecordLifecycleState::DeletedRetained
    } else {
        current_lifecycle
    }
}

#[allow(dead_code)]
fn _entity_payload_visible_in_arena_at_version(
    arena: &EntityArena,
    slot: usize,
    version_id: crate::identity::data::VersionId,
) -> bool {
    let bound = VersionBound::new(version_id);
    lifecycle_storage_visible(arena.lifecycle[slot])
        && bound.includes_created(arena.created_at[slot])
        && arena.retired_at[slot].is_none_or(|retired| bound.retains_retired(retired))
}
