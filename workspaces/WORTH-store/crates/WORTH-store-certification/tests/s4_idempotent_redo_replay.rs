#[path = "s4_idempotent_redo_replay/redo_replay_fixture.rs"]
mod redo_replay_fixture;

use worth_store_physical_format::PhysicalPageId;
use worth_store_recovery_physics::{
    AdmittedRedoFrame, PageRedoDigestState, RecoveredPhysicalState, RecoveryRedoPlan,
    RecoverySourceCandidate, RecoverySourceDecisionKind, RecoverySourcePrecedenceGraph,
    RedoApplicationCursor, RedoApplicationPageFact, RedoPlanningDenialKind, RedoRecordGrammar,
    RedoRecordGrammarDenialKind, RedoRecordIdempotenceBasis, RedoRecordIntegrityBinding,
    RedoRecordOperationForm, RedoRecordTargetGeneration, WalPrefixIntegrityObservation,
    WalSegmentGeneration, WalValidPrefix,
};

use redo_replay_fixture::*;

#[test]
fn checkpoint_plus_tail_replay_matches_control_rebuild_and_replay_skip() {
    let source = checkpoint_plus_tail_source(20, 21);
    let prefix = valid_prefix(&source, 20, 21, [frame(20)]);
    let eligibility = redo_eligibility(19, 20);
    let grammar = grammar_for(&eligibility, 20, page_lsn(20)).unwrap();
    let admitted = AdmittedRedoFrame::admit(grammar, &prefix).unwrap();
    let plan = RecoveryRedoPlan::from_valid_prefix(&source, prefix, vec![admitted]).unwrap();

    let replay = plan
        .execute(&cursor(&eligibility, 19, "checkpoint-page"))
        .unwrap();
    let replayed_cursor = replay.recovered_cursor().unwrap();
    let control =
        RecoveredPhysicalState::from_control_rebuild(plan.source_trace(), replayed_cursor.pages());
    let second_replay = plan.execute(&replayed_cursor).unwrap();

    assert_eq!(
        replay.recovered_state().recovered_physical_root(),
        control.recovered_physical_root()
    );
    assert_eq!(
        replay.recovered_state().recovered_physical_root(),
        second_replay.recovered_state().recovered_physical_root()
    );
    assert_eq!(replay.applied_frame_count(), 1);
    assert_eq!(replay.skipped_frames().len(), 0);
    assert_eq!(second_replay.planned_frame_count(), 1);
    assert_eq!(second_replay.applied_frame_count(), 0);
    assert_eq!(second_replay.skipped_frames().len(), 1);
    assert_eq!(second_replay.skipped_frames()[0].frame_lsn(), lsn(20));
}

#[test]
fn outside_range_wrong_page_lsn_and_blocking_damage_deny_before_execution() {
    let source = checkpoint_plus_tail_source(20, 21);
    let prefix = valid_prefix(&source, 20, 21, [frame(20)]);
    let eligibility = redo_eligibility(19, 20);

    let outside = grammar_for(&eligibility, 21, page_lsn(21)).unwrap();
    assert!(matches!(
        AdmittedRedoFrame::admit(outside, &prefix)
            .unwrap_err()
            .kind(),
        RedoPlanningDenialKind::FrameOutsideAdmittedSourceRange { .. }
    ));

    let wrong_page_lsn = grammar_for(&eligibility, 20, page_lsn(19)).unwrap();
    assert!(matches!(
        AdmittedRedoFrame::admit(wrong_page_lsn, &prefix)
            .unwrap_err()
            .kind(),
        RedoPlanningDenialKind::WrongPageLsnBasis { .. }
    ));

    let blocked = RecoverySourcePrecedenceGraph::new("strict-test-profile")
        .discover(RecoverySourceCandidate::recovery_blocked(
            blocked_manifest_damage(),
            trace("blocked", 9),
        ))
        .admit_sources();
    assert_eq!(
        blocked.trace().kind(),
        RecoverySourceDecisionKind::RecoveryBlocked
    );
    assert!(matches!(
        WalValidPrefix::from_selected_source(
            &blocked,
            wal_generation(),
            wal_range(20, 21),
            vec![frame(20),]
        )
        .unwrap_err()
        .kind(),
        RedoPlanningDenialKind::RecoveryBlocked { .. }
    ));
}

