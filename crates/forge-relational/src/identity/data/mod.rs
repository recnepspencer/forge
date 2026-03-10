use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PartitionId(pub u32);

impl PartitionId {
    pub const fn main() -> Self {
        Self(0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct LocalSlot(pub u64);

pub type Slot = LocalSlot;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Generation(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct EntityStorageId {
    pub partition_id: PartitionId,
    pub local_slot: LocalSlot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RelationStorageId {
    pub partition_id: PartitionId,
    pub local_slot: LocalSlot,
}

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

    pub const fn storage_id(&self) -> EntityStorageId {
        EntityStorageId {
            partition_id: self.partition_id,
            local_slot: self.local_slot,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RelationId {
    pub partition_id: PartitionId,
    pub local_slot: LocalSlot,
    pub generation: Generation,
}

impl RelationId {
    pub const fn new(partition_id: PartitionId, local_slot: u64, generation: u32) -> Self {
        Self {
            partition_id,
            local_slot: LocalSlot(local_slot),
            generation: Generation(generation),
        }
    }

    pub const fn storage_id(&self) -> RelationStorageId {
        RelationStorageId {
            partition_id: self.partition_id,
            local_slot: self.local_slot,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct VersionId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct LineageId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct KindId(pub u32);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructuralFingerprint {
    pub family: String,
    pub value: String,
}

impl StructuralFingerprint {
    pub fn new(family: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            family: family.into(),
            value: value.into(),
        }
    }
}
