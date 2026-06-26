use crate::{
    ExtentGenerationCell, ManifestTraversalReport, PersistedExtentBytes, PersistedPageBytes,
    PersistedPhysicalLayout, PhysicalGenerationAuthority, PhysicalPageKind,
    PhysicalPublicationState, PhysicalReference, PhysicalReferenceAuthority,
    PlatformPhysicalFacadeDenial, PlatformPhysicalFacadeDenialKind, RootPublicationCell,
    SlotGenerationCell, PHYSICAL_HEADER_LENGTH,
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct PlatformPhysicalFacadeStorage {
    pages: Vec<StoredPageBytes>,
    extents: Vec<StoredExtentBytes>,
    page_slots: Vec<SlotGenerationCell>,
    extent_cells: Vec<ExtentGenerationCell>,
    root_manifest_candidates: Vec<Vec<u8>>,
    segment_manifest: Vec<u8>,
    extent_manifest: Vec<u8>,
    free_space_map: Vec<u8>,
    root_publication: Option<RootPublicationCell>,
    verifier_admitted_references: Vec<PhysicalReference>,
}

impl PlatformPhysicalFacadeStorage {
    pub(crate) fn empty() -> Self {
        Self::default()
    }

    pub(crate) fn from_persisted_layout(
        layout: &PersistedPhysicalLayout,
        verifier_admitted_references: Vec<PhysicalReference>,
    ) -> Self {
        Self {
            pages: layout
                .pages()
                .iter()
                .map(|page| StoredPageBytes::new(page.cell(), page.bytes().to_vec()))
                .collect(),
            extents: layout
                .extents()
                .iter()
                .map(|extent| StoredExtentBytes::new(extent.cell(), extent.bytes().to_vec()))
                .collect(),
            page_slots: verifier_admitted_references
                .iter()
                .filter_map(reference_to_slot_cell)
                .collect(),
            extent_cells: verifier_admitted_references
                .iter()
                .filter_map(reference_to_extent_cell)
                .collect(),
            root_manifest_candidates: layout.root_manifest_candidates().to_vec(),
            segment_manifest: layout.segment_manifest().to_vec(),
            extent_manifest: layout.extent_manifest().to_vec(),
            free_space_map: layout.free_space_map().to_vec(),
            root_publication: verifier_admitted_references
                .iter()
                .find_map(reference_to_root_publication_cell),
            verifier_admitted_references,
        }
    }

    pub(crate) fn page_bytes_for_append(&mut self, slot_cell: SlotGenerationCell) -> &[u8] {
        let page_cell = PhysicalGenerationAuthority::s1()
            .page_cell(slot_cell.segment_id(), slot_cell.page_id())
            .with_page_generation(slot_cell.generation());
        if self.find_page_index(slot_cell).is_none() {
            self.pages.push(StoredPageBytes::new(
                page_cell,
                encode_empty_page(slot_cell.generation()),
            ));
        }
        let index = self
            .find_page_index(slot_cell)
            .expect("page was inserted before borrowing bytes");
        self.pages[index].bytes()
    }

    pub(crate) fn replace_page_payload(
        &mut self,
        slot_cell: SlotGenerationCell,
        page_payload: &[u8],
    ) {
        let index = self
            .find_page_index(slot_cell)
            .expect("append page exists before replacement");
        let generation = self.pages[index].cell().generation();
        self.pages[index].replace_bytes(encode_page(generation, page_payload));
        if !self.page_slots.contains(&slot_cell) {
            self.page_slots.push(slot_cell);
        }
    }

    pub(crate) fn put_extent(&mut self, extent_cell: ExtentGenerationCell, extent_bytes: &[u8]) {
        if let Some(index) = self
            .extents
            .iter()
            .position(|extent| extent.cell() == extent_cell)
        {
            self.extents[index].replace_bytes(extent_bytes.to_vec());
        } else {
            self.extents
                .push(StoredExtentBytes::new(extent_cell, extent_bytes.to_vec()));
        }
        if !self.extent_cells.contains(&extent_cell) {
            self.extent_cells.push(extent_cell);
        }
    }

    pub(crate) fn page_for_reference(
        &self,
        reference: PhysicalReference,
    ) -> Result<&StoredPageBytes, PlatformPhysicalFacadeDenial> {
        let segment_id = reference.segment_id().ok_or_else(missing_record)?;
        let page_id = reference.page_id().ok_or_else(missing_record)?;
        self.pages
            .iter()
            .find(|page| page.cell().segment_id() == segment_id && page.cell().page_id() == page_id)
            .ok_or_else(missing_record)
    }

    pub(crate) fn extent_for_reference(
        &self,
        reference: PhysicalReference,
    ) -> Result<&StoredExtentBytes, PlatformPhysicalFacadeDenial> {
        let segment_id = reference.segment_id().ok_or_else(missing_record)?;
        let extent_id = reference.extent_id().ok_or_else(missing_record)?;
        self.extents
            .iter()
            .find(|extent| {
                extent.cell().segment_id() == segment_id && extent.cell().extent_id() == extent_id
            })
            .ok_or_else(missing_record)
    }

    pub(crate) fn has_admitted_reference(&self, reference: PhysicalReference) -> bool {
        self.verifier_admitted_references.contains(&reference)
            || self
                .page_slots
                .iter()
                .map(|cell| {
                    PhysicalReferenceAuthority::s1()
                        .admit_page_slot(*cell)
                        .reference()
                })
                .any(|admitted| admitted == reference)
            || self
                .extent_cells
                .iter()
                .map(|cell| {
                    PhysicalReferenceAuthority::s1()
                        .admit_extent(*cell)
                        .reference()
                })
                .any(|admitted| admitted == reference)
    }

    pub(crate) fn page_slots(&self) -> &[SlotGenerationCell] {
        &self.page_slots
    }

    pub(crate) fn extent_cells(&self) -> &[ExtentGenerationCell] {
        &self.extent_cells
    }

    pub(crate) fn runtime_discovered_references(&self) -> Vec<PhysicalReference> {
        let references = PhysicalReferenceAuthority::s1();
        let mut discovered = Vec::new();
        if let Some(root_publication) = self.root_publication {
            discovered.push(
                references
                    .admit_root_publication(root_publication)
                    .reference(),
            );
        }
        for slot_cell in &self.page_slots {
            discovered.push(references.admit_page_slot(*slot_cell).reference());
        }
        for extent_cell in &self.extent_cells {
            discovered.push(references.admit_extent(*extent_cell).reference());
        }
        discovered
    }

    pub(crate) fn runtime_traversal_report(&self) -> ManifestTraversalReport {
        ManifestTraversalReport::from_runtime_counts(
            u32::from(self.root_publication.is_some()),
            self.runtime_segment_count(),
            self.page_slots.len() as u32,
            self.extent_cells.len() as u32,
            self.runtime_allocation_class_count(),
            0,
        )
    }

    pub(crate) fn persisted_layout(&self) -> PersistedPhysicalLayout {
        let mut builder = PersistedPhysicalLayout::builder()
            .segment_manifest(self.segment_manifest.clone())
            .extent_manifest(self.extent_manifest.clone())
            .free_space_map(self.free_space_map.clone());
        for root in &self.root_manifest_candidates {
            builder = builder.root_manifest(root.clone());
        }
        for page in &self.pages {
            builder = builder.page(PersistedPageBytes::new(page.cell(), page.bytes().to_vec()));
        }
        for extent in &self.extents {
            builder = builder.extent(PersistedExtentBytes::new(
                extent.cell(),
                extent.bytes().to_vec(),
            ));
        }
        builder.build()
    }

    pub(crate) fn replace_manifest_bytes(
        &mut self,
        root_publication: Option<RootPublicationCell>,
        root_manifest_candidates: Vec<Vec<u8>>,
        segment_manifest: Vec<u8>,
        extent_manifest: Vec<u8>,
        free_space_map: Vec<u8>,
    ) {
        self.root_publication = root_publication;
        self.root_manifest_candidates = root_manifest_candidates;
        self.segment_manifest = segment_manifest;
        self.extent_manifest = extent_manifest;
        self.free_space_map = free_space_map;
    }

    fn find_page_index(&self, slot_cell: SlotGenerationCell) -> Option<usize> {
        self.pages.iter().position(|page| {
            page.cell().segment_id() == slot_cell.segment_id()
                && page.cell().page_id() == slot_cell.page_id()
        })
    }

    fn runtime_segment_count(&self) -> u32 {
        let mut segments = Vec::new();
        for slot in &self.page_slots {
            if !segments.contains(&slot.segment_id()) {
                segments.push(slot.segment_id());
            }
        }
        for extent in &self.extent_cells {
            if !segments.contains(&extent.segment_id()) {
                segments.push(extent.segment_id());
            }
        }
        segments.len() as u32
    }

    fn runtime_allocation_class_count(&self) -> u32 {
        u32::from(!self.page_slots.is_empty()) + u32::from(!self.extent_cells.is_empty())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StoredPageBytes {
    cell: crate::PageGenerationCell,
    bytes: Vec<u8>,
}

impl StoredPageBytes {
    fn new(cell: crate::PageGenerationCell, bytes: Vec<u8>) -> Self {
        Self { cell, bytes }
    }

    pub(crate) const fn cell(&self) -> crate::PageGenerationCell {
        self.cell
    }

    pub(crate) fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    fn replace_bytes(&mut self, bytes: Vec<u8>) {
        self.bytes = bytes;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StoredExtentBytes {
    cell: ExtentGenerationCell,
    bytes: Vec<u8>,
}

impl StoredExtentBytes {
    fn new(cell: ExtentGenerationCell, bytes: Vec<u8>) -> Self {
        Self { cell, bytes }
    }

    pub(crate) const fn cell(&self) -> ExtentGenerationCell {
        self.cell
    }

    pub(crate) fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    fn replace_bytes(&mut self, bytes: Vec<u8>) {
        self.bytes = bytes;
    }
}

fn encode_empty_page(generation: crate::PhysicalGeneration) -> Vec<u8> {
    encode_page(generation, &[])
}

fn encode_page(generation: crate::PhysicalGeneration, payload: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(PHYSICAL_HEADER_LENGTH as usize + payload.len());
    bytes.push(PhysicalPageKind::DataPage.tag());
    bytes.extend_from_slice(
        &crate::PhysicalFormatVersion::s1_initial()
            .value()
            .to_le_bytes(),
    );
    bytes.extend_from_slice(&PHYSICAL_HEADER_LENGTH.to_le_bytes());
    bytes.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&generation.get().to_le_bytes());
    bytes.push(PhysicalPublicationState::Published.code());
    bytes.extend_from_slice(&0u32.to_le_bytes());
    bytes.extend_from_slice(&0u64.to_le_bytes());
    bytes.extend_from_slice(payload);
    bytes
}

fn reference_to_slot_cell(reference: &PhysicalReference) -> Option<SlotGenerationCell> {
    Some(
        PhysicalGenerationAuthority::s1()
            .slot_cell(
                reference.segment_id()?,
                reference.page_id()?,
                reference.slot()?,
            )
            .with_slot_generation(reference.generation()),
    )
}

fn reference_to_extent_cell(reference: &PhysicalReference) -> Option<ExtentGenerationCell> {
    Some(
        PhysicalGenerationAuthority::s1()
            .extent_cell(reference.segment_id()?, reference.extent_id()?)
            .with_extent_generation(reference.generation()),
    )
}

fn reference_to_root_publication_cell(
    reference: &PhysicalReference,
) -> Option<RootPublicationCell> {
    Some(
        PhysicalGenerationAuthority::s1()
            .root_publication_cell(reference.root_reference()?)
            .with_root_publication_generation(reference.generation()),
    )
}

fn missing_record() -> PlatformPhysicalFacadeDenial {
    PlatformPhysicalFacadeDenial::new(PlatformPhysicalFacadeDenialKind::MissingPhysicalRecord)
}
