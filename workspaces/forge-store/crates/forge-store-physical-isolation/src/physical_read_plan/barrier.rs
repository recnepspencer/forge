use super::{PhysicalReadPlanReleaseSemantics, PhysicalReadProtectedFootprintBasis};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalReadReachabilityBarrier {
    footprint_basis: PhysicalReadProtectedFootprintBasis,
    release: PhysicalReadPlanReleaseSemantics,
}

impl PhysicalReadReachabilityBarrier {
    pub(crate) const fn from_footprint_basis(
        footprint_basis: PhysicalReadProtectedFootprintBasis,
        release: PhysicalReadPlanReleaseSemantics,
    ) -> Self {
        Self {
            footprint_basis,
            release,
        }
    }

    pub const fn protected_references(self) -> u64 {
        self.footprint_basis.protected_references()
    }

    pub const fn footprint_basis(self) -> PhysicalReadProtectedFootprintBasis {
        self.footprint_basis
    }

    pub const fn release_semantics(self) -> PhysicalReadPlanReleaseSemantics {
        self.release
    }
}
