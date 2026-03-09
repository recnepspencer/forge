use crate::logic::runtime::{
    EntityReadRecord, RecordLifecycleState, RelationReadRecord, RelationalRuntime,
};

use super::state::{VersionedValue, WorkingState};

impl RelationalRuntime {
    pub(super) fn visible_entities_from_state(
        &self,
        state: &WorkingState,
        version_id: crate::data::identity::VersionId,
    ) -> Vec<EntityReadRecord> {
        let mut records = Vec::new();
        for slot in 0..state.entity_arena.generations.len() {
            if !entity_visible_at_version(state, slot, version_id) {
                continue;
            }
            let kind_id = state.entity_arena.kind_ids[slot].expect("kind id for live entity");
            let kind = self
                .config
                .schema_registry
                .resolve_entity(kind_id)
                .expect("kind resolution for live entity");
            let payload = visible_payload(&state.entity_arena.payload_history[slot], version_id)
                .expect("payload for visible entity")
                .clone();
            records.push(EntityReadRecord {
                entity_id: crate::data::identity::EntityId::new(
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
        records
    }

    pub(super) fn visible_relations_from_state(
        &self,
        state: &WorkingState,
        version_id: crate::data::identity::VersionId,
    ) -> Vec<RelationReadRecord> {
        let mut records = Vec::new();
        for slot in 0..state.relation_arena.generations.len() {
            if !relation_visible_at_version(state, slot, version_id) {
                continue;
            }
            let kind_id = state.relation_arena.kind_ids[slot].expect("kind id for live relation");
            let kind = self
                .config
                .schema_registry
                .resolve_relation(kind_id)
                .expect("kind resolution for live relation");
            let payload = visible_payload(&state.relation_arena.payload_history[slot], version_id)
                .expect("payload for visible relation")
                .clone();
            let endpoints = state.relation_arena.endpoints[slot]
                .as_ref()
                .expect("endpoints for live relation");
            records.push(RelationReadRecord {
                relation_id: crate::data::identity::RelationId::new(
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
        records
    }
}

fn visible_payload(
    history: &[VersionedValue],
    version_id: crate::data::identity::VersionId,
) -> Option<&serde_json::Value> {
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
    lifecycle_storage_visible(state.relation_arena.lifecycle[slot])
        && state.relation_arena.created_at[slot] <= version_id
        && state.relation_arena.retired_at[slot].is_none_or(|retired| version_id < retired)
}

fn lifecycle_storage_visible(lifecycle: RecordLifecycleState) -> bool {
    lifecycle != RecordLifecycleState::Reusable
}
