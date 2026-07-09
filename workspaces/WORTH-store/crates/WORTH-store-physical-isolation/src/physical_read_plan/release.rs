use super::PhysicalReadProtectedFootprintBasis;
use crate::{CurrentPhysicalRoot, RootEpoch};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalReadPlanReleaseSemantics {
    release_required: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalReadPlanReleaseReceipt {
    root: CurrentPhysicalRoot,
    footprint_basis: PhysicalReadProtectedFootprintBasis,
}

impl PhysicalReadPlanReleaseSemantics {
    pub const fn reader_releases_all() -> Self {
        Self {
            release_required: true,
        }
    }

    pub const fn release_required(self) -> bool {
        self.release_required
    }
}

impl PhysicalReadPlanReleaseReceipt {
    pub(crate) const fn new(
        root: CurrentPhysicalRoot,
        footprint_basis: PhysicalReadProtectedFootprintBasis,
    ) -> Self {
        Self {
            root,
            footprint_basis,
        }
    }

    pub const fn root(self) -> CurrentPhysicalRoot {
        self.root
    }

    pub const fn root_epoch(self) -> RootEpoch {
        self.root.epoch()
    }

    pub const fn protected_references_released(self) -> u64 {
        self.footprint_basis.protected_references()
    }

    pub const fn footprint_basis(self) -> PhysicalReadProtectedFootprintBasis {
        self.footprint_basis
    }
}
