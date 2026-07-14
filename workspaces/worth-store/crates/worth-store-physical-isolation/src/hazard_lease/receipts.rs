use crate::PhysicalReadProtectedFootprintBasis;

use super::{HazardLeaseGeneration, HazardLeaseKind, HazardLeaseSlot};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HazardLeaseReleaseReceipt {
    slot: HazardLeaseSlot,
    generation: HazardLeaseGeneration,
    kind: HazardLeaseKind,
    footprint_basis: PhysicalReadProtectedFootprintBasis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReadHandleRevocationReceipt {
    slot: HazardLeaseSlot,
    generation: HazardLeaseGeneration,
    footprint_basis: PhysicalReadProtectedFootprintBasis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OwnedCopyStableReadReceipt {
    slot: HazardLeaseSlot,
    generation: HazardLeaseGeneration,
    footprint_basis: PhysicalReadProtectedFootprintBasis,
}

impl HazardLeaseReleaseReceipt {
    pub(crate) const fn new(
        slot: HazardLeaseSlot,
        generation: HazardLeaseGeneration,
        kind: HazardLeaseKind,
        footprint_basis: PhysicalReadProtectedFootprintBasis,
    ) -> Self {
        Self {
            slot,
            generation,
            kind,
            footprint_basis,
        }
    }

    pub const fn slot(self) -> HazardLeaseSlot {
        self.slot
    }

    pub const fn generation(self) -> HazardLeaseGeneration {
        self.generation
    }

    pub const fn kind(self) -> HazardLeaseKind {
        self.kind
    }

    pub const fn footprint_basis(self) -> PhysicalReadProtectedFootprintBasis {
        self.footprint_basis
    }
}

impl ReadHandleRevocationReceipt {
    pub(crate) const fn new(
        slot: HazardLeaseSlot,
        generation: HazardLeaseGeneration,
        footprint_basis: PhysicalReadProtectedFootprintBasis,
    ) -> Self {
        Self {
            slot,
            generation,
            footprint_basis,
        }
    }

    pub const fn slot(self) -> HazardLeaseSlot {
        self.slot
    }

    pub const fn generation(self) -> HazardLeaseGeneration {
        self.generation
    }

    pub const fn footprint_basis(self) -> PhysicalReadProtectedFootprintBasis {
        self.footprint_basis
    }
}

impl OwnedCopyStableReadReceipt {
    pub(crate) const fn new(
        slot: HazardLeaseSlot,
        generation: HazardLeaseGeneration,
        footprint_basis: PhysicalReadProtectedFootprintBasis,
    ) -> Self {
        Self {
            slot,
            generation,
            footprint_basis,
        }
    }

    pub const fn slot(self) -> HazardLeaseSlot {
        self.slot
    }

    pub const fn generation(self) -> HazardLeaseGeneration {
        self.generation
    }

    pub const fn footprint_basis(self) -> PhysicalReadProtectedFootprintBasis {
        self.footprint_basis
    }
}
