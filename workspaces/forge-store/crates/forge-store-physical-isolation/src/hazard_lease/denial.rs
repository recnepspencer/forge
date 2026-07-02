use crate::PhysicalReadProtectedFootprintBasis;

use super::{HazardLeaseGeneration, HazardLeaseSlot};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HazardLeaseDenial {
    EmptyCapacity,
    TableFull,
    MissingProtectedRanges,
    HazardFootprintMismatch {
        expected: PhysicalReadProtectedFootprintBasis,
        observed: PhysicalReadProtectedFootprintBasis,
    },
    UnknownLeaseSlot {
        slot: HazardLeaseSlot,
    },
    StaleLeaseGeneration {
        slot: HazardLeaseSlot,
        expected: HazardLeaseGeneration,
        observed: HazardLeaseGeneration,
    },
    LeaseAlreadyReleased {
        slot: HazardLeaseSlot,
    },
    ExpiredLeaseWithoutReleaseRevocationOrOwnedCopy {
        slot: HazardLeaseSlot,
    },
}
