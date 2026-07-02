use crate::{
    physical_scope_admission_test_support::{validation, with_checked_frame},
    pre_decode_physical_admission_test_support::with_entry_seed,
};
use forge_store_buffer_pool::{
    AllocationAdmission, AllocationByteBudget, AllocationEnvelopeDeclaration,
    BackgroundEnvelopeAdmission, BackgroundEnvelopeRequest, BackgroundWorkBudgetSnapshot,
    FixedMetadataReservation,
};
use forge_store_maintenance::PhysicalIntegrityScrubWorkflow;
use forge_store_physical_integrity::{
    ChunkIntegrityStreamingWindow, CompactionSourceIntegrityClearance, OfflineScrubInspectionInput,
    ScrubCounterSnapshot, ScrubExecution, ScrubExecutionDenialKind, ScrubMode, ScrubPlan,
    ScrubPlanBudget, ScrubPlanDenialKind, ScrubPlanRequest, ScrubPlanningMemoryEnvelope,
    ScrubWindow, ScrubWindowOrdinal,
};

#[test]
fn online_and_offline_scrub_converge_on_overlapping_integrity_evidence() {
    with_scrub_budget(|budget| {
        with_checked_frame(b"same-window", validation(1, 2, 3, 7), |checked| {
            let protected = checked.checked_bytes();
            let persisted_windows = persisted_scrub_fixture_windows(b"same-window");
            let online = ScrubPlan::build(ScrubPlanRequest::online(
                vec![ScrubWindow::online_from_protected_view(
                    ordinal(0),
                    protected,
                )],
                budget,
            ))
            .unwrap();
            let offline_input = OfflineScrubInspectionInput::from_declared_windows(vec![(
                ordinal(0),
                persisted_windows[0].as_slice(),
            )])
            .unwrap();
            assert!(!offline_input.proves_live_runtime_state());
            let offline =
                ScrubPlan::build(ScrubPlanRequest::offline(offline_input, budget)).unwrap();

            let online_receipt = ScrubExecution::run(online).unwrap();
            let offline_receipt = ScrubExecution::run(offline).unwrap();

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
        });
    });
}

