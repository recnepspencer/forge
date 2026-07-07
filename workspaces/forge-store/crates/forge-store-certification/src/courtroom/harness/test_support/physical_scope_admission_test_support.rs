use super::pre_decode_physical_admission_test_support::{
    checksum_declaration, checksum_scope, crc32c, with_entry_seed,
};
use forge_store_physical_format::{
    AllocationClassKind, CheckpointAdjacencyPosture, ExtentGenerationCell, ManifestMembershipProof,
    PageGenerationCell, PhysicalBinaryEncodingWitness, PhysicalExtentId, PhysicalFrameKind,
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalHeaderAuthority,
    PhysicalHeaderDecodeWitness, PhysicalManifestUniverseBuilder, PhysicalPageId, PhysicalPageKind,
    PhysicalPublicationState, PhysicalRecordSlot, PhysicalReferenceAdmissionWitness,
    PhysicalReferenceAuthority, PhysicalReferenceScope, PhysicalReferenceValidationWitness,
    PhysicalRootManifest, PhysicalRootReference, PhysicalSegmentId, RootManifestIntegrityPosture,
    SlotGenerationCell, PHYSICAL_HEADER_LENGTH,
};
use forge_store_physical_integrity::{
    ChecksumAlgorithmId, DeclaredPhysicalChecksum, IntegrityCheckedFrame, IntegrityCheckedPage,
    PhysicalIntegrityAdmissionRequest, PhysicalScopeAdmissionRequest,
};

