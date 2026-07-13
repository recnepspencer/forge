use super::{
    storage_reference_index::{build_reference_index, StoredReferenceIndex},
    storage_segment_occupancy::{build_segment_occupancy, StoredSegmentOccupancy},
    storage_support::{
        encode_empty_page, encode_page, reference_to_extent_cell,
        reference_to_root_publication_cell, reference_to_slot_cell,
    },
};
use crate::{
    ExtentGenerationCell, ManifestTraversalReport, PersistedExtentBytes, PersistedPageBytes,
    PersistedPhysicalLayout, PhysicalGenerationAuthority, PhysicalReference,
    PhysicalReferenceAuthority, PhysicalSegmentId, PlatformPhysicalFacadeDenial,
    PlatformPhysicalFacadeDenialKind, RootPublicationCell, SlotGenerationCell,
};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct PlatformPhysicalFacadeStorage {
    pages: Vec<StoredPageBytes>,
    extents: Vec<StoredExtentBytes>,
    page_slots: Vec<SlotGenerationCell>,
    extent_cells: Vec<ExtentGenerationCell>,
    segment_occupancy: BTreeMap<PhysicalSegmentId, StoredSegmentOccupancy>,
    reference_index: StoredReferenceIndex,
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
        let page_slots: Vec<_> = verifier_admitted_references
            .iter()
            .filter_map(reference_to_slot_cell)
            .collect();
        let extent_cells: Vec<_> = verifier_admitted_references
            .iter()
            .filter_map(reference_to_extent_cell)
            .collect();
        let pages: Vec<_> = layout
            .pages()
            .iter()
            .map(|page| StoredPageBytes::new(page.cell(), page.bytes().to_vec()))
            .collect();
        let extents: Vec<_> = layout
            .extents()
            .iter()
            .map(|extent| StoredExtentBytes::new(extent.cell(), extent.bytes().to_vec()))
            .collect();
        let root_publication = verifier_admitted_references
            .iter()
            .find_map(reference_to_root_publication_cell);
        let reference_index = build_reference_index(
            &pages,
            &extents,
            &page_slots,
            &extent_cells,
            root_publication,
            &verifier_admitted_references,
        );
        Self {
            pages,
            extents,
            page_slots: page_slots.clone(),
            extent_cells: extent_cells.clone(),
            segment_occupancy: build_segment_occupancy(&page_slots, &extent_cells),
            reference_index,
            root_manifest_candidates: layout.root_manifest_candidates().to_vec(),
            segment_manifest: layout.segment_manifest().to_vec(),
            extent_manifest: layout.extent_manifest().to_vec(),
            free_space_map: layout.free_space_map().to_vec(),
            root_publication,
            verifier_admitted_references,
        }
    }

    pub(crate) fn page_bytes_for_append(&mut self, slot_cell: SlotGenerationCell) -> &[u8] {
        let page_cell = PhysicalGenerationAuthority::for_canonical_physical_format()
            .page_cell(slot_cell.segment_id(), slot_cell.page_id())
            .with_page_generation(slot_cell.generation());
        if self.find_page_index(slot_cell).is_none() {
            self.pages.push(StoredPageBytes::new(
                page_cell,
                encode_empty_page(slot_cell.generation()),
            ));
            self.reference_index.record_page_cell(
                slot_cell.segment_id(),
                slot_cell.page_id(),
                self.pages.len() - 1,
            );
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
            self.reference_index.record_page_slot(slot_cell);
            self.segment_occupancy
                .entry(slot_cell.segment_id())
                .or_default()
                .record_page_slot();
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
            self.reference_index.record_extent_cell(
                extent_cell.segment_id(),
                extent_cell.extent_id(),
                self.extents.len() - 1,
            );
        }
        if !self.extent_cells.contains(&extent_cell) {
            self.extent_cells.push(extent_cell);
            self.reference_index.record_extent(extent_cell);
            self.segment_occupancy
                .entry(extent_cell.segment_id())
                .or_default()
                .record_extent();
        }
    }

    pub(crate) fn page_for_reference(
        &self,
        reference: PhysicalReference,
    ) -> Result<&StoredPageBytes, PlatformPhysicalFacadeDenial> {
        let segment_id = reference.segment_id().ok_or_else(missing_record)?;
        let page_id = reference.page_id().ok_or_else(missing_record)?;
        let index = self
            .reference_index
            .page_index(segment_id, page_id)
            .ok_or_else(missing_record)?;
        self.pages.get(index).ok_or_else(missing_record)
    }

    pub(crate) fn extent_for_reference(
        &self,
        reference: PhysicalReference,
    ) -> Result<&StoredExtentBytes, PlatformPhysicalFacadeDenial> {
        let segment_id = reference.segment_id().ok_or_else(missing_record)?;
        let extent_id = reference.extent_id().ok_or_else(missing_record)?;
        let index = self
            .reference_index
            .extent_index(segment_id, extent_id)
            .ok_or_else(missing_record)?;
        self.extents.get(index).ok_or_else(missing_record)
    }

    pub(crate) fn has_admitted_reference(&self, reference: PhysicalReference) -> bool {
        self.reference_index.contains(reference)
    }

    pub(crate) fn page_slots(&self) -> &[SlotGenerationCell] {
        &self.page_slots
    }

    pub(crate) fn extent_cells(&self) -> &[ExtentGenerationCell] {
        &self.extent_cells
    }

    pub(crate) fn segment_occupancy(
        &self,
        segment_id: PhysicalSegmentId,
    ) -> Option<StoredSegmentOccupancy> {
        self.segment_occupancy.get(&segment_id).copied()
    }

    pub(crate) fn runtime_discovered_references(&self) -> Vec<PhysicalReference> {
        let references = PhysicalReferenceAuthority::for_canonical_physical_format();
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
        self.reference_index = build_reference_index(
            &self.pages,
            &self.extents,
            &self.page_slots,
            &self.extent_cells,
            self.root_publication,
            &self.verifier_admitted_references,
        );
    }

    fn find_page_index(&self, slot_cell: SlotGenerationCell) -> Option<usize> {
        self.reference_index
            .page_index(slot_cell.segment_id(), slot_cell.page_id())
    }

    fn runtime_segment_count(&self) -> u32 {
        self.segment_occupancy.len() as u32
    }

    fn runtime_allocation_class_count(&self) -> u32 {
        u32::from(!self.page_slots.is_empty()) + u32::from(!self.extent_cells.is_empty())
    }

    pub(crate) fn admit_bootstrap_open_witness(
        &self,
        headers: &crate::PhysicalHeaderAuthority,
    ) -> Result<crate::PhysicalBootstrapCatalogOpenWitness, crate::PhysicalBootstrapCatalogDenial>
    {
        crate::PhysicalBootstrapCatalogOpenWitness::admit_persisted_layout(
            headers,
            &self.persisted_layout(),
        )
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

fn missing_record() -> PlatformPhysicalFacadeDenial {
    PlatformPhysicalFacadeDenial::new(PlatformPhysicalFacadeDenialKind::MissingPhysicalRecord)
}
