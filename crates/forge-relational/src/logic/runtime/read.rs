use crate::logic::runtime::{
    EntityReadRecord, RecordLifecycleState, RelationReadRecord, RelationalRuntime,
};

use super::state::WorkingState;

impl RelationalRuntime {
    pub(super) fn live_entities_from_state(&self, state: &WorkingState) -> Vec<EntityReadRecord> {
        let mut records = Vec::new();
        for slot in 0..state.entity_arena.generations.len() {
            if state.entity_arena.lifecycle[slot] != RecordLifecycleState::Live {
                continue;
            }
            let kind_id = state.entity_arena.kind_ids[slot].expect("kind id for live entity");
            let kind = self
                .config
                .schema_registry
                .resolve_entity(kind_id)
                .expect("kind resolution for live entity");
            let payload = state.entity_arena.payloads[slot]
                .clone()
                .expect("payload for live entity");
            records.push(EntityReadRecord {
                entity_id: crate::data::identity::EntityId::new(
                    slot as u64,
                    state.entity_arena.generations[slot],
                ),
                kind,
                lifecycle: state.entity_arena.lifecycle[slot],
                payload,
            });
        }
        records
    }

    pub(super) fn live_relations_from_state(
        &self,
        state: &WorkingState,
    ) -> Vec<RelationReadRecord> {
        let mut records = Vec::new();
        for slot in 0..state.relation_arena.generations.len() {
            if state.relation_arena.lifecycle[slot] != RecordLifecycleState::Live {
                continue;
            }
            let kind_id = state.relation_arena.kind_ids[slot].expect("kind id for live relation");
            let kind = self
                .config
                .schema_registry
                .resolve_relation(kind_id)
                .expect("kind resolution for live relation");
            let payload = state.relation_arena.payloads[slot]
                .clone()
                .expect("payload for live relation");
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
                source: endpoints.source,
                target: endpoints.target,
                payload,
            });
        }
        records
    }
}
