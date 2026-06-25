use crate::{OfflineVerifierCounterSnapshot, PhysicalReference};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalLayoutReport {
    discovered_references: Vec<PhysicalReference>,
}

impl PhysicalLayoutReport {
    pub(crate) fn new(discovered_references: Vec<PhysicalReference>) -> Self {
        Self {
            discovered_references,
        }
    }

    pub fn discovered_references(&self) -> &[PhysicalReference] {
        &self.discovered_references
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestTraversalReport {
    root_count: u32,
    segment_count: u32,
    page_slot_count: u32,
    extent_count: u32,
    allocation_class_count: u32,
    free_space_count: u32,
}

impl ManifestTraversalReport {
    pub(crate) const fn new(
        root_count: u32,
        segment_count: u32,
        page_slot_count: u32,
        extent_count: u32,
        allocation_class_count: u32,
        free_space_count: u32,
    ) -> Self {
        Self {
            root_count,
            segment_count,
            page_slot_count,
            extent_count,
            allocation_class_count,
            free_space_count,
        }
    }

    pub(crate) const fn from_runtime_counts(
        root_count: u32,
        segment_count: u32,
        page_slot_count: u32,
        extent_count: u32,
        allocation_class_count: u32,
        free_space_count: u32,
    ) -> Self {
        Self::new(
            root_count,
            segment_count,
            page_slot_count,
            extent_count,
            allocation_class_count,
            free_space_count,
        )
    }

    pub const fn root_count(&self) -> u32 {
        self.root_count
    }

    pub const fn segment_count(&self) -> u32 {
        self.segment_count
    }

    pub const fn page_slot_count(&self) -> u32 {
        self.page_slot_count
    }

    pub const fn extent_count(&self) -> u32 {
        self.extent_count
    }

    pub const fn allocation_class_count(&self) -> u32 {
        self.allocation_class_count
    }

    pub const fn free_space_count(&self) -> u32 {
        self.free_space_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MinimalManifestVerifierReport {
    layout: PhysicalLayoutReport,
    traversal: ManifestTraversalReport,
    counters: OfflineVerifierCounterSnapshot,
}

impl MinimalManifestVerifierReport {
    pub(crate) const fn new(
        layout: PhysicalLayoutReport,
        traversal: ManifestTraversalReport,
        counters: OfflineVerifierCounterSnapshot,
    ) -> Self {
        Self {
            layout,
            traversal,
            counters,
        }
    }

    pub const fn layout(&self) -> &PhysicalLayoutReport {
        &self.layout
    }

    pub const fn traversal(&self) -> &ManifestTraversalReport {
        &self.traversal
    }

    pub const fn counters(&self) -> OfflineVerifierCounterSnapshot {
        self.counters
    }

    pub const fn semantic_decode_attempts(&self) -> u32 {
        self.counters.semantic_decode_attempts()
    }
}
