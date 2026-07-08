use forge_store_buffer_pool::{
    BufferPoolBudget, DirtyPageBudget, DirtyPublicationReceipt, PinnedPageBudget,
    ResidentFrameAdmission, ResidentFrameLoadRequest, ResidentFrameTable,
    ResidentFrameTableCapacity, ResidentMemoryBudget, S2PhysicalResidencyEntry,
};
use forge_store_contracts::{
    AcceptedHandoffReadiness, HandoffEvidenceDigestSet, StableDigest, ROADMAP_2_S1_SCOPE,
};
use forge_store_physical_backend::{
    PosixFileFsyncDirFsyncProfile, WalDurabilityBarrier, WalDurabilityBarrierSet,
};
use forge_store_physical_format::{
    PageGenerationCell, PhysicalBinaryEncodingWitness, PhysicalFrameKind, PhysicalGeneration,
    PhysicalGenerationAuthority, PhysicalHeaderAuthority, PhysicalHeaderDecodeWitness,
    PhysicalPageId, PhysicalPublicationState, PhysicalRecordSlot, PhysicalReferenceAuthority,
    PhysicalReferenceValidationWitness, PhysicalSegmentId, PHYSICAL_HEADER_LENGTH,
};
use forge_store_readiness::{
    close_s1_physical_substrate_readiness, prove_s2_physical_substrate_readiness,
};

use crate::{
    AcknowledgmentPrecondition, DurableAckReceipt, LogSequenceNumber, WalAppendReceipt,
    WalFrameDigest, WalLsnRange, WalSegmentGeneration, WalSegmentId,
};

use super::{
    DirtyPublicationEvidence, PageFlushRecoveryReceipt, PageLsn, PageRedoApplicationBasis,
    PageRedoDigestState, PageRedoEligibility, ReopenedPageRecoveryEvidence,
    RollbackImagePublicationDeclaration, StalePageRecoveryClassification,
    StalePageRecoveryClassificationKind, UnadmittedDirtyPagePublicationDenialKind,
    WalBeforeDataOrderingProof,
};

#[test]
fn store_local_no_undo_authority_publishes_after_wal_before_data_proof() {
    let page = page_generation(7, 2);
    let ack = durable_ack();
    let flush = publish_page(
        scheduled_dirty_publication(b"store-local-publication"),
        PageLsn::from_lsn(ack.ack_basis().lsn_range().start()),
        &ack,
    );

    assert_eq!(flush.page_generation(), page);
    assert_eq!(
        flush.page_lsn(),
        PageLsn::from_lsn(ack.ack_basis().lsn_range().start())
    );
    assert!(flush.counters().page_flush_receipt_count() > 0);
}

#[test]
fn store_local_rollback_image_posture_allows_protected_publication() {
    let page = page_generation(7, 2);
    let ack = durable_ack();
    let page_lsn = PageLsn::from_lsn(ack.ack_basis().lsn_range().start());
    let evidence = DirtyPublicationEvidence::from_s2_publication(
        scheduled_dirty_publication(b"rollback-protected-publication"),
        page_lsn,
    );
    let declaration = RollbackImagePublicationDeclaration::declare(
        evidence.dirty_identity(),
        evidence.page_generation(),
        evidence.page_lsn(),
        "declared-rollback-image",
    );
    let ordering =
        WalBeforeDataOrderingProof::<PosixFileFsyncDirFsyncProfile>::prove(evidence, &ack).unwrap();
    let flush =
        PageFlushRecoveryReceipt::publish_rollback_image_protected(ordering, declaration).unwrap();

    assert_eq!(flush.page_generation(), page);
    assert_eq!(flush.page_lsn(), page_lsn);
    assert!(flush.counters().page_flush_receipt_count() > 0);
}

#[test]
fn equivalent_store_publications_produce_same_page_lsn_frontier_after_restart() {
    let page = page_generation(7, 2);
    let ack = durable_ack();
    let page_lsn = PageLsn::from_lsn(ack.ack_basis().lsn_range().start());
    let first = publish_page(
        scheduled_dirty_publication(b"equivalent-first"),
        page_lsn,
        &ack,
    );
    let second = publish_page(
        scheduled_dirty_publication(b"equivalent-second"),
        page_lsn,
        &ack,
    );

    assert_eq!(first.page_lsn(), second.page_lsn());
    assert_eq!(first.redo_frontier(), second.redo_frontier());

    let first_classification = StalePageRecoveryClassification::classify_reopened_page(
        reopened_page(page, Some(page_lsn)),
        &first,
    )
    .unwrap();
    let second_classification = StalePageRecoveryClassification::classify_reopened_page(
        reopened_page(page, Some(page_lsn)),
        &second,
    )
    .unwrap();
    assert_eq!(first_classification.kind(), second_classification.kind());
}

