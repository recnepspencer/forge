use super::model_storage::{StoredExtentBytes, StoredPageBytes};
use crate::{
    ExtentGenerationCell, PhysicalExtentId, PhysicalPageId, PhysicalReference,
    PhysicalReferenceAuthority, PhysicalReferenceKind, PhysicalRootReference, PhysicalSegmentId,
    RootPublicationCell, SlotGenerationCell,
};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct StoredReferenceIndex {
    admitted_references: BTreeSet<StoredReferenceKey>,
    page_positions: BTreeMap<(PhysicalSegmentId, PhysicalPageId), usize>,
    extent_positions: BTreeMap<(PhysicalSegmentId, PhysicalExtentId), usize>,
}

impl StoredReferenceIndex {
    pub(crate) fn contains(&self, reference: PhysicalReference) -> bool {
        self.admitted_references
            .contains(&StoredReferenceKey::from_reference(reference))
    }

    pub(crate) fn page_index(
        &self,
        segment_id: PhysicalSegmentId,
        page_id: PhysicalPageId,
    ) -> Option<usize> {
        self.page_positions.get(&(segment_id, page_id)).copied()
    }

    pub(crate) fn extent_index(
        &self,
        segment_id: PhysicalSegmentId,
        extent_id: PhysicalExtentId,
    ) -> Option<usize> {
        self.extent_positions.get(&(segment_id, extent_id)).copied()
    }

    pub(crate) fn record_page_cell(
        &mut self,
        segment_id: PhysicalSegmentId,
        page_id: PhysicalPageId,
        index: usize,
    ) {
        self.page_positions.insert((segment_id, page_id), index);
    }

    pub(crate) fn record_extent_cell(
        &mut self,
        segment_id: PhysicalSegmentId,
        extent_id: PhysicalExtentId,
        index: usize,
    ) {
        self.extent_positions.insert((segment_id, extent_id), index);
    }

    pub(crate) fn record_page_slot(&mut self, slot_cell: SlotGenerationCell) {
        self.admitted_references
            .insert(StoredReferenceKey::from_reference(
                PhysicalReferenceAuthority::for_canonical_physical_format()
                    .admit_page_slot(slot_cell)
                    .reference(),
            ));
    }

    pub(crate) fn record_extent(&mut self, extent_cell: ExtentGenerationCell) {
        self.admitted_references
            .insert(StoredReferenceKey::from_reference(
                PhysicalReferenceAuthority::for_canonical_physical_format()
                    .admit_extent(extent_cell)
                    .reference(),
            ));
    }
}

pub(crate) fn build_reference_index(
    pages: &[StoredPageBytes],
    extents: &[StoredExtentBytes],
    page_slots: &[SlotGenerationCell],
    extent_cells: &[ExtentGenerationCell],
    root_publication: Option<RootPublicationCell>,
    verifier_admitted_references: &[PhysicalReference],
) -> StoredReferenceIndex {
    let mut index = StoredReferenceIndex::default();
    for (position, page) in pages.iter().enumerate() {
        index.record_page_cell(page.cell().segment_id(), page.cell().page_id(), position);
    }
    for (position, extent) in extents.iter().enumerate() {
        index.record_extent_cell(
            extent.cell().segment_id(),
            extent.cell().extent_id(),
            position,
        );
    }
    for slot_cell in page_slots {
        index.record_page_slot(*slot_cell);
    }
    for extent_cell in extent_cells {
        index.record_extent(*extent_cell);
    }
    if let Some(root_publication) = root_publication {
        index
            .admitted_references
            .insert(StoredReferenceKey::from_reference(
                PhysicalReferenceAuthority::for_canonical_physical_format()
                    .admit_root_publication(root_publication)
                    .reference(),
            ));
    }
    for reference in verifier_admitted_references {
        index
            .admitted_references
            .insert(StoredReferenceKey::from_reference(*reference));
    }
    index
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct StoredReferenceKey {
    kind_code: u8,
    segment_id: Option<PhysicalSegmentId>,
    page_id: Option<PhysicalPageId>,
    extent_id: Option<PhysicalExtentId>,
    slot: Option<crate::PhysicalRecordSlot>,
    root_reference: Option<PhysicalRootReference>,
    generation: crate::PhysicalGeneration,
}

impl StoredReferenceKey {
    fn from_reference(reference: PhysicalReference) -> Self {
        Self {
            kind_code: kind_code(reference.kind()),
            segment_id: reference.segment_id(),
            page_id: reference.page_id(),
            extent_id: reference.extent_id(),
            slot: reference.slot(),
            root_reference: reference.root_reference(),
            generation: reference.generation(),
        }
    }
}

const fn kind_code(kind: PhysicalReferenceKind) -> u8 {
    match kind {
        PhysicalReferenceKind::PageSlot => 0,
        PhysicalReferenceKind::ExtentBacked => 1,
        PhysicalReferenceKind::FreeSpaceReuse => 2,
        PhysicalReferenceKind::RootPublication => 3,
    }
}
