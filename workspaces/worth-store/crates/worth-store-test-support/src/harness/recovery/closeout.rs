use worth_store_recovery_physics::{
    complete_recovery, AdmittedRedoFrame, BoundedRecoveryPlan, BoundedRecoveryReceipt,
    CheckpointIntervalContract, RecoveryBudget, RecoveryCandidateDiscoveryTrace,
    RecoveryCompletion, RecoveryRedoPlan, RecoverySourceCandidate, RecoverySourceDecisionTrace,
    WalTailRedoSource, WalTailReplayBudget,
};

use super::memory_budget::with_recovery_memory_allocation;
use super::redo_replay::{
    cursor, frame, grammar_for_operation_digest, page_lsn, redo_eligibility_for_page, valid_prefix,
    wal_range,
};
use super::source_precedence::checkpoint_base;
use super::wal_tail::wal_only_tail_proof;

pub fn executed_recovery_receipt() -> BoundedRecoveryReceipt {
    executed_recovery_receipt_with_operation_digest("op-20")
}

pub fn executed_recovery_receipt_with_operation_digest(
    operation_digest: &str,
) -> BoundedRecoveryReceipt {
    with_bounded_recovery_plan_and_trace(operation_digest, |plan, _, cursor| {
        plan.execute(&cursor)
            .expect("permanent recovery execution should complete")
    })
}

pub fn recovery_completion() -> RecoveryCompletion {
    recovery_completion_with_operation_digest("op-20")
}

pub fn recovery_completion_with_operation_digest(operation_digest: &str) -> RecoveryCompletion {
    with_bounded_recovery_plan_and_trace(operation_digest, |plan, source_trace, cursor| {
        let receipt = plan
            .execute(&cursor)
            .expect("permanent recovery execution should complete");
        complete_recovery(receipt.execution().clone(), source_trace)
            .expect("recovery completion should bind execution to source precedence")
    })
}

fn with_bounded_recovery_plan_and_trace<R>(
    operation_digest: &str,
    run: impl FnOnce(
        BoundedRecoveryPlan<'_>,
        RecoverySourceDecisionTrace,
        worth_store_recovery_physics::RedoApplicationCursor,
    ) -> R,
) -> R {
    let eligibility = redo_eligibility_for_page(19, 20, 1);
    let application_cursor = cursor(&eligibility, 19, "checkpoint-page");
    let (checkpoint, _) = checkpoint_base(10, 20, 19, 1);
    let tail = WalTailRedoSource::from_reopened_checkpoint(
        &checkpoint,
        wal_only_tail_proof(wal_range(20, 21)),
        RecoveryCandidateDiscoveryTrace::new("strict-test-profile", "wal-tail", 2),
    )
    .expect("the reopened checkpoint frontier is contiguous with the vetted WAL tail");
    let source = with_recovery_budget(|budget| {
        budget
            .source_precedence_graph("strict-test-profile")
            .discover(RecoverySourceCandidate::checkpoint_base(checkpoint))
            .unwrap()
            .discover(RecoverySourceCandidate::wal_tail(tail))
            .unwrap()
            .admit_sources()
    });
    let source_trace = source.source().trace().clone();
    let prefix = valid_prefix(source.source(), 20, 21, [frame(20)]);
    let grammar =
        grammar_for_operation_digest(&eligibility, 20, page_lsn(20), operation_digest).unwrap();
    let admitted = AdmittedRedoFrame::admit(grammar, &prefix).unwrap();
    let plan =
        RecoveryRedoPlan::from_valid_prefix(source.source(), prefix, vec![admitted]).unwrap();
    with_recovery_budget(|budget| {
        let bounded = budget.admit_recovery(source, plan).unwrap();
        run(bounded, source_trace, application_cursor)
    })
}

fn with_recovery_budget<R>(run: impl FnOnce(RecoveryBudget<'_>) -> R) -> R {
    with_recovery_memory_allocation(|memory_allocation| {
        run(RecoveryBudget::new(
            CheckpointIntervalContract::max_tail_frames(4),
            WalTailReplayBudget::max_frames(4)
                .with_max_scanned_segments(2)
                .with_max_page_redos(4),
            memory_allocation,
        )
        .with_max_memory_envelope_bytes(128)
        .with_max_allocation_bytes(128)
        .with_checkpoint_discovery_candidates(2))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use worth_store_recovery_physics::{RecoveryBudgetDenialKind, RecoverySourceDecisionKind};

    #[test]
    fn bounded_closeout_joins_reopened_checkpoint_to_vetted_wal_tail() {
        with_bounded_recovery_plan_and_trace("checkpoint-tail-regression", |plan, trace, _| {
            assert_eq!(
                trace.kind(),
                RecoverySourceDecisionKind::CheckpointPlusWalTail
            );
            assert_eq!(plan.checkpoint().covered_lsn_range(), wal_range(10, 20));
            assert_eq!(
                plan.tail().checkpoint_id(),
                Some(plan.checkpoint().checkpoint_id())
            );
        });
    }

    #[test]
    fn bounded_recovery_still_denies_a_genuine_wal_only_source() {
        let operation_digest = "wal-only-bounded-denial";
        let eligibility = redo_eligibility_for_page(19, 20, 1);
        let tail = WalTailRedoSource::wal_only(
            wal_only_tail_proof(wal_range(20, 21)),
            RecoveryCandidateDiscoveryTrace::new("strict-test-profile", "wal-only", 1),
        );
        let source = with_recovery_budget(|budget| {
            budget
                .source_precedence_graph("strict-test-profile")
                .discover(RecoverySourceCandidate::wal_tail(tail))
                .unwrap()
                .admit_sources()
        });
        assert_eq!(
            source.source().trace().kind(),
            RecoverySourceDecisionKind::WalOnly
        );
        let prefix = valid_prefix(source.source(), 20, 21, [frame(20)]);
        let grammar =
            grammar_for_operation_digest(&eligibility, 20, page_lsn(20), operation_digest).unwrap();
        let admitted = AdmittedRedoFrame::admit(grammar, &prefix).unwrap();
        let redo_plan =
            RecoveryRedoPlan::from_valid_prefix(source.source(), prefix, vec![admitted]).unwrap();

        let denial =
            with_recovery_budget(|budget| budget.admit_recovery(source, redo_plan).unwrap_err());
        assert!(matches!(
            denial.kind(),
            RecoveryBudgetDenialKind::MissingCheckpointBaseForBoundedRecovery {
                source_kind: RecoverySourceDecisionKind::WalOnly,
            }
        ));
    }
}
