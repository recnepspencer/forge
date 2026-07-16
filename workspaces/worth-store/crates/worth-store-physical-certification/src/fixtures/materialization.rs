use worth_store_physical_format::{
    AllocationClassKind, FreeSpaceManifestEntry, OfflineManifestCodec, PersistedExtentBytes,
    PersistedPageBytes, PersistedPhysicalLayout, PhysicalBinaryEncodingWitness,
    PhysicalExtentId, PhysicalGeneration, PhysicalGenerationAuthority, PhysicalHeaderAuthority,
    PhysicalPageId, PhysicalPageKind, PhysicalPageRecordAuthority, PhysicalRecordSlot,
    PhysicalRootReference, PhysicalSegmentId, PlatformPhysicalReplayArtifact, SlotAppendRequest,
};

use super::{FixtureScaleDeclaration, LargeStoreFixtureProfile, SyntheticFixtureAuthorityDenied};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProductionBackedFixtureSource {
    root_reference: PhysicalRootReference,
}

impl ProductionBackedFixtureSource {
    pub const fn root_reference(&self) -> u64 {
        self.root_reference.get()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ProductionBackedFixtureMaterialization {
    profile: LargeStoreFixtureProfile,
    scale: FixtureScaleDeclaration,
    source: ProductionBackedFixtureSource,
    layout: PersistedPhysicalLayout,
    replay_artifact: Option<PlatformPhysicalReplayArtifact>,
}

impl ProductionBackedFixtureMaterialization {
    pub fn build_profile(
        profile: LargeStoreFixtureProfile,
        root_reference: u64,
    ) -> Result<Self, SyntheticFixtureAuthorityDenied> {
        let root_reference = PhysicalRootReference::from_raw(root_reference)
            .map_err(|_| SyntheticFixtureAuthorityDenied::InvalidRootReference(root_reference))?;
        let source = ProductionBackedFixtureSource { root_reference };
        Ok(Self {
            profile,
            scale: profile.scale_declaration(),
            source,
            layout: build_persisted_layout(root_reference),
            replay_artifact: None,
        })
    }

    pub fn from_replay_artifact(
        profile: LargeStoreFixtureProfile,
        root_reference: u64,
        replay_artifact: PlatformPhysicalReplayArtifact,
    ) -> Result<Self, SyntheticFixtureAuthorityDenied> {
        let root_reference = PhysicalRootReference::from_raw(root_reference)
            .map_err(|_| SyntheticFixtureAuthorityDenied::InvalidRootReference(root_reference))?;
        let source = ProductionBackedFixtureSource { root_reference };
        Ok(Self {
            profile,
            scale: profile.scale_declaration(),
            source,
            layout: replay_artifact.persisted_layout().clone(),
            replay_artifact: Some(replay_artifact),
        })
    }

    pub const fn profile(&self) -> LargeStoreFixtureProfile {
        self.profile
    }

    pub const fn scale(&self) -> FixtureScaleDeclaration {
        self.scale
    }

    pub const fn source(&self) -> ProductionBackedFixtureSource {
        self.source
    }

    pub const fn persisted_layout(&self) -> &PersistedPhysicalLayout {
        &self.layout
    }

    pub const fn replay_artifact(&self) -> Option<&PlatformPhysicalReplayArtifact> {
        self.replay_artifact.as_ref()
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        LargeStoreFixtureProfile,
        FixtureScaleDeclaration,
        ProductionBackedFixtureSource,
        PersistedPhysicalLayout,
        Option<PlatformPhysicalReplayArtifact>,
    ) {
        (
            self.profile,
            self.scale,
            self.source,
            self.layout,
            self.replay_artifact,
        )
    }
}

fn build_persisted_layout(root_reference: PhysicalRootReference) -> PersistedPhysicalLayout {
    let encoding = PhysicalBinaryEncodingWitness::physical_format_canonical().unwrap();
    let byte_order = encoding.declaration().byte_order();
    let headers = PhysicalHeaderAuthority::for_canonical_physical_format(encoding);
    let generation = PhysicalGeneration::from_raw(7).unwrap();
    let cells = PhysicalFixtureCells::new(root_reference, generation);
    let root = cells.root_manifest();
    PersistedPhysicalLayout::builder()
        .root_manifest(OfflineManifestCodec::encode_root_manifest(
            byte_order,
            cells.root_cell,
        ))
        .segment_manifest(OfflineManifestCodec::encode_segment_manifest(
            byte_order,
            root.segments(),
            root.page_slots(),
        ))
        .extent_manifest(OfflineManifestCodec::encode_extent_manifest(
            byte_order,
            root.extents(),
            root.allocation_classes(),
        ))
        .free_space_map(OfflineManifestCodec::encode_free_space_map(
            byte_order,
            &[FreeSpaceManifestEntry::new(cells.free_space)],
        ))
        .page(PersistedPageBytes::new(
            cells.page_cell,
            record_page_bytes(&headers, cells.page_cell, cells.slot_cell),
        ))
        .extent(PersistedExtentBytes::new(
            cells.extent_cell,
            extent_record_bytes(&headers, cells.extent_cell, b"fixture-large-record"),
        ))
        .build()
}

struct PhysicalFixtureCells {
    root_cell: worth_store_physical_format::RootPublicationCell,
    segment_cell: worth_store_physical_format::SegmentGenerationCell,
    slot_cell: worth_store_physical_format::SlotGenerationCell,
    page_cell: worth_store_physical_format::PageGenerationCell,
    extent_cell: worth_store_physical_format::ExtentGenerationCell,
    free_space: worth_store_physical_format::FreeSpaceReuseCell,
}

impl PhysicalFixtureCells {
    fn new(root_reference: PhysicalRootReference, generation: PhysicalGeneration) -> Self {
        let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
        let segment_id = PhysicalSegmentId::from_raw(1).unwrap();
        let page_id = PhysicalPageId::from_raw(1).unwrap();
        let extent_id = PhysicalExtentId::from_raw(1).unwrap();
        let slot = PhysicalRecordSlot::from_raw(1).unwrap();
        Self {
            root_cell: generations
                .root_publication_cell(root_reference)
                .with_root_publication_generation(generation),
            segment_cell: generations
                .segment_cell(segment_id)
                .with_segment_generation(generation),
            slot_cell: generations
                .slot_cell(segment_id, page_id, slot)
                .with_slot_generation(generation),
            page_cell: generations
                .page_cell(segment_id, page_id)
                .with_page_generation(generation),
            extent_cell: generations
                .extent_cell(segment_id, extent_id)
                .with_extent_generation(generation),
            free_space: generations
                .free_space_slot_cell(
                    segment_id,
                    page_id,
                    slot,
                    AllocationClassKind::OrdinaryRecordPage,
                )
                .unwrap()
                .with_free_space_generation(generation),
        }
    }

    fn root_manifest(&self) -> worth_store_physical_format::PhysicalRootManifest {
        worth_store_physical_format::PhysicalManifestUniverseBuilder::for_canonical_physical_format(
            self.root_cell,
        )
        .segment(self.segment_cell)
        .ordinary_page(self.slot_cell)
        .extent(self.extent_cell)
        .free_space_reuse(self.free_space)
        .publish()
    }
}

fn record_page_bytes(
    headers: &PhysicalHeaderAuthority,
    page_cell: worth_store_physical_format::PageGenerationCell,
    slot_cell: worth_store_physical_format::SlotGenerationCell,
) -> Vec<u8> {
    let authority = PhysicalPageRecordAuthority::for_canonical_physical_format(headers.clone());
    let empty_page = page_bytes(headers, page_cell, &[]);
    let header = authority
        .decode_record_page_header(page_cell, &empty_page, PhysicalPageKind::DataPage)
        .unwrap();
    let payload = authority
        .admit_record_page_payload(&empty_page, header.witness())
        .unwrap();
    let append = authority
        .append_record(payload, SlotAppendRequest::ordinary(slot_cell, b"fixture"))
        .unwrap();
    page_bytes(headers, page_cell, append.page_payload())
}

fn page_bytes(
    headers: &PhysicalHeaderAuthority,
    cell: worth_store_physical_format::PageGenerationCell,
    payload: &[u8],
) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(
        usize::from(worth_store_physical_format::PHYSICAL_HEADER_LENGTH) + payload.len(),
    );
    bytes.extend_from_slice(&headers.encode_page_header(
        cell,
        PhysicalPageKind::DataPage,
        u32::try_from(payload.len()).expect("fixture payload length should fit physical format"),
    ));
    bytes.extend_from_slice(payload);
    bytes
}

fn extent_record_bytes(
    headers: &PhysicalHeaderAuthority,
    cell: worth_store_physical_format::ExtentGenerationCell,
    payload: &[u8],
) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(
        usize::from(worth_store_physical_format::PHYSICAL_HEADER_LENGTH) + payload.len(),
    );
    bytes.extend_from_slice(&headers.encode_extent_frame_header(
        cell,
        u32::try_from(payload.len()).expect("fixture payload length should fit physical format"),
    ));
    bytes.extend_from_slice(payload);
    bytes
}
