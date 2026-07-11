use forge_store_recovery_physics::{
    AdmittedRedoFrame, BackendResidueKind, BackendResidueRejection, BoundedRecoverySourceAdmission,
    CheckpointBaseAdmission, CheckpointIntervalContract, RecoveryBudget, RecoveryCounterSnapshot,
    RecoveryRedoPlan, RecoverySourceCandidate, RecoveryStoreFootprint, RedoApplicationCursor,
    RedoApplicationPageFact, WalTailRedoSource, WalTailReplayBudget,
};

use super::memory_budget_fixture::recovery_memory_envelope;
use super::redo_replay_fixture::{
    cursor, frame, grammar_for, page_lsn, redo_eligibility, redo_eligibility_for_page, valid_prefix,
};
use super::source_precedence_fixture::{checkpoint_base, trace, wal_tail_for_checkpoint};

pub struct EquivalentEnvelopeResult {
    pub total_store_pages: u64,
    pub recovered_root: String,
    pub counters: RecoveryCounterSnapshot,
}

pub fn execute_equivalent_envelope(total_store_pages: u64) -> EquivalentEnvelopeResult {
    let fixture = budget_fixture();
    let receipt = admit_bounded_with_store_pages(&fixture, total_store_pages)
        .execute(&fixture.cursor_from_lsn(19, "checkpoint-page"))
        .unwrap();
    EquivalentEnvelopeResult {
        total_store_pages,
        recovered_root: receipt
            .execution()
            .recovered_state()
            .recovered_physical_root()
            .to_string(),
        counters: receipt.counters(),
    }
}

#[derive(Clone)]
pub struct BudgetFixture {
    pub checkpoint: CheckpointBaseAdmission,
    pub tail: WalTailRedoSource,
    pub source_admission: BoundedRecoverySourceAdmission,
    pub redo_plan: RecoveryRedoPlan,
    eligibility: forge_store_recovery_physics::PageRedoEligibility,
    mismatched_tail: WalTailRedoSource,
}

impl BudgetFixture {
    pub fn cursor_from_lsn(&self, page_lsn_value: u64, digest: &str) -> RedoApplicationCursor {
        cursor(&self.eligibility, page_lsn_value, digest)
    }
}

pub fn budget_fixture() -> BudgetFixture {
    let (checkpoint, cutover) = checkpoint_base(10, 20, 19, 1);
    let tail = wal_tail_for_checkpoint(&cutover, 21, 2);
    let source_admission = budget_with(4, 4, 2, 4, 128, 128, 2)
        .source_precedence_graph("strict-test-profile")
        .discover(RecoverySourceCandidate::checkpoint_base(checkpoint.clone()))
        .unwrap()
        .discover(RecoverySourceCandidate::wal_tail(tail.clone()))
        .unwrap()
        .admit_sources();
    let prefix = valid_prefix(source_admission.source(), 20, 21, [frame(20)]);
    let eligibility = redo_eligibility(19, 20);
    let grammar = grammar_for(&eligibility, 20, page_lsn(20)).unwrap();
    let admitted = AdmittedRedoFrame::admit(grammar, &prefix).unwrap();
    let redo_plan =
        RecoveryRedoPlan::from_valid_prefix(source_admission.source(), prefix, vec![admitted])
            .unwrap();

    BudgetFixture {
        checkpoint,
        tail,
        source_admission,
        redo_plan,
        eligibility,
        mismatched_tail: wal_tail_for_checkpoint(&cutover, 22, 3),
    }
}

pub fn multi_frame_budget_fixture() -> BudgetFixture {
    let (checkpoint, cutover) = checkpoint_base(10, 20, 19, 1);
    let tail = wal_tail_for_checkpoint(&cutover, 23, 2);
    let decoy_tail = wal_tail_for_checkpoint(&cutover, 24, 3);
    let source_admission = budget_with(4, 4, 3, 4, 128, 128, 3)
        .source_precedence_graph("strict-test-profile")
        .discover(RecoverySourceCandidate::checkpoint_base(checkpoint.clone()))
        .unwrap()
        .discover(RecoverySourceCandidate::wal_tail(decoy_tail))
        .unwrap()
        .discover(RecoverySourceCandidate::wal_tail(tail.clone()))
        .unwrap()
        .admit_sources();
    let prefix = valid_prefix(
        source_admission.source(),
        20,
        23,
        [frame(20), frame(21), frame(22)],
    );
    let redo_plan = RecoveryRedoPlan::from_valid_prefix(
        source_admission.source(),
        prefix.clone(),
        vec![
            admitted_frame_for_page(20, 20, &prefix),
            admitted_frame_for_page(21, 21, &prefix),
            admitted_frame_for_page(22, 22, &prefix),
        ],
    )
    .unwrap();

    BudgetFixture {
        checkpoint,
        tail,
        source_admission,
        redo_plan,
        eligibility: redo_eligibility(19, 20),
        mismatched_tail: wal_tail_for_checkpoint(&cutover, 25, 4),
    }
}

