use crate::identity::data::{EntityId, RelationId};
use crate::logic::runtime::RecordLifecycleState;
use crate::storage::logic::state::{
    EntityRecordKind, PartitionAccess, RecordId, RecordKind, RelationRecordKind,
};

pub(super) fn entity_exists_in_state(state: &impl PartitionAccess, entity_id: EntityId) -> bool {
    record_exists_in_state::<EntityRecordKind>(state, entity_id)
}

pub(super) fn relation_exists_in_state(
    state: &impl PartitionAccess,
    relation_id: RelationId,
) -> bool {
    record_exists_in_state::<RelationRecordKind>(state, relation_id)
}

fn record_exists_in_state<K: RecordKind>(
    state: &impl PartitionAccess,
    record_id: K::Id,
) -> bool {
    let slot = record_id.local_slot();
    state.get_partition(record_id.partition_id()).is_some_and(|partition| {
        let arena = K::arena(partition);
        arena.generations.get(slot) == Some(&record_id.generation())
            && arena.lifecycle.get(slot) == Some(&RecordLifecycleState::Live)
    })
}
