use crate::capabilities::StorageRead;
use crate::identity::data::{EntityId, RecordId, RelationId};
use crate::storage::substrate::{partition_of, EntityRecordKind, RecordKind, RelationRecordKind};

pub(crate) fn entity_exists_in_state(state: &impl StorageRead, entity_id: EntityId) -> bool {
    record_exists_in_state::<EntityRecordKind>(state, entity_id)
}

pub(crate) fn relation_exists_in_state(state: &impl StorageRead, relation_id: RelationId) -> bool {
    record_exists_in_state::<RelationRecordKind>(state, relation_id)
}

fn record_exists_in_state<K: RecordKind>(
    state: &impl StorageRead,
    record_id: RecordId<K::Domain>,
) -> bool {
    state
        .get_partition(partition_of::<K>(&record_id))
        .and_then(|partition| K::arena(partition).get(&record_id))
        .is_some_and(|slot| slot.is_live())
}
