use crate::{
    AllocationClassKind, AllocationClassManifestEntry, ExtentGenerationCell, ExtentManifestEntry,
    FreeSpaceManifestEntry, FreeSpaceReuseCell, ManifestDiscoveryCounterSnapshot,
    RootPublicationCell, SegmentGenerationCell, SegmentManifestEntry, SegmentPageManifestEntry,
    SlotGenerationCell,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalRootManifest {
    root_publication: RootPublicationCell,
    segments: Vec<SegmentManifestEntry>,
    page_slots: Vec<SegmentPageManifestEntry>,
    extents: Vec<ExtentManifestEntry>,
    allocation_classes: Vec<AllocationClassManifestEntry>,
    free_space: Vec<FreeSpaceManifestEntry>,
    publish_counters: ManifestDiscoveryCounterSnapshot,
}

impl PhysicalRootManifest {
    pub(crate) fn new(
        root_publication: RootPublicationCell,
        segments: Vec<SegmentManifestEntry>,
        page_slots: Vec<SegmentPageManifestEntry>,
        extents: Vec<ExtentManifestEntry>,
        allocation_classes: Vec<AllocationClassManifestEntry>,
        free_space: Vec<FreeSpaceManifestEntry>,
    ) -> Self {
        let publish_counters = manifest_shape_counters(
            ManifestDiscoveryCounterSnapshot::for_publish(),
            segments.len(),
            page_slots.len(),
            extents.len(),
            allocation_classes.len(),
            free_space.len(),
        );
        Self {
            root_publication,
            segments,
            page_slots,
            extents,
            allocation_classes,
            free_space,
            publish_counters,
        }
    }

    pub const fn root_publication(&self) -> RootPublicationCell {
        self.root_publication
    }

    pub fn segments(&self) -> &[SegmentManifestEntry] {
        &self.segments
    }

    pub fn page_slots(&self) -> &[SegmentPageManifestEntry] {
        &self.page_slots
    }

    pub fn extents(&self) -> &[ExtentManifestEntry] {
        &self.extents
    }

    pub fn allocation_classes(&self) -> &[AllocationClassManifestEntry] {
        &self.allocation_classes
    }

    pub fn free_space(&self) -> &[FreeSpaceManifestEntry] {
        &self.free_space
    }

    pub const fn publish_counters(&self) -> ManifestDiscoveryCounterSnapshot {
        self.publish_counters
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalManifestUniverseBuilder {
    root_publication: RootPublicationCell,
    segments: Vec<SegmentManifestEntry>,
    page_slots: Vec<SegmentPageManifestEntry>,
    extents: Vec<ExtentManifestEntry>,
    allocation_classes: Vec<AllocationClassManifestEntry>,
    free_space: Vec<FreeSpaceManifestEntry>,
}

impl PhysicalManifestUniverseBuilder {
    pub fn for_canonical_physical_format(root_publication: RootPublicationCell) -> Self {
        Self {
            root_publication,
            segments: Vec::new(),
            page_slots: Vec::new(),
            extents: Vec::new(),
            allocation_classes: Vec::new(),
            free_space: Vec::new(),
        }
    }

    pub fn segment(mut self, segment: SegmentGenerationCell) -> Self {
        self.segments.push(SegmentManifestEntry::new(segment));
        self
    }

    pub fn ordinary_page(mut self, slot: SlotGenerationCell) -> Self {
        self.page_slots.push(SegmentPageManifestEntry::new(slot));
        self.allocation_class(AllocationClassKind::OrdinaryRecordPage)
    }

    pub fn extent(mut self, extent: ExtentGenerationCell) -> Self {
        self.extents.push(ExtentManifestEntry::new(extent));
        self.allocation_class(AllocationClassKind::LargeRecordExtent)
    }

    pub fn free_space_reuse(mut self, reuse_cell: FreeSpaceReuseCell) -> Self {
        self.free_space
            .push(FreeSpaceManifestEntry::new(reuse_cell));
        self.allocation_class(AllocationClassKind::FreeSpaceMap)
    }

    pub fn allocation_class(mut self, allocation_class: AllocationClassKind) -> Self {
        if !self
            .allocation_classes
            .iter()
            .any(|entry| entry.allocation_class() == allocation_class)
        {
            self.allocation_classes
                .push(AllocationClassManifestEntry::new(allocation_class));
        }
        self
    }

    pub fn publish(self) -> PhysicalRootManifest {
        PhysicalRootManifest::new(
            self.root_publication,
            self.segments,
            self.page_slots,
            self.extents,
            self.allocation_classes,
            self.free_space,
        )
    }
}

pub(crate) fn manifest_shape_counters(
    counters: ManifestDiscoveryCounterSnapshot,
    segment_count: usize,
    page_slot_count: usize,
    extent_count: usize,
    allocation_count: usize,
    free_space_count: usize,
) -> ManifestDiscoveryCounterSnapshot {
    counters
        .with_root_entries(root_entry_count(
            segment_count,
            extent_count,
            allocation_count,
            free_space_count,
        ))
        .with_segment_manifest(page_slot_count as u32)
        .with_extent_manifest(extent_count as u32)
        .with_allocation_entries(allocation_count as u32)
        .with_free_space_entries(free_space_count as u32)
}

const fn root_entry_count(
    segment_count: usize,
    extent_count: usize,
    allocation_count: usize,
    free_space_count: usize,
) -> u32 {
    (segment_count + extent_count + allocation_count + free_space_count) as u32
}
