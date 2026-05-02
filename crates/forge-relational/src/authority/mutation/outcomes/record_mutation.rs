use crate::identity::data::{EntityId, KindId, RelationId};
use crate::payloads::data::RecordPayload;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum RecordMutation {
    EntityCreated {
        entity_id: EntityId,
        kind_id: KindId,
        payload: RecordPayload,
    },
    EntityUpdated {
        entity_id: EntityId,
        kind_id: KindId,
        old_payload: RecordPayload,
        new_payload: RecordPayload,
    },
    EntityDeleted {
        entity_id: EntityId,
        kind_id: KindId,
        payload: RecordPayload,
    },
    RelationCreated {
        relation_id: RelationId,
        kind_id: KindId,
        source: EntityId,
        target: EntityId,
        payload: Option<RecordPayload>,
    },
    RelationUpdated {
        relation_id: RelationId,
        kind_id: KindId,
        old_source: EntityId,
        old_target: EntityId,
        new_source: EntityId,
        new_target: EntityId,
        payload: Option<RecordPayload>,
    },
    RelationDeleted {
        relation_id: RelationId,
        kind_id: KindId,
        source: EntityId,
        target: EntityId,
        payload: Option<RecordPayload>,
    },
    RelationRetainedForAudit {
        relation_id: RelationId,
        kind_id: KindId,
        source: EntityId,
        target: EntityId,
        payload: Option<RecordPayload>,
    },
}
