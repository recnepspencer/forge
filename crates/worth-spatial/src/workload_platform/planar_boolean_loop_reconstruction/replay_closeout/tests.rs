use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::{
    prepared_phase_fourteen_subject, retained_replay_receipt_chain, LoopFixtureEntryOrder,
};
use crate::workload_platform::planar_boolean_loop_reconstruction::{
    ComparePlanarBooleanLoopReplayParity, PlanarBooleanLoopDecisionLog,
    PlanarBooleanLoopReconstructionEvidenceInput, PlanarBooleanLoopReconstructionEvidenceReceipt,
    PlanarBooleanLoopReconstructionLedger, PlanarBooleanLoopReplayParityDenialKind,
    PlanarBooleanLoopReplayParityInput, PlanarBooleanLoopReplayParityRowKind,
};

#[test]
fn replay_closeout_compares_real_phase_sixteen_products() {
    let canonical = prepared_phase_fourteen_subject(LoopFixtureEntryOrder::Canonical);
    let replayed = prepared_phase_fourteen_subject(LoopFixtureEntryOrder::Replayed);
    let canonical_log = PlanarBooleanLoopDecisionLog::record(canonical.decision_log_input())
        .expect("canonical phase fourteen products should admit a loop decision log");
    let replayed_log = PlanarBooleanLoopDecisionLog::record(replayed.decision_log_input())
        .expect("replayed phase fourteen products should admit a loop decision log");
    let (_, canonical_receipt) =
        PlanarBooleanLoopReconstructionLedger::assemble(canonical.ledger_input(&canonical_log))
            .expect("canonical phase fourteen products should assemble a loop ledger");
    let (_, replayed_receipt) =
        PlanarBooleanLoopReconstructionLedger::assemble(replayed.ledger_input(&replayed_log))
            .expect("replayed phase fourteen products should assemble a loop ledger");
    let replay_receipts = retained_replay_receipt_chain("loop-replay-closeout");
    let canonical_evidence = PlanarBooleanLoopReconstructionEvidenceReceipt::admit(
        PlanarBooleanLoopReconstructionEvidenceInput::from_phase_sixteen_products(
            &canonical.reconstructed_boundary,
            &canonical.island_partition,
            &canonical.split_attribution,
            canonical.role_boundary.role_outcomes(),
            canonical.degenerate_boundary.outcomes(),
            &canonical_log,
            &canonical_receipt,
            &replay_receipts,
        ),
    );
    let replayed_evidence = PlanarBooleanLoopReconstructionEvidenceReceipt::admit(
        PlanarBooleanLoopReconstructionEvidenceInput::from_phase_sixteen_products(
            &replayed.reconstructed_boundary,
            &replayed.island_partition,
            &replayed.split_attribution,
            replayed.role_boundary.role_outcomes(),
            replayed.degenerate_boundary.outcomes(),
            &replayed_log,
            &replayed_receipt,
            &replay_receipts,
        ),
    );

    let replay = ComparePlanarBooleanLoopReplayParity::compare(
        PlanarBooleanLoopReplayParityInput::admit_from_ledger_and_evidence(
            &canonical_receipt,
            &replayed_receipt,
            &canonical_evidence,
            &replayed_evidence,
            &replay_receipts,
        )
        .expect("phase sixteen replay parity input should admit from real ledger and evidence"),
    )
    .expect("real phase sixteen products should compare across replay");

    assert!(!replay.replay_identity().is_empty());
    assert_eq!(replay.rows().len(), 11);
    assert_eq!(
        replay
            .rows()
            .iter()
            .map(|row| row.kind())
            .collect::<Vec<_>>(),
        vec![
            PlanarBooleanLoopReplayParityRowKind::LoopEvidenceReceipt,
            PlanarBooleanLoopReplayParityRowKind::ReconstructedLoopSet,
            PlanarBooleanLoopReplayParityRowKind::BornLoopSet,
            PlanarBooleanLoopReplayParityRowKind::IslandPartition,
            PlanarBooleanLoopReplayParityRowKind::SplitAttribution,
            PlanarBooleanLoopReplayParityRowKind::RoleOutcomeSet,
            PlanarBooleanLoopReplayParityRowKind::DegenerateOutcomeSet,
            PlanarBooleanLoopReplayParityRowKind::DecisionLog,
            PlanarBooleanLoopReplayParityRowKind::LoopLedgerReceipt,
            PlanarBooleanLoopReplayParityRowKind::DownstreamConsumption,
            PlanarBooleanLoopReplayParityRowKind::RetainedReplayCheckpoint,
        ]
    );
    assert_eq!(
        replay.checkpoint_receipt().checkpoint_identity(),
        replay_receipts.replay_checkpoint_identity()
    );
}

#[test]
fn replay_closeout_rejects_foreign_retained_replay_authority() {
    let canonical = prepared_phase_fourteen_subject(LoopFixtureEntryOrder::Canonical);
    let replayed = prepared_phase_fourteen_subject(LoopFixtureEntryOrder::Replayed);
    let canonical_log = PlanarBooleanLoopDecisionLog::record(canonical.decision_log_input())
        .expect("canonical phase fourteen products should admit a loop decision log");
    let replayed_log = PlanarBooleanLoopDecisionLog::record(replayed.decision_log_input())
        .expect("replayed phase fourteen products should admit a loop decision log");
    let (_, canonical_receipt) =
        PlanarBooleanLoopReconstructionLedger::assemble(canonical.ledger_input(&canonical_log))
            .expect("canonical phase fourteen products should assemble a loop ledger");
    let (_, replayed_receipt) =
        PlanarBooleanLoopReconstructionLedger::assemble(replayed.ledger_input(&replayed_log))
            .expect("replayed phase fourteen products should assemble a loop ledger");
    let replay_receipts = retained_replay_receipt_chain("loop-replay-closeout");
    let foreign_replay_receipts = retained_replay_receipt_chain("loop-replay-closeout-foreign");
    let canonical_evidence = PlanarBooleanLoopReconstructionEvidenceReceipt::admit(
        PlanarBooleanLoopReconstructionEvidenceInput::from_phase_sixteen_products(
            &canonical.reconstructed_boundary,
            &canonical.island_partition,
            &canonical.split_attribution,
            canonical.role_boundary.role_outcomes(),
            canonical.degenerate_boundary.outcomes(),
            &canonical_log,
            &canonical_receipt,
            &replay_receipts,
        ),
    );
    let replayed_evidence = PlanarBooleanLoopReconstructionEvidenceReceipt::admit(
        PlanarBooleanLoopReconstructionEvidenceInput::from_phase_sixteen_products(
            &replayed.reconstructed_boundary,
            &replayed.island_partition,
            &replayed.split_attribution,
            replayed.role_boundary.role_outcomes(),
            replayed.degenerate_boundary.outcomes(),
            &replayed_log,
            &replayed_receipt,
            &replay_receipts,
        ),
    );

    let denial = PlanarBooleanLoopReplayParityInput::admit_from_ledger_and_evidence(
        &canonical_receipt,
        &replayed_receipt,
        &canonical_evidence,
        &replayed_evidence,
        &foreign_replay_receipts,
    )
    .expect_err("foreign retained replay authority must fail phase sixteen admission");

    assert_eq!(
        denial.kind(),
        PlanarBooleanLoopReplayParityDenialKind::CheckpointAuthorityMismatch
    );
}
