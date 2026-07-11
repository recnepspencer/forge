use crate::{
    courtroom::harness::test_support::dirty_publication_evidence_test_support::{
        admit_payload_frame, resident_frame_table,
    },
    DirtyPublicationEvidenceDenial, DirtyPublicationEvidenceReport, DirtyPublicationEvidenceRow,
};
use forge_store_buffer_pool::ResidentFrameDenialKind;

#[test]
fn dirty_state_evidence_consumes_real_dirty_admission() {
    let mut table = resident_frame_table(2, 2);
    let admission = admit_payload_frame(&mut table, 7, 2, b"dirty-evidence");

    let dirty = table.mark_dirty(admission.resident_frame_token()).unwrap();
    let evidence = DirtyPublicationEvidenceReport::from_dirty_state(dirty);

    assert_eq!(
        evidence.row(),
        DirtyPublicationEvidenceRow::DirtyStateAdmittedAndCounted
    );
    assert_eq!(evidence.counters().dirty_pages().as_pages(), 1);
    assert_eq!(evidence.counters().newly_dirty_count(), 1);
}

#[test]
fn dirty_budget_denial_evidence_requires_recorded_denial_counter() {
    let mut table = resident_frame_table(2, 1);
    let first = admit_payload_frame(&mut table, 7, 2, b"first");
    let second = admit_payload_frame(&mut table, 8, 3, b"second");

    table.mark_dirty(first.resident_frame_token()).unwrap();
    let denial = table.mark_dirty(second.resident_frame_token()).unwrap_err();
    let evidence = DirtyPublicationEvidenceReport::from_denial(
        DirtyPublicationEvidenceRow::DirtyBudgetDeniedBeforeScheduling,
        denial,
        table.dirty_counters(),
    )
    .unwrap();

    assert_eq!(
        denial.kind(),
        ResidentFrameDenialKind::DirtyPageBudgetExceeded
    );
    assert_eq!(
        evidence.row(),
        DirtyPublicationEvidenceRow::DirtyBudgetDeniedBeforeScheduling
    );
    assert_eq!(evidence.counters().dirty_budget_denial_count(), 1);
    assert_eq!(evidence.counters().write_scheduling_attempt_count(), 0);
}

#[test]
fn conflicting_lease_publication_evidence_requires_before_scheduling_denial() {
    let mut table = resident_frame_table(1, 1);
    let admission = admit_payload_frame(&mut table, 7, 2, b"protected");
    let dirty = table.mark_dirty(admission.resident_frame_token()).unwrap();
    let plan = table.plan_dirty_publication(dirty.identity()).unwrap();
    let pinned = table
        .lease_page(admission.resident_frame_token())
        .unwrap()
        .pin()
        .unwrap();
    std::mem::forget(pinned);

    let denial = table.record_dirty_write_scheduled(plan).unwrap_err();
    let evidence = DirtyPublicationEvidenceReport::from_denial(
        DirtyPublicationEvidenceRow::ConflictingLeasePublicationDeniedBeforeScheduling,
        denial,
        table.dirty_counters(),
    )
    .unwrap();

    assert_eq!(
        denial.kind(),
        ResidentFrameDenialKind::DirtyPublicationBehindActiveLease
    );
    assert_eq!(
        evidence.row(),
        DirtyPublicationEvidenceRow::ConflictingLeasePublicationDeniedBeforeScheduling
    );
    assert_eq!(evidence.counters().write_scheduling_attempt_count(), 0);
    assert_eq!(evidence.counters().write_scheduling_denial_count(), 1);
}

#[test]
fn stale_publication_plan_evidence_requires_recorded_schedule_denial() {
    let mut table = resident_frame_table(1, 1);
    let admission = admit_payload_frame(&mut table, 7, 2, b"stale-plan-evidence");
    let dirty = table.mark_dirty(admission.resident_frame_token()).unwrap();
    let first_plan = table.plan_dirty_publication(dirty.identity()).unwrap();
    let stale_plan = table.plan_dirty_publication(dirty.identity()).unwrap();

    table.record_dirty_write_scheduled(first_plan).unwrap();
    table.mark_dirty(admission.resident_frame_token()).unwrap();
    let denial = table.record_dirty_write_scheduled(stale_plan).unwrap_err();
    let evidence = DirtyPublicationEvidenceReport::from_denial(
        DirtyPublicationEvidenceRow::StalePublicationPlanDeniedBeforeScheduling,
        denial,
        table.dirty_counters(),
    )
    .unwrap();

    assert_eq!(
        denial.kind(),
        ResidentFrameDenialKind::DirtyPublicationPlanStale
    );
    assert_eq!(
        evidence.row(),
        DirtyPublicationEvidenceRow::StalePublicationPlanDeniedBeforeScheduling
    );
    assert_eq!(evidence.counters().stale_publication_plan_denial_count(), 1);
    assert_eq!(evidence.counters().write_scheduling_attempt_count(), 1);
    assert_eq!(evidence.counters().publication_receipt_count(), 1);
    assert_eq!(evidence.counters().dirty_pages().as_pages(), 1);
}

