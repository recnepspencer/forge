use forge_store_physical_format::PhysicalReclaimRegion;
use forge_store_physical_isolation::S6ReclaimReachabilityRemovalEvidence;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReclaimPolicyReachabilityProof {
    covered_region: PhysicalReclaimRegion,
    root_epoch: u64,
    protected_ranges: u32,
    eligible: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReclaimPolicyReachabilityDenial {
    NoProtectedReachabilityEvidence,
    ProtectedReachabilityStillActive,
    RegionNotCoveredByReachabilityRemoval,
}

impl ReclaimPolicyReachabilityProof {
    pub fn from_s5_reclaim_reachability_removal(
        evidence: S6ReclaimReachabilityRemovalEvidence,
        requested_region: PhysicalReclaimRegion,
    ) -> Result<Self, ReclaimPolicyReachabilityDenial> {
        if evidence.protected_ranges() == 0 {
            return Err(ReclaimPolicyReachabilityDenial::NoProtectedReachabilityEvidence);
        }
        if evidence.region() != requested_region {
            return Err(ReclaimPolicyReachabilityDenial::RegionNotCoveredByReachabilityRemoval);
        }
        Ok(Self {
            covered_region: requested_region,
            root_epoch: evidence.root_epoch(),
            protected_ranges: evidence.protected_ranges(),
            eligible: true,
        })
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub fn for_certification_test_authority(region: PhysicalReclaimRegion) -> Self {
        Self {
            covered_region: region,
            root_epoch: 0,
            protected_ranges: 1,
            eligible: true,
        }
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub fn blocked_for_certification_test_authority(region: PhysicalReclaimRegion) -> Self {
        Self {
            covered_region: region,
            root_epoch: 0,
            protected_ranges: 1,
            eligible: false,
        }
    }

    pub const fn is_eligible(&self) -> bool {
        self.eligible
    }

    pub const fn protected_ranges(&self) -> u32 {
        self.protected_ranges
    }

    pub const fn root_epoch(&self) -> u64 {
        self.root_epoch
    }

    pub fn covers_region(&self, region: PhysicalReclaimRegion) -> bool {
        self.covered_region == region
    }
}