pub(crate) fn with_checked_frame(
    payload: &[u8],
    validation: PhysicalReferenceValidationWitness,
    run: impl FnOnce(IntegrityCheckedFrame<'_>),
) {
    with_entry_seed(payload, |seed| {
        let declaration = checksum_declaration().admit_for_s3_entry(seed.entry_witness());
        let admission = seed.with_checksum_declaration(declaration).unwrap();
        let checked = admission
            .admit_frame(PhysicalIntegrityAdmissionRequest::frame(
                validation,
                frame_witness(payload, validation),
                PhysicalFrameKind::RecordFrame,
                DeclaredPhysicalChecksum::new(crc32c(payload)),
            ))
            .unwrap();
        run(checked);
    });
}

pub(crate) fn with_checked_page(
    payload: &[u8],
    cell: PageGenerationCell,
    run: impl FnOnce(IntegrityCheckedPage<'_>),
) {
    with_entry_seed(payload, |seed| {
        let declaration = checksum_declaration().admit_for_s3_entry(seed.entry_witness());
        let admission = seed.with_checksum_declaration(declaration).unwrap();
        let checked = admission
            .admit_page(PhysicalIntegrityAdmissionRequest::page(
                cell,
                page_witness(payload, cell),
                PhysicalPageKind::DataPage,
                DeclaredPhysicalChecksum::new(crc32c(payload)),
            ))
            .unwrap();
        run(checked);
    });
}

pub(crate) fn frame_request(
    checked: &IntegrityCheckedFrame<'_>,
    scope: PhysicalReferenceScope,
    membership: ManifestMembershipProof,
) -> PhysicalScopeAdmissionRequest {
    PhysicalScopeAdmissionRequest::frame(
        scope,
        membership,
        RootManifestIntegrityPosture::current_root_admitted(membership),
        CheckpointAdjacencyPosture::NotApplicable,
        checked.gate_evidence().coverage_basis().clone(),
    )
}

pub(crate) fn page_request(
    checked: &IntegrityCheckedPage<'_>,
    scope: PhysicalReferenceScope,
    membership: ManifestMembershipProof,
) -> PhysicalScopeAdmissionRequest {
    PhysicalScopeAdmissionRequest::page(
        scope,
        membership,
        RootManifestIntegrityPosture::current_root_admitted(membership),
        checked.gate_evidence().coverage_basis().clone(),
    )
}

pub(crate) fn mismatched_checksum_request(
    scope: PhysicalReferenceScope,
    membership: ManifestMembershipProof,
) -> PhysicalScopeAdmissionRequest {
    let declaration = ChecksumAlgorithmId::crc64_nvme()
        .declare_for_scope(checksum_scope())
        .unwrap();
    PhysicalScopeAdmissionRequest::frame(
        scope,
        membership,
        RootManifestIntegrityPosture::current_root_admitted(membership),
        CheckpointAdjacencyPosture::NotApplicable,
        declaration.coverage_basis().clone(),
    )
}

pub(crate) fn scope_membership(
    root: &PhysicalRootManifest,
    scope: PhysicalReferenceScope,
) -> ManifestMembershipProof {
    ManifestMembershipProof::from_root(root, scope).unwrap()
}

pub(crate) fn validation(
    segment: u64,
    page: u64,
    slot: u64,
    generation: u64,
) -> PhysicalReferenceValidationWitness {
    let references = PhysicalReferenceAuthority::s1();
    let cell = slot_cell(segment, page, slot, generation);
    references
        .validate_page_slot(references.admit_page_slot(cell), cell)
        .unwrap()
}

pub(crate) fn extent_validation(
    segment: u64,
    extent: u64,
    generation: u64,
) -> PhysicalReferenceValidationWitness {
    let references = PhysicalReferenceAuthority::s1();
    let cell = extent_cell(segment, extent, generation);
    references
        .validate_extent(references.admit_extent(cell), cell)
        .unwrap()
}

pub(crate) fn page_cell(segment: u64, page: u64, generation: u64) -> PageGenerationCell {
    PhysicalGenerationAuthority::s1()
        .page_cell(segment_id(segment), page_id(page))
        .with_page_generation(physical_generation(generation))
}

pub(crate) fn root_with_slot(
    segment: u64,
    page: u64,
    slot: u64,
    generation: u64,
) -> PhysicalRootManifest {
    root_with_slot_under_root(99, segment, page, slot, generation)
}

pub(crate) fn root_with_slot_under_root(
    root_reference: u64,
    segment: u64,
    page: u64,
    slot: u64,
    generation: u64,
) -> PhysicalRootManifest {
    PhysicalManifestUniverseBuilder::s1(root_publication(root_reference))
        .segment(segment_cell(segment))
        .ordinary_page(slot_cell(segment, page, slot, generation))
        .publish()
}

pub(crate) fn root_with_slot_root_generation(
    root_reference: u64,
    root_generation: u64,
    segment: u64,
    page: u64,
    slot: u64,
    generation: u64,
) -> PhysicalRootManifest {
    PhysicalManifestUniverseBuilder::s1(root_publication_generation(
        root_reference,
        root_generation,
    ))
    .segment(segment_cell(segment))
    .ordinary_page(slot_cell(segment, page, slot, generation))
    .publish()
}

pub(crate) fn root_with_extent(segment: u64, extent: u64, generation: u64) -> PhysicalRootManifest {
    PhysicalManifestUniverseBuilder::s1(root_publication(99))
        .segment(segment_cell(segment))
        .extent(extent_cell(segment, extent, generation))
        .publish()
}

pub(crate) fn root_admission(root: &PhysicalRootManifest) -> PhysicalReferenceAdmissionWitness {
    PhysicalReferenceAuthority::s1().admit_root_publication(root.root_publication())
}

pub(crate) fn page_slot_admission(
    segment: u64,
    page: u64,
    slot: u64,
    generation: u64,
) -> PhysicalReferenceAdmissionWitness {
    PhysicalReferenceAuthority::s1().admit_page_slot(slot_cell(segment, page, slot, generation))
}

pub(crate) fn extent_admission(
    segment: u64,
    extent: u64,
    generation: u64,
) -> PhysicalReferenceAdmissionWitness {
    PhysicalReferenceAuthority::s1().admit_extent(extent_cell(segment, extent, generation))
}

pub(crate) fn free_space_slot_admission(
    segment: u64,
    page: u64,
    slot: u64,
    generation: u64,
) -> PhysicalReferenceAdmissionWitness {
    let cell = PhysicalGenerationAuthority::s1()
        .free_space_slot_cell(
            segment_id(segment),
            page_id(page),
            record_slot(slot),
            AllocationClassKind::OrdinaryRecordPage,
        )
        .unwrap()
        .with_free_space_generation(physical_generation(generation));
    PhysicalReferenceAuthority::s1().admit_free_space_reuse(cell)
}

fn frame_witness(
    payload: &[u8],
    validation: PhysicalReferenceValidationWitness,
) -> PhysicalHeaderDecodeWitness {
    header_authority()
        .decode_frame_header(
            validation,
            &frame_bytes(validation.owner().generation().get(), payload),
            PhysicalFrameKind::RecordFrame,
        )
        .unwrap()
        .witness()
}

fn page_witness(payload: &[u8], cell: PageGenerationCell) -> PhysicalHeaderDecodeWitness {
    header_authority()
        .decode_page_header(
            cell,
            &page_bytes(cell.generation().get(), payload),
            PhysicalPageKind::DataPage,
        )
        .unwrap()
        .witness()
}

fn header_authority() -> PhysicalHeaderAuthority {
    PhysicalHeaderAuthority::s1(PhysicalBinaryEncodingWitness::s1_canonical().unwrap())
}

fn frame_bytes(generation: u64, payload: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(PHYSICAL_HEADER_LENGTH as usize + payload.len());
    bytes.push(PhysicalFrameKind::RecordFrame.tag());
    write_header_tail(&mut bytes, generation, payload);
    bytes
}

fn page_bytes(generation: u64, payload: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(PHYSICAL_HEADER_LENGTH as usize + payload.len());
    bytes.push(PhysicalPageKind::DataPage.tag());
    write_header_tail(&mut bytes, generation, payload);
    bytes
}

fn write_header_tail(bytes: &mut Vec<u8>, generation: u64, payload: &[u8]) {
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&PHYSICAL_HEADER_LENGTH.to_le_bytes());
    bytes.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&generation.to_le_bytes());
    bytes.push(PhysicalPublicationState::Published.code());
    bytes.extend_from_slice(&0u32.to_le_bytes());
    bytes.extend_from_slice(&0u64.to_le_bytes());
    bytes.extend_from_slice(payload);
}

