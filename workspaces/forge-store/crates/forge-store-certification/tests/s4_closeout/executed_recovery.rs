use forge_store_recovery_physics::{
    AdmittedRedoFrame, BoundedRecoveryPlan, BoundedRecoveryReceipt, CheckpointIntervalContract,
    FreshRuntimeRecoveryExecution, PersistedRecoveryArtifacts, RecoveryBudget, RecoveryRedoPlan,
    RecoverySourceCandidate, RecoverySourceDecisionTrace, ReopenedRecoveryDenial,
    WalTailReplayBudget,
};
use forge_store_test_support::{
    deterministic_s4_recovery_artifacts, runtime_state_mismatch_s4_recovery_artifacts,
    FreshRuntimeRecoveryDriver,
};

use super::memory_budget_fixture::recovery_memory_envelope;
use super::redo_replay_fixture::{
    cursor, frame, grammar_for_operation_digest, page_lsn, redo_eligibility, valid_prefix,
};
use super::source_precedence_fixture::{checkpoint_base, wal_tail_for_checkpoint};

#[derive(Clone)]
pub struct CloseoutFixture {
    pub receipt: BoundedRecoveryReceipt,
    pub source_trace: RecoverySourceDecisionTrace,
    pub foundational_evidence: forge_store_recovery_physics::FoundationalRecoveryEvidenceBundle,
    pub artifacts: PersistedRecoveryArtifacts,
    pub operation_digest: String,
}

pub fn executed_recovery_fixture() -> CloseoutFixture {
    executed_recovery_fixture_with_page_digest("checkpoint-page")
}

pub fn executed_recovery_fixture_with_cursor_lsn(cursor_lsn: u64) -> CloseoutFixture {
    let (receipt, source_trace) =
        executed_recovery_receipt_and_trace_with_cursor_lsn(cursor_lsn, "checkpoint-page");
    closeout_fixture(
        receipt,
        source_trace,
        deterministic_s4_recovery_artifacts(),
        "op-20",
    )
}

pub fn executed_recovery_fixture_with_runtime_state_mismatch_artifacts() -> CloseoutFixture {
    let artifacts = runtime_state_mismatch_s4_recovery_artifacts();
    let (receipt, source_trace) =
        executed_recovery_receipt_and_trace_with_operation_digest("op-21");
    closeout_fixture(receipt, source_trace, artifacts, "op-21")
}

fn closeout_fixture(
    receipt: BoundedRecoveryReceipt,
    source_trace: RecoverySourceDecisionTrace,
    artifacts: PersistedRecoveryArtifacts,
    operation_digest: &str,
) -> CloseoutFixture {
    let foundational_evidence =
        super::foundational_evidence::from_receipt_and_artifacts(&receipt, &artifacts);
    CloseoutFixture {
        receipt,
        source_trace,
        foundational_evidence,
        artifacts,
        operation_digest: operation_digest.to_string(),
    }
}

pub fn executed_recovery_fixture_with_page_digest(page_digest: &str) -> CloseoutFixture {
    let (receipt, source_trace) =
        executed_recovery_receipt_and_trace_with_cursor_lsn(19, page_digest);
    closeout_fixture(
        receipt,
        source_trace,
        deterministic_s4_recovery_artifacts(),
        "op-20",
    )
}

pub fn executed_recovery_fixture_with_redo_lsn(redo_lsn: u64) -> CloseoutFixture {
    let (receipt, source_trace) =
        executed_recovery_receipt_and_trace_with_redo_lsn(redo_lsn, redo_lsn - 1);
    let operation_digest = format!("op-{redo_lsn}");
    closeout_fixture(
        receipt,
        source_trace,
        deterministic_s4_recovery_artifacts(),
        &operation_digest,
    )
}

pub fn executed_recovery_receipt_with_cursor_lsn(cursor_lsn: u64) -> BoundedRecoveryReceipt {
    executed_recovery_receipt_and_trace_with_cursor_lsn(cursor_lsn, "checkpoint-page").0
}

