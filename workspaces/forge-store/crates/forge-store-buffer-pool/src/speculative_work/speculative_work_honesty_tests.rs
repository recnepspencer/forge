use crate::dirty_pages::dirty_state_test_support::{admit_payload_frame, resident_frame_table};
use crate::{
    AllocationAdmission, AllocationByteBudget, AllocationEnvelopeDeclaration, AllocationRequest,
    AllocationScope, DirtyPageCount, FixedMetadataReservation, PrefetchRequest, PrefetchWindow,
    ReadAheadRequest, SpeculativePhysicalWorkAdmission, SpeculativePhysicalWorkDenialKind,
    WriteBehindRequest,
};

fn allocation_admission(bytes: u64) -> AllocationAdmission {
    let budget = AllocationByteBudget::bytes(bytes).unwrap();
    let declaration = AllocationEnvelopeDeclaration::declare()
        .foreground(budget)
        .maintenance(budget)
        .recovery(budget)
        .scrub(budget)
        .import_export(budget)
        .streaming(budget)
        .fixed_metadata(FixedMetadataReservation::constant_bytes(8).unwrap())
        .seal()
        .unwrap();
    AllocationAdmission::from_declaration(declaration)
}

#[test]
fn speculative_lowering_does_not_inflate_resident_cache_counters() {
    let mut table = resident_frame_table(8192, 4, 2);
    let dirty = admit_payload_frame(&mut table, 7, 2, b"dirty");
    table.mark_dirty(dirty.resident_frame_token()).unwrap();
    let before = table.counters();
    let mut allocation = allocation_admission(64);
    let mut admission = SpeculativePhysicalWorkAdmission::new();

    let _read_ahead = admission
        .lower_read_ahead(
            ReadAheadRequest::new(PrefetchWindow::resident_frames(1).unwrap(), None),
            table.speculative_work_budget_snapshot(),
            &mut allocation,
        )
        .unwrap();
    let _prefetch = admission
        .lower_prefetch(
            PrefetchRequest::new(PrefetchWindow::resident_frames(1).unwrap(), None),
            table.speculative_work_budget_snapshot(),
            &mut allocation,
        )
        .unwrap();
    let _write_behind = admission
        .lower_write_behind(
            WriteBehindRequest::dirty_pages(DirtyPageCount::from_observed_pages(1), None).unwrap(),
            table.speculative_work_budget_snapshot(),
            &mut allocation,
        )
        .unwrap();

    let after = table.counters();
    assert_eq!(after.hit_count(), before.hit_count());
    assert_eq!(after.miss_count(), before.miss_count());
    assert_eq!(
        after.resident_bytes().as_bytes(),
        before.resident_bytes().as_bytes()
    );
}

#[test]
fn admitted_speculative_work_receipts_count_memory_and_do_not_claim_qos() {
    let table = resident_frame_table(8192, 2, 2);
    let request = ReadAheadRequest::new(
        PrefetchWindow::resident_frames(1).unwrap(),
        Some(AllocationRequest::background_work_memory(AllocationScope::Maintenance, 16).unwrap()),
    );
    let mut allocation = allocation_admission(64);
    let mut admission = SpeculativePhysicalWorkAdmission::new();

    let plan = admission
        .lower_read_ahead(
            request,
            table.speculative_work_budget_snapshot(),
            &mut allocation,
        )
        .unwrap();
    let receipt = admission
        .record_read_ahead_admitted(plan, &mut allocation)
        .unwrap();

    assert_eq!(receipt.counters().read_ahead_admitted_count(), 1);
    assert_eq!(receipt.counters().allocation_bytes_admitted(), 16);
    assert_eq!(receipt.allocation_receipt().unwrap().bytes(), 16);
    assert!(!receipt.proves_io_qos());
    assert!(!receipt.proves_queue_depth_correctness());
    assert!(!receipt.proves_backend_pacing());
    assert!(!receipt.proves_fsync_policy());
    assert!(!receipt.proves_fairness());
    assert!(!receipt.proves_throughput_improvement());
}

#[test]
fn unsupported_qos_claim_defers_without_scheduling_denial() {
    let mut admission = SpeculativePhysicalWorkAdmission::new();

    let denial = admission.reject_unsupported_qos_claim();

    assert_eq!(
        denial.kind(),
        SpeculativePhysicalWorkDenialKind::UnsupportedQosClaim
    );
    assert_eq!(denial.counters().deferred_count(), 1);
    assert_eq!(denial.counters().read_ahead_denied_count(), 0);
    assert_eq!(denial.counters().prefetch_denied_count(), 0);
    assert_eq!(denial.counters().write_behind_denied_count(), 0);
}
