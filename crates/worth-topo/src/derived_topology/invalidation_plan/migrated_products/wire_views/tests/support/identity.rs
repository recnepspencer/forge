use forge_relational::facade::identity::{EntityId, PartitionId, RelationId};

pub(super) fn entity_id(slot: u64) -> EntityId {
    EntityId::new(PartitionId::main(), slot, 1)
}

pub(super) fn relation_id(slot: u64) -> RelationId {
    RelationId::new(PartitionId::main(), slot, 1)
}
