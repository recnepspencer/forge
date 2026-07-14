use crate::{ContainerIntegrityCounters, PhysicalBoundaryLocalization, PhysicalScopeBasis};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageIntegrityReport {
    basis: PhysicalScopeBasis,
    counters: ContainerIntegrityCounters,
    boundary: PhysicalBoundaryLocalization,
    slot_directory: SlotDirectoryIntegrityReport,
}

impl PageIntegrityReport {
    pub(crate) const fn new(
        basis: PhysicalScopeBasis,
        counters: ContainerIntegrityCounters,
        slot_directory: SlotDirectoryIntegrityReport,
    ) -> Self {
        Self {
            basis,
            counters,
            boundary: PhysicalBoundaryLocalization::PageBody,
            slot_directory,
        }
    }

    pub const fn basis(&self) -> &PhysicalScopeBasis {
        &self.basis
    }

    pub const fn counters(&self) -> ContainerIntegrityCounters {
        self.counters
    }

    pub const fn boundary(&self) -> PhysicalBoundaryLocalization {
        self.boundary
    }

    pub const fn slot_directory(&self) -> &SlotDirectoryIntegrityReport {
        &self.slot_directory
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrameIntegrityReport {
    basis: PhysicalScopeBasis,
    counters: ContainerIntegrityCounters,
    boundary: PhysicalBoundaryLocalization,
}

impl FrameIntegrityReport {
    pub(crate) const fn new(
        basis: PhysicalScopeBasis,
        counters: ContainerIntegrityCounters,
        boundary: PhysicalBoundaryLocalization,
    ) -> Self {
        Self {
            basis,
            counters,
            boundary,
        }
    }

    pub const fn basis(&self) -> &PhysicalScopeBasis {
        &self.basis
    }

    pub const fn counters(&self) -> ContainerIntegrityCounters {
        self.counters
    }

    pub const fn boundary(&self) -> PhysicalBoundaryLocalization {
        self.boundary
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtentIntegrityReport {
    basis: PhysicalScopeBasis,
    counters: ContainerIntegrityCounters,
    boundary: PhysicalBoundaryLocalization,
    frame: FrameIntegrityReport,
}

impl ExtentIntegrityReport {
    pub(crate) const fn new(
        basis: PhysicalScopeBasis,
        counters: ContainerIntegrityCounters,
        frame: FrameIntegrityReport,
    ) -> Self {
        Self {
            basis,
            counters,
            boundary: PhysicalBoundaryLocalization::ExtentBoundary,
            frame,
        }
    }

    pub const fn basis(&self) -> &PhysicalScopeBasis {
        &self.basis
    }

    pub const fn counters(&self) -> ContainerIntegrityCounters {
        self.counters
    }

    pub const fn boundary(&self) -> PhysicalBoundaryLocalization {
        self.boundary
    }

    pub const fn frame(&self) -> &FrameIntegrityReport {
        &self.frame
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SlotDirectoryIntegrityReport {
    slot_count: u16,
    occupied_slots: u16,
    free_or_reserved_slots: u16,
}

impl SlotDirectoryIntegrityReport {
    pub(crate) const fn new(
        slot_count: u16,
        occupied_slots: u16,
        free_or_reserved_slots: u16,
    ) -> Self {
        Self {
            slot_count,
            occupied_slots,
            free_or_reserved_slots,
        }
    }

    pub const fn slot_count(self) -> u16 {
        self.slot_count
    }

    pub const fn occupied_slots(self) -> u16 {
        self.occupied_slots
    }

    pub const fn free_or_reserved_slots(self) -> u16 {
        self.free_or_reserved_slots
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SlotStateIntegrityReport {
    boundary: PhysicalBoundaryLocalization,
}

impl SlotStateIntegrityReport {
    pub const fn boundary(self) -> PhysicalBoundaryLocalization {
        self.boundary
    }
}
