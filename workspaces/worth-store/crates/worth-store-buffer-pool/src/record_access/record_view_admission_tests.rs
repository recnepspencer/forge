use crate::{
    dirty_pages::dirty_state_test_support::{admit_payload_frame, resident_frame_table},
    AllocationAdmission, AllocationByteBudget, AllocationEnvelopeDeclaration, AllocationRequest,
    AllocationRequestKind, AllocationScope, RecordViewDenialKind, RecordViewMaterializationProfile,
    ResidentFrameDenialKind,
};
use worth_store_physical_format::{
    FramedRecordView, PageGenerationCell, PhysicalBinaryEncodingWitness, PhysicalGeneration,
    PhysicalGenerationAuthority, PhysicalHeaderAuthority, PhysicalPageId,
    PhysicalPageRecordAuthority, PhysicalPublicationState, PhysicalRecordSlot,
    PhysicalReferenceAuthority, PhysicalSegmentId, SlotAppendRequest, SlotGenerationCell,
    PHYSICAL_HEADER_LENGTH,
};

#[test]
fn zero_copy_record_view_is_lease_scoped_physical_bytes() {
    let mut table = resident_frame_table(8192, 1, 1);
    let admission = admit_payload_frame(&mut table, 7, 2, b"record-view");
    let framed = framed_record(7, 2, b"record-view");

    let lease = table.lease_page(admission.resident_frame_token()).unwrap();
    let mut pinned = lease.pin().unwrap();
    let view = pinned
        .zero_copy_record_view(framed, RecordViewMaterializationProfile::PhysicalBytesOnly)
        .unwrap();

    assert_eq!(view.physical_record_bytes(), b"record-view");
    assert_eq!(view.admission().reference(), framed.placement().reference());
    assert_eq!(view.counters().zero_copy_admission_attempt_count(), 1);
    assert_eq!(view.counters().zero_copy_admission_count(), 1);
    assert!(!view.proves_semantic_domain_object());
}

#[test]
fn bounded_copy_requires_admitted_copy_receipt_and_counts_exact_bytes() {
    let mut table = resident_frame_table(8192, 1, 1);
    let admission = admit_payload_frame(&mut table, 7, 2, b"copy-me");
    let framed = framed_record(7, 2, b"copy-me");
    let mut allocation = allocation_admission(16);

    let lease = table.lease_page(admission.resident_frame_token()).unwrap();
    let mut pinned = lease.pin().unwrap();
    let zero_copy = pinned
        .zero_copy_record_view(framed, RecordViewMaterializationProfile::PhysicalBytesOnly)
        .unwrap();
    let request = AllocationRequest::copied_payload(AllocationScope::Foreground, 7).unwrap();
    let grant = allocation.admit(request).unwrap();
    let receipt = allocation.record_allocation(grant).unwrap();

    let copied = zero_copy.bounded_copy(receipt).unwrap();

    assert_eq!(copied.physical_record_bytes(), b"copy-me");
    assert_eq!(copied.counters().bounded_copy_attempt_count(), 1);
    assert_eq!(copied.counters().bounded_copy_count(), 1);
    assert_eq!(copied.counters().copied_bytes(), 7);
    assert_eq!(receipt.kind(), AllocationRequestKind::CopiedPayload);
    assert!(!copied.proves_semantic_domain_object());
}

#[test]
fn materialized_record_set_copy_still_counts_copied_bytes_exactly() {
    let mut table = resident_frame_table(8192, 1, 1);
    let admission = admit_payload_frame(&mut table, 7, 2, b"copy-me");
    let framed = framed_record(7, 2, b"copy-me");
    let mut allocation = allocation_admission(16);

    let lease = table.lease_page(admission.resident_frame_token()).unwrap();
    let mut pinned = lease.pin().unwrap();
    let zero_copy = pinned
        .zero_copy_record_view(framed, RecordViewMaterializationProfile::PhysicalBytesOnly)
        .unwrap();
    let request =
        AllocationRequest::materialized_record_set(AllocationScope::Foreground, 7).unwrap();
    let grant = allocation.admit(request).unwrap();
    let receipt = allocation.record_allocation(grant).unwrap();

    let copied = zero_copy.bounded_copy(receipt).unwrap();

    assert_eq!(copied.physical_record_bytes(), b"copy-me");
    assert_eq!(copied.counters().bounded_copy_count(), 1);
    assert_eq!(copied.counters().copied_bytes(), 7);
    assert_eq!(copied.counters().materialized_bytes(), 7);
}

