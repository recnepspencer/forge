use serde::{Deserialize, Serialize};

use super::aspect::{Aspect, MAX_ASPECTS};
use super::mask::AspectMask;

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
