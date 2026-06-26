use crate::{
    ExtentGenerationCell, ExtentMembership, ExtentRecordCounterSnapshot, ExtentRecordDenial,
    ExtentRecordDenialKind, FramedRecordPayload, PhysicalByteOrder, PhysicalFrameKind,
    PhysicalGeneration, PhysicalHeaderAuthority, PhysicalHeaderDecodeDenial,
    PhysicalPublicationState, PhysicalReference, PhysicalReferenceAdmissionWitness,
    PhysicalReferenceAuthority, PhysicalReferenceKind, PhysicalReferenceValidationWitness,
    RecordPlacementClass, PHYSICAL_HEADER_LENGTH,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalExtentRecordAuthority {
    headers: PhysicalHeaderAuthority,
    references: PhysicalReferenceAuthority,
}

impl PhysicalExtentRecordAuthority {
    pub const fn s1(headers: PhysicalHeaderAuthority) -> Self {
        Self {
            headers,
            references: PhysicalReferenceAuthority::s1(),
        }
    }

    pub fn append_extent_record(
        &self,
        membership: ExtentMembership,
        request: ExtentRecordAppendRequest<'_>,
    ) -> Result<ExtentRecordAppendReport, ExtentRecordDenial> {
        let counters = ExtentRecordCounterSnapshot::for_append_attempt().with_membership_check();
        let cell = admitted_large_record_membership(membership, counters)?;
        reject_membership_cell_mismatch(cell, request.extent_cell(), counters)?;
        let frame_bytes = encode_extent_record_frame(
            self.byte_order(),
            request.extent_cell().generation(),
            request.payload(),
        );
        let counters = counters.with_length_check();
        reject_extent_length_mismatch(membership, frame_bytes.len(), counters)?;
        let admission = self.references.admit_extent(request.extent_cell());
        let counters = counters.with_extent_write();
        Ok(ExtentRecordAppendReport::new(
            frame_bytes,
            admission,
            ExtentBackedRecordPlacement::from_admission(admission, counters),
            counters,
        ))
    }

    pub fn locate_extent_record<'a>(
        &self,
        extent_bytes: &'a [u8],
        membership: ExtentMembership,
        validation: PhysicalReferenceValidationWitness,
    ) -> Result<ExtentRecordLocateReport<'a>, ExtentRecordDenial> {
        let counters = ExtentRecordCounterSnapshot::for_locate_attempt();
        reject_moved_slot_misuse(validation, counters)?;
        let counters = counters.with_membership_check();
        let cell = admitted_large_record_membership(membership, counters)?;
        reject_reference_membership_mismatch(validation.reference(), cell, counters)?;
        let counters = counters.with_length_check();
        reject_extent_length_mismatch(membership, extent_bytes.len(), counters)?;
        let counters = counters.with_header_decode();
        let header = self
            .headers
            .decode_frame_header(
                validation,
                extent_bytes,
                PhysicalFrameKind::ExtentRecordFrame,
            )
            .map_err(|denial| header_denial(denial, counters))?;
        let payload = self
            .headers
            .payload_view(extent_bytes, header.witness())
            .map_err(|denial| header_denial(denial, counters))?;
        let counters = counters.with_payload_view();
        let placement = ExtentBackedRecordPlacement::from_validation(validation, counters);
        let view = ExtentBackedRecordView::new(
            PhysicalFrameKind::ExtentRecordFrame,
            FramedRecordPayload::new(payload.view().as_bytes()),
            placement,
        );
        Ok(ExtentRecordLocateReport::new(
            validation.reference(),
            view,
            counters,
        ))
    }

    const fn byte_order(&self) -> PhysicalByteOrder {
        self.headers.byte_order()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtentRecordAppendRequest<'a> {
    extent_cell: ExtentGenerationCell,
    payload: &'a [u8],
}

impl<'a> ExtentRecordAppendRequest<'a> {
    pub const fn large_record(extent_cell: ExtentGenerationCell, payload: &'a [u8]) -> Self {
        Self {
            extent_cell,
            payload,
        }
    }

    pub const fn extent_cell(self) -> ExtentGenerationCell {
        self.extent_cell
    }

    pub const fn payload(self) -> &'a [u8] {
        self.payload
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtentRecordAppendReport {
    extent_bytes: Vec<u8>,
    reference_admission: PhysicalReferenceAdmissionWitness,
    placement: ExtentBackedRecordPlacement,
    counters: ExtentRecordCounterSnapshot,
}

impl ExtentRecordAppendReport {
    fn new(
        extent_bytes: Vec<u8>,
        reference_admission: PhysicalReferenceAdmissionWitness,
        placement: ExtentBackedRecordPlacement,
        counters: ExtentRecordCounterSnapshot,
    ) -> Self {
        Self {
            extent_bytes,
            reference_admission,
            placement,
            counters,
        }
    }

    pub fn extent_bytes(&self) -> &[u8] {
        &self.extent_bytes
    }

    pub const fn reference_admission(&self) -> PhysicalReferenceAdmissionWitness {
        self.reference_admission
    }

    pub const fn placement(&self) -> ExtentBackedRecordPlacement {
        self.placement
    }

    pub const fn counters(&self) -> ExtentRecordCounterSnapshot {
        self.counters
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtentRecordLocateReport<'a> {
    reference: PhysicalReference,
    record_view: ExtentBackedRecordView<'a>,
    counters: ExtentRecordCounterSnapshot,
}

impl<'a> ExtentRecordLocateReport<'a> {
    const fn new(
        reference: PhysicalReference,
        record_view: ExtentBackedRecordView<'a>,
        counters: ExtentRecordCounterSnapshot,
    ) -> Self {
        Self {
            reference,
            record_view,
            counters,
        }
    }

