use crate::dirty_pages::dirty_state_test_support::{admit_payload_frame, resident_frame_table};
use crate::{
    AllocationAdmission, AllocationByteBudget, AllocationDenial, AllocationEnvelopeDeclaration,
    AllocationRequest, AllocationRequestKind, AllocationScope, DirtyPageCount,
    EvictionProtectionReason, FixedMetadataReservation, PhysicalSpeculativeWorkKind,
    PrefetchRequest, PrefetchWindow, ReadAheadRequest, SpeculativePhysicalWorkAdmission,
    SpeculativePhysicalWorkDenialKind, WriteBehindRequest,
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
fn same_read_ahead_hint_lowers_to_same_replay_identity_under_same_budget_state() {
    let mut first_table = resident_frame_table(8192, 4, 2);
    let mut second_table = resident_frame_table(8192, 4, 2);
    admit_payload_frame(&mut first_table, 7, 2, b"resident");
    admit_payload_frame(&mut second_table, 7, 2, b"resident");
    let request = ReadAheadRequest::new(PrefetchWindow::resident_frames(1).unwrap(), None);
    let mut first_allocation = allocation_admission(64);
    let mut second_allocation = allocation_admission(64);
    let mut first_admission = SpeculativePhysicalWorkAdmission::new();
    let mut second_admission = SpeculativePhysicalWorkAdmission::new();

    let first_plan = first_admission
        .lower_read_ahead(
            request,
            first_table.speculative_work_budget_snapshot(),
            &mut first_allocation,
        )
        .unwrap();
    let second_plan = second_admission
        .lower_read_ahead(
            request,
            second_table.speculative_work_budget_snapshot(),
            &mut second_allocation,
        )
        .unwrap();

    assert_eq!(first_plan.replay_identity(), second_plan.replay_identity());
    assert_eq!(
        first_plan.replay_identity().kind(),
        PhysicalSpeculativeWorkKind::ReadAhead
    );
    assert_eq!(first_plan.counters().read_ahead_admitted_count(), 1);
}

#[test]
fn same_write_behind_hint_lowers_to_same_replay_identity_under_same_dirty_state() {
    let mut first_table = resident_frame_table(8192, 4, 2);
    let mut second_table = resident_frame_table(8192, 4, 2);
    let first_dirty = admit_payload_frame(&mut first_table, 7, 2, b"dirty");
    let second_dirty = admit_payload_frame(&mut second_table, 7, 2, b"dirty");
    first_table
        .mark_dirty(first_dirty.resident_frame_token())
        .unwrap();
    second_table
        .mark_dirty(second_dirty.resident_frame_token())
        .unwrap();
    let request =
        WriteBehindRequest::dirty_pages(DirtyPageCount::from_observed_pages(1), None).unwrap();
    let mut first_allocation = allocation_admission(64);
    let mut second_allocation = allocation_admission(64);
    let mut first_admission = SpeculativePhysicalWorkAdmission::new();
    let mut second_admission = SpeculativePhysicalWorkAdmission::new();

    let first_plan = first_admission
        .lower_write_behind(
            request,
            first_table.speculative_work_budget_snapshot(),
            &mut first_allocation,
        )
        .unwrap();
    let second_plan = second_admission
        .lower_write_behind(
            request,
            second_table.speculative_work_budget_snapshot(),
            &mut second_allocation,
        )
        .unwrap();

    assert_eq!(first_plan.replay_identity(), second_plan.replay_identity());
    assert_eq!(
        first_plan.replay_identity().kind(),
        PhysicalSpeculativeWorkKind::WriteBehind
    );
    assert_eq!(first_plan.counters().write_behind_admitted_count(), 1);
}

#[test]
fn prefetch_denies_before_foreground_allocation_interference() {
    let table = resident_frame_table(8192, 4, 2);
    let request = PrefetchRequest::new(
        PrefetchWindow::resident_frames(1).unwrap(),
        Some(AllocationRequest::background_work_memory(AllocationScope::Foreground, 8).unwrap()),
    );
    let mut allocation = allocation_admission(64);
    let mut admission = SpeculativePhysicalWorkAdmission::new();

    let denial = admission
        .lower_prefetch(
            request,
            table.speculative_work_budget_snapshot(),
            &mut allocation,
        )
        .unwrap_err();

    assert_eq!(
        denial.kind(),
        SpeculativePhysicalWorkDenialKind::ForegroundAllocationInterference { requested_bytes: 8 }
    );
    assert_eq!(denial.counters().prefetch_denied_count(), 1);
    assert_eq!(allocation.remaining(AllocationScope::Foreground), 64);
}

#[test]
fn read_ahead_denies_allocation_envelope_excess_before_scheduling() {
    let table = resident_frame_table(8192, 4, 2);
    let request = ReadAheadRequest::new(
        PrefetchWindow::resident_frames(1).unwrap(),
        Some(AllocationRequest::background_work_memory(AllocationScope::Maintenance, 16).unwrap()),
    );
    let mut allocation = allocation_admission(8);
    let mut admission = SpeculativePhysicalWorkAdmission::new();

    let denial = admission
        .lower_read_ahead(
            request,
            table.speculative_work_budget_snapshot(),
            &mut allocation,
        )
        .unwrap_err();

    assert_eq!(
        denial.kind(),
        SpeculativePhysicalWorkDenialKind::AllocationDenied(AllocationDenial::EnvelopeExceeded {
            scope: AllocationScope::Maintenance,
            kind: AllocationRequestKind::BackgroundWorkMemory,
            requested_bytes: 16,
            remaining_bytes: 8
        })
    );
    assert_eq!(denial.counters().read_ahead_denied_count(), 1);
    assert_eq!(allocation.remaining(AllocationScope::Maintenance), 8);
}

#[test]
fn write_behind_admits_existing_dirty_work_even_when_dirty_budget_is_full() {
    let mut table = resident_frame_table(8192, 2, 1);
    let dirty = admit_payload_frame(&mut table, 7, 2, b"dirty");
    table.mark_dirty(dirty.resident_frame_token()).unwrap();
    let request =
        WriteBehindRequest::dirty_pages(DirtyPageCount::from_observed_pages(1), None).unwrap();
    let mut allocation = allocation_admission(64);
    let mut admission = SpeculativePhysicalWorkAdmission::new();

    let plan = admission
        .lower_write_behind(
            request,
            table.speculative_work_budget_snapshot(),
            &mut allocation,
        )
        .unwrap();
    let receipt = admission
        .record_write_behind_admitted(plan, &mut allocation)
        .unwrap();

    assert_eq!(receipt.counters().write_behind_admitted_count(), 1);
    assert_eq!(receipt.counters().dirty_pages_requested(), 1);
}

#[test]
fn write_behind_denies_synthetic_dirty_work_before_scheduling() {
    let table = resident_frame_table(8192, 2, 3);
    let request =
        WriteBehindRequest::dirty_pages(DirtyPageCount::from_observed_pages(1), None).unwrap();
    let mut allocation = allocation_admission(64);
    let mut admission = SpeculativePhysicalWorkAdmission::new();

    let denial = admission
        .lower_write_behind(
            request,
            table.speculative_work_budget_snapshot(),
            &mut allocation,
        )
        .unwrap_err();

    assert_eq!(
        denial.kind(),
        SpeculativePhysicalWorkDenialKind::DirtyWorkNotResident {
            requested_pages: 1,
            dirty_pages_used: 0
        }
    );
    assert_eq!(denial.counters().write_behind_denied_count(), 1);
}

#[test]
fn prefetch_denies_pin_budget_pressure_before_allocation() {
    let mut table = resident_frame_table(8192, 5, 2);
    let first = admit_payload_frame(&mut table, 7, 2, b"first");
    let second = admit_payload_frame(&mut table, 8, 3, b"second");
    let third = admit_payload_frame(&mut table, 9, 4, b"third");
    let fourth = admit_payload_frame(&mut table, 10, 5, b"fourth");
    let first_pin = table
        .lease_page(first.resident_frame_token())
        .unwrap()
        .pin()
        .unwrap();
    std::mem::forget(first_pin);
    let second_pin = table
        .lease_page(second.resident_frame_token())
        .unwrap()
        .pin()
        .unwrap();
    std::mem::forget(second_pin);
    let third_pin = table
        .lease_page(third.resident_frame_token())
        .unwrap()
        .pin()
        .unwrap();
    std::mem::forget(third_pin);
    let fourth_pin = table
        .lease_page(fourth.resident_frame_token())
        .unwrap()
        .pin()
        .unwrap();
    std::mem::forget(fourth_pin);
    let request = PrefetchRequest::new(
        PrefetchWindow::resident_frames(1).unwrap(),
        Some(AllocationRequest::background_work_memory(AllocationScope::Maintenance, 8).unwrap()),
    );
    let mut allocation = allocation_admission(64);
    let mut admission = SpeculativePhysicalWorkAdmission::new();

    let denial = admission
        .lower_prefetch(
            request,
            table.speculative_work_budget_snapshot(),
            &mut allocation,
        )
        .unwrap_err();

    assert_eq!(
        denial.kind(),
        SpeculativePhysicalWorkDenialKind::PinBudgetWouldBeExceeded {
            requested_pages: 1,
            pinned_pages_used: 4,
            pinned_page_budget: 4
        }
    );
    assert_eq!(denial.counters().prefetch_denied_count(), 1);
    assert_eq!(allocation.remaining(AllocationScope::Maintenance), 64);
}

#[test]
fn read_ahead_denies_before_protected_eviction_pressure() {
    let mut table = resident_frame_table(8192, 1, 1);
    let resident = admit_payload_frame(&mut table, 7, 2, b"streaming");
    table
        .protect_frame_for_streaming(resident.resident_frame_token())
        .unwrap();
    let mut allocation = allocation_admission(64);
    let mut admission = SpeculativePhysicalWorkAdmission::new();

    let denial = admission
        .lower_read_ahead(
            ReadAheadRequest::new(PrefetchWindow::resident_frames(1).unwrap(), None),
            table.speculative_work_budget_snapshot(),
            &mut allocation,
        )
        .unwrap_err();

    assert_eq!(
        denial.kind(),
        SpeculativePhysicalWorkDenialKind::ProtectedEvictionPressure {
            requested_frames: 1
        }
    );
    assert!(table
        .speculative_work_budget_snapshot()
        .protection_summary()
        .contains(EvictionProtectionReason::StreamingProtected));
}

#[test]
fn prefetch_and_write_behind_counters_are_exact() {
    let mut table = resident_frame_table(8192, 3, 3);
    let dirty = admit_payload_frame(&mut table, 7, 2, b"dirty");
    table.mark_dirty(dirty.resident_frame_token()).unwrap();
    let mut allocation = allocation_admission(64);
    let mut admission = SpeculativePhysicalWorkAdmission::new();
    let prefetch = admission
        .lower_prefetch(
            PrefetchRequest::new(PrefetchWindow::resident_frames(2).unwrap(), None),
            table.speculative_work_budget_snapshot(),
            &mut allocation,
        )
        .unwrap();
    let write_behind = admission
        .lower_write_behind(
            WriteBehindRequest::dirty_pages(DirtyPageCount::from_observed_pages(1), None).unwrap(),
            table.speculative_work_budget_snapshot(),
            &mut allocation,
        )
        .unwrap();

    let prefetch_receipt = admission
        .record_prefetch_admitted(prefetch, &mut allocation)
        .unwrap();
    let write_behind_receipt = admission
        .record_write_behind_admitted(write_behind, &mut allocation)
        .unwrap();

    assert_eq!(prefetch_receipt.counters().prefetch_admitted_count(), 1);
    assert_eq!(prefetch_receipt.counters().resident_frames_requested(), 2);
    assert_eq!(
        write_behind_receipt
            .counters()
            .write_behind_admitted_count(),
        1
    );
    assert_eq!(write_behind_receipt.counters().dirty_pages_requested(), 1);
}
