use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvariantGroup {
    Structural,
    Mutation,
    Publication,
    Snapshot,
    Uniqueness,
    History,
    Harness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvariantCostClass {
    Constant,
    TargetedScan,
    FullScan,
    HarnessHeavy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvariantGroupSet {
    mask: u16,
}

impl InvariantGroupSet {
    pub const fn empty() -> Self {
        Self { mask: 0 }
    }

    pub const fn all() -> Self {
        Self { mask: (1 << 7) - 1 }
    }

    pub const fn of(group: InvariantGroup) -> Self {
        Self {
            mask: Self::group_bit(group),
        }
    }

    pub const fn union(self, other: Self) -> Self {
        Self {
            mask: self.mask | other.mask,
        }
    }

    pub const fn contains(self, group: InvariantGroup) -> bool {
        (self.mask & Self::group_bit(group)) != 0
    }

    pub const fn intersects(self, other: Self) -> bool {
        (self.mask & other.mask) != 0
    }

    const fn group_bit(group: InvariantGroup) -> u16 {
        match group {
            InvariantGroup::Structural => 1 << 0,
            InvariantGroup::Mutation => 1 << 1,
            InvariantGroup::Publication => 1 << 2,
            InvariantGroup::Snapshot => 1 << 3,
            InvariantGroup::Uniqueness => 1 << 4,
            InvariantGroup::History => 1 << 5,
            InvariantGroup::Harness => 1 << 6,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{InvariantGroup, InvariantGroupSet};

    #[test]
    fn group_sets_intersect_when_they_share_a_group() {
        let left = InvariantGroupSet::of(InvariantGroup::Mutation)
            .union(InvariantGroupSet::of(InvariantGroup::Uniqueness));
        let right = InvariantGroupSet::of(InvariantGroup::Uniqueness);
        let disjoint = InvariantGroupSet::of(InvariantGroup::Snapshot);

        assert!(left.intersects(right));
        assert!(!left.intersects(disjoint));
        assert!(InvariantGroupSet::all().intersects(disjoint));
    }
}
