use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

use super::{Generation, LocalSlot, PartitionId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EntityDomain {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RelationDomain {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct StorageId<K> {
    pub partition_id: PartitionId,
    pub local_slot: LocalSlot,
    #[serde(skip)]
    _marker: PhantomData<K>,
}

impl<K> StorageId<K> {
    pub const fn new(partition_id: PartitionId, local_slot: u64) -> Self {
        Self {
            partition_id,
            local_slot: LocalSlot(local_slot),
            _marker: PhantomData,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RecordId<K> {
    pub partition_id: PartitionId,
    pub local_slot: LocalSlot,
    pub generation: Generation,
    #[serde(skip)]
    _marker: PhantomData<K>,
}

impl<K> RecordId<K> {
    pub const fn new(partition_id: PartitionId, local_slot: u64, generation: u32) -> Self {
        Self {
            partition_id,
            local_slot: LocalSlot(local_slot),
            generation: Generation(generation),
            _marker: PhantomData,
        }
    }

    pub const fn storage_id(&self) -> StorageId<K> {
        StorageId {
            partition_id: self.partition_id,
            local_slot: self.local_slot,
            _marker: PhantomData,
        }
    }
}

pub type EntityId = RecordId<EntityDomain>;
pub type RelationId = RecordId<RelationDomain>;
pub type EntityStorageId = StorageId<EntityDomain>;
pub type RelationStorageId = StorageId<RelationDomain>;
