use worth_store_test_support::harness::recovery::memory_budget as memory_budget_fixture;
use worth_store_test_support::harness::recovery::redo_replay as redo_replay_fixture;
use worth_store_test_support::harness::recovery::source_precedence as source_precedence_fixture;

use worth_store_recovery_physics::{
    AdmittedRedoFrame, BoundedRecoveryReceipt, CheckpointIntervalContract,
    FreshRuntimeRecoveryDriver, FreshRuntimeRecoveryExecution, FreshRuntimeReopenHarnessDenial,
    OfflineRecoveryVerificationReport, PersistedRecoveryArtifacts, RecoveryBudget,
    RecoveryOfflineVerifier, RecoveryRedoPlan, ReopenedRecoveryDenial, RuntimeRecoveryReportDenial,
    WalTailReplayBudget,
};

use memory_budget_fixture::with_recovery_memory_allocation;
use redo_replay_fixture::{cursor, frame, grammar_for, page_lsn, redo_eligibility, valid_prefix};
use source_precedence_fixture::{checkpoint_base, wal_tail_for_checkpoint};

#[allow(dead_code)]
pub fn execute_bounded_recovery_fixture() -> BoundedRecoveryReceipt {
    with_bounded_recovery_plan_fixture(|plan| {
        plan.execute(&cursor(&redo_eligibility(19, 20), 19, "checkpoint-page"))
            .unwrap()
    })
}

pub fn execute_reopened_recovery_fixture(
    offline: &OfflineRecoveryVerificationReport,
    artifacts: &PersistedRecoveryArtifacts,
) -> Result<(BoundedRecoveryReceipt, FreshRuntimeRecoveryExecution), ReopenedRecoveryDenial> {
    let evidence = RecoveryOfflineVerifier::for_profile(
        offline.format_version(),
        offline.backend_profile(),
        offline.recovery_profile().clone(),
    )
    .verify_fresh_runtime_reopen(artifacts)
    .map_err(|denial| match denial {
        FreshRuntimeReopenHarnessDenial::Admission(denial) => {
            ReopenedRecoveryDenial::Admission(*denial)
        }
        FreshRuntimeReopenHarnessDenial::Verifier(_) => {
            ReopenedRecoveryDenial::Runtime(RuntimeRecoveryReportDenial::VerifierConclusionMismatch)
        }
    })?;
    let driver = FreshRuntimeRecoveryDriver::from_reopen_harness_evidence(evidence);
    with_bounded_recovery_plan_fixture(|plan| driver.execute_reopened_runtime_recovery(&plan))
}

fn with_bounded_recovery_plan_fixture<R>(
    run: impl FnOnce(worth_store_recovery_physics::BoundedRecoveryPlan<'_>) -> R,
) -> R {
    let (checkpoint, cutover) = checkpoint_base(10, 20, 19, 1);
    let tail = wal_tail_for_checkpoint(&cutover, 21, 2);
    let source = with_recovery_memory_allocation(|memory_allocation| {
        RecoveryBudget::new(
            CheckpointIntervalContract::max_tail_frames(4),
            WalTailReplayBudget::max_frames(4)
                .with_max_scanned_segments(2)
                .with_max_page_redos(4),
            memory_allocation,
        )
        .with_max_memory_envelope_bytes(128)
        .with_max_allocation_bytes(128)
        .with_checkpoint_discovery_candidates(2)
        .source_precedence_graph("strict-test-profile")
        .discover(
            worth_store_recovery_physics::RecoverySourceCandidate::checkpoint_base(checkpoint),
        )
        .unwrap()
        .discover(worth_store_recovery_physics::RecoverySourceCandidate::wal_tail(tail))
        .unwrap()
        .admit_sources()
    });
    let prefix = valid_prefix(source.source(), 20, 21, [frame(20)]);
    let eligibility = redo_eligibility(19, 20);
    let grammar = grammar_for(&eligibility, 20, page_lsn(20)).unwrap();
    let admitted = AdmittedRedoFrame::admit(grammar, &prefix).unwrap();
    let plan =
        RecoveryRedoPlan::from_valid_prefix(source.source(), prefix, vec![admitted]).unwrap();
    with_recovery_memory_allocation(|memory_allocation| {
        let bounded = RecoveryBudget::new(
            CheckpointIntervalContract::max_tail_frames(4),
            WalTailReplayBudget::max_frames(4)
                .with_max_scanned_segments(2)
                .with_max_page_redos(4),
            memory_allocation,
        )
        .with_max_memory_envelope_bytes(128)
        .with_max_allocation_bytes(128)
        .with_checkpoint_discovery_candidates(2)
        .admit_recovery(source, plan)
        .unwrap();
        run(bounded)
    })
}
