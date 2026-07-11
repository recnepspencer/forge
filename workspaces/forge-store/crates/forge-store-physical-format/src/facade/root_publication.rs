use crate::{
    AllocationClassManifestEntry, ExtentManifestEntry, FreeSpaceManifestEntry,
    ManifestDiscoveryAuthority, OfflineManifestCodec, PhysicalGeneration,
    PhysicalGenerationAuthority, PhysicalHeaderAuthority, PhysicalManifestUniverseBuilder,
    PhysicalReferenceAuthority, PhysicalRootManifest, PhysicalRootReference, RootPublicationCell,
    SegmentManifestEntry, SegmentPageManifestEntry,
};

use super::denials::{PlatformPhysicalFacadeDenial, PlatformPhysicalFacadeDenialKind};
use super::storage::PlatformPhysicalFacadeStorage;
use super::{
    PlatformPhysicalFacade, PlatformPhysicalFacadeCounterSnapshot,
    PlatformPhysicalRootPublicationReport,
};

impl PlatformPhysicalFacade {
    pub fn publish_physical_root(
        &mut self,
    ) -> Result<PlatformPhysicalRootPublicationReport, PlatformPhysicalFacadeDenial> {
        let published = encode_next_root_publication(
            &mut self.next_root_generation,
            &self.storage,
            &self.headers,
            self.references,
        )?;
        self.storage.replace_manifest_bytes(
            Some(published.root.root_publication()),
            vec![published.root_manifest],
            published.segment_manifest,
            published.extent_manifest,
            published.free_space_map,
        );
        self.counters = self
            .counters
            .with_root_publication()
            .with_flush()
            .with_rename();
        Ok(PlatformPhysicalRootPublicationReport::new(
            self.headers.clone(),
            self.storage.persisted_layout(),
            self.counters,
        ))
    }

    pub fn publish_interrupted_physical_root(
        &mut self,
    ) -> Result<PlatformPhysicalRootPublicationReport, PlatformPhysicalFacadeDenial> {
        let first = encode_next_root_publication(
            &mut self.next_root_generation,
            &self.storage,
            &self.headers,
            self.references,
        )?;
        let second = encode_next_root_publication(
            &mut self.next_root_generation,
            &self.storage,
            &self.headers,
            self.references,
        )?;
        self.storage.replace_manifest_bytes(
            None,
            vec![first.root_manifest, second.root_manifest],
            first.segment_manifest,
            first.extent_manifest,
            first.free_space_map,
        );
        self.counters = self.counters.with_root_publication().with_flush();
        Err(PlatformPhysicalFacadeDenial::new(
            PlatformPhysicalFacadeDenialKind::AmbiguousRootPublication,
        ))
    }
}

pub(crate) struct EncodedRootPublication {
    pub(crate) root: PhysicalRootManifest,
    pub(crate) root_manifest: Vec<u8>,
    pub(crate) segment_manifest: Vec<u8>,
    pub(crate) extent_manifest: Vec<u8>,
    pub(crate) free_space_map: Vec<u8>,
}

pub(crate) fn encode_next_root_publication(
    next_root_generation: &mut u64,
    storage: &PlatformPhysicalFacadeStorage,
    headers: &PhysicalHeaderAuthority,
    references: PhysicalReferenceAuthority,
) -> Result<EncodedRootPublication, PlatformPhysicalFacadeDenial> {
    let generation = advance_root_generation(next_root_generation)?;
    let encoded = collect_root_publication_evidence(storage, generation, headers.byte_order());
    verify_manifest_discovery_on_encoded_root(&encoded, references)?;
    Ok(encoded)
}

fn advance_root_generation(
    next_root_generation: &mut u64,
) -> Result<PhysicalGeneration, PlatformPhysicalFacadeDenial> {
    let generation = PhysicalGeneration::from_raw(*next_root_generation).map_err(|_| {
        PlatformPhysicalFacadeDenial::new(PlatformPhysicalFacadeDenialKind::MissingPhysicalRoot)
    })?;
    *next_root_generation += 1;
    Ok(generation)
}

fn collect_root_publication_evidence(
    storage: &PlatformPhysicalFacadeStorage,
    generation: PhysicalGeneration,
    byte_order: crate::PhysicalByteOrder,
) -> EncodedRootPublication {
    encode_root_publication(storage, generation, byte_order)
}

fn verify_manifest_discovery_on_encoded_root(
    encoded: &EncodedRootPublication,
    references: PhysicalReferenceAuthority,
) -> Result<(), PlatformPhysicalFacadeDenial> {
    ManifestDiscoveryAuthority::for_canonical_physical_format()
        .reopen_from_root(
            &encoded.root,
            references.admit_root_publication(encoded.root.root_publication()),
        )
        .map(|_| ())
        .map_err(|denial| {
            PlatformPhysicalFacadeDenial::new(
                PlatformPhysicalFacadeDenialKind::ManifestDiscoveryDenied,
            )
            .with_manifest_denial(denial)
        })
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
    PhysicalGenerationAuthority::for_canonical_physical_format()
        .root_publication_cell(PhysicalRootReference::from_raw(1).expect("nonzero root id"))
        .with_root_publication_generation(generation)
}

fn build_root_manifest(
    storage: &PlatformPhysicalFacadeStorage,
    root_cell: RootPublicationCell,
) -> PhysicalRootManifest {
    let mut builder = PhysicalManifestUniverseBuilder::for_canonical_physical_format(root_cell);
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
        let segment = PhysicalGenerationAuthority::for_canonical_physical_format()
            .segment_cell(slot.segment_id())
            .with_segment_generation(slot.generation());
        if !segments.contains(&segment) {
            segments.push(segment);
        }
    }
    for extent in storage.extent_cells() {
        let segment = PhysicalGenerationAuthority::for_canonical_physical_format()
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
