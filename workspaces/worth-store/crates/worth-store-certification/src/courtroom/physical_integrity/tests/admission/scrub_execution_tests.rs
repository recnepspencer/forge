use std::num::NonZeroU64;

use crate::courtroom::harness::test_support::physical_scope_admission_test_support::{
    validation, with_store_checked_frame,
};
use worth_store::physical_runtime::{PhysicalOperationAllocationScope, ServingPhysicalRuntime};
use worth_store_maintenance::PhysicalIntegrityScrubWorkflow;
use worth_store_physical_integrity::{
    CompactionSourceIntegrityClearance, OfflineScrubInspectionInput, ScrubCounterSnapshot,
    ScrubExecutionOutcome, ScrubMode, ScrubPlan, ScrubPlanDenial, ScrubPlanDenialKind,
    ScrubPlanPolicy, ScrubPlanRequest, ScrubWindow, ScrubWindowOrdinal,
};
use worth_store_test_support::harness::physical_residency::PhysicalResidencyStoreWorld;

#[test]
fn online_and_offline_scrub_converge_on_overlapping_integrity_evidence() {
    with_store_checked_frame(
        b"same-window",
        validation(1, 2, 3, 7),
        |serving, checked| {
            let protected = checked.checked_bytes();
            let persisted_windows = persisted_scrub_fixture_windows(protected.as_bytes());
            let online = ScrubPlan::build(ScrubPlanRequest::online(
                serving
                    .physical_allocations()
                    .admit_scrub(nonzero(64))
                    .unwrap(),
                vec![ScrubWindow::online_from_protected_view(
                    ordinal(0),
                    protected,
                )],
                policy(64, 1),
            ))
            .unwrap();
            let online_receipt = completed(PhysicalIntegrityScrubWorkflow::run(online));

            let offline_input = OfflineScrubInspectionInput::from_declared_windows(vec![(
                ordinal(0),
                persisted_windows[0].as_slice(),
            )])
            .unwrap();
            assert!(!offline_input.proves_live_runtime_state());
            let offline = ScrubPlan::build(ScrubPlanRequest::offline(
                serving
                    .physical_allocations()
                    .admit_scrub(nonzero(64))
                    .unwrap(),
                offline_input,
                policy(64, 1),
            ))
            .unwrap();
            let offline_receipt = completed(PhysicalIntegrityScrubWorkflow::run(offline));

            assert_eq!(online_receipt.finding(), offline_receipt.finding());
            assert_eq!(online_receipt.locality(), offline_receipt.locality());
            assert_eq!(online_receipt.counters(), offline_receipt.counters());
            assert!(!online_receipt.proves_recovery_behavior());
            assert!(!online_receipt.proves_repair_behavior());
            assert!(!online_receipt.proves_blob_lifecycle());
            let clearance =
                CompactionSourceIntegrityClearance::from_scrub_execution(&online_receipt).unwrap();
            assert!(!clearance.permits_compaction_movement());
            assert_eq!(clearance.locality_owner(), None);
        },
    );
}

#[test]
fn scrub_plan_denies_before_inspection_when_exact_limits_are_exceeded() {
    with_scrub_runtime("scrub-limit-denials", |serving| {
        assert_denial(
            serving,
            4,
            policy(64, 1),
            vec![(ordinal(0), b"12345".as_slice())],
            ScrubPlanDenialKind::AllocationLimitExceeded {
                requested: 5,
                limit: 4,
            },
        );
        assert_denial(
            serving,
            64,
            policy(4, 1),
            vec![(ordinal(0), b"12345".as_slice())],
            ScrubPlanDenialKind::StreamingWindowLimitExceeded {
                requested: 5,
                limit: 4,
            },
        );

        let zero_yield_input = OfflineScrubInspectionInput::from_declared_windows(vec![(
            ordinal(0),
            b"12345".as_slice(),
        )])
        .unwrap();
        let zero_yield_denial = ScrubPlan::build(
            ScrubPlanRequest::offline(
                serving
                    .physical_allocations()
                    .admit_scrub(nonzero(64))
                    .unwrap(),
                zero_yield_input,
                policy(64, 1),
            )
            .with_yield_after_windows(0),
        )
        .unwrap_err();
        assert_eq!(
            zero_yield_denial.kind(),
            ScrubPlanDenialKind::ZeroYieldWindowBudget
        );
        assert_eq!(zero_yield_denial.counters(), ScrubCounterSnapshot::empty());
    });

    with_store_checked_frame(b"read-limit", validation(1, 2, 3, 7), |serving, checked| {
        let protected = checked.checked_bytes();
        let denial = ScrubPlan::build(ScrubPlanRequest::online(
            serving
                .physical_allocations()
                .admit_scrub(nonzero(64))
                .unwrap(),
            vec![
                ScrubWindow::online_from_protected_view(ordinal(0), protected),
                ScrubWindow::online_from_protected_view(ordinal(1), protected),
            ],
            policy(64, 1),
        ))
        .unwrap_err();

        assert_eq!(
            denial.kind(),
            ScrubPlanDenialKind::ProtectedReadLimitExceeded {
                requested: 2,
                limit: 1,
            }
        );
        assert_eq!(denial.counters(), ScrubCounterSnapshot::empty());
    });
}

