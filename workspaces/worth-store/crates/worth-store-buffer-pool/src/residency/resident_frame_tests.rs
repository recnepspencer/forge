use crate::{
    BufferPoolBudget, DirtyPageBudget, PinnedPageBudget, ResidentFrameDenial,
    ResidentFrameDenialKind, ResidentFrameIdentity, ResidentFrameLoadRequest,
    ResidentFrameShortcutAttempt, ResidentFrameSlot, ResidentFrameTable,
    ResidentFrameTableCapacity, ResidentMemoryBudget, S2PhysicalResidencyEntry,
};
use worth_store_contracts::PhysicalSubstrateReadinessSnapshot;
use worth_store_physical_format::{
    PhysicalBinaryEncodingWitness, PhysicalFrameKind, PhysicalGeneration,
    PhysicalGenerationAuthority, PhysicalHeaderAuthority, PhysicalHeaderDecodeWitness,
    PhysicalPageId, PhysicalPageKind, PhysicalRecordSlot, PhysicalReferenceAuthority,
    PhysicalReferenceValidationWitness, PhysicalSegmentId, PHYSICAL_HEADER_LENGTH,
};

#[test]
fn resident_frame_table_is_authority_for_resident_identity_and_counters() {
    let mut table = resident_frame_table(8192, 2);
    let first_request = load_request(7, 2, payload_len_for_frame_size(4096));
    let second_request = load_request(7, 2, payload_len_for_frame_size(4096));

    let first = table.admit_frame(first_request).unwrap();
    let second = table.admit_frame(second_request).unwrap();

    assert_eq!(first.resident_frame_token(), second.resident_frame_token());
    assert_eq!(second.hit_miss_report().hit_count(), 1);
    assert_eq!(second.hit_miss_report().miss_count(), 1);
    assert_eq!(table.counters().resident_bytes().as_bytes(), 4096);
    assert_eq!(table.counters().frame_table_lookup_count(), 2);
}

#[test]
fn resident_generation_changes_on_frame_slot_reuse_and_stales_old_token() {
    let mut table = resident_frame_table(8192, 1);
    let first = table
        .admit_frame(load_request(7, 2, payload_len_for_frame_size(4096)))
        .unwrap();
    let old_token = first.resident_frame_token();

    let replacement = table
        .reuse_frame_slot(
            first.slot(),
            load_request(7, 3, payload_len_for_frame_size(4096)),
        )
        .unwrap();
    let denial = table.resident_frame(old_token).unwrap_err();

    assert_ne!(
        old_token.resident_generation(),
        replacement.resident_frame_token().resident_generation()
    );
    assert_eq!(
        denial.kind(),
        ResidentFrameDenialKind::StaleResidentGeneration
    );
    assert!(denial.is_stale_resident_generation());
}

#[test]
fn generation_separation_proof_is_produced_by_executed_table_reuse() {
    let mut table = resident_frame_table(8192, 1);
    let first = table
        .admit_frame(load_request(7, 2, payload_len_for_frame_size(4096)))
        .unwrap();

    let proof = table
        .reuse_frame_slot_with_generation_separation(
            first.slot(),
            load_request(8, 3, payload_len_for_frame_size(4096)),
        )
        .unwrap();

    assert_eq!(proof.previous_identity(), first.identity());
    assert_ne!(
        proof.previous_identity().generation(),
        proof.replacement_identity().generation()
    );
    assert_eq!(proof.stale_token(), first.resident_frame_token());
    assert_eq!(
        proof.stale_denial().kind(),
        ResidentFrameDenialKind::StaleResidentGeneration
    );
    assert_eq!(proof.counters().miss_count(), 2);
    assert_eq!(proof.counters().frame_table_lookup_count(), 3);
}

#[test]
fn resident_bytes_adjust_on_reuse_and_denied_reuse_keeps_authority_state() {
    let mut table = resident_frame_table(4096, 1);
    let first = table
        .admit_frame(load_request(5, 2, payload_len_for_frame_size(2048)))
        .unwrap();

    let replacement = table
        .reuse_frame_slot(
            first.slot(),
            load_request(6, 3, payload_len_for_frame_size(4096)),
        )
        .unwrap();
    let denied = table
        .reuse_frame_slot(
            replacement.slot(),
            load_request(7, 4, payload_len_for_frame_size(4097)),
        )
        .unwrap_err();

    assert_eq!(
        denied.kind(),
        ResidentFrameDenialKind::ResidentMemoryBudgetExceeded
    );
    assert_eq!(table.counters().resident_bytes().as_bytes(), 4096);
    assert_eq!(
        table
            .resident_frame(replacement.resident_frame_token())
            .unwrap()
            .frame_size_bytes(),
        4096
    );
}