#[test]
fn bounded_copy_rejects_wrong_receipt_size_before_copying() {
    let mut table = resident_frame_table(8192, 1, 1);
    let admission = admit_payload_frame(&mut table, 7, 2, b"copy-me");
    let framed = framed_record(7, 2, b"copy-me");
    let mut allocation = allocation_admission(16);

    let lease = table.lease_page(admission.resident_frame_token()).unwrap();
    let mut pinned = lease.pin().unwrap();
    let zero_copy = pinned
        .zero_copy_record_view(framed, RecordViewMaterializationProfile::PhysicalBytesOnly)
        .unwrap();
    let request = AllocationRequest::copied_payload(AllocationScope::Foreground, 6).unwrap();
    let grant = allocation.admit(request).unwrap();
    let receipt = allocation.record_allocation(grant).unwrap();

    let denial = zero_copy.bounded_copy(receipt).unwrap_err();

    assert_eq!(
        denial.kind(),
        RecordViewDenialKind::AllocationReceiptByteMismatch
    );
    assert_eq!(denial.counters().bounded_copy_attempt_count(), 1);
    assert_eq!(denial.counters().bounded_copy_count(), 0);
    assert_eq!(denial.counters().copied_bytes(), 0);
}

#[test]
fn mutable_record_view_denies_without_exclusive_lease_authority() {
    let mut table = resident_frame_table(8192, 1, 1);
    let admission = admit_payload_frame(&mut table, 7, 2, b"mutable-denied");
    let framed = framed_record(7, 2, b"mutable-denied");

    let lease = table.lease_page(admission.resident_frame_token()).unwrap();
    let mut pinned = lease.pin().unwrap();
    let denial = pinned
        .mutable_zero_copy_record_view(framed, RecordViewMaterializationProfile::PhysicalBytesOnly)
        .unwrap_err();

    assert_eq!(
        denial.kind(),
        RecordViewDenialKind::MutableViewRequiresExclusiveLease
    );
    assert_eq!(denial.counters().zero_copy_admission_attempt_count(), 1);
    assert_eq!(denial.counters().zero_copy_admission_count(), 0);
    assert_eq!(denial.counters().denied_before_view_construction_count(), 1);
}

#[test]
fn invalid_reference_and_profile_deny_before_view_construction() {
    let mut table = resident_frame_table(8192, 1, 1);
    let admission = admit_payload_frame(&mut table, 7, 2, b"record-view");
    let wrong_reference = framed_record(7, 3, b"record-view");

    let lease = table.lease_page(admission.resident_frame_token()).unwrap();
    let mut pinned = lease.pin().unwrap();
    let denial = pinned
        .zero_copy_record_view(
            wrong_reference,
            RecordViewMaterializationProfile::PhysicalBytesOnly,
        )
        .unwrap_err();

    assert_eq!(
        denial.kind(),
        RecordViewDenialKind::PhysicalReferenceMismatch
    );
    assert_eq!(denial.counters().denied_before_view_construction_count(), 1);

    let semantic_denial = pinned
        .zero_copy_record_view(
            framed_record(7, 2, b"record-view"),
            RecordViewMaterializationProfile::RichSemanticMaterialization,
        )
        .unwrap_err();

    assert_eq!(
        semantic_denial.kind(),
        RecordViewDenialKind::ProfileForbidsMaterialization
    );
    assert_eq!(
        semantic_denial
            .counters()
            .denied_before_view_construction_count(),
        2
    );
}

#[test]
fn publication_scheduling_records_record_view_conflict_denial() {
    let mut table = resident_frame_table(8192, 1, 1);
    let admission = admit_payload_frame(&mut table, 7, 2, b"publish-protected");
    let dirty = table.mark_dirty(admission.resident_frame_token()).unwrap();
    let plan = table.plan_dirty_publication(dirty.identity()).unwrap();
    let pinned = table
        .lease_page(admission.resident_frame_token())
        .unwrap()
        .pin()
        .unwrap();
    std::mem::forget(pinned);

    let denial = table.record_dirty_write_scheduled(plan).unwrap_err();

    assert_eq!(
        denial.kind(),
        ResidentFrameDenialKind::DirtyPublicationBehindActiveLease
    );
    assert_eq!(
        table
            .record_view_counters()
            .publication_conflict_denial_count(),
        1
    );
    assert_eq!(
        table
            .dirty_counters()
            .scheduled_not_durable_pages()
            .as_pages(),
        0
    );
}

