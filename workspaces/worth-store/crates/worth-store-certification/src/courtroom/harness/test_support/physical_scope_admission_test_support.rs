use super::pre_decode_physical_admission_test_support::{
    checksum_declaration, checksum_scope, crc32c, with_entry_seed,
};
use worth_store_physical_format::{
    AllocationClassKind, CheckpointAdjacencyPosture, ExtentGenerationCell, ManifestMembershipProof,
    PageGenerationCell, PhysicalBinaryEncodingWitness, PhysicalCellReuseDomain, PhysicalExtentId,
    PhysicalFrameKind, PhysicalGeneration, PhysicalGenerationAuthority, PhysicalHeaderAuthority,
    PhysicalHeaderDecodeWitness, PhysicalManifestUniverseBuilder, PhysicalPageId, PhysicalPageKind,
    PhysicalRecordSlot, PhysicalReferenceAdmissionWitness, PhysicalReferenceAuthority,
    PhysicalReferenceScope, PhysicalReferenceValidationWitness, PhysicalRootManifest,
    PhysicalRootReference, PhysicalSegmentId, RootManifestIntegrityPosture, SlotGenerationCell,
};
use worth_store_physical_integrity::{
    ChecksumAlgorithmId, DeclaredPhysicalChecksum, IntegrityCheckedFrame, IntegrityCheckedPage,
    PhysicalIntegrityAdmissionRequest, PhysicalScopeAdmissionRequest,
};