#[test]
fn full_resident_frame_table_denies_without_resident_byte_drift() {
    let mut table = resident_frame_table(8192, 1);
    table
        .admit_frame(load_request(7, 2, payload_len_for_frame_size(4096)))
        .unwrap();

    let denied = table
        .admit_frame(load_request(8, 3, payload_len_for_frame_size(2048)))
        .unwrap_err();

    assert_eq!(
        denied.kind(),
        ResidentFrameDenialKind::ResidentFrameTableFull
    );
    assert_eq!(table.counters().resident_bytes().as_bytes(), 4096);
    assert_eq!(table.counters().miss_count(), 2);
    assert_eq!(table.counters().frame_table_lookup_count(), 2);
}

#[test]
fn empty_resident_frame_slot_denies_as_nonresident_authority() {
    let mut table = resident_frame_table(8192, 1);
    let empty_token = ResidentFrameIdentity::new(
        ResidentFrameSlot::from_index(0),
        crate::ResidentFrameGeneration::initial(),
    )
    .token();

    let denied = table.resident_frame(empty_token).unwrap_err();

    assert_eq!(
        denied.kind(),
        ResidentFrameDenialKind::ResidentFrameSlotNotResident
    );
    assert_eq!(table.counters().frame_table_lookup_count(), 1);
    assert_eq!(table.counters().resident_bytes().as_bytes(), 0);
}

#[test]
fn resident_frame_size_is_derived_from_admitted_header_witness() {
    let request = load_request(3, 2, 11);

    assert_eq!(
        request.frame_size().as_bytes(),
        PHYSICAL_HEADER_LENGTH as u64 + 11
    );
}

#[test]
fn resident_request_rejects_header_reference_mismatch() {
    let reference = validated_slot_reference(3, 2);
    let mismatched_header = frame_header_witness(4, 2, payload_len_for_frame_size(4096));

    let denial =
        ResidentFrameLoadRequest::from_physical_format_physical_frame(reference, mismatched_header)
            .unwrap_err();

    assert_eq!(denial.kind(), ResidentFrameDenialKind::HeaderOwnerMismatch);
}

#[test]
fn resident_request_rejects_non_frame_header() {
    let reference = validated_slot_reference(3, 2);
    let page_header = page_header_witness(3, 2);

    let denial =
        ResidentFrameLoadRequest::from_physical_format_physical_frame(reference, page_header)
            .unwrap_err();

    assert_eq!(
        denial.kind(),
        ResidentFrameDenialKind::PhysicalHeaderKindRejected
    );
}

#[test]
fn forbidden_residency_proofs_are_typed_denials_not_authority() {
    let expected = [
        ResidentFrameDenialKind::BackendPrivateResidueRejected,
        ResidentFrameDenialKind::OsPageCacheStateRejected,
        ResidentFrameDenialKind::SemanticObjectPresenceRejected,
        ResidentFrameDenialKind::PhysicalGenerationAsResidentProofRejected,
        ResidentFrameDenialKind::ResidentGenerationAsPhysicalProofRejected,
    ];

    for (attempt, denial_kind) in
        ResidentFrameShortcutAttempt::physical_substrate_forbidden_attempts()
            .iter()
            .zip(expected)
    {
        let denial = ResidentFrameDenial::from_shortcut_attempt(*attempt);

        assert_eq!(denial.kind(), denial_kind);
    }
}

#[test]
fn table_metadata_is_denied_before_an_unbounded_allocation() {
    let budget = BufferPoolBudget::declare(
        ResidentMemoryBudget::bytes(64).unwrap(),
        PinnedPageBudget::pages(1).unwrap(),
        DirtyPageBudget::pages(1).unwrap(),
    );
    let admitted =
        S2PhysicalResidencyEntry::from_physical_substrate_snapshot(algorithm_model_snapshot())
            .unwrap()
            .with_budget(budget)
            .admit()
            .unwrap();
    let denial = ResidentFrameTable::open(
        admitted,
        ResidentFrameTableCapacity::frames(u32::MAX).unwrap(),
    )
    .unwrap_err();
    assert_eq!(
        denial.kind(),
        ResidentFrameDenialKind::TableMetadataBudgetExceeded,
    );
}