    pub const fn reference(self) -> PhysicalReference {
        self.reference
    }

    pub const fn record_view(self) -> ExtentBackedRecordView<'a> {
        self.record_view
    }

    pub const fn counters(self) -> ExtentRecordCounterSnapshot {
        self.counters
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtentBackedRecordView<'a> {
    frame_kind: PhysicalFrameKind,
    payload: FramedRecordPayload<'a>,
    placement: ExtentBackedRecordPlacement,
}

impl<'a> ExtentBackedRecordView<'a> {
    const fn new(
        frame_kind: PhysicalFrameKind,
        payload: FramedRecordPayload<'a>,
        placement: ExtentBackedRecordPlacement,
    ) -> Self {
        Self {
            frame_kind,
            payload,
            placement,
        }
    }

    pub const fn frame_kind(self) -> PhysicalFrameKind {
        self.frame_kind
    }

    pub const fn payload(self) -> FramedRecordPayload<'a> {
        self.payload
    }

    pub const fn placement(self) -> ExtentBackedRecordPlacement {
        self.placement
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtentBackedRecordPlacement {
    reference: PhysicalReference,
    placement_class: RecordPlacementClass,
    counters: ExtentRecordCounterSnapshot,
}

impl ExtentBackedRecordPlacement {
    const fn from_admission(
        admission: PhysicalReferenceAdmissionWitness,
        counters: ExtentRecordCounterSnapshot,
    ) -> Self {
        Self {
            reference: admission.reference(),
            placement_class: RecordPlacementClass::ExtentBackedReference,
            counters,
        }
    }

    const fn from_validation(
        validation: PhysicalReferenceValidationWitness,
        counters: ExtentRecordCounterSnapshot,
    ) -> Self {
        Self {
            reference: validation.reference(),
            placement_class: RecordPlacementClass::ExtentBackedReference,
            counters,
        }
    }

    pub const fn reference(self) -> PhysicalReference {
        self.reference
    }

    pub const fn placement_class(self) -> RecordPlacementClass {
        self.placement_class
    }

    pub const fn counters(self) -> ExtentRecordCounterSnapshot {
        self.counters
    }
}

fn admitted_large_record_membership(
    membership: ExtentMembership,
    counters: ExtentRecordCounterSnapshot,
) -> Result<ExtentGenerationCell, ExtentRecordDenial> {
    membership.cell().ok_or_else(|| {
        ExtentRecordDenial::new(ExtentRecordDenialKind::MissingExtentMembership, counters)
    })
}

fn reject_membership_cell_mismatch(
    membership_cell: ExtentGenerationCell,
    request_cell: ExtentGenerationCell,
    counters: ExtentRecordCounterSnapshot,
) -> Result<(), ExtentRecordDenial> {
    if membership_cell != request_cell {
        return Err(ExtentRecordDenial::new(
            ExtentRecordDenialKind::ExtentReferenceMismatch,
            counters,
        ));
    }
    Ok(())
}

fn reject_reference_membership_mismatch(
    reference: PhysicalReference,
    membership_cell: ExtentGenerationCell,
    counters: ExtentRecordCounterSnapshot,
) -> Result<(), ExtentRecordDenial> {
    if reference.kind() != PhysicalReferenceKind::ExtentBacked
        || reference.segment_id() != Some(membership_cell.segment_id())
        || reference.extent_id() != Some(membership_cell.extent_id())
        || reference.generation() != membership_cell.generation()
    {
        return Err(ExtentRecordDenial::new(
            ExtentRecordDenialKind::ExtentReferenceMismatch,
            counters,
        ));
    }
    Ok(())
}

fn reject_extent_length_mismatch(
    membership: ExtentMembership,
    actual_length: usize,
    counters: ExtentRecordCounterSnapshot,
) -> Result<(), ExtentRecordDenial> {
    let expected = membership
        .declared_extent_length()
        .expect("large-record membership was admitted");
    if expected != actual_length {
        return Err(ExtentRecordDenial::new(
            ExtentRecordDenialKind::ExtentLengthMismatch,
            counters,
        )
        .with_lengths(expected, actual_length));
    }
    Ok(())
}

fn reject_moved_slot_misuse(
    validation: PhysicalReferenceValidationWitness,
    counters: ExtentRecordCounterSnapshot,
) -> Result<(), ExtentRecordDenial> {
    if validation.reference().kind() == PhysicalReferenceKind::PageSlot {
        return Err(ExtentRecordDenial::new(
            ExtentRecordDenialKind::MovedSlotMisuse,
            counters.with_moved_slot_misuse_rejection(),
        ));
    }
    Ok(())
}

fn header_denial(
    denial: PhysicalHeaderDecodeDenial,
    counters: ExtentRecordCounterSnapshot,
) -> ExtentRecordDenial {
    ExtentRecordDenial::new(ExtentRecordDenialKind::HeaderDecodeDenied, counters)
        .with_header_denial(denial)
}

fn encode_extent_record_frame(
    byte_order: PhysicalByteOrder,
    generation: PhysicalGeneration,
    payload: &[u8],
) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(PHYSICAL_HEADER_LENGTH as usize + payload.len());
    bytes.push(PhysicalFrameKind::ExtentRecordFrame.tag());
    bytes.extend_from_slice(
        &byte_order.write_u16(crate::PhysicalFormatVersion::s1_initial().value()),
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
