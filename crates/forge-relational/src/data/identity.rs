use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Slot(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Generation(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct EntityId {
    pub slot: Slot,
    pub generation: Generation,
}

impl EntityId {
    pub const fn new(slot: u64, generation: u32) -> Self {
        Self {
            slot: Slot(slot),
            generation: Generation(generation),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RelationId {
    pub slot: Slot,
    pub generation: Generation,
}

impl RelationId {
    pub const fn new(slot: u64, generation: u32) -> Self {
        Self {
            slot: Slot(slot),
            generation: Generation(generation),
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
