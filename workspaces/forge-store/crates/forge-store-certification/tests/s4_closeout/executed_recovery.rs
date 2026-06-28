use forge_store_recovery_physics::{
    AdmittedRedoFrame, BoundedRecoveryPlan, BoundedRecoveryReceipt, CheckpointIntervalContract,
    FreshRuntimeRecoveryExecution, RecoveryBudget, RecoveryRedoPlan, RecoverySourceCandidate,
    RecoverySourceDecisionTrace, ReopenedRecoveryDenial, WalTailReplayBudget,
};
use forge_store_test_support::FreshRuntimeRecoveryDriver;

use super::memory_budget_fixture::recovery_memory_envelope;
use super::redo_replay_fixture::{
    cursor, frame, grammar_for, page_lsn, redo_eligibility, valid_prefix,
};
use super::source_precedence_fixture::{checkpoint_base, wal_tail_for_checkpoint};

#[derive(Clone)]
pub struct CloseoutFixture {
    pub receipt: BoundedRecoveryReceipt,
    pub source_trace: RecoverySourceDecisionTrace,
    pub foundational_evidence: forge_store_recovery_physics::FoundationalRecoveryEvidenceBundle,
}

pub fn executed_recovery_fixture() -> CloseoutFixture {
    executed_recovery_fixture_with_cursor_lsn(19)
}

pub fn executed_recovery_fixture_with_cursor_lsn(cursor_lsn: u64) -> CloseoutFixture {
    let (receipt, source_trace) = executed_recovery_receipt_and_trace_with_cursor_lsn(cursor_lsn);
    let foundational_evidence = super::foundational_evidence::from_receipt(&receipt);
    CloseoutFixture {
        receipt,
        source_trace,
        foundational_evidence,
    }
}

pub fn executed_recovery_receipt_with_cursor_lsn(cursor_lsn: u64) -> BoundedRecoveryReceipt {
    executed_recovery_receipt_and_trace_with_cursor_lsn(cursor_lsn).0
}

pub fn executed_reopened_recovery_from_admission(
    driver: &FreshRuntimeRecoveryDriver,
) -> Result<(BoundedRecoveryReceipt, FreshRuntimeRecoveryExecution), ReopenedRecoveryDenial> {
    let (plan, _) = bounded_recovery_plan_and_trace();
    driver.execute_reopened_runtime_recovery(&plan)
}

fn executed_recovery_receipt_and_trace_with_cursor_lsn(
    cursor_lsn: u64,
) -> (BoundedRecoveryReceipt, RecoverySourceDecisionTrace) {
    let (plan, source_trace) = bounded_recovery_plan_and_trace();
    let receipt = plan
        .execute(&cursor(
            &redo_eligibility(19, 20),
            cursor_lsn,
            "checkpoint-page",
        ))
        .unwrap();
    (receipt, source_trace)
}

fn bounded_recovery_plan_and_trace() -> (BoundedRecoveryPlan, RecoverySourceDecisionTrace) {
    let (checkpoint, cutover) = checkpoint_base(10, 20, 19, 1);
    let tail = wal_tail_for_checkpoint(&cutover, 21, 2);
    let source = RecoveryBudget::new(
        CheckpointIntervalContract::max_tail_frames(4),
        WalTailReplayBudget::max_frames(4)
            .with_max_scanned_segments(2)
            .with_max_page_redos(4),
        recovery_memory_envelope(),
    )
    .with_max_memory_envelope_bytes(128)
    .with_max_allocation_bytes(128)
    .with_checkpoint_discovery_candidates(2)
    .source_precedence_graph("strict-test-profile")
    .discover(RecoverySourceCandidate::checkpoint_base(checkpoint))
    .unwrap()
    .discover(RecoverySourceCandidate::wal_tail(tail))
    .unwrap()
    .admit_sources();
    let source_trace = source.source().trace().clone();
    let prefix = valid_prefix(source.source(), 20, 21, [frame(20)]);
    let eligibility = redo_eligibility(19, 20);
    let grammar = grammar_for(&eligibility, 20, page_lsn(20)).unwrap();
    let admitted = AdmittedRedoFrame::admit(grammar, &prefix).unwrap();
    let plan =
        RecoveryRedoPlan::from_valid_prefix(source.source(), prefix, vec![admitted]).unwrap();
    let bounded_plan = RecoveryBudget::new(
        CheckpointIntervalContract::max_tail_frames(4),
        WalTailReplayBudget::max_frames(4)
            .with_max_scanned_segments(2)
            .with_max_page_redos(4),
        recovery_memory_envelope(),
    )
    .with_max_memory_envelope_bytes(128)
    .with_max_allocation_bytes(128)
    .with_checkpoint_discovery_candidates(2)
    .admit_recovery(source, plan)
    .unwrap();
    (bounded_plan, source_trace)
}