fn slot_cell(segment: u64, page: u64, slot: u64, generation: u64) -> SlotGenerationCell {
    PhysicalGenerationAuthority::s1()
        .slot_cell(segment_id(segment), page_id(page), record_slot(slot))
        .with_slot_generation(physical_generation(generation))
}

fn extent_cell(segment: u64, extent: u64, generation: u64) -> ExtentGenerationCell {
    PhysicalGenerationAuthority::s1()
        .extent_cell(segment_id(segment), extent_id(extent))
        .with_extent_generation(physical_generation(generation))
}

fn segment_cell(segment: u64) -> forge_store_physical_format::SegmentGenerationCell {
    PhysicalGenerationAuthority::s1()
        .segment_cell(segment_id(segment))
        .with_segment_generation(physical_generation(1))
}

fn root_publication(root_reference: u64) -> forge_store_physical_format::RootPublicationCell {
    root_publication_generation(root_reference, 1)
}

fn root_publication_generation(
    root_reference: u64,
    generation: u64,
) -> forge_store_physical_format::RootPublicationCell {
    PhysicalGenerationAuthority::s1()
        .root_publication_cell(PhysicalRootReference::from_raw(root_reference).unwrap())
        .with_root_publication_generation(physical_generation(generation))
}

fn segment_id(value: u64) -> PhysicalSegmentId {
    PhysicalSegmentId::from_raw(value).unwrap()
}

fn page_id(value: u64) -> PhysicalPageId {
    PhysicalPageId::from_raw(value).unwrap()
}

fn extent_id(value: u64) -> PhysicalExtentId {
    PhysicalExtentId::from_raw(value).unwrap()
}

fn record_slot(value: u64) -> PhysicalRecordSlot {
    PhysicalRecordSlot::from_raw(value as u16).unwrap()
}

fn physical_generation(value: u64) -> PhysicalGeneration {
    PhysicalGeneration::from_raw(value).unwrap()
}