#[test]
fn scrub_plan_denies_before_inspection_when_declared_limits_are_exceeded() {
    with_scrub_budget(|budget| {
        assert_denial(
            budget.constrained_by_policy(4, 1, 64, 64, 8),
            vec![(ordinal(0), b"12345".as_slice())],
            ScrubPlanDenialKind::ResidentMemoryLimitExceeded {
                requested: 5,
                limit: 4,
            },
        );
        assert_denial(
            budget.constrained_by_policy(64, 1, 4, 64, 8),
            vec![(ordinal(0), b"12345".as_slice())],
            ScrubPlanDenialKind::AllocationLimitExceeded {
                requested: 5,
                limit: 4,
            },
        );
        assert_denial(
            budget.constrained_by_policy(64, 1, 64, 4, 8),
            vec![(ordinal(0), b"12345".as_slice())],
            ScrubPlanDenialKind::StreamingWindowLimitExceeded {
                requested: 5,
                limit: 4,
            },
        );
        assert_denial(
            budget.constrained_by_policy(64, 0, 64, 64, 8),
            vec![(ordinal(0), b"12345".as_slice())],
            ScrubPlanDenialKind::PinPageLimitExceeded {
                requested: 1,
                limit: 0,
            },
        );

        let zero_yield_input = OfflineScrubInspectionInput::from_declared_windows(vec![(
            ordinal(0),
            b"12345".as_slice(),
        )])
        .unwrap();
        let zero_yield_denial = ScrubPlan::build(
            ScrubPlanRequest::offline(zero_yield_input, budget).with_yield_after_windows(0),
        )
        .unwrap_err();
        assert_eq!(
            zero_yield_denial.kind(),
            ScrubPlanDenialKind::ZeroYieldWindowBudget
        );
        assert_eq!(zero_yield_denial.counters(), ScrubCounterSnapshot::empty());
    });

    with_scrub_budget(|budget| {
        with_checked_frame(b"read-limit", validation(1, 2, 3, 7), |checked| {
            let protected = checked.checked_bytes();
            let denial = ScrubPlan::build(ScrubPlanRequest::online(
                vec![
                    ScrubWindow::online_from_protected_view(ordinal(0), protected),
                    ScrubWindow::online_from_protected_view(ordinal(1), protected),
                ],
                budget.constrained_by_policy(64, 1, 64, 64, 1),
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
    });
}

#[test]
fn resume_token_cannot_cross_order_distinct_scrub_plans() {
    with_scrub_budget(|budget| {
        let first_plan = ScrubPlan::build(
            ScrubPlanRequest::offline(
                OfflineScrubInspectionInput::from_declared_windows(vec![
                    (ordinal(0), b"ab".as_slice()),
                    (ordinal(1), b"cd".as_slice()),
                ])
                .unwrap(),
                budget,
            )
            .with_yield_after_windows(1),
        )
        .unwrap();
        let permuted_plan = ScrubPlan::build(
            ScrubPlanRequest::offline(
                OfflineScrubInspectionInput::from_declared_windows(vec![
                    (ordinal(1), b"cd".as_slice()),
                    (ordinal(0), b"ab".as_slice()),
                ])
                .unwrap(),
                budget,
            )
            .with_yield_after_windows(1),
        )
        .unwrap();

        let interrupted = ScrubExecution::run(first_plan).unwrap();
        let denial =
            ScrubExecution::resume(permuted_plan, interrupted.resume_token().unwrap()).unwrap_err();

        assert_eq!(
            denial.kind(),
            ScrubExecutionDenialKind::ResumeTokenForDifferentPlan
        );
    });
}

#[test]
fn scrub_counters_remain_exact_for_larger_than_memory_and_resumed_work() {
    with_scrub_budget(|budget| {
        let input = OfflineScrubInspectionInput::from_declared_windows(vec![
            (ordinal(0), b"abcd".as_slice()),
            (ordinal(1), b"abcd".as_slice()),
            (ordinal(2), b"abcd".as_slice()),
            (ordinal(3), b"abcd".as_slice()),
        ])
        .unwrap();
        let plan = ScrubPlan::build(
            ScrubPlanRequest::offline(input, budget.constrained_by_policy(8, 1, 8, 4, 8))
                .with_deferred_over_budget_windows()
                .with_skipped_window(ordinal(1))
                .with_yield_after_windows(2),
        )
        .unwrap();

        let interrupted = PhysicalIntegrityScrubWorkflow::run(plan.clone()).unwrap();
        let resumed =
            PhysicalIntegrityScrubWorkflow::resume(plan, interrupted.resume_token().unwrap())
                .unwrap();
        let counters = resumed.counters();

        assert_eq!(interrupted.counters().completed_window_count(), 2);
        assert_eq!(interrupted.counters().interrupted_window_count(), 1);
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
    });
}

fn assert_denial(
    budget: ScrubPlanBudget,
    windows: Vec<(ScrubWindowOrdinal, &[u8])>,
    expected: ScrubPlanDenialKind,
) {
    let denial = scrub_plan_denial(budget, windows);
    assert_eq!(denial.kind(), expected);
    assert_eq!(denial.counters(), ScrubCounterSnapshot::empty());
}

pub(crate) fn resident_memory_over_budget_scrub_denial(
) -> forge_store_physical_integrity::ScrubPlanDenial {
    let mut denial = None;
    with_scrub_budget(|budget| {
        denial = Some(scrub_plan_denial(
            budget.constrained_by_policy(8, 1, 8, 4, 8),
            vec![(ordinal(0), b"offline-window")],
        ));
    });
    denial.unwrap()
}

fn scrub_plan_denial(
    budget: ScrubPlanBudget,
    windows: Vec<(ScrubWindowOrdinal, &[u8])>,
) -> forge_store_physical_integrity::ScrubPlanDenial {
    let input = OfflineScrubInspectionInput::from_declared_windows(windows).unwrap();
    ScrubPlan::build(ScrubPlanRequest::offline(input, budget)).unwrap_err()
}

fn with_scrub_budget(run: impl FnOnce(ScrubPlanBudget)) {
    with_entry_seed(b"scrub-budget", |seed| {
        let mut allocation = AllocationAdmission::from_declaration(allocation_envelopes());
        let mut envelopes = BackgroundEnvelopeAdmission::new();
        let work_budget = BackgroundWorkBudgetSnapshot::foreground_reserved(32, 0, 0, 32);
        let scrub_envelope = envelopes
            .admit(
                BackgroundEnvelopeRequest::scrub_planning()
                    .resident_frames(1)
                    .resident_bytes(64)
                    .pin_pages_for_bounded_step(1)
                    .allocation_bytes(64)
                    .finish(),
                work_budget,
                &mut allocation,
            )
            .unwrap();
        let streaming_envelope = envelopes
            .admit(
                BackgroundEnvelopeRequest::large_record_streaming()
                    .allocation_bytes(64)
                    .streaming_window(256, 64)
                    .finish(),
                work_budget,
                &mut allocation,
            )
            .unwrap();
        let budget = ScrubPlanBudget::new(
            seed.entry_witness(),
            ScrubPlanningMemoryEnvelope::from_admitted(scrub_envelope).unwrap(),
            ChunkIntegrityStreamingWindow::from_admitted_streaming_envelope(streaming_envelope)
                .unwrap(),
        );
        run(budget);
    });
}

fn allocation_envelopes() -> forge_store_buffer_pool::AllocationEnvelopeSet {
    AllocationEnvelopeDeclaration::declare()
        .foreground(bytes(64))
        .maintenance(bytes(64))
        .recovery(bytes(64))
        .scrub(bytes(64))
        .import_export(bytes(64))
        .streaming(bytes(64))
        .fixed_metadata(FixedMetadataReservation::constant_bytes(64).unwrap())
        .seal()
        .unwrap()
}

fn bytes(value: u64) -> AllocationByteBudget {
    AllocationByteBudget::bytes(value).unwrap()
}

fn ordinal(value: u64) -> ScrubWindowOrdinal {
    ScrubWindowOrdinal::from_zero_based(value)
}

fn persisted_scrub_fixture_windows(payload: &[u8]) -> Vec<Vec<u8>> {
    let path = std::env::temp_dir().join(format!(
        "forge-store-s3-phase11-scrub-{}-{}.bin",
        std::process::id(),
        payload.len()
    ));
    std::fs::write(&path, payload).unwrap();
    let read_back = std::fs::read(&path).unwrap();
    std::fs::remove_file(&path).unwrap();
    vec![read_back]
}
