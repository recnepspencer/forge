use forge_store_recovery_physics::{
    complete_recovery, AdmittedRedoFrame, BoundedRecoveryPlan, BoundedRecoveryReceipt,
    CheckpointIntervalContract, RecoveryBudget, RecoveryCompletion, RecoveryRedoPlan,
    RecoverySourceCandidate, RecoverySourceDecisionTrace, WalTailReplayBudget,
};

use super::memory_budget_fixture::recovery_memory_envelope;
use super::redo_replay_fixture::{
    cursor, frame, grammar_for_operation_digest, page_lsn, redo_eligibility, valid_prefix,
};
use super::source_precedence_fixture::{checkpoint_base, wal_tail_for_checkpoint};

pub fn executed_recovery_receipt() -> BoundedRecoveryReceipt {
    executed_recovery_receipt_with_operation_digest("op-20")
}

pub fn executed_recovery_receipt_with_operation_digest(
    operation_digest: &str,
) -> BoundedRecoveryReceipt {
    let (plan, _) = bounded_recovery_plan_and_trace(operation_digest);
    plan.execute(&cursor(&redo_eligibility(19, 20), 19, "checkpoint-page"))
        .expect("permanent recovery execution should complete")
}

pub fn recovery_completion() -> RecoveryCompletion {
    recovery_completion_with_operation_digest("op-20")
}

pub fn recovery_completion_with_operation_digest(operation_digest: &str) -> RecoveryCompletion {
    let (plan, source_trace) = bounded_recovery_plan_and_trace(operation_digest);
    let receipt = plan
        .execute(&cursor(&redo_eligibility(19, 20), 19, "checkpoint-page"))
        .expect("permanent recovery execution should complete");
    complete_recovery(receipt.execution().clone(), source_trace)
        .expect("recovery completion should bind execution to source precedence")
}

fn bounded_recovery_plan_and_trace(
    operation_digest: &str,
) -> (BoundedRecoveryPlan, RecoverySourceDecisionTrace) {
    let (checkpoint, cutover) = checkpoint_base(10, 20, 19, 1);
    let tail = wal_tail_for_checkpoint(&cutover, 21, 2);
    let source = recovery_budget()
        .source_precedence_graph("strict-test-profile")
        .discover(RecoverySourceCandidate::checkpoint_base(checkpoint))
        .unwrap()
        .discover(RecoverySourceCandidate::wal_tail(tail))
        .unwrap()
        .admit_sources();
    let source_trace = source.source().trace().clone();
    let prefix = valid_prefix(source.source(), 20, 21, [frame(20)]);
    let eligibility = redo_eligibility(19, 20);
    let grammar =
        grammar_for_operation_digest(&eligibility, 20, page_lsn(20), operation_digest).unwrap();
    let admitted = AdmittedRedoFrame::admit(grammar, &prefix).unwrap();
    let plan =
        RecoveryRedoPlan::from_valid_prefix(source.source(), prefix, vec![admitted]).unwrap();
    let bounded = recovery_budget().admit_recovery(source, plan).unwrap();
    (bounded, source_trace)
}

fn recovery_budget() -> RecoveryBudget {
    RecoveryBudget::new(
        CheckpointIntervalContract::max_tail_frames(4),
        WalTailReplayBudget::max_frames(4)
            .with_max_scanned_segments(2)
            .with_max_page_redos(4),
        recovery_memory_envelope(),
    )
    .with_max_memory_envelope_bytes(128)
    .with_max_allocation_bytes(128)
    .with_checkpoint_discovery_candidates(2)
}
