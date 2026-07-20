use crate::{ResidentFrameAuthorityEvidenceReport, ResidentFrameAuthorityEvidenceRow};
use worth_store_buffer_pool::{
    BufferPoolBudget, DirtyPageBudget, PinnedPageBudget, ResidentFrameDenial,
    ResidentFrameLoadRequest, ResidentFrameShortcutAttempt, ResidentFrameTable,
    ResidentFrameTableCapacity, ResidentMemoryBudget, S2PhysicalResidencyEntry,
};
use worth_store_contracts::PhysicalSubstrateReadinessSnapshot;
use worth_store_physical_format::{
    PhysicalBinaryEncodingWitness, PhysicalFrameKind, PhysicalGeneration,
    PhysicalGenerationAuthority, PhysicalHeaderAuthority, PhysicalPageId, PhysicalRecordSlot,
    PhysicalReferenceAuthority, PhysicalReferenceValidationWitness, PhysicalSegmentId,
    PHYSICAL_HEADER_LENGTH,
};

#[test]
fn resident_frame_authority_evidence_uses_executed_table_counters() {
    let mut table = resident_frame_table();
    let first = table
        .admit_frame(load_request(7, 2, payload_len_for_frame_size(4096)))
        .unwrap();
    let second = table
        .admit_frame(load_request(7, 2, payload_len_for_frame_size(4096)))
        .unwrap();

    for row in ResidentFrameAuthorityEvidenceRow::physical_substrate_table_rows() {
        let report = ResidentFrameAuthorityEvidenceReport::from_table(*row, &table).unwrap();

        assert_eq!(report.row(), *row);
        assert_eq!(report.counters().resident_bytes().as_bytes(), 4096);
        assert_eq!(report.counters().hit_count(), 1);
        assert_eq!(report.counters().miss_count(), 1);
    }
    assert_eq!(first.resident_frame_token(), second.resident_frame_token());
}

#[test]
fn resident_generation_separation_evidence_observes_slot_reuse() {
    let mut table = resident_frame_table();
    let first = table
        .admit_frame(load_request(7, 2, payload_len_for_frame_size(4096)))
        .unwrap();
    let proof = table
        .reuse_frame_slot_with_generation_separation(
            first.slot(),
            load_request(8, 3, payload_len_for_frame_size(4096)),
        )
        .unwrap();
    let report = ResidentFrameAuthorityEvidenceReport::from_generation_separation(proof);

    assert_ne!(
        proof.previous_identity().generation(),
        proof.replacement_identity().generation()
    );
    assert_eq!(proof.stale_token(), first.resident_frame_token());
    assert_eq!(
        report.row(),
        ResidentFrameAuthorityEvidenceRow::ResidentGenerationDomainSeparated
    );
    assert_eq!(report.counters().miss_count(), 2);
    assert_eq!(report.counters().frame_table_lookup_count(), 3);
}

#[test]
fn forbidden_residency_proofs_are_certified_as_denials() {
    for attempt in ResidentFrameShortcutAttempt::physical_substrate_forbidden_attempts() {
        let denial = ResidentFrameDenial::from_shortcut_attempt(*attempt);
        let report =
            ResidentFrameAuthorityEvidenceReport::from_forbidden_denial(*attempt, denial).unwrap();

        assert_eq!(
            report.row(),
            ResidentFrameAuthorityEvidenceRow::ForbiddenResidencyProofRejected(*attempt)
        );
    }
}

#[test]
fn forbidden_residency_evidence_rejects_mismatched_denial() {
    let denial = ResidentFrameDenial::from_shortcut_attempt(
        ResidentFrameShortcutAttempt::SemanticObjectPresence,
    );
    let rejected = ResidentFrameAuthorityEvidenceReport::from_forbidden_denial(
        ResidentFrameShortcutAttempt::BackendPrivateResidue,
        denial,
    )
    .unwrap_err();

    assert_eq!(
        rejected,
        crate::ResidentFrameAuthorityEvidenceDenial::ForbiddenDenialMismatch
    );
}

#[test]
fn forbidden_shortcut_row_cannot_be_reported_from_table_counters() {
    let table = resident_frame_table();
    let denial = ResidentFrameAuthorityEvidenceReport::from_table(
        ResidentFrameAuthorityEvidenceRow::ForbiddenResidencyProofRejected(
            ResidentFrameShortcutAttempt::BackendPrivateResidue,
        ),
        &table,
    )
    .unwrap_err();

    assert_eq!(
        denial,
        crate::ResidentFrameAuthorityEvidenceDenial::WrongEvidenceRow
    );
}

#[test]
fn generation_separation_row_requires_executed_table_proof() {
    let table = resident_frame_table();
    let denial = ResidentFrameAuthorityEvidenceReport::from_table(
        ResidentFrameAuthorityEvidenceRow::ResidentGenerationDomainSeparated,
        &table,
    )
    .unwrap_err();

    assert_eq!(
        denial,
        crate::ResidentFrameAuthorityEvidenceDenial::WrongEvidenceRow
    );
}

fn resident_frame_table() -> ResidentFrameTable {
    let budget = BufferPoolBudget::declare(
        ResidentMemoryBudget::bytes(8192).unwrap(),
        PinnedPageBudget::pages(4).unwrap(),
        DirtyPageBudget::pages(2).unwrap(),
    );
    let admitted =
        S2PhysicalResidencyEntry::from_physical_substrate_snapshot(algorithm_model_snapshot())
            .unwrap()
            .with_budget(budget)
            .admit()
            .unwrap();
    ResidentFrameTable::open(admitted, ResidentFrameTableCapacity::frames(2).unwrap())
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
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let cell = slot_generation_cell(generation_value, page_value);
    let admitted = references.admit_page_slot(cell);
    references.validate_page_slot(admitted, cell).unwrap()
}

fn frame_header_witness(
    generation_value: u64,
    page_value: u64,
    payload_len: usize,
) -> worth_store_physical_format::PhysicalHeaderDecodeWitness {
    header_authority()
        .decode_frame_header(
            validated_slot_reference(generation_value, page_value),
            &crate::physical_fixture_encoding::record_frame_bytes(
                slot_generation_cell(generation_value, page_value),
                &vec![0xAB; payload_len],
            ),
            PhysicalFrameKind::RecordFrame,
        )
        .unwrap()
        .witness()
}

fn header_authority() -> PhysicalHeaderAuthority {
    PhysicalHeaderAuthority::for_canonical_physical_format(
        PhysicalBinaryEncodingWitness::physical_format_canonical().unwrap(),
    )
}

fn slot_generation_cell(
    generation_value: u64,
    page_value: u64,
) -> worth_store_physical_format::SlotGenerationCell {
    PhysicalGenerationAuthority::for_canonical_physical_format()
        .slot_cell(segment(1), page(page_value), slot(3))
        .with_slot_generation(generation(generation_value))
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
