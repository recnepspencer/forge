use super::physical_scope_admission_test_support::{
    extent_validation, page_cell, page_request, root_with_extent, root_with_slot, scope_membership,
    validation, with_checked_frame, with_checked_page,
};
use super::pre_decode_physical_admission_test_support::{
    checksum_declaration, crc32c, frame_witness, with_entry_seed,
};
use forge_store_physical_format::{
    PageGenerationCell, PhysicalBinaryEncodingWitness, PhysicalFrameKind, PhysicalGeneration,
    PhysicalGenerationAuthority, PhysicalHeaderAuthority, PhysicalPageId, PhysicalPageKind,
    PhysicalPageRecordAuthority, PhysicalPublicationState, PhysicalRecordSlot,
    PhysicalReferenceScope, PhysicalSegmentId, SlotAppendRequest, SlotGenerationCell,
    PHYSICAL_HEADER_LENGTH,
};
use forge_store_physical_integrity::{
    DeclaredPhysicalChecksum, PhysicalContainerIntegrity, PhysicalIntegrityAdmissionRequest,
    PhysicalScopeAdmission, ScopedPhysicalValidatorInput,
};

#[derive(Clone, Copy)]
pub(crate) struct PageReportFixtureCell {
    segment: u64,
    page: u64,
    slot: u64,
    generation: u64,
}

impl PageReportFixtureCell {
    pub(crate) const fn new(segment: u64, page: u64, slot: u64, generation: u64) -> Self {
        Self {
            segment,
            page,
            slot,
            generation,
        }
    }

    const fn default_authority_scope() -> Self {
        Self::new(1, 2, 3, 7)
    }
}

pub(crate) fn inspect_page_report(
    page_payload: &[u8],
) -> forge_store_physical_integrity::PageIntegrityReport {
    inspect_page_report_for_cell(
        page_payload,
        PageReportFixtureCell::default_authority_scope(),
    )
}

pub(crate) fn inspect_page_report_for_cell(
    page_payload: &[u8],
    fixture_cell: PageReportFixtureCell,
) -> forge_store_physical_integrity::PageIntegrityReport {
    let mut report = None;
    with_scoped_page(page_payload, fixture_cell, |input| {
        report = Some(PhysicalContainerIntegrity::inspect_page(input).unwrap());
    });
    report.unwrap()
}

pub(crate) fn inspect_page_denial(
    page_payload: &[u8],
) -> forge_store_physical_integrity::PhysicalContainerIntegrityDenial {
    let mut denial = None;
    with_scoped_page(
        page_payload,
        PageReportFixtureCell::default_authority_scope(),
        |input| {
            denial = Some(PhysicalContainerIntegrity::inspect_page(input).unwrap_err());
        },
    );
    denial.unwrap()
}

pub(crate) fn inspect_extent_report(
    payload: &[u8],
) -> forge_store_physical_integrity::ExtentIntegrityReport {
    let mut report = None;
    let validation = extent_validation(1, 5, 7);
    with_checked_frame(payload, validation, |checked| {
        let scope = PhysicalReferenceScope::chunk_like(validation);
        let root = root_with_extent(1, 5, 7);
        let membership = scope_membership(&root, scope);
        let request = crate::courtroom::harness::test_support::physical_scope_admission_test_support::frame_request(
            &checked, scope, membership,
        );
        let admission = PhysicalScopeAdmission::admit_frame(checked, request).unwrap();
        let input = ScopedPhysicalValidatorInput::chunk_like(admission).unwrap();
        report = Some(PhysicalContainerIntegrity::inspect_extent(input).unwrap());
    });
    report.unwrap()
}

pub(crate) fn inspect_frame_with_witness_payload(
    protected_payload: &[u8],
    witness_payload: &[u8],
) -> forge_store_physical_integrity::PhysicalContainerIntegrityDenial {
    let mut denial = None;
    let validation = validation(1, 2, 3, 7);
    with_entry_seed(protected_payload, |seed| {
        let declaration = checksum_declaration().admit_for_s3_entry(seed.entry_witness());
        let admission = seed.with_checksum_declaration(declaration).unwrap();
        let checked = admission
            .admit_frame(PhysicalIntegrityAdmissionRequest::frame(
                validation,
                frame_witness(witness_payload),
                PhysicalFrameKind::RecordFrame,
                DeclaredPhysicalChecksum::new(crc32c(protected_payload)),
            ))
            .unwrap();
        let scope = PhysicalReferenceScope::frame(validation);
        let root = root_with_slot(1, 2, 3, 7);
        let membership = scope_membership(&root, scope);
        let request = crate::courtroom::harness::test_support::physical_scope_admission_test_support::frame_request(
            &checked, scope, membership,
        );
        let scoped = PhysicalScopeAdmission::admit_frame(checked, request).unwrap();
        let input = ScopedPhysicalValidatorInput::frame(scoped).unwrap();
        denial = Some(PhysicalContainerIntegrity::inspect_frame(input).unwrap_err());
    });
    denial.unwrap()
}

