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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Generation(pub u32);

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
