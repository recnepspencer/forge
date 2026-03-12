use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvariantGroup {
    StorageCoherence = 0,
    VersionVisibility = 1,
    AdjacencyIntegrity = 2,
    IdentityCoherence = 3,
    SchemaCompliance = 4,
    LineageIntegrity = 5,
    PublicationCoherence = 6,
    DurabilityConsistency = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvariantCostClass {
    Touched,
    Partition,
    Global,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvariantGroupSet {
    mask: u32,
}

impl InvariantGroupSet {
    pub const COUNT: usize = 8;

    pub const fn empty() -> Self {
        Self { mask: 0 }
    }

    pub const fn all() -> Self {
        Self { mask: (1u32 << Self::COUNT) - 1 }
    }

    pub const fn of(group: InvariantGroup) -> Self {
        Self { mask: group.mask() }
    }

    pub const fn union(self, other: Self) -> Self {
        Self {
            mask: self.mask | other.mask,
        }
    }

    pub const fn contains(self, group: InvariantGroup) -> bool {
        (self.mask & group.mask()) != 0
    }

    pub const fn intersects(self, other: Self) -> bool {
        (self.mask & other.mask) != 0
    }

    pub const fn mask(self) -> u32 {
        self.mask
    }

    pub const fn from_mask(mask: u32) -> Self {
        Self { mask }
    }
}

impl InvariantGroup {
    pub const COUNT: usize = 8;

    pub const fn mask(self) -> u32 {
        1u32 << (self as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::{InvariantGroup, InvariantGroupSet};

    #[test]
    fn group_sets_intersect_when_they_share_a_group() {
        let left = InvariantGroupSet::of(InvariantGroup::StorageCoherence)
            .union(InvariantGroupSet::of(InvariantGroup::SchemaCompliance));
        let right = InvariantGroupSet::of(InvariantGroup::SchemaCompliance);
        let disjoint = InvariantGroupSet::of(InvariantGroup::VersionVisibility);

        assert!(left.intersects(right));
        assert!(!left.intersects(disjoint));
        assert!(InvariantGroupSet::all().intersects(disjoint));
    }
}