pub fn executed_reopened_recovery_from_admission(
    driver: &FreshRuntimeRecoveryDriver,
) -> Result<(BoundedRecoveryReceipt, FreshRuntimeRecoveryExecution), ReopenedRecoveryDenial> {
    executed_reopened_recovery_from_admission_with_operation_digest(driver, "op-20")
}

pub fn executed_reopened_recovery_from_admission_with_operation_digest(
    driver: &FreshRuntimeRecoveryDriver,
    operation_digest: &str,
) -> Result<(BoundedRecoveryReceipt, FreshRuntimeRecoveryExecution), ReopenedRecoveryDenial> {
    let (plan, _) = bounded_recovery_plan_and_trace_with_operation_digest(20, operation_digest);
    driver.execute_reopened_runtime_recovery(&plan)
}

fn executed_recovery_receipt_and_trace_with_cursor_lsn(
    cursor_lsn: u64,
    page_digest: &str,
) -> (BoundedRecoveryReceipt, RecoverySourceDecisionTrace) {
    let (plan, source_trace) = bounded_recovery_plan_and_trace();
    let receipt = plan
        .execute(&cursor(&redo_eligibility(19, 20), cursor_lsn, page_digest))
        .unwrap();
    (receipt, source_trace)
}

fn executed_recovery_receipt_and_trace_with_redo_lsn(
    redo_lsn: u64,
    cursor_lsn: u64,
) -> (BoundedRecoveryReceipt, RecoverySourceDecisionTrace) {
    let (plan, source_trace) = bounded_recovery_plan_and_trace_with_redo_lsn(redo_lsn);
    let receipt = plan
        .execute(&cursor(
            &redo_eligibility(redo_lsn - 1, redo_lsn),
            cursor_lsn,
            "checkpoint-page",
        ))
        .unwrap();
    (receipt, source_trace)
}

fn bounded_recovery_plan_and_trace() -> (BoundedRecoveryPlan, RecoverySourceDecisionTrace) {
    bounded_recovery_plan_and_trace_with_redo_lsn(20)
}

fn executed_recovery_receipt_and_trace_with_operation_digest(
    operation_digest: &str,
) -> (BoundedRecoveryReceipt, RecoverySourceDecisionTrace) {
    let (plan, source_trace) =
        bounded_recovery_plan_and_trace_with_operation_digest(20, operation_digest);
    let receipt = plan
        .execute(&cursor(&redo_eligibility(19, 20), 19, "checkpoint-page"))
        .unwrap();
    (receipt, source_trace)
}

fn bounded_recovery_plan_and_trace_with_redo_lsn(
    redo_lsn: u64,
) -> (BoundedRecoveryPlan, RecoverySourceDecisionTrace) {
    let operation_digest = format!("op-{redo_lsn}");
    bounded_recovery_plan_and_trace_with_operation_digest(redo_lsn, &operation_digest)
}

fn bounded_recovery_plan_and_trace_with_operation_digest(
    redo_lsn: u64,
    operation_digest: &str,
) -> (BoundedRecoveryPlan, RecoverySourceDecisionTrace) {
    let checkpoint_lsn = redo_lsn - 1;
    let tail_end_lsn = redo_lsn + 1;
    let (checkpoint, cutover) = checkpoint_base(10, redo_lsn, checkpoint_lsn, 1);
    let tail = wal_tail_for_checkpoint(&cutover, tail_end_lsn, 2);
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
    let prefix = valid_prefix(source.source(), redo_lsn, tail_end_lsn, [frame(redo_lsn)]);
    let eligibility = redo_eligibility(checkpoint_lsn, redo_lsn);
    let grammar =
        grammar_for_operation_digest(&eligibility, redo_lsn, page_lsn(redo_lsn), operation_digest)
            .unwrap();
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
