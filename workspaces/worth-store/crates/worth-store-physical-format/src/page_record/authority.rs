use crate::page_record::slot_directory::append_occupied_entry;
use crate::{
    FramedRecordPayload, FramedRecordView, PageGenerationCell, PageRecordCounterSnapshot,
    PageRecordDenial, PageRecordDenialKind, PhysicalByteOrder, PhysicalFrameKind,
    PhysicalGeneration, PhysicalGenerationOwner, PhysicalHeaderAuthority,
    PhysicalHeaderDecodeDenial, PhysicalHeaderDecodeDenialKind, PhysicalHeaderDecodeReport,
    PhysicalHeaderDecodeWitness, PhysicalHeaderKind, PhysicalPageKind, PhysicalPublicationState,
    PhysicalReference, PhysicalReferenceAdmissionWitness, PhysicalReferenceAuthority,
    PhysicalReferenceValidationWitness, RecordPagePayload, RecordPlacementWitness, SlotDirectory,
    SlotDirectoryEntry, SlotDirectoryEntryState, SlotGenerationCell, PHYSICAL_HEADER_LENGTH,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalPageRecordAuthority {
    headers: PhysicalHeaderAuthority,
    references: PhysicalReferenceAuthority,
}

impl PhysicalPageRecordAuthority {
    pub const fn for_canonical_physical_format(headers: PhysicalHeaderAuthority) -> Self {
        Self {
            headers,
            references: PhysicalReferenceAuthority::for_canonical_physical_format(),
        }
    }

    pub fn decode_record_page_header(
        &self,
        cell: PageGenerationCell,
        bytes: &[u8],
        expected_kind: PhysicalPageKind,
    ) -> Result<PhysicalHeaderDecodeReport, PhysicalHeaderDecodeDenial> {
        self.headers.decode_page_header(cell, bytes, expected_kind)
    }

    pub fn admit_record_page_payload<'a>(
        &self,
        bytes: &'a [u8],
        witness: PhysicalHeaderDecodeWitness,
    ) -> Result<RecordPagePayload<'a>, PhysicalHeaderDecodeDenial> {
        reject_non_record_page_witness(witness)?;
        self.headers
            .payload_view(bytes, witness)
            .map(|admission| RecordPagePayload::new(admission.view()))
    }

    pub fn append_record(
        &self,
        page: RecordPagePayload<'_>,
        request: SlotAppendRequest<'_>,
    ) -> Result<RecordAppendReport, PageRecordDenial> {
        let counters = PageRecordCounterSnapshot::for_append(0);
        reject_page_slot_cell_mismatch(page.page_owner(), request.slot_cell(), counters)?;
        let admission = self.references.admit_page_slot(request.slot_cell());
        let frame_bytes = encode_record_frame(
            self.byte_order(),
            request.slot_cell().generation(),
            request.payload(),
        );
        let page_payload = append_occupied_entry(
            page.bytes(),
            self.byte_order(),
            request.slot_cell().slot(),
            request.slot_cell().generation(),
            &frame_bytes,
        )?;
        let counters = PageRecordCounterSnapshot::for_append(1).with_page_write();
        Ok(RecordAppendReport::new(
            page_payload,
            admission,
            RecordPlacementWitness::from_admission(admission, counters),
            counters,
        ))
    }

    pub fn locate_record<'a>(
        &self,
        page: RecordPagePayload<'a>,
        validation: PhysicalReferenceValidationWitness,
    ) -> Result<RecordLocateReport<'a>, PageRecordDenial> {
        let counters = PageRecordCounterSnapshot::for_locate_attempt();
        let reference = validation.reference();
        reject_page_reference_mismatch(page.page_owner(), reference, counters)?;
        let counters = counters.with_slot_lookup();
        let slot = reference
            .slot()
            .expect("validated page-slot reference has slot");
        let directory = SlotDirectory::decode(page.bytes(), self.byte_order(), counters)?;
        let entry = directory.locate(slot, self.byte_order(), counters)?;
        reject_unviewable_slot(entry, validation, counters)?;
        let frame_bytes = frame_slice(page.bytes(), entry, counters)?;
        let framed = self.decode_framed_record(frame_bytes, validation, entry, counters)?;
        Ok(RecordLocateReport::new(
            reference,
            framed,
            RecordPlacementWitness::from_validation(validation, framed.placement().counters()),
            framed.placement().counters(),
        ))
    }

    fn decode_framed_record<'a>(
        &self,
        frame_bytes: &'a [u8],
        validation: PhysicalReferenceValidationWitness,
        entry: SlotDirectoryEntry,
        counters: PageRecordCounterSnapshot,
    ) -> Result<FramedRecordView<'a>, PageRecordDenial> {
        let counters = counters.with_frame_decode();
        let header = self
            .headers
            .decode_frame_header(validation, frame_bytes, PhysicalFrameKind::RecordFrame)
            .map_err(|denial| {
                PageRecordDenial::new(PageRecordDenialKind::HeaderDecodeDenied, counters)
                    .with_header_denial(denial)
            })?;
        let expected = PHYSICAL_HEADER_LENGTH as usize + header.witness().payload_length() as usize;
        if expected != entry.frame_length() as usize {
            return Err(
                PageRecordDenial::new(PageRecordDenialKind::FrameLengthMismatch, counters)
                    .with_lengths(expected, entry.frame_length() as usize),
            );
        }
        let payload = self
            .headers
            .payload_view(frame_bytes, header.witness())
            .map_err(|denial| {
                PageRecordDenial::new(PageRecordDenialKind::HeaderDecodeDenied, counters)
                    .with_header_denial(denial)
            })?;
        let placement = RecordPlacementWitness::from_validation(
            validation,
            counters.with_record_payload_view(),
        );
        Ok(FramedRecordView::new(
            PhysicalFrameKind::RecordFrame,
            FramedRecordPayload::new(payload.view().as_bytes()),
            placement,
        ))
    }

    const fn byte_order(&self) -> PhysicalByteOrder {
        self.headers.byte_order()
    }
}

