use crate::identity::data::{EntityId, RelationId};
use crate::payloads::data::RecordPayload;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum RecordMutation {
    EntityCreated {
        entity_id: EntityId,
        payload: RecordPayload,
    },
    EntityUpdated {
        entity_id: EntityId,
        payload: RecordPayload,
    },
    EntityDeleted {
        entity_id: EntityId,
    },
    RelationCreated {
        relation_id: RelationId,
        source: EntityId,
        target: EntityId,
        payload: Option<RecordPayload>,
    },
    RelationDeleted {
        relation_id: RelationId,
        source: EntityId,
        target: EntityId,
    },
    RelationRetainedForAudit {
        relation_id: RelationId,
        source: EntityId,
        target: EntityId,
        payload: Option<RecordPayload>,
    },
}
