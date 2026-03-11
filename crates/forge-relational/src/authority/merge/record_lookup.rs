use crate::capabilities::StorageRead;
use crate::identity::data::{EntityId, RelationId};
use crate::storage::logic::state::{EntityRecordKind, RecordId, RecordKind, RelationRecordKind};

pub(super) fn entity_exists_in_state(state: &impl StorageRead, entity_id: EntityId) -> bool {
    record_exists_in_state::<EntityRecordKind>(state, entity_id)
}

pub(super) fn relation_exists_in_state(
    state: &impl StorageRead,
    relation_id: RelationId,
) -> bool {
    record_exists_in_state::<RelationRecordKind>(state, relation_id)
}

fn record_exists_in_state<K: RecordKind>(
    state: &impl StorageRead,
    record_id: K::Id,
) -> bool {
    state
        .get_partition(record_id.partition_id())
        .and_then(|partition| K::arena(partition).get(&record_id))
        .is_some_and(|slot| slot.is_live())
}
