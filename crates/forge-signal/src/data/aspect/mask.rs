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
    /// Topology aspect bit.
    pub const TOPOLOGY: Self = Self(1 << 0);
    /// Geometry aspect bit.
    pub const GEOMETRY: Self = Self(1 << 1);

    /// Build a mask from one aspect.
    pub fn from_aspect(aspect: Aspect) -> Self {
        match aspect {
            Aspect::Topology => Self::TOPOLOGY,
            Aspect::Geometry => Self::GEOMETRY,
        }
    }

    /// Raw bit representation.
    pub fn bits(self) -> u8 {
        self.0
    }

    /// Whether this mask contains any bits.
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// True if all bits in `other` are set in this mask.
    pub fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    /// True if this mask shares at least one bit with `other`.
    pub fn intersects(self, other: Self) -> bool {
        (self.0 & other.0) != 0
    }
}

impl std::ops::BitOr for AspectMask {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
