use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::aspect::{Aspect, MAX_ASPECTS};
use super::mask::AspectMask;
use crate::data::output::{ChangedRegion, PartitionSubscription, PartitionToken};

/// Per-aspect version counters carried by each signal node.
///
/// Embedding runtimes assign meaning to aspect slots. `forge-signal` only
/// provides deterministic storage and comparison mechanics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AspectVersion {
    slots: [u64; MAX_ASPECTS],
}

impl AspectVersion {
    /// Create a new aspect version with all counters at zero.
    pub const fn zero() -> Self {
        Self {
            slots: [0; MAX_ASPECTS],
        }
    }

    /// Create a new aspect version from a full slot array.
    pub const fn from_slots(slots: [u64; MAX_ASPECTS]) -> Self {
        Self { slots }
    }

    /// Create a new aspect version from explicit slot/value pairs.
    pub fn from_updates<const N: usize>(updates: [(Aspect, u64); N]) -> Self {
        let mut version = Self::zero();
        let mut i = 0;
        while i < N {
            let (aspect, value) = updates[i];
            version.slots[aspect.index()] = value;
            i += 1;
        }
        version
    }

    /// Read the version for a specific aspect.
    pub const fn get(self, aspect: Aspect) -> u64 {
        self.slots[aspect.index()]
    }

    /// Return a copy with one aspect set to an explicit value.
    pub fn with(mut self, aspect: Aspect, value: u64) -> Self {
        self.slots[aspect.index()] = value;
        self
    }

    /// Bump one aspect version by one.
    pub fn bump(mut self, aspect: Aspect) -> Self {
        self.slots[aspect.index()] += 1;
        self
    }

    /// Bump all aspects included in the provided mask.
    pub fn bump_mask(mut self, mask: AspectMask) -> Self {
        let mut bits = mask.bits();
        while bits != 0 {
            let index = bits.trailing_zeros() as usize;
            self.slots[index] += 1;
            bits &= bits - 1;
        }
        self
    }

    /// Borrow all aspect slots.
    pub const fn slots(&self) -> &[u64; MAX_ASPECTS] {
        &self.slots
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PartitionVersionMap {
    global: AspectVersion,
    #[serde(default)]
    partitions: BTreeMap<PartitionToken, AspectVersion>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AspectVersionHeader {
    global: AspectVersion,
    has_partition_overrides: bool,
}

impl Default for AspectVersionHeader {
    fn default() -> Self {
        Self::zero()
    }
}

#[allow(dead_code)]
impl AspectVersionHeader {
    pub const fn zero() -> Self {
        Self {
            global: AspectVersion::zero(),
            has_partition_overrides: false,
        }
    }

    pub const fn global(&self) -> AspectVersion {
        self.global
    }

    pub const fn has_partition_overrides(&self) -> bool {
        self.has_partition_overrides
    }

    pub fn set_global(&mut self, version: AspectVersion) {
        self.global = version;
    }

    pub fn set_has_partition_overrides(&mut self, has_partition_overrides: bool) {
        self.has_partition_overrides = has_partition_overrides;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PartitionVersionOverrides {
    #[serde(default)]
    partitions: BTreeMap<PartitionToken, AspectVersion>,
}

impl PartitionVersionOverrides {
    pub fn scoped_or_global(
        &self,
        scope: &PartitionSubscription,
        global: AspectVersion,
    ) -> AspectVersion {
        self.partitions
            .get(&scope.partition)
            .copied()
            .unwrap_or(global)
    }

    pub fn version_for_scope(
        &self,
        aspect: Aspect,
        scope: Option<&PartitionSubscription>,
        global: AspectVersion,
    ) -> u64 {
        match scope {
            Some(scope) => self.scoped_or_global(scope, global).get(aspect),
            None => global.get(aspect),
        }
    }

    pub fn set_global(&mut self, version: AspectVersion) {
        for partition in self.partitions.values_mut() {
            *partition = version;
        }
    }

    pub fn apply_evaluation(&mut self, version: AspectVersion, changed_regions: &[ChangedRegion]) {
        if changed_regions.is_empty() {
            return;
        }
        for region in changed_regions {
            self.partitions.insert(region.partition.clone(), version);
        }
    }

    pub fn has_overrides(&self) -> bool {
        !self.partitions.is_empty()
    }
}

impl Default for PartitionVersionMap {
    fn default() -> Self {
        Self::zero()
    }
}

#[allow(dead_code)]
impl PartitionVersionMap {
    pub const fn zero() -> Self {
        Self {
            global: AspectVersion::zero(),
            partitions: BTreeMap::new(),
        }
    }

    pub const fn global(&self) -> AspectVersion {
        self.global
    }

    pub fn scoped(&self, scope: &PartitionSubscription) -> AspectVersion {
        self.partitions
            .get(&scope.partition)
            .copied()
            .unwrap_or(self.global)
    }

    pub fn version_for_scope(&self, aspect: Aspect, scope: Option<&PartitionSubscription>) -> u64 {
        match scope {
            Some(scope) => self.scoped(scope).get(aspect),
            None => self.global.get(aspect),
        }
    }

    pub fn set_global(&mut self, version: AspectVersion) {
        self.global = version;
        for partition in self.partitions.values_mut() {
            *partition = version;
        }
    }

    pub fn apply_evaluation(&mut self, version: AspectVersion, changed_regions: &[ChangedRegion]) {
        self.global = version;
        if changed_regions.is_empty() {
            return;
        }
        for region in changed_regions {
            self.partitions.insert(region.partition.clone(), version);
        }
    }

    pub fn into_storage_parts(self) -> (AspectVersionHeader, PartitionVersionOverrides) {
        let overrides = PartitionVersionOverrides {
            partitions: self.partitions,
        };
        (
            AspectVersionHeader {
                global: self.global,
                has_partition_overrides: overrides.has_overrides(),
            },
            overrides,
        )
    }

    pub fn from_storage_parts(
        header: AspectVersionHeader,
        overrides: PartitionVersionOverrides,
    ) -> Self {
        Self {
            global: header.global(),
            partitions: overrides.partitions,
        }
    }
}
