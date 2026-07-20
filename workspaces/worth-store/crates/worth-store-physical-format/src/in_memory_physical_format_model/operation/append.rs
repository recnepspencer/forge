use super::storage::InMemoryPhysicalFormatModelStorage;
use super::InMemoryPhysicalFormatModel;
use crate::{
    ExtentMembership, ExtentRecordAppendRequest, InMemoryPhysicalFormatModelCounterSnapshot,
    InMemoryPhysicalFormatModelDenial, InMemoryPhysicalFormatModelDenialKind,
    PhysicalExtentRecordAuthority, PhysicalPageKind, PhysicalPageRecordAuthority,
    PlatformPhysicalAppendReport, PlatformPhysicalAppendRequest, PlatformPhysicalRecordTarget,
    SlotAppendRequest, SlotGenerationCell, PHYSICAL_HEADER_LENGTH,
};

impl InMemoryPhysicalFormatModel {
    pub fn append_physical_record(
        &mut self,
        request: PlatformPhysicalAppendRequest<'_>,
    ) -> Result<PlatformPhysicalAppendReport, InMemoryPhysicalFormatModelDenial> {
        let append = append_physical_record(
            &mut self.storage,
            &self.page_records,
            &self.extent_records,
            self.counters,
            request,
        )?;
        self.counters = append.counters();
        Ok(append.report())
    }
}

pub(crate) struct PlatformPhysicalAppendOutcome {
    report: PlatformPhysicalAppendReport,
    counters: InMemoryPhysicalFormatModelCounterSnapshot,
}

impl PlatformPhysicalAppendOutcome {
    const fn new(
        report: PlatformPhysicalAppendReport,
        counters: InMemoryPhysicalFormatModelCounterSnapshot,
    ) -> Self {
        Self { report, counters }
    }

    pub(crate) const fn report(&self) -> PlatformPhysicalAppendReport {
        self.report
    }

    pub(crate) const fn counters(&self) -> InMemoryPhysicalFormatModelCounterSnapshot {
        self.counters
    }
}

enum AppendTarget<'a> {
    PageSlot {
        slot_cell: SlotGenerationCell,
        payload: &'a [u8],
    },
    Extent {
        extent_cell: crate::ExtentGenerationCell,
        payload: &'a [u8],
    },
}

pub(crate) fn append_physical_record(
    storage: &mut InMemoryPhysicalFormatModelStorage,
    page_records: &PhysicalPageRecordAuthority,
    extent_records: &PhysicalExtentRecordAuthority,
    counters: InMemoryPhysicalFormatModelCounterSnapshot,
    request: PlatformPhysicalAppendRequest<'_>,
) -> Result<PlatformPhysicalAppendOutcome, InMemoryPhysicalFormatModelDenial> {
    match classify_append_target(&request) {
        AppendTarget::PageSlot { slot_cell, payload } => {
            append_page_slot_record(storage, page_records, counters, slot_cell, payload)
        }
        AppendTarget::Extent {
            extent_cell,
            payload,
        } => append_extent_record(storage, extent_records, counters, extent_cell, payload),
    }
}

fn classify_append_target<'a>(request: &'a PlatformPhysicalAppendRequest<'_>) -> AppendTarget<'a> {
    match request.target() {
        PlatformPhysicalRecordTarget::PageSlot(slot_cell) => AppendTarget::PageSlot {
            slot_cell,
            payload: request.payload(),
        },
        PlatformPhysicalRecordTarget::Extent(extent_cell) => AppendTarget::Extent {
            extent_cell,
            payload: request.payload(),
        },
    }
}

fn append_page_slot_record(
    storage: &mut InMemoryPhysicalFormatModelStorage,
    page_records: &PhysicalPageRecordAuthority,
    counters: InMemoryPhysicalFormatModelCounterSnapshot,
    slot_cell: SlotGenerationCell,
    payload: &[u8],
) -> Result<PlatformPhysicalAppendOutcome, InMemoryPhysicalFormatModelDenial> {
    let evidence = collect_page_slot_append_evidence(storage, slot_cell);
    let admitted = verify_page_slot_header_admission(page_records, &evidence)?;
    let append = verify_page_slot_record_admission(page_records, admitted, slot_cell, payload)?;
    apply_page_slot_storage_mutation(storage, slot_cell, append.page_payload());
    construct_page_slot_append_outcome(append.reference(), counters)
}

fn append_extent_record(
    storage: &mut InMemoryPhysicalFormatModelStorage,
    extent_records: &PhysicalExtentRecordAuthority,
    counters: InMemoryPhysicalFormatModelCounterSnapshot,
    extent_cell: crate::ExtentGenerationCell,
    payload: &[u8],
) -> Result<PlatformPhysicalAppendOutcome, InMemoryPhysicalFormatModelDenial> {
    let membership = classify_extent_append_membership(extent_cell, payload.len());
    let append = verify_extent_record_admission(extent_records, membership, extent_cell, payload)?;
    apply_extent_storage_mutation(storage, extent_cell, append.extent_bytes());
    construct_extent_append_outcome(append.reference_admission().reference(), counters)
}

