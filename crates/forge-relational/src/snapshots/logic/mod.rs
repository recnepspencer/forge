use crate::logic::runtime::RelationalRuntime;
use crate::storage::data::{EntityReadRecord, RecordLifecycleState, RelationReadRecord};

use crate::storage::logic::state::{
    EntityArena, PartitionAccess, PartitionState, RelationArena, VersionedValue,
};

impl RelationalRuntime {
    pub(crate) fn visible_entities_from_state(
        &self,
        state: &impl PartitionAccess,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<EntityReadRecord> {
        let mut records = Vec::new();
        for partition_id in state.partition_ids() {
            let partition = state
                .get_partition(partition_id)
                .expect("partition visible during entity scan");
            self.complexity_counters
                .borrow_mut()
                .visibility_entity_slot_scans += partition.entity_arena.generations.len();
            for slot in 0..partition.entity_arena.generations.len() {
                if !entity_visible_in_partition_at_version(partition, slot, version_id) {
                    continue;
                }
                let kind_id =
                    partition.entity_arena.kind_ids[slot].expect("kind id for visible entity");
                let kind = self
                    .config
                    .schema_registry
                    .resolve_entity(kind_id)
                    .expect("kind resolution for visible entity");
                let payload =
                    visible_payload(&partition.entity_arena.payload_history[slot], version_id)
                        .expect("payload for visible entity")
                        .clone();
                records.push(EntityReadRecord {
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
                });
            }
        }
        self.complexity_counters
            .borrow_mut()
            .visible_entity_records_materialized += records.len();
        records
    }

    pub(crate) fn visible_relations_from_state(
        &self,
        state: &impl PartitionAccess,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<RelationReadRecord> {
        let mut records = Vec::new();
        for partition_id in state.partition_ids() {
            let partition = state
                .get_partition(partition_id)
                .expect("partition visible during relation scan");
            self.complexity_counters
                .borrow_mut()
                .visibility_relation_slot_scans += partition.relation_arena.generations.len();
            for slot in 0..partition.relation_arena.generations.len() {
                if !relation_visible_in_partition_at_version(partition, slot, version_id) {
                    continue;
                }
                let kind_id =
                    partition.relation_arena.kind_ids[slot].expect("kind id for visible relation");
                let kind = self
                    .config
                    .schema_registry
                    .resolve_relation(kind_id)
                    .expect("kind resolution for visible relation");
                let payload = partition
                    .relation_arena
                    .payload_history
                    .get(&slot)
                    .and_then(|history| visible_payload(history, version_id))
                    .cloned();
                let endpoints = partition.relation_arena.endpoints[slot]
                    .as_ref()
                    .expect("endpoints for visible relation");
                records.push(RelationReadRecord {
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
                });
            }
        }
        self.complexity_counters
            .borrow_mut()
            .visible_relation_records_materialized += records.len();
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