#[test]
fn online_scrub_rejects_an_allocation_from_another_store() {
    let mut observed = None;
    with_store_checked_frame(
        b"same-store-required",
        validation(1, 2, 3, 7),
        |_serving, checked| {
            let other =
                PhysicalResidencyStoreWorld::initialize("foreign-scrub-allocation").unwrap();
            let request = ScrubPlanRequest::online(
                other
                    .serving()
                    .physical_allocations()
                    .admit_scrub(nonzero(64))
                    .unwrap(),
                vec![ScrubWindow::online_from_protected_view(
                    ordinal(0),
                    checked.checked_bytes(),
                )],
                policy(64, 1),
            );
            observed = Some(ScrubPlan::build(request).unwrap_err());
            assert!(!other.close().residency().requires_inspection());
        },
    );

    assert!(matches!(
        observed.unwrap().kind(),
        ScrubPlanDenialKind::OnlineWindowStoreMismatch { .. }
    ));
}

#[test]
fn paused_execution_retains_the_exact_scrub_allocation_until_resume_completes() {
    with_scrub_runtime("paused-scrub-allocation", |serving| {
        let input = OfflineScrubInspectionInput::from_declared_windows(vec![
            (ordinal(0), b"abcd".as_slice()),
            (ordinal(1), b"abcd".as_slice()),
            (ordinal(2), b"abcd".as_slice()),
            (ordinal(3), b"abcd".as_slice()),
        ])
        .unwrap();
        let plan = ScrubPlan::build(
            ScrubPlanRequest::offline(
                serving
                    .physical_allocations()
                    .admit_scrub(nonzero(8))
                    .unwrap(),
                input,
                policy(4, 1),
            )
            .with_deferred_over_budget_windows()
            .with_skipped_window(ordinal(1))
            .with_yield_after_windows(2),
        )
        .unwrap();

        let paused = match PhysicalIntegrityScrubWorkflow::run(plan) {
            ScrubExecutionOutcome::Yielded(paused) => paused,
            ScrubExecutionOutcome::Completed(_) => panic!("the declared yield must pause work"),
        };
        let interrupted = paused.progress();
        assert_eq!(interrupted.counters().completed_window_count(), 2);
        assert_eq!(interrupted.counters().interrupted_window_count(), 1);
        assert_eq!(
            serving
                .residency_observation()
                .counters()
                .active_operation_bytes_for(PhysicalOperationAllocationScope::Scrub),
            8,
        );

        let resumed = completed(PhysicalIntegrityScrubWorkflow::resume(paused));
        let counters = resumed.counters();
        assert_eq!(counters.planned_window_count(), 4);
        assert_eq!(counters.completed_window_count(), 2);
        assert_eq!(counters.checked_page_count(), 3);
        assert_eq!(counters.checked_byte_count(), 12);
        assert_eq!(counters.skipped_window_count(), 1);
        assert_eq!(counters.deferred_window_count(), 1);
        assert_eq!(counters.over_budget_window_count(), 1);
        assert_eq!(counters.interrupted_window_count(), 1);
        assert_eq!(counters.revalidated_window_count(), 1);
        assert_eq!(counters.skipped_decode_count(), 2);
        assert_eq!(counters.yielded_background_work_count(), 1);
        assert_eq!(resumed.mode(), ScrubMode::Offline);
        assert_eq!(
            serving
                .residency_observation()
                .counters()
                .active_operation_bytes_for(PhysicalOperationAllocationScope::Scrub),
            0,
        );
    });
}

fn assert_denial(
    serving: &ServingPhysicalRuntime,
    allocation_bytes: u64,
    policy: ScrubPlanPolicy,
    windows: Vec<(ScrubWindowOrdinal, &[u8])>,
    expected: ScrubPlanDenialKind,
) {
    let denial = scrub_plan_denial(serving, allocation_bytes, policy, windows);
    assert_eq!(denial.kind(), expected);
    assert_eq!(denial.counters(), ScrubCounterSnapshot::empty());
}

fn scrub_plan_denial(
    serving: &ServingPhysicalRuntime,
    allocation_bytes: u64,
    policy: ScrubPlanPolicy,
    windows: Vec<(ScrubWindowOrdinal, &[u8])>,
) -> ScrubPlanDenial {
    let input = OfflineScrubInspectionInput::from_declared_windows(windows).unwrap();
    ScrubPlan::build(ScrubPlanRequest::offline(
        serving
            .physical_allocations()
            .admit_scrub(nonzero(allocation_bytes))
            .unwrap(),
        input,
        policy,
    ))
    .unwrap_err()
}

fn completed(
    outcome: ScrubExecutionOutcome<'_, '_>,
) -> worth_store_physical_integrity::ScrubExecutionReceipt {
    match outcome {
        ScrubExecutionOutcome::Completed(receipt) => receipt,
        ScrubExecutionOutcome::Yielded(_) => panic!("the scrub plan should complete in this slice"),
    }
}

fn with_scrub_runtime(label: &str, run: impl FnOnce(&ServingPhysicalRuntime)) {
    let world = PhysicalResidencyStoreWorld::initialize(label).unwrap();
    run(world.serving());
    assert!(!world.close().residency().requires_inspection());
}

fn policy(streaming_window_bytes: u64, protected_reads: u64) -> ScrubPlanPolicy {
    ScrubPlanPolicy::bounded(nonzero(streaming_window_bytes), nonzero(protected_reads))
}

fn nonzero(value: u64) -> NonZeroU64 {
    NonZeroU64::new(value).unwrap()
}

fn ordinal(value: u64) -> ScrubWindowOrdinal {
    ScrubWindowOrdinal::from_zero_based(value)
}

fn persisted_scrub_fixture_windows(payload: &[u8]) -> Vec<Vec<u8>> {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("scrub-fixture.bin");
    std::fs::write(&path, payload).unwrap();
    let read_back = std::fs::read(&path).unwrap();
    vec![read_back]
}