#[test]
fn dirty_mutation_denies_before_state_change_while_frame_is_protected() {
    let mut table = resident_frame_table(8192, 1, 1);
    let admission = admit_payload_frame(&mut table, 7, 2, b"protected-view");
    let lease = table.lease_page(admission.resident_frame_token()).unwrap();
    let pinned = lease.pin().unwrap();
    std::mem::forget(pinned);

    let denial = table
        .mark_dirty(admission.resident_frame_token())
        .unwrap_err();

    assert_eq!(
        denial.kind(),
        ResidentFrameDenialKind::DirtyMutationBehindLiveRecordView
    );
    assert_eq!(
        table
            .record_view_counters()
            .dirty_mutation_conflict_denial_count(),
        1
    );
    assert_eq!(table.dirty_counters().dirty_pages().as_pages(), 0);
}

fn allocation_admission(bytes: u64) -> AllocationAdmission {
    let envelopes = AllocationEnvelopeDeclaration::declare()
        .foreground(AllocationByteBudget::bytes(bytes).unwrap())
        .maintenance(AllocationByteBudget::bytes(bytes).unwrap())
        .recovery(AllocationByteBudget::bytes(bytes).unwrap())
        .scrub(AllocationByteBudget::bytes(bytes).unwrap())
        .import_export(AllocationByteBudget::bytes(bytes).unwrap())
        .streaming(AllocationByteBudget::bytes(bytes).unwrap())
        .fixed_metadata(crate::FixedMetadataReservation::constant_bytes(1).unwrap())
        .seal()
        .unwrap();
    AllocationAdmission::from_declaration(envelopes)
}

fn framed_record(
    generation_value: u64,
    page_value: u64,
    payload: &[u8],
) -> FramedRecordView<'static> {
    let records = record_authority();
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let page_cell = page_cell(&generations, 5, page_value);
    let slot_cell = slot_cell(&generations, generation_value, page_value);
    let empty_page = page_bytes(generation(5), &[]);
    let append = records
        .append_record(
            admitted_page(&records, page_cell, &empty_page),
            SlotAppendRequest::ordinary(slot_cell, payload),
        )
        .unwrap();
    let reopened_page = page_bytes(generation(5), append.page_payload());
    let reopened_page = Box::leak(reopened_page.into_boxed_slice());
    let validation = references
        .validate_page_slot(append.reference_admission(), slot_cell)
        .unwrap();
    records
        .locate_record(
            admitted_page(&records, page_cell, reopened_page),
            validation,
        )
        .unwrap()
        .record_view()
}

fn admitted_page<'a>(
    records: &PhysicalPageRecordAuthority,
    cell: PageGenerationCell,
    bytes: &'a [u8],
) -> worth_store_physical_format::RecordPagePayload<'a> {
    let header = records
        .decode_record_page_header(
            cell,
            bytes,
            worth_store_physical_format::PhysicalPageKind::DataPage,
        )
        .unwrap();
    records
        .admit_record_page_payload(bytes, header.witness())
        .unwrap()
}

fn record_authority() -> PhysicalPageRecordAuthority {
    PhysicalPageRecordAuthority::for_canonical_physical_format(
        PhysicalHeaderAuthority::for_canonical_physical_format(
            PhysicalBinaryEncodingWitness::physical_format_canonical().unwrap(),
        ),
    )
}

fn page_cell(
    generations: &PhysicalGenerationAuthority,
    page_generation: u64,
    page_value: u64,
) -> PageGenerationCell {
    generations
        .page_cell(segment(1), page(page_value))
        .with_page_generation(generation(page_generation))
}

fn slot_cell(
    generations: &PhysicalGenerationAuthority,
    slot_generation: u64,
    page_value: u64,
) -> SlotGenerationCell {
    generations
        .slot_cell(segment(1), page(page_value), slot(3))
        .with_slot_generation(generation(slot_generation))
}

fn page_bytes(generation: PhysicalGeneration, payload: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(PHYSICAL_HEADER_LENGTH as usize + payload.len());
    bytes.push(worth_store_physical_format::PhysicalPageKind::DataPage.tag());
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&PHYSICAL_HEADER_LENGTH.to_le_bytes());
    bytes.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&generation.get().to_le_bytes());
    bytes.push(PhysicalPublicationState::Published.code());
    bytes.extend_from_slice(&0u32.to_le_bytes());
    bytes.extend_from_slice(&0u64.to_le_bytes());
    bytes.extend_from_slice(payload);
    bytes
}

fn segment(value: u64) -> PhysicalSegmentId {
    PhysicalSegmentId::from_raw(value).unwrap()
}

fn page(value: u64) -> PhysicalPageId {
    PhysicalPageId::from_raw(value).unwrap()
}

fn slot(value: u16) -> PhysicalRecordSlot {
    PhysicalRecordSlot::from_raw(value).unwrap()
}

fn generation(value: u64) -> PhysicalGeneration {
    PhysicalGeneration::from_raw(value).unwrap()
}
