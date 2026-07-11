use crate::ManifestIntegrityCounters;
use forge_store_physical_format::{
    PhysicalGenerationOwner, PhysicalReferenceScope, RootManifestIntegrityPosture,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestIntegrityReport {
    root: RootManifestIntegrityReport,
    segment: SegmentManifestIntegrityReport,
    allocation: AllocationMapIntegrityReport,
    reference_basis: ManifestReferenceBasis,
    counters: ManifestIntegrityCounters,
}

impl ManifestIntegrityReport {
    pub(crate) const fn new(
        root: RootManifestIntegrityReport,
        segment: SegmentManifestIntegrityReport,
        allocation: AllocationMapIntegrityReport,
        reference_basis: ManifestReferenceBasis,
        counters: ManifestIntegrityCounters,
    ) -> Self {
        Self {
            root,
            segment,
            allocation,
            reference_basis,
            counters,
        }
    }

    pub const fn root(&self) -> &RootManifestIntegrityReport {
        &self.root
    }

    pub const fn segment(&self) -> &SegmentManifestIntegrityReport {
        &self.segment
    }

    pub const fn allocation(&self) -> &AllocationMapIntegrityReport {
        &self.allocation
    }

    pub const fn reference_basis(&self) -> &ManifestReferenceBasis {
        &self.reference_basis
    }

    pub const fn counters(&self) -> ManifestIntegrityCounters {
        self.counters
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootManifestIntegrityReport {
    posture: RootManifestIntegrityPosture,
    root_owner: Option<PhysicalGenerationOwner>,
}

impl RootManifestIntegrityReport {
    pub(crate) const fn new(posture: RootManifestIntegrityPosture) -> Self {
        Self {
            posture,
            root_owner: posture.root_owner(),
        }
    }

    pub const fn posture(&self) -> RootManifestIntegrityPosture {
        self.posture
    }

    pub const fn root_owner(&self) -> Option<PhysicalGenerationOwner> {
        self.root_owner
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SegmentManifestIntegrityReport {
    segment_entries: u32,
    page_slot_entries: u32,
    extent_entries: u32,
}

impl SegmentManifestIntegrityReport {
    pub(crate) const fn new(
        segment_entries: u32,
        page_slot_entries: u32,
        extent_entries: u32,
    ) -> Self {
        Self {
            segment_entries,
            page_slot_entries,
            extent_entries,
        }
    }

    pub const fn segment_entries(self) -> u32 {
        self.segment_entries
    }

    pub const fn page_slot_entries(self) -> u32 {
        self.page_slot_entries
    }

    pub const fn extent_entries(self) -> u32 {
        self.extent_entries
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AllocationMapIntegrityReport {
    allocation_entries: u32,
    free_space_entries: u32,
    counters: ManifestIntegrityCounters,
}

impl AllocationMapIntegrityReport {
    pub(crate) const fn new(
        allocation_entries: u32,
        free_space_entries: u32,
        counters: ManifestIntegrityCounters,
    ) -> Self {
        Self {
            allocation_entries,
            free_space_entries,
            counters,
        }
    }

    pub const fn allocation_entries(self) -> u32 {
        self.allocation_entries
    }

    pub const fn free_space_entries(self) -> u32 {
        self.free_space_entries
    }

    pub const fn counters(self) -> ManifestIntegrityCounters {
        self.counters
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestReferenceBasis {
    root_owner: Option<PhysicalGenerationOwner>,
    physical_owners: Vec<PhysicalGenerationOwner>,
    admitted_scopes: Vec<PhysicalReferenceScope>,
}

impl ManifestReferenceBasis {
    pub(crate) fn new(
        root_owner: Option<PhysicalGenerationOwner>,
        physical_owners: Vec<PhysicalGenerationOwner>,
        admitted_scopes: Vec<PhysicalReferenceScope>,
    ) -> Self {
        Self {
            root_owner,
            physical_owners,
            admitted_scopes,
        }
    }

    pub const fn root_owner(&self) -> Option<PhysicalGenerationOwner> {
        self.root_owner
    }

    pub fn physical_owners(&self) -> &[PhysicalGenerationOwner] {
        &self.physical_owners
    }

    pub fn admitted_scopes(&self) -> &[PhysicalReferenceScope] {
        &self.admitted_scopes
    }
}
