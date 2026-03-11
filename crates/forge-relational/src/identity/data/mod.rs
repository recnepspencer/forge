use serde::{Deserialize, Serialize};

use crate::symbols::data::Symbol;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VersionBound(pub VersionId);

impl VersionBound {
    pub const fn new(version: VersionId) -> Self {
        Self(version)
    }

    pub const fn version(self) -> VersionId {
        self.0
    }

    pub const fn includes_created(self, created_at: VersionId) -> bool {
        created_at.0 <= self.0.0
    }

    pub const fn retains_retired(self, retired_at: VersionId) -> bool {
        retired_at.0 > self.0.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct LineageId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct KindId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct StructuralFingerprint {
    pub family: Symbol,
    pub value: u128,
}

impl StructuralFingerprint {
    pub const fn new(family: Symbol, value: u128) -> Self {
        Self { family, value }
    }
}
