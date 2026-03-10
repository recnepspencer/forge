use crate::logic::runtime::{
    EntityReadRecord, RecordLifecycleState, RelationReadRecord, RelationalRuntime,
};

use super::state::{EntityArena, RelationArena, VersionedValue, WorkingState};

impl RelationalRuntime {
    pub(super) fn visible_entities_from_state(
        &self,
        state: &WorkingState,
        version_id: crate::data::identity::VersionId,
    ) -> Vec<EntityReadRecord> {
        self.complexity_counters
            .borrow_mut()
            .visibility_entity_slot_scans += state.entity_arena.generations.len();
        let mut records = Vec::new();
        for slot in 0..state.entity_arena.generations.len() {
            if !entity_visible_at_version(state, slot, version_id) {
                continue;
            }
            let kind_id = state.entity_arena.kind_ids[slot].expect("kind id for visible entity");
            let kind = self
                .config
                .schema_registry
                .resolve_entity(kind_id)
                .expect("kind resolution for visible entity");
            let payload = visible_payload(&state.entity_arena.payload_history[slot], version_id)
                .expect("payload for visible entity")
                .clone();
            records.push(EntityReadRecord {
                entity_id: crate::data::identity::EntityId::new(
                    state.entity_arena.partition_ids[slot],
                    slot as u64,
                    state.entity_arena.generations[slot],
                ),
                kind,
                lifecycle: state.entity_arena.lifecycle[slot],
                created_at_version: state.entity_arena.created_at[slot],
                retired_at_version: state.entity_arena.retired_at[slot],
                payload,
            });
        }
        self.complexity_counters
            .borrow_mut()
            .visible_entity_records_materialized += records.len();
        records
    }

    pub(super) fn visible_relations_from_state(
        &self,
        state: &WorkingState,
        version_id: crate::data::identity::VersionId,
    ) -> Vec<RelationReadRecord> {
        self.complexity_counters
            .borrow_mut()
            .visibility_relation_slot_scans += state.relation_arena.generations.len();
        let mut records = Vec::new();
        for slot in 0..state.relation_arena.generations.len() {
            if !relation_visible_at_version(state, slot, version_id) {
                continue;
            }
            let kind_id = state.relation_arena.kind_ids[slot].expect("kind id for visible relation");
            let kind = self
                .config
                .schema_registry
                .resolve_relation(kind_id)
                .expect("kind resolution for visible relation");
            let payload = state
                .relation_arena
                .payload_history
                .get(&slot)
                .and_then(|history| visible_payload(history, version_id))
                .cloned();
            let endpoints = state.relation_arena.endpoints[slot]
                .as_ref()
                .expect("endpoints for visible relation");
            records.push(RelationReadRecord {
                relation_id: crate::data::identity::RelationId::new(
                    state.relation_arena.partition_ids[slot],
                    slot as u64,
                    state.relation_arena.generations[slot],
                ),
                kind,
                lifecycle: state.relation_arena.lifecycle[slot],
                created_at_version: state.relation_arena.created_at[slot],
                retired_at_version: state.relation_arena.retired_at[slot],
                source: endpoints.source,
                target: endpoints.target,
                payload,
            });
        }
        self.complexity_counters
            .borrow_mut()
            .visible_relation_records_materialized += records.len();
        records
    }

    pub(super) fn relation_visible_at_version(
        &self,
        relation_id: crate::data::identity::RelationId,
        version_id: crate::data::identity::VersionId,
    ) -> bool {
        let slot = relation_id.local_slot.0 as usize;
        if slot >= self.relation_arena.generations.len() {
            return false;
        }
        relation_visible_in_arena_at_version(&self.relation_arena, slot, version_id)
    }
}

fn visible_payload(
    history: &[VersionedValue],
    version_id: crate::data::identity::VersionId,
) -> Option<&crate::data::payload::RecordPayload> {
    history
        .iter()
        .find(|entry| {
            entry.effective_at <= version_id
                && entry.retired_at.is_none_or(|retired| version_id < retired)
        })
        .map(|entry| &entry.value)
}

fn entity_visible_at_version(
    state: &WorkingState,
    slot: usize,
    version_id: crate::data::identity::VersionId,
) -> bool {
    lifecycle_storage_visible(state.entity_arena.lifecycle[slot])
        && state.entity_arena.created_at[slot] <= version_id
        && state.entity_arena.retired_at[slot].is_none_or(|retired| version_id < retired)
}

fn relation_visible_at_version(
    state: &WorkingState,
    slot: usize,
    version_id: crate::data::identity::VersionId,
) -> bool {
    relation_visible_in_arena_at_version(&state.relation_arena, slot, version_id)
}

fn relation_visible_in_arena_at_version(
    arena: &RelationArena,
    slot: usize,
    version_id: crate::data::identity::VersionId,
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
    version_id: crate::data::identity::VersionId,
) -> bool {
    lifecycle_storage_visible(arena.lifecycle[slot])
        && arena.created_at[slot] <= version_id
        && arena.retired_at[slot].is_none_or(|retired| version_id < retired)
}