#[test]
fn missing_page_lsn_and_generation_mismatch_deny_after_store_publication() {
    let page = page_generation(7, 2);
    let ack = durable_ack();
    let flush = publish_page(
        scheduled_dirty_publication(b"classification-denials"),
        PageLsn::from_lsn(ack.ack_basis().lsn_range().start()),
        &ack,
    );

    let missing =
        StalePageRecoveryClassification::classify_reopened_page(reopened_page(page, None), &flush)
            .unwrap_err();
    assert_eq!(
        missing.kind(),
        UnadmittedDirtyPagePublicationDenialKind::MissingPageLsn
    );
    assert!(missing.counters().missing_page_lsn_classification_count() > 0);

    let mismatch = StalePageRecoveryClassification::classify_reopened_page(
        reopened_page(page_generation(9, 2), Some(flush.page_lsn())),
        &flush,
    )
    .unwrap_err();
    assert_eq!(
        mismatch.kind(),
        UnadmittedDirtyPagePublicationDenialKind::MismatchedPageGeneration
    );
    assert!(mismatch.counters().generation_mismatch_denial_count() > 0);
}

#[test]
fn redo_application_cannot_downgrade_already_current_page_state() {
    let page = page_generation(7, 2);
    let ack = durable_ack();
    let flush = publish_page(
        scheduled_dirty_publication(b"current-page-redo-skip"),
        PageLsn::from_lsn(ack.ack_basis().lsn_range().start()),
        &ack,
    );
    let stale = StalePageRecoveryClassification::classify_reopened_page(
        reopened_page(page, Some(PageLsn::from_lsn(LogSequenceNumber::new(99)))),
        &flush,
    )
    .unwrap();
    assert_eq!(
        stale.kind(),
        StalePageRecoveryClassificationKind::RedoRequired
    );
    let redo = PageRedoEligibility::from_recovery_classification(stale);
    let basis = PageRedoApplicationBasis::new(page, flush.page_lsn(), "redo-op", "slot-3");
    let initial_stale = PageRedoDigestState::new(
        page,
        PageLsn::from_lsn(LogSequenceNumber::new(99)),
        "checkpoint-page",
    );
    let after_first = redo.apply_idempotent_redo(initial_stale, &basis).unwrap();
    let already_current = PageRedoDigestState::new(page, flush.page_lsn(), "already-current");

    let after = redo
        .apply_idempotent_redo(already_current.clone(), &basis)
        .unwrap();

    assert_eq!(after, already_current);
    assert_eq!(
        redo.apply_idempotent_redo(after_first.clone(), &basis)
            .unwrap(),
        after_first
    );
}

#[test]
fn redo_application_rejects_different_stale_page_lsn_under_same_generation() {
    let page = page_generation(7, 2);
    let ack = durable_ack();
    let flush = publish_page(
        scheduled_dirty_publication(b"wrong-stale-page-lsn"),
        PageLsn::from_lsn(ack.ack_basis().lsn_range().start()),
        &ack,
    );
    let stale = StalePageRecoveryClassification::classify_reopened_page(
        reopened_page(page, Some(PageLsn::from_lsn(LogSequenceNumber::new(98)))),
        &flush,
    )
    .unwrap();
    let redo = PageRedoEligibility::from_recovery_classification(stale);
    let basis = PageRedoApplicationBasis::new(page, flush.page_lsn(), "redo-op", "slot-3");
    let different_stale_page = PageRedoDigestState::new(
        page,
        PageLsn::from_lsn(LogSequenceNumber::new(99)),
        "different-stale",
    );

    let denial = redo
        .apply_idempotent_redo(different_stale_page, &basis)
        .unwrap_err();

    assert_eq!(
        denial.kind(),
        UnadmittedDirtyPagePublicationDenialKind::RedoCurrentPageLsnMismatch
    );
    assert!(
        denial
            .counters()
            .redo_current_page_lsn_mismatch_denial_count()
            > 0
    );
}

fn reopened_page(
    page: PageGenerationCell,
    page_lsn: Option<PageLsn>,
) -> ReopenedPageRecoveryEvidence {
    ReopenedPageRecoveryEvidence::from_reopened_page(page, page_lsn)
}

fn publish_page(
    receipt: DirtyPublicationReceipt,
    page_lsn: PageLsn,
    ack: &DurableAckReceipt<PosixFileFsyncDirFsyncProfile>,
) -> PageFlushRecoveryReceipt {
    let evidence = DirtyPublicationEvidence::from_s2_publication(receipt, page_lsn);
    let ordering =
        WalBeforeDataOrderingProof::<PosixFileFsyncDirFsyncProfile>::prove(evidence, ack).unwrap();
    PageFlushRecoveryReceipt::publish_admitted_redo_only(ordering)
}

fn durable_ack() -> DurableAckReceipt<PosixFileFsyncDirFsyncProfile> {
    DurableAckReceipt::acknowledge(
        AcknowledgmentPrecondition::from_append_receipt(completed_posix_receipt()).unwrap(),
    )
}