pub fn residue_budget_fixture() -> BudgetFixture {
    let (checkpoint, cutover) = checkpoint_base(10, 20, 19, 1);
    let tail = wal_tail_for_checkpoint(&cutover, 21, 3);
    let source_admission = budget_with(4, 4, 2, 4, 128, 128, 3)
        .source_precedence_graph("strict-test-profile")
        .discover(RecoverySourceCandidate::checkpoint_base(checkpoint.clone()))
        .unwrap()
        .discover(RecoverySourceCandidate::backend_residue(
            BackendResidueRejection::new(
                BackendResidueKind::BackendDirectoryResidue,
                trace("backend-residue", 2),
            ),
        ))
        .unwrap()
        .discover(RecoverySourceCandidate::wal_tail(tail.clone()))
        .unwrap()
        .admit_sources();
    let prefix = valid_prefix(source_admission.source(), 20, 21, [frame(20)]);
    let eligibility = redo_eligibility(19, 20);
    let grammar = grammar_for(&eligibility, 20, page_lsn(20)).unwrap();
    let admitted = AdmittedRedoFrame::admit(grammar, &prefix).unwrap();
    let redo_plan =
        RecoveryRedoPlan::from_valid_prefix(source_admission.source(), prefix, vec![admitted])
            .unwrap();

    BudgetFixture {
        checkpoint,
        tail,
        source_admission,
        redo_plan,
        eligibility,
        mismatched_tail: wal_tail_for_checkpoint(&cutover, 22, 4),
    }
}

pub fn admit_bounded(fixture: &BudgetFixture) -> forge_store_recovery_physics::BoundedRecoveryPlan {
    admit_bounded_with_store_pages(fixture, 1_024)
}

pub fn admit_bounded_with_store_pages(
    fixture: &BudgetFixture,
    total_store_pages: u64,
) -> forge_store_recovery_physics::BoundedRecoveryPlan {
    budget_with(4, 4, 2, 4, 128, 128, 2)
        .with_store_footprint(RecoveryStoreFootprint::admitted_persisted_pages(
            total_store_pages,
        ))
        .admit_recovery(fixture.source_admission.clone(), fixture.redo_plan.clone())
        .unwrap()
}

pub fn admit_hostile_bounded(
    fixture: &BudgetFixture,
) -> forge_store_recovery_physics::BoundedRecoveryPlan {
    budget_with(4, 4, 2, 4, 128, 128, 3)
        .admit_recovery(fixture.source_admission.clone(), fixture.redo_plan.clone())
        .unwrap()
}

pub fn wrong_same_count_source_admission(
    fixture: &BudgetFixture,
) -> BoundedRecoverySourceAdmission {
    budget_with(4, 4, 2, 4, 128, 128, 2)
        .source_precedence_graph("strict-test-profile")
        .discover(RecoverySourceCandidate::wal_tail(fixture.tail.clone()))
        .unwrap()
        .discover(RecoverySourceCandidate::wal_tail(
            fixture.mismatched_tail.clone(),
        ))
        .unwrap()
        .admit_sources()
}

pub fn budget_with(
    max_tail_interval_frames: usize,
    max_replay_frames: usize,
    max_scanned_segments: usize,
    max_page_redos: usize,
    max_memory_bytes: u64,
    max_allocation_bytes: u64,
    max_checkpoint_candidates: usize,
) -> RecoveryBudget {
    RecoveryBudget::new(
        CheckpointIntervalContract::max_tail_frames(max_tail_interval_frames),
        WalTailReplayBudget::max_frames(max_replay_frames)
            .with_max_scanned_segments(max_scanned_segments)
            .with_max_page_redos(max_page_redos),
        recovery_memory_envelope(),
    )
    .with_max_memory_envelope_bytes(max_memory_bytes)
    .with_max_allocation_bytes(max_allocation_bytes)
    .with_checkpoint_discovery_candidates(max_checkpoint_candidates)
}

pub fn multi_page_cursor(pages: [(u64, u64, u64); 3]) -> RedoApplicationCursor {
    let facts = pages
        .into_iter()
        .map(|(page_value, current_lsn, redo_lsn)| {
            let eligibility = redo_eligibility_for_page(redo_lsn - 1, redo_lsn, page_value);
            let page_generation = eligibility.page_generation();
            RedoApplicationPageFact::new(
                page_generation.page_id(),
                eligibility,
                forge_store_recovery_physics::PageRedoDigestState::new(
                    page_generation,
                    page_lsn(current_lsn),
                    format!("page-{page_value}-lsn-{current_lsn}"),
                ),
            )
        })
        .collect();
    RedoApplicationCursor::new(facts).unwrap()
}

fn admitted_frame_for_page(
    redo_lsn: u64,
    page_value: u64,
    prefix: &forge_store_recovery_physics::WalValidPrefix,
) -> AdmittedRedoFrame {
    let eligibility = redo_eligibility_for_page(redo_lsn - 1, redo_lsn, page_value);
    let grammar = grammar_for(&eligibility, redo_lsn, page_lsn(redo_lsn)).unwrap();
    AdmittedRedoFrame::admit(grammar, prefix).unwrap()
}