#[test]
fn plan_prefix_and_page_generation_authority_cannot_be_cross_wired() {
    let source = checkpoint_plus_tail_source(20, 21);
    let prefix = valid_prefix(&source, 20, 21, [frame(20)]);
    let mismatched_source = checkpoint_plus_tail_source(30, 31);
    assert!(matches!(
        RecoveryRedoPlan::from_valid_prefix(&mismatched_source, prefix, vec![])
            .unwrap_err()
            .kind(),
        RedoPlanningDenialKind::WalPrefixSourceMismatch { .. }
    ));

    let source = checkpoint_plus_tail_source(20, 21);
    let prefix = valid_prefix(&source, 20, 21, [frame(20)]);
    let eligibility = redo_eligibility(19, 20);
    let wrong_target_page = RedoRecordGrammar::admit(
        Some(PhysicalPageId::from_raw(99).unwrap()),
        Some(RedoRecordTargetGeneration::new(
            eligibility.page_generation(),
        )),
        Some(lsn(20)),
        Some(RedoRecordOperationForm::declared_digest("op-20")),
        Some(RedoRecordIntegrityBinding::declared_digest("integrity-20")),
        Some(RedoRecordIdempotenceBasis::declared_digest("idem-20")),
        Some(page_lsn(20)),
    )
    .unwrap();
    assert!(matches!(
        AdmittedRedoFrame::admit(wrong_target_page, &prefix)
            .unwrap_err()
            .kind(),
        RedoPlanningDenialKind::RedoTargetPageGenerationMismatch { .. }
    ));

    let page_generation = eligibility.page_generation();
    let wrong_cursor_page = RedoApplicationPageFact::new(
        PhysicalPageId::from_raw(99).unwrap(),
        eligibility.clone(),
        PageRedoDigestState::new(page_generation, page_lsn(19), "checkpoint-page"),
    );
    assert!(matches!(
        RedoApplicationCursor::new(vec![wrong_cursor_page])
            .unwrap_err()
            .kind(),
        RedoPlanningDenialKind::CursorPageGenerationMismatch { .. }
    ));
}

#[test]
fn valid_prefix_classifies_torn_tail_middle_corruption_stale_and_missing() {
    let source = checkpoint_plus_tail_source(20, 23);
    let torn = WalValidPrefix::from_selected_source(
        &source,
        wal_generation(),
        wal_range(20, 21),
        vec![frame(20), torn_frame(21)],
    )
    .unwrap();
    assert_eq!(torn.admitted_frame_count(), 1);
    assert!(torn.torn_tail().is_some());

    assert!(matches!(
        WalValidPrefix::from_selected_source(
            &source,
            wal_generation(),
            wal_range(20, 22),
            vec![frame(20), middle_corruption_frame(21)],
        )
        .unwrap_err()
        .kind(),
        RedoPlanningDenialKind::MiddleWalCorruption(_)
    ));
    assert!(matches!(
        WalPrefixIntegrityObservation::from_recovery_blocking_damage(
            &blocked_manifest_damage(),
            lsn(21),
            wal_generation(),
        )
        .unwrap_err()
        .kind(),
        RedoPlanningDenialKind::RecoveryBlocked { .. }
    ));
    assert!(matches!(
        WalPrefixIntegrityObservation::from_recovery_blocking_damage(
            &recovery_blocking_torn_wal_frame_damage(wal_range(21, 22)),
            lsn(21),
            wal_generation(),
        )
        .unwrap_err()
        .kind(),
        RedoPlanningDenialKind::RecoveryBlocked { .. }
    ));
    assert!(matches!(
        WalValidPrefix::from_selected_source(
            &source,
            wal_generation(),
            wal_range(20, 21),
            vec![stale_generation_frame(
                20,
                WalSegmentGeneration::new(99).unwrap(),
            )],
        )
        .unwrap_err()
        .kind(),
        RedoPlanningDenialKind::StaleWalGeneration(_)
    ));
    assert!(matches!(
        WalValidPrefix::from_selected_source(
            &source,
            wal_generation(),
            wal_range(20, 22),
            vec![frame(20), frame(22)],
        )
        .unwrap_err()
        .kind(),
        RedoPlanningDenialKind::MissingAcknowledgedWalRange(_)
    ));
}

#[test]
fn redo_record_grammar_requires_target_generation_operation_integrity_and_bases() {
    let eligibility = redo_eligibility(19, 20);
    let complete = grammar_for(&eligibility, 20, page_lsn(20)).unwrap();
    assert_eq!(complete.redo_lsn(), lsn(20));

    assert_grammar_denial(
        missing_generation(&eligibility),
        RedoRecordGrammarDenialKind::MissingTargetGeneration,
    );
    assert_grammar_denial(
        missing_operation(&eligibility),
        RedoRecordGrammarDenialKind::MissingOperationForm,
    );
    assert_grammar_denial(
        missing_integrity(&eligibility),
        RedoRecordGrammarDenialKind::MissingIntegrityBinding,
    );
    assert_grammar_denial(
        missing_idempotence(&eligibility),
        RedoRecordGrammarDenialKind::MissingIdempotenceBasis,
    );
    assert_grammar_denial(
        missing_page_lsn(&eligibility),
        RedoRecordGrammarDenialKind::MissingPageLsnBasis,
    );
}