fn completed_posix_receipt() -> WalAppendReceipt<PosixFileFsyncDirFsyncProfile> {
    WalAppendReceipt::new(
        WalSegmentId::new(42).unwrap(),
        WalSegmentGeneration::new(7).unwrap(),
        WalLsnRange::new(LogSequenceNumber::new(100), LogSequenceNumber::new(101)).unwrap(),
        WalFrameDigest::new("page-lsn-frame-digest-posix").unwrap(),
        4096,
        4096,
        WalDurabilityBarrierSet::of(WalDurabilityBarrier::WalFileFsync)
            .insert(WalDurabilityBarrier::WalDirectoryFsync),
        None,
    )
}

fn scheduled_dirty_publication(payload: &[u8]) -> DirtyPublicationReceipt {
    let mut table = resident_frame_table();
    let admission = admit_payload_frame(&mut table, payload);
    let dirty = table.mark_dirty(admission.resident_frame_token()).unwrap();
    let plan = table.plan_dirty_publication(dirty.identity()).unwrap();
    table.record_dirty_write_scheduled(plan).unwrap()
}

fn resident_frame_table() -> ResidentFrameTable {
    let readiness = prove_s2_physical_substrate_readiness(
        close_s1_physical_substrate_readiness(accepted_s1_readiness()).unwrap(),
    )
    .unwrap();
    let budget = BufferPoolBudget::declare(
        ResidentMemoryBudget::bytes(8192).unwrap(),
        PinnedPageBudget::pages(4).unwrap(),
        DirtyPageBudget::pages(2).unwrap(),
    );
    let admitted = S2PhysicalResidencyEntry::from_physical_substrate_snapshot(
        readiness.physical_substrate_snapshot(),
    )
    .unwrap()
    .with_budget(budget)
    .admit()
    .unwrap();
    ResidentFrameTable::open(admitted, ResidentFrameTableCapacity::frames(2).unwrap())
}

fn admit_payload_frame(table: &mut ResidentFrameTable, payload: &[u8]) -> ResidentFrameAdmission {
    let frame = frame_bytes(7, payload);
    let request = load_request_from_frame(7, 2, &frame);
    let payload = header_authority()
        .payload_view(&frame, request.header())
        .unwrap();
    table.admit_resident_frame_bytes(request, payload).unwrap()
}

fn load_request_from_frame(
    generation_value: u64,
    page_value: u64,
    frame_bytes: &[u8],
) -> ResidentFrameLoadRequest {
    ResidentFrameLoadRequest::from_s1_physical_frame(
        validated_slot_reference(generation_value, page_value),
        frame_header_witness(generation_value, page_value, frame_bytes),
    )
    .unwrap()
}

fn validated_slot_reference(
    generation_value: u64,
    page_value: u64,
) -> PhysicalReferenceValidationWitness {
    let generations = PhysicalGenerationAuthority::s1();
    let references = PhysicalReferenceAuthority::s1();
    let cell = generations
        .slot_cell(segment(1), page(page_value), slot(3))
        .with_slot_generation(generation(generation_value));
    let admitted = references.admit_page_slot(cell);
    references.validate_page_slot(admitted, cell).unwrap()
}

fn frame_header_witness(
    generation_value: u64,
    page_value: u64,
    bytes: &[u8],
) -> PhysicalHeaderDecodeWitness {
    header_authority()
        .decode_frame_header(
            validated_slot_reference(generation_value, page_value),
            bytes,
            PhysicalFrameKind::RecordFrame,
        )
        .unwrap()
        .witness()
}

fn header_authority() -> PhysicalHeaderAuthority {
    PhysicalHeaderAuthority::s1(PhysicalBinaryEncodingWitness::s1_canonical().unwrap())
}

fn frame_bytes(generation_value: u64, payload: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(PHYSICAL_HEADER_LENGTH as usize + payload.len());
    bytes.push(PhysicalFrameKind::RecordFrame.tag());
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&PHYSICAL_HEADER_LENGTH.to_le_bytes());
    bytes.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&generation_value.to_le_bytes());
    bytes.push(PhysicalPublicationState::Published.code());
    bytes.extend_from_slice(&0u32.to_le_bytes());
    bytes.extend_from_slice(&0u64.to_le_bytes());
    bytes.extend_from_slice(payload);
    bytes
}

fn page_generation(generation_value: u64, page_value: u64) -> PageGenerationCell {
    PhysicalGenerationAuthority::s1()
        .page_cell(segment(1), page(page_value))
        .with_page_generation(generation(generation_value))
}

fn accepted_s1_readiness() -> AcceptedHandoffReadiness {
    AcceptedHandoffReadiness::from_s0_artifacts(
        ROADMAP_2_S1_SCOPE,
        HandoffEvidenceDigestSet::new(
            digest("backend"),
            digest("deferred"),
            digest("harness"),
            digest("terms"),
            digest("audit"),
            digest("complexity"),
            digest("provenance"),
        ),
    )
    .unwrap()
}

fn digest(name: &str) -> StableDigest {
    StableDigest::new(format!("sha256:{name}")).unwrap()
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