fn with_scoped_page(
    page_payload: &[u8],
    fixture_cell: PageReportFixtureCell,
    run: impl FnOnce(ScopedPhysicalValidatorInput<'_>),
) {
    let cell = page_cell(
        fixture_cell.segment,
        fixture_cell.page,
        fixture_cell.generation,
    );
    with_checked_page(page_payload, cell, |checked| {
        let scope = PhysicalReferenceScope::page(cell);
        let root = root_with_slot(
            fixture_cell.segment,
            fixture_cell.page,
            fixture_cell.slot,
            fixture_cell.generation,
        );
        let membership = scope_membership(&root, scope);
        let request = page_request(&checked, scope, membership);
        let admission = PhysicalScopeAdmission::admit_page(checked, request).unwrap();
        run(ScopedPhysicalValidatorInput::page(admission).unwrap());
    });
}

pub(crate) fn page_payload_with_record(payload: &[u8]) -> Vec<u8> {
    page_payload_with_record_for_cell(payload, PageReportFixtureCell::default_authority_scope())
}

pub(crate) fn page_payload_with_record_for_cell(
    payload: &[u8],
    fixture_cell: PageReportFixtureCell,
) -> Vec<u8> {
    let records = PhysicalPageRecordAuthority::s1(header_authority());
    let cell = page_cell(
        fixture_cell.segment,
        fixture_cell.page,
        fixture_cell.generation,
    );
    let empty_page = page_bytes(cell, &[]);
    let header = header_authority()
        .decode_page_header(cell, &empty_page, PhysicalPageKind::DataPage)
        .unwrap();
    let admitted = records
        .admit_record_page_payload(&empty_page, header.witness())
        .unwrap();
    records
        .append_record(
            admitted,
            SlotAppendRequest::ordinary(
                slot_cell(
                    fixture_cell.segment,
                    fixture_cell.page,
                    fixture_cell.slot,
                    fixture_cell.generation,
                ),
                payload,
            ),
        )
        .unwrap()
        .page_payload()
        .to_vec()
}

pub(crate) fn frame_start(page_payload: &[u8]) -> usize {
    let offset = occupied_slot_entry_offset();
    u32::from_le_bytes([
        page_payload[offset + 4],
        page_payload[offset + 5],
        page_payload[offset + 6],
        page_payload[offset + 7],
    ]) as usize
}

fn occupied_slot_entry_offset() -> usize {
    4 + ((3 - 1) * 24)
}

fn header_authority() -> PhysicalHeaderAuthority {
    PhysicalHeaderAuthority::s1(PhysicalBinaryEncodingWitness::s1_canonical().unwrap())
}

fn page_bytes(cell: PageGenerationCell, payload: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(PHYSICAL_HEADER_LENGTH as usize + payload.len());
    bytes.push(PhysicalPageKind::DataPage.tag());
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&PHYSICAL_HEADER_LENGTH.to_le_bytes());
    bytes.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&cell.generation().get().to_le_bytes());
    bytes.push(PhysicalPublicationState::Published.code());
    bytes.extend_from_slice(&0u32.to_le_bytes());
    bytes.extend_from_slice(&0u64.to_le_bytes());
    bytes.extend_from_slice(payload);
    bytes
}

fn slot_cell(segment: u64, page: u64, slot: u64, generation: u64) -> SlotGenerationCell {
    PhysicalGenerationAuthority::s1()
        .slot_cell(segment_id(segment), page_id(page), record_slot(slot))
        .with_slot_generation(physical_generation(generation))
}

fn segment_id(value: u64) -> PhysicalSegmentId {
    PhysicalSegmentId::from_raw(value).unwrap()
}

fn page_id(value: u64) -> PhysicalPageId {
    PhysicalPageId::from_raw(value).unwrap()
}

fn record_slot(value: u64) -> PhysicalRecordSlot {
    PhysicalRecordSlot::from_raw(value as u16).unwrap()
}

fn physical_generation(value: u64) -> PhysicalGeneration {
    PhysicalGeneration::from_raw(value).unwrap()
}
