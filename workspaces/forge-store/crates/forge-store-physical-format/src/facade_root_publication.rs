use crate::{
    AllocationClassManifestEntry, ExtentManifestEntry, FreeSpaceManifestEntry,
    OfflineManifestCodec, PhysicalGeneration, PhysicalGenerationAuthority,
    PhysicalManifestUniverseBuilder, PhysicalRootManifest, PhysicalRootReference,
    RootPublicationCell, SegmentManifestEntry, SegmentPageManifestEntry,
};

use crate::facade_storage::PlatformPhysicalFacadeStorage;

pub(crate) struct EncodedRootPublication {
    pub(crate) root: PhysicalRootManifest,
    pub(crate) root_manifest: Vec<u8>,
    pub(crate) segment_manifest: Vec<u8>,
    pub(crate) extent_manifest: Vec<u8>,
    pub(crate) free_space_map: Vec<u8>,
}

pub(crate) fn encode_root_publication(
    storage: &PlatformPhysicalFacadeStorage,
    generation: PhysicalGeneration,
    byte_order: crate::PhysicalByteOrder,
) -> EncodedRootPublication {
    let root_cell = root_publication_cell(generation);
    let root = build_root_manifest(storage, root_cell);
    let root_manifest = OfflineManifestCodec::encode_root_manifest(byte_order, root_cell);
    let segment_manifest = OfflineManifestCodec::encode_segment_manifest(
        byte_order,
        &segment_entries(&root),
        &page_entries(&root),
    );
    let extent_manifest = OfflineManifestCodec::encode_extent_manifest(
        byte_order,
        &extent_entries(&root),
        &allocation_entries(&root),
    );
    let free_space_map =
        OfflineManifestCodec::encode_free_space_map(byte_order, &free_space_entries(&root));
    EncodedRootPublication {
        root,
        root_manifest,
        segment_manifest,
        extent_manifest,
        free_space_map,
    }
}

fn root_publication_cell(generation: PhysicalGeneration) -> RootPublicationCell {
    PhysicalGenerationAuthority::s1()
        .root_publication_cell(PhysicalRootReference::from_raw(1).expect("nonzero root id"))
        .with_root_publication_generation(generation)
}

fn build_root_manifest(
    storage: &PlatformPhysicalFacadeStorage,
    root_cell: RootPublicationCell,
) -> PhysicalRootManifest {
    let mut builder = PhysicalManifestUniverseBuilder::s1(root_cell);
    for segment in manifested_segments(storage) {
        builder = builder.segment(segment);
    }
    for slot in storage.page_slots() {
        builder = builder.ordinary_page(*slot);
    }
    for extent in storage.extent_cells() {
        builder = builder.extent(*extent);
    }
    builder.publish()
}

fn manifested_segments(
    storage: &PlatformPhysicalFacadeStorage,
) -> Vec<crate::SegmentGenerationCell> {
    let mut segments = Vec::new();
    for slot in storage.page_slots() {
        let segment = PhysicalGenerationAuthority::s1()
            .segment_cell(slot.segment_id())
            .with_segment_generation(slot.generation());
        if !segments.contains(&segment) {
            segments.push(segment);
        }
    }
    for extent in storage.extent_cells() {
        let segment = PhysicalGenerationAuthority::s1()
            .segment_cell(extent.segment_id())
            .with_segment_generation(extent.generation());
        if !segments
            .iter()
            .any(|existing| existing.segment_id() == segment.segment_id())
        {
            segments.push(segment);
        }
    }
    segments
}

fn segment_entries(root: &PhysicalRootManifest) -> Vec<SegmentManifestEntry> {
    root.segments().to_vec()
}

fn page_entries(root: &PhysicalRootManifest) -> Vec<SegmentPageManifestEntry> {
    root.page_slots().to_vec()
}

fn extent_entries(root: &PhysicalRootManifest) -> Vec<ExtentManifestEntry> {
    root.extents().to_vec()
}

fn allocation_entries(root: &PhysicalRootManifest) -> Vec<AllocationClassManifestEntry> {
    root.allocation_classes().to_vec()
}

fn free_space_entries(root: &PhysicalRootManifest) -> Vec<FreeSpaceManifestEntry> {
    root.free_space().to_vec()
}
