use super::PhysicalPublicationDenial;
use crate::{PhysicalReadPlanReleaseReceipt, PhysicalReadProtectedFootprintBasis};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OldReachabilityPreservation {
    footprint_basis: PhysicalReadProtectedFootprintBasis,
    retained_until_release: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReleasedOldReachability {
    footprint_basis: PhysicalReadProtectedFootprintBasis,
    release_receipt: PhysicalReadPlanReleaseReceipt,
}

impl OldReachabilityPreservation {
    pub fn from_protected_footprint(
        footprint_basis: PhysicalReadProtectedFootprintBasis,
    ) -> Result<Self, PhysicalPublicationDenial> {
        if footprint_basis.protected_references() == 0 {
            return Err(PhysicalPublicationDenial::MissingReachabilityEvidence);
        }
        Ok(Self {
            footprint_basis,
            retained_until_release: true,
        })
    }

    pub const fn footprint_basis(self) -> PhysicalReadProtectedFootprintBasis {
        self.footprint_basis
    }

    pub const fn retained_until_release(self) -> bool {
        self.retained_until_release
    }

    pub fn deny_reclaim_before_release(self) -> PhysicalPublicationDenial {
        PhysicalPublicationDenial::ReclaimBeforeReadPlanRelease {
            old_reachability: self.footprint_basis,
        }
    }

    pub fn admit_release(
        self,
        release_receipt: PhysicalReadPlanReleaseReceipt,
    ) -> Result<ReleasedOldReachability, PhysicalPublicationDenial> {
        if release_receipt.footprint_basis() != self.footprint_basis {
            return Err(self.deny_reclaim_before_release());
        }
        Ok(ReleasedOldReachability {
            footprint_basis: self.footprint_basis,
            release_receipt,
        })
    }
}

impl ReleasedOldReachability {
    pub const fn footprint_basis(self) -> PhysicalReadProtectedFootprintBasis {
        self.footprint_basis
    }

    pub const fn release_receipt(self) -> PhysicalReadPlanReleaseReceipt {
        self.release_receipt
    }
}
