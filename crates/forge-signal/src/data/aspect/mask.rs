use serde::{Deserialize, Serialize};

use super::aspect::Aspect;

/// Bitmask representation of one or more subscribed aspects.
///
/// This is the runtime-friendly representation used for fast filter checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct AspectMask(u8);

impl AspectMask {
    /// Empty mask.
    pub const EMPTY: Self = Self(0);

    /// Build a mask from one aspect.
    pub const fn from_aspect(aspect: Aspect) -> Self {
        Self(aspect.bit())
    }

    /// Build a mask directly from raw bits.
    pub const fn from_bits(bits: u8) -> Self {
        Self(bits)
    }

    /// Raw bit representation.
    pub const fn bits(self) -> u8 {
        self.0
    }

    /// Whether this mask contains any bits.
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// True if all bits in `other` are set in this mask.
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    /// True if this mask shares at least one bit with `other`.
    pub const fn intersects(self, other: Self) -> bool {
        (self.0 & other.0) != 0
    }

    /// Insert one aspect bit into this mask.
    pub fn insert(&mut self, aspect: Aspect) {
        self.0 |= Self::from_aspect(aspect).0;
    }
}

impl std::ops::BitOr for AspectMask {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