fn resident_frame_table(resident_bytes: u64, frame_count: u32) -> ResidentFrameTable {
    let budget = BufferPoolBudget::declare(
        ResidentMemoryBudget::bytes(resident_bytes).unwrap(),
        PinnedPageBudget::pages(4).unwrap(),
        DirtyPageBudget::pages(2).unwrap(),
    );
    let admitted =
        S2PhysicalResidencyEntry::from_physical_substrate_snapshot(algorithm_model_snapshot())
            .unwrap()
            .with_budget(budget)
            .admit()
            .unwrap();
    ResidentFrameTable::open(
        admitted,
        ResidentFrameTableCapacity::frames(frame_count).unwrap(),
    )
    .unwrap()
}

fn algorithm_model_snapshot() -> PhysicalSubstrateReadinessSnapshot {
    PhysicalSubstrateReadinessSnapshot::from_exact_counts(true, 4, 2, 2, 3, 1, 9)
}

fn load_request(
    generation_value: u64,
    page_value: u64,
    payload_len: usize,
) -> ResidentFrameLoadRequest {
    ResidentFrameLoadRequest::from_physical_format_physical_frame(
        validated_slot_reference(generation_value, page_value),
        frame_header_witness(generation_value, page_value, payload_len),
    )
    .unwrap()
}

fn validated_slot_reference(
    generation_value: u64,
    page_value: u64,
) -> PhysicalReferenceValidationWitness {
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let cell = generations
        .slot_cell(segment(1), page(page_value), slot(3))
        .with_slot_generation(generation(generation_value));
    let admitted = references.admit_page_slot(cell);
    references.validate_page_slot(admitted, cell).unwrap()
}

fn frame_header_witness(
    generation_value: u64,
    page_value: u64,
    payload_len: usize,
) -> PhysicalHeaderDecodeWitness {
    header_authority()
        .decode_frame_header(
            validated_slot_reference(generation_value, page_value),
            &frame_bytes(generation_value, page_value, payload_len),
            PhysicalFrameKind::RecordFrame,
        )
        .unwrap()
        .witness()
}

fn page_header_witness(generation_value: u64, page_value: u64) -> PhysicalHeaderDecodeWitness {
    let cell = PhysicalGenerationAuthority::for_canonical_physical_format()
        .page_cell(segment(1), page(page_value))
        .with_page_generation(generation(generation_value));
    header_authority()
        .decode_page_header(
            cell,
            &page_bytes(generation_value, page_value, 4),
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

fn frame_bytes(generation_value: u64, page_value: u64, payload_len: usize) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(PHYSICAL_HEADER_LENGTH as usize + payload_len);
    let cell = PhysicalGenerationAuthority::for_canonical_physical_format()
        .slot_cell(segment(1), page(page_value), slot(3))
        .with_slot_generation(generation(generation_value));
    bytes.extend_from_slice(&header_authority().encode_record_frame_header(
        cell,
        payload_len.try_into().expect("bounded fixture payload"),
    ));
    bytes.resize(PHYSICAL_HEADER_LENGTH as usize + payload_len, 0xAB);
    bytes
}

fn page_bytes(generation_value: u64, page_value: u64, payload_len: usize) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(PHYSICAL_HEADER_LENGTH as usize + payload_len);
    let cell = PhysicalGenerationAuthority::for_canonical_physical_format()
        .page_cell(segment(1), page(page_value))
        .with_page_generation(generation(generation_value));
    bytes.extend_from_slice(&header_authority().encode_page_header(
        cell,
        PhysicalPageKind::DataPage,
        payload_len.try_into().expect("bounded fixture payload"),
    ));
    bytes.resize(PHYSICAL_HEADER_LENGTH as usize + payload_len, 0xAB);
    bytes
}

fn payload_len_for_frame_size(frame_size: u64) -> usize {
    (frame_size - PHYSICAL_HEADER_LENGTH as u64) as usize
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
