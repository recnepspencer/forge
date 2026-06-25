use forge_relational::facade::identity::{EntityId, PartitionId, RelationId};

pub(crate) fn entity_id(slot: u64) -> EntityId {
    EntityId::new(PartitionId::main(), slot, 1)
}

pub(crate) fn relation_id(slot: u64) -> RelationId {
    RelationId::new(PartitionId::main(), slot, 1)
}