#[test]
fn publication_plan_and_receipt_evidence_stay_scheduling_only() {
    let mut table = resident_frame_table(1, 1);
    let admission = admit_payload_frame(&mut table, 7, 2, b"scheduled");
    let dirty = table.mark_dirty(admission.resident_frame_token()).unwrap();

    let plan = table.plan_dirty_publication(dirty.identity()).unwrap();
    let plan_evidence = DirtyPublicationEvidenceReport::from_publication_plan(&plan).unwrap();
    let receipt = table.record_dirty_write_scheduled(plan).unwrap();
    let receipt_evidence =
        DirtyPublicationEvidenceReport::from_publication_receipt(receipt).unwrap();

    assert_eq!(
        plan_evidence.row(),
        DirtyPublicationEvidenceRow::PublicationPlanIsSchedulingOnly
    );
    assert_eq!(
        receipt_evidence.row(),
        DirtyPublicationEvidenceRow::PublicationReceiptScheduledWriteOnly
    );
    assert_eq!(receipt_evidence.counters().publication_receipt_count(), 1);
    assert_eq!(receipt_evidence.counters().dirty_pages().as_pages(), 0);
    assert_eq!(
        receipt_evidence
            .counters()
            .scheduled_not_durable_pages()
            .as_pages(),
        1
    );
}

#[test]
fn dirty_shutdown_evidence_cannot_turn_unflushed_dirty_into_durability() {
    let mut table = resident_frame_table(1, 1);
    let admission = admit_payload_frame(&mut table, 7, 2, b"unflushed");
    table.mark_dirty(admission.resident_frame_token()).unwrap();

    let report = table.dirty_shutdown_closeout();
    let evidence = DirtyPublicationEvidenceReport::from_shutdown(report).unwrap();

    assert_eq!(
        evidence.row(),
        DirtyPublicationEvidenceRow::UnflushedDirtyShutdownObserved
    );
    assert_eq!(evidence.counters().dirty_shutdown_unflushed_count(), 1);
    assert_eq!(evidence.counters().dirty_pages().as_pages(), 1);
}

#[test]
fn scheduled_publication_shutdown_evidence_remains_unflushed_not_durable() {
    let mut table = resident_frame_table(1, 1);
    let admission = admit_payload_frame(&mut table, 7, 2, b"scheduled-shutdown");
    let dirty = table.mark_dirty(admission.resident_frame_token()).unwrap();
    let plan = table.plan_dirty_publication(dirty.identity()).unwrap();
    table.record_dirty_write_scheduled(plan).unwrap();

    let report = table.dirty_shutdown_closeout();
    let evidence = DirtyPublicationEvidenceReport::from_shutdown(report).unwrap();

    assert_eq!(
        evidence.row(),
        DirtyPublicationEvidenceRow::UnflushedDirtyShutdownObserved
    );
    assert_eq!(evidence.counters().dirty_shutdown_unflushed_count(), 1);
    assert_eq!(evidence.counters().dirty_pages().as_pages(), 0);
    assert_eq!(
        evidence.counters().scheduled_not_durable_pages().as_pages(),
        1
    );
}

#[test]
fn redirty_after_scheduled_publication_evidence_counts_one_unflushed_frame() {
    let mut table = resident_frame_table(1, 1);
    let admission = admit_payload_frame(&mut table, 7, 2, b"redirty-evidence");
    let dirty = table.mark_dirty(admission.resident_frame_token()).unwrap();
    let plan = table.plan_dirty_publication(dirty.identity()).unwrap();
    table.record_dirty_write_scheduled(plan).unwrap();

    table.mark_dirty(admission.resident_frame_token()).unwrap();
    let report = table.dirty_shutdown_closeout();
    let evidence = DirtyPublicationEvidenceReport::from_shutdown(report).unwrap();

    assert_eq!(
        evidence.row(),
        DirtyPublicationEvidenceRow::UnflushedDirtyShutdownObserved
    );
    assert_eq!(evidence.counters().dirty_pages().as_pages(), 1);
    assert_eq!(
        evidence.counters().scheduled_not_durable_pages().as_pages(),
        1
    );
    assert_eq!(
        evidence
            .counters()
            .scheduled_dirty_overlap_pages()
            .as_pages(),
        1
    );
    assert_eq!(evidence.counters().unflushed_dirty_pages().as_pages(), 1);
}

#[test]
fn clean_shutdown_evidence_rejects_unproven_denial_rows() {
    let mut table = resident_frame_table(1, 1);
    let report = table.dirty_shutdown_closeout();
    let evidence = DirtyPublicationEvidenceReport::from_shutdown(report).unwrap();
    let fake_denial = forge_store_buffer_pool::ResidentFrameDenial::from_shortcut_attempt(
        forge_store_buffer_pool::ResidentFrameShortcutAttempt::BackendPrivateResidue,
    );
    let denial = DirtyPublicationEvidenceReport::from_denial(
        DirtyPublicationEvidenceRow::DirtyBudgetDeniedBeforeScheduling,
        fake_denial,
        table.dirty_counters(),
    )
    .unwrap_err();

    assert_eq!(
        evidence.row(),
        DirtyPublicationEvidenceRow::CleanDirtyShutdownObserved
    );
    assert_eq!(denial, DirtyPublicationEvidenceDenial::DenialMismatch);
}