struct PageSlotAppendEvidence<'a> {
    page_bytes: &'a [u8],
    page_cell: crate::PageGenerationCell,
}

fn collect_page_slot_append_evidence(
    storage: &mut InMemoryPhysicalFormatModelStorage,
    slot_cell: SlotGenerationCell,
) -> PageSlotAppendEvidence<'_> {
    PageSlotAppendEvidence {
        page_bytes: storage.page_bytes_for_append(slot_cell),
        page_cell: page_cell_for_slot(slot_cell),
    }
}

fn verify_page_slot_header_admission<'a>(
    page_records: &PhysicalPageRecordAuthority,
    evidence: &'a PageSlotAppendEvidence<'a>,
) -> Result<crate::RecordPagePayload<'a>, InMemoryPhysicalFormatModelDenial> {
    let header = page_records
        .decode_record_page_header(
            evidence.page_cell,
            evidence.page_bytes,
            PhysicalPageKind::DataPage,
        )
        .map_err(header_decode_denial)?;
    page_records
        .admit_record_page_payload(evidence.page_bytes, header.witness())
        .map_err(header_decode_denial)
}

fn verify_page_slot_record_admission(
    page_records: &PhysicalPageRecordAuthority,
    page: crate::RecordPagePayload<'_>,
    slot_cell: SlotGenerationCell,
    payload: &[u8],
) -> Result<crate::RecordAppendReport, InMemoryPhysicalFormatModelDenial> {
    page_records
        .append_record(page, SlotAppendRequest::ordinary(slot_cell, payload))
        .map_err(page_record_denial)
}

fn apply_page_slot_storage_mutation(
    storage: &mut InMemoryPhysicalFormatModelStorage,
    slot_cell: SlotGenerationCell,
    page_payload: &[u8],
) {
    storage.replace_page_payload(slot_cell, page_payload);
}

fn construct_page_slot_append_outcome(
    reference: crate::PhysicalReference,
    counters: InMemoryPhysicalFormatModelCounterSnapshot,
) -> Result<PlatformPhysicalAppendOutcome, InMemoryPhysicalFormatModelDenial> {
    let updated_counters = counters.with_append().with_write();
    Ok(PlatformPhysicalAppendOutcome::new(
        PlatformPhysicalAppendReport::new(reference, updated_counters),
        updated_counters,
    ))
}

fn classify_extent_append_membership(
    extent_cell: crate::ExtentGenerationCell,
    payload_len: usize,
) -> ExtentMembership {
    let frame_length = PHYSICAL_HEADER_LENGTH as usize + payload_len;
    ExtentMembership::large_record(extent_cell, frame_length)
}

fn verify_extent_record_admission(
    extent_records: &PhysicalExtentRecordAuthority,
    membership: ExtentMembership,
    extent_cell: crate::ExtentGenerationCell,
    payload: &[u8],
) -> Result<crate::ExtentRecordAppendReport, InMemoryPhysicalFormatModelDenial> {
    extent_records
        .append_extent_record(
            membership,
            ExtentRecordAppendRequest::large_record(extent_cell, payload),
        )
        .map_err(extent_record_denial)
}

fn apply_extent_storage_mutation(
    storage: &mut InMemoryPhysicalFormatModelStorage,
    extent_cell: crate::ExtentGenerationCell,
    extent_bytes: &[u8],
) {
    storage.put_extent(extent_cell, extent_bytes);
}

fn construct_extent_append_outcome(
    reference: crate::PhysicalReference,
    counters: InMemoryPhysicalFormatModelCounterSnapshot,
) -> Result<PlatformPhysicalAppendOutcome, InMemoryPhysicalFormatModelDenial> {
    let updated_counters = counters.with_append().with_write();
    Ok(PlatformPhysicalAppendOutcome::new(
        PlatformPhysicalAppendReport::new(reference, updated_counters),
        updated_counters,
    ))
}

fn page_cell_for_slot(slot_cell: SlotGenerationCell) -> crate::PageGenerationCell {
    crate::PhysicalGenerationAuthority::for_canonical_physical_format()
        .page_cell(slot_cell.segment_id(), slot_cell.page_id())
        .with_page_generation(slot_cell.generation())
}

fn header_decode_denial(
    denial: crate::PhysicalHeaderDecodeDenial,
) -> InMemoryPhysicalFormatModelDenial {
    InMemoryPhysicalFormatModelDenial::new(
        InMemoryPhysicalFormatModelDenialKind::HeaderDecodeDenied,
    )
    .with_header_denial(denial)
}

fn page_record_denial(denial: crate::PageRecordDenial) -> InMemoryPhysicalFormatModelDenial {
    InMemoryPhysicalFormatModelDenial::new(InMemoryPhysicalFormatModelDenialKind::PageRecordDenied)
        .with_page_denial(denial)
}

fn extent_record_denial(denial: crate::ExtentRecordDenial) -> InMemoryPhysicalFormatModelDenial {
    InMemoryPhysicalFormatModelDenial::new(
        InMemoryPhysicalFormatModelDenialKind::ExtentRecordDenied,
    )
    .with_extent_denial(denial)
}