pub(crate) fn with_checked_frame(
    payload: &[u8],
    validation: PhysicalReferenceValidationWitness,
    run: impl FnOnce(IntegrityCheckedFrame<'_>),
) {
    with_entry_seed(payload, |seed| {
        let declaration =
            checksum_declaration().admit_for_physical_integrity_entry(seed.entry_witness());
        let admission = seed.with_checksum_declaration(declaration).unwrap();
        let (kind, witness) = frame_fixture(payload, validation);
        let checked = admission
            .admit_frame(PhysicalIntegrityAdmissionRequest::frame(
                validation,
                witness,
                kind,
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
        let declaration =
            checksum_declaration().admit_for_physical_integrity_entry(seed.entry_witness());
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
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
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
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let cell = extent_cell(segment, extent, generation);
    references
        .validate_extent(references.admit_extent(cell), cell)
        .unwrap()
}

pub(crate) fn page_cell(segment: u64, page: u64, generation: u64) -> PageGenerationCell {
    PhysicalGenerationAuthority::for_canonical_physical_format()
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
    PhysicalManifestUniverseBuilder::for_canonical_physical_format(root_publication(root_reference))
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
    PhysicalManifestUniverseBuilder::for_canonical_physical_format(root_publication_generation(
        root_reference,
        root_generation,
    ))
    .segment(segment_cell(segment))
    .ordinary_page(slot_cell(segment, page, slot, generation))
    .publish()
}

pub(crate) fn root_with_extent(segment: u64, extent: u64, generation: u64) -> PhysicalRootManifest {
    PhysicalManifestUniverseBuilder::for_canonical_physical_format(root_publication(99))
        .segment(segment_cell(segment))
        .extent(extent_cell(segment, extent, generation))
        .publish()
}

pub(crate) fn root_admission(root: &PhysicalRootManifest) -> PhysicalReferenceAdmissionWitness {
    PhysicalReferenceAuthority::for_canonical_physical_format()
        .admit_root_publication(root.root_publication())
}

pub(crate) fn page_slot_admission(
    segment: u64,
    page: u64,
    slot: u64,
    generation: u64,
) -> PhysicalReferenceAdmissionWitness {
    PhysicalReferenceAuthority::for_canonical_physical_format()
        .admit_page_slot(slot_cell(segment, page, slot, generation))
}

pub(crate) fn extent_admission(
    segment: u64,
    extent: u64,
    generation: u64,
) -> PhysicalReferenceAdmissionWitness {
    PhysicalReferenceAuthority::for_canonical_physical_format()
        .admit_extent(extent_cell(segment, extent, generation))
}

pub(crate) fn free_space_slot_admission(
    segment: u64,
    page: u64,
    slot: u64,
    generation: u64,
) -> PhysicalReferenceAdmissionWitness {
    let cell = PhysicalGenerationAuthority::for_canonical_physical_format()
        .free_space_slot_cell(
            segment_id(segment),
            page_id(page),
            record_slot(slot),
            AllocationClassKind::OrdinaryRecordPage,
        )
        .unwrap()
        .with_free_space_generation(physical_generation(generation));
    PhysicalReferenceAuthority::for_canonical_physical_format().admit_free_space_reuse(cell)
}

fn frame_fixture(
    payload: &[u8],
    validation: PhysicalReferenceValidationWitness,
) -> (PhysicalFrameKind, PhysicalHeaderDecodeWitness) {
    let owner = validation.owner();
    let (kind, bytes) = match owner.domain() {
        PhysicalCellReuseDomain::SlotAllocation => {
            let cell = slot_cell(
                owner.segment_id().expect("slot owner segment").get(),
                owner.page_id().expect("slot owner page").get(),
                u64::from(owner.slot().expect("slot owner slot").get()),
                owner.generation().get(),
            );
            (
                PhysicalFrameKind::RecordFrame,
                crate::physical_fixture_encoding::record_frame_bytes(cell, payload),
            )
        }
        PhysicalCellReuseDomain::ExtentAllocation => {
            let cell = extent_cell(
                owner.segment_id().expect("extent owner segment").get(),
                owner.extent_id().expect("extent owner extent").get(),
                owner.generation().get(),
            );
            (
                PhysicalFrameKind::ExtentRecordFrame,
                crate::physical_fixture_encoding::extent_frame_bytes(cell, payload),
            )
        }
        _ => panic!("frame fixture requires slot or extent owner"),
    };
    let witness = header_authority()
        .decode_frame_header(validation, &bytes, kind)
        .unwrap()
        .witness();
    (kind, witness)
}

fn page_witness(payload: &[u8], cell: PageGenerationCell) -> PhysicalHeaderDecodeWitness {
    header_authority()
        .decode_page_header(
            cell,
            &crate::physical_fixture_encoding::data_page_bytes(cell, payload),
            PhysicalPageKind::DataPage,
        )
        .unwrap()
        .witness()
}

fn header_authority() -> PhysicalHeaderAuthority {
    PhysicalHeaderAuthority::for_canonical_physical_format(
        PhysicalBinaryEncodingWitness::physical_format_canonical().unwrap(),
    )
}

fn slot_cell(segment: u64, page: u64, slot: u64, generation: u64) -> SlotGenerationCell {
    PhysicalGenerationAuthority::for_canonical_physical_format()
        .slot_cell(segment_id(segment), page_id(page), record_slot(slot))
        .with_slot_generation(physical_generation(generation))
}

fn extent_cell(segment: u64, extent: u64, generation: u64) -> ExtentGenerationCell {
    PhysicalGenerationAuthority::for_canonical_physical_format()
        .extent_cell(segment_id(segment), extent_id(extent))
        .with_extent_generation(physical_generation(generation))
}

fn segment_cell(segment: u64) -> worth_store_physical_format::SegmentGenerationCell {
    PhysicalGenerationAuthority::for_canonical_physical_format()
        .segment_cell(segment_id(segment))
        .with_segment_generation(physical_generation(1))
}

fn root_publication(root_reference: u64) -> worth_store_physical_format::RootPublicationCell {
    root_publication_generation(root_reference, 1)
}

fn root_publication_generation(
    root_reference: u64,
    generation: u64,
) -> worth_store_physical_format::RootPublicationCell {
    PhysicalGenerationAuthority::for_canonical_physical_format()
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
