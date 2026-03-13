use crate::identity::data::{EntityId, KindId, PartitionId, RelationId};
use crate::payloads::data::RecordPayload;

use super::{MutationEvent, RecordMutation};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct MutationOutcome {
    pub(crate) changes: Vec<RecordMutation>,
    pub(crate) events: Vec<MutationEvent>,
}

impl MutationOutcome {
    pub(crate) fn entity_created(
        entity_id: EntityId,
        kind_id: KindId,
        payload: RecordPayload,
    ) -> Self {
        let mut outcome = Self::default();
        outcome.record_change(RecordMutation::EntityCreated { entity_id, payload });
        outcome.record_event(MutationEvent::EntityCreated { entity_id, kind_id });
        outcome
    }

    pub(crate) fn bulk_entities_created(partition_id: PartitionId, kind_id: KindId) -> Self {
        let mut outcome = Self::default();
        outcome.record_event(MutationEvent::BulkEntitiesCreated {
            partition_id,
            kind_id,
            count: 0,
        });
        outcome
    }

    pub(crate) fn entity_updated(entity_id: EntityId, payload: RecordPayload) -> Self {
        let mut outcome = Self::default();
        outcome.record_change(RecordMutation::EntityUpdated { entity_id, payload });
        outcome.record_event(MutationEvent::EntityUpdated { entity_id });
        outcome
    }

    pub(crate) fn entity_deleted(entity_id: EntityId) -> Self {
        let mut outcome = Self::default();
        outcome.record_event(MutationEvent::EntityDeleted { entity_id });
        outcome
    }

    pub(crate) fn entity_replaced(
        replaced_entity_id: EntityId,
        replacement_entity_id: EntityId,
        kind_id: KindId,
        payload: RecordPayload,
    ) -> Self {
        let mut outcome = Self::default();
        outcome.record_change(RecordMutation::EntityCreated {
            entity_id: replacement_entity_id,
            payload,
        });
        outcome.record_event(MutationEvent::EntityReplaced {
            replaced_entity_id,
            replacement_entity_id,
            kind_id,
        });
        outcome
    }

    pub(crate) fn relation_created(
        relation_id: RelationId,
        source: EntityId,
        target: EntityId,
        kind_id: KindId,
        payload: Option<RecordPayload>,
    ) -> Self {
        let mut outcome = Self::default();
        outcome.record_change(RecordMutation::RelationCreated {
            relation_id,
            source,
            target,
            payload,
        });
        outcome.record_event(MutationEvent::RelationCreated {
            relation_id,
            source,
            target,
            kind_id,
        });
        outcome
    }

    pub(crate) fn bulk_relations_created(partition_id: PartitionId, kind_id: KindId) -> Self {
        let mut outcome = Self::default();
        outcome.record_event(MutationEvent::BulkRelationsCreated {
            partition_id,
            kind_id,
            count: 0,
        });
        outcome
    }

    pub(crate) fn relation_deleted(relation_id: RelationId) -> Self {
        let mut outcome = Self::default();
        outcome.record_event(MutationEvent::RelationDeleted { relation_id });
        outcome
    }

    pub(crate) fn record_change(&mut self, change: RecordMutation) {
        self.changes.push(change);
    }

    pub(crate) fn record_event(&mut self, event: MutationEvent) {
        self.events.push(event);
    }

    pub(crate) fn extend(&mut self, other: Self) {
        self.changes.extend(other.changes);
        self.events.extend(other.events);
    }

    pub(crate) fn set_last_event_count(&mut self, count: usize) {
        if let Some(
            MutationEvent::BulkEntitiesCreated {
                count: event_count, ..
            }
            | MutationEvent::BulkRelationsCreated {
                count: event_count, ..
            },
        ) = self.events.last_mut()
        {
            *event_count = count;
        }
    }
}