fn reject_non_record_page_witness(
    witness: PhysicalHeaderDecodeWitness,
) -> Result<(), PhysicalHeaderDecodeDenial> {
    if witness.kind() != PhysicalHeaderKind::Page(PhysicalPageKind::DataPage) {
        return Err(PhysicalHeaderDecodeDenial::new(
            PhysicalHeaderDecodeDenialKind::UnexpectedPageKind,
            witness.counters(),
        )
        .with_expected_page_kind(PhysicalPageKind::DataPage)
        .with_observed_kind_tag(witness.kind().tag()));
    }
    Ok(())
}

fn reject_page_slot_cell_mismatch(
    page_owner: PhysicalGenerationOwner,
    slot_cell: SlotGenerationCell,
    counters: PageRecordCounterSnapshot,
) -> Result<(), PageRecordDenial> {
    if page_owner.segment_id() != Some(slot_cell.segment_id())
        || page_owner.page_id() != Some(slot_cell.page_id())
    {
        return Err(
            PageRecordDenial::new(PageRecordDenialKind::PageReferenceMismatch, counters)
                .with_slot(slot_cell.slot()),
        );
    }
    Ok(())
}

fn reject_page_reference_mismatch(
    page_owner: PhysicalGenerationOwner,
    reference: PhysicalReference,
    counters: PageRecordCounterSnapshot,
) -> Result<(), PageRecordDenial> {
    if page_owner.segment_id() != reference.segment_id()
        || page_owner.page_id() != reference.page_id()
    {
        return Err(PageRecordDenial::new(
            PageRecordDenialKind::PageReferenceMismatch,
            counters,
        ));
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SlotAppendRequest<'a> {
    slot_cell: SlotGenerationCell,
    payload: &'a [u8],
}

impl<'a> SlotAppendRequest<'a> {
    pub const fn ordinary(slot_cell: SlotGenerationCell, payload: &'a [u8]) -> Self {
        Self { slot_cell, payload }
    }

    pub const fn slot_cell(self) -> SlotGenerationCell {
        self.slot_cell
    }

    pub const fn payload(self) -> &'a [u8] {
        self.payload
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordAppendReport {
    page_payload: Vec<u8>,
    reference_admission: PhysicalReferenceAdmissionWitness,
    placement: RecordPlacementWitness,
    counters: PageRecordCounterSnapshot,
}

impl RecordAppendReport {
    fn new(
        page_payload: Vec<u8>,
        reference_admission: PhysicalReferenceAdmissionWitness,
        placement: RecordPlacementWitness,
        counters: PageRecordCounterSnapshot,
    ) -> Self {
        Self {
            page_payload,
            reference_admission,
            placement,
            counters,
        }
    }

    pub fn page_payload(&self) -> &[u8] {
        &self.page_payload
    }

    pub const fn reference_admission(&self) -> PhysicalReferenceAdmissionWitness {
        self.reference_admission
    }

    pub const fn reference(&self) -> crate::PhysicalReference {
        self.reference_admission.reference()
    }

    pub const fn placement(&self) -> RecordPlacementWitness {
        self.placement
    }

    pub const fn counters(&self) -> PageRecordCounterSnapshot {
        self.counters
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordLocateReport<'a> {
    reference: crate::PhysicalReference,
    record_view: FramedRecordView<'a>,
    placement: RecordPlacementWitness,
    counters: PageRecordCounterSnapshot,
}

impl<'a> RecordLocateReport<'a> {
    const fn new(
        reference: crate::PhysicalReference,
        record_view: FramedRecordView<'a>,
        placement: RecordPlacementWitness,
        counters: PageRecordCounterSnapshot,
    ) -> Self {
        Self {
            reference,
            record_view,
            placement,
            counters,
        }
    }

    pub const fn reference(self) -> crate::PhysicalReference {
        self.reference
    }

    pub const fn record_view(self) -> FramedRecordView<'a> {
        self.record_view
    }

    pub const fn placement(self) -> RecordPlacementWitness {
        self.placement
    }

    pub const fn counters(self) -> PageRecordCounterSnapshot {
        self.counters
    }
}

fn reject_unviewable_slot(
    entry: SlotDirectoryEntry,
    validation: PhysicalReferenceValidationWitness,
    counters: PageRecordCounterSnapshot,
) -> Result<(), PageRecordDenial> {
    if entry.generation() != validation.reference().generation() {
        return Err(
            PageRecordDenial::new(PageRecordDenialKind::SlotGenerationMismatch, counters)
                .with_slot(entry.slot()),
        );
    }
    match entry.state() {
        SlotDirectoryEntryState::Occupied => Ok(()),
        SlotDirectoryEntryState::Deleted => Err(slot_state_denial(
            PageRecordDenialKind::DeletedSlot,
            entry,
            counters,
        )),
        SlotDirectoryEntryState::Free => Err(slot_state_denial(
            PageRecordDenialKind::FreeSlot,
            entry,
            counters,
        )),
        SlotDirectoryEntryState::Reserved => Err(slot_state_denial(
            PageRecordDenialKind::ReservedSlot,
            entry,
            counters,
        )),
        SlotDirectoryEntryState::Moved => Err(slot_state_denial(
            PageRecordDenialKind::MovedSlotWithoutAdmittedReference,
            entry,
            counters,
        )),
    }
}

fn slot_state_denial(
    kind: PageRecordDenialKind,
    entry: SlotDirectoryEntry,
    counters: PageRecordCounterSnapshot,
) -> PageRecordDenial {
    PageRecordDenial::new(kind, counters).with_slot(entry.slot())
}

fn frame_slice(
    page_payload: &[u8],
    entry: SlotDirectoryEntry,
    counters: PageRecordCounterSnapshot,
) -> Result<&[u8], PageRecordDenial> {
    let start = entry.offset() as usize;
    let end = start.saturating_add(entry.frame_length() as usize);
    if start >= page_payload.len() || end > page_payload.len() {
        return Err(
            PageRecordDenial::new(PageRecordDenialKind::FrameOutOfBounds, counters)
                .with_lengths(end, page_payload.len()),
        );
    }
    Ok(&page_payload[start..end])
}

fn encode_record_frame(
    byte_order: PhysicalByteOrder,
    generation: PhysicalGeneration,
    payload: &[u8],
) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(PHYSICAL_HEADER_LENGTH as usize + payload.len());
    bytes.push(PhysicalFrameKind::RecordFrame.tag());
    bytes.extend_from_slice(
        &byte_order.write_u16(crate::PhysicalFormatVersion::initial_format_version().value()),
    );
    bytes.extend_from_slice(&byte_order.write_u16(PHYSICAL_HEADER_LENGTH));
    bytes.extend_from_slice(&byte_order.write_u32(payload.len() as u32));
    bytes.extend_from_slice(&byte_order.write_u64(generation.get()));
    bytes.push(PhysicalPublicationState::Published.code());
    bytes.extend_from_slice(&byte_order.write_u32(0));
    bytes.extend_from_slice(&byte_order.write_u64(0));
    bytes.extend_from_slice(payload);
    bytes
}
