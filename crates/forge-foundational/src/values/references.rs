use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ContentRefId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PartitionId(pub u32);

impl PartitionId {
    pub const fn main() -> Self {
        Self(0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct LocalSlot(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Generation(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct EntityId {
    pub partition_id: PartitionId,
    pub local_slot: LocalSlot,
    pub generation: Generation,
}

impl EntityId {
    pub const fn new(partition_id: PartitionId, local_slot: u64, generation: u32) -> Self {
        Self {
            partition_id,
            local_slot: LocalSlot(local_slot),
            generation: Generation(generation),
        }
    }
}
