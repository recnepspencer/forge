#[path = "../../../certification/public_facade_contracts/contracts/public_api_planar_boolean_loop_reconstruction_metaboss_closeout_support/mod.rs"]
mod closeout_support;

use worth_spatial::facade::planar_boolean_loop_reconstruction::{
    ComparePlanarBooleanLoopReplayParity, PlanarBooleanLoopReconstructionEvidenceInput,
    PlanarBooleanLoopReconstructionEvidenceReceipt, PlanarBooleanLoopReplayParityDenialKind,
    PlanarBooleanLoopReplayParityInput,
};

use super::support::{
    build_edge_split_replay_parity_subject, run_with_large_stack, MetabossEventExtractionSubject,
    ReplayBranch,
};
use super::support_phase16::real_loop_replay_closeout_products;

#[test]
fn planar_boolean_loop_reconstruction_metaboss_chain_is_canonical_replayable_role_preserving_and_unforgeable(
) {
    run_with_large_stack(|| {
        closeout_support::assert_loop_reconstruction_summum_bonum_closeout_certifies_real_production_chain();
        closeout_support::assert_loop_reconstruction_summum_bonum_replay_closeout_holds();
    });
}

#[test]
fn planar_boolean_loop_reconstruction_replay_closeout_rejects_foreign_loop_authority() {
    run_with_large_stack(|| {
        let label = "phase7.4 loop reconstruction replay authority closeout";
        let replay_subject =
            build_edge_split_replay_parity_subject(&MetabossEventExtractionSubject::certify(label));
        let original = real_loop_replay_closeout_products(label, ReplayBranch::Original);
        let _replayed = real_loop_replay_closeout_products(label, ReplayBranch::Replayed);
        let foreign_subject =
            MetabossEventExtractionSubject::certify("phase7.4 foreign loop authority closeout");
        let _foreign_replay_subject = build_edge_split_replay_parity_subject(&foreign_subject);
        let foreign = real_loop_replay_closeout_products(
            "phase7.4 foreign loop authority closeout",
            ReplayBranch::Original,
        );
        let foreign_evidence = PlanarBooleanLoopReconstructionEvidenceReceipt::admit(
            PlanarBooleanLoopReconstructionEvidenceInput::from_phase_sixteen_products(
                &foreign.degenerate.role_products.reconstructed.boundary,
                &foreign.degenerate.role_products.reconstructed.partition,
                &foreign
                    .degenerate
                    .role_products
                    .reconstructed
                    .split_attribution,
                &foreign.degenerate.role_products.role_outcomes,
                &foreign.degenerate.outcomes,
                &foreign.decision_log,
                &foreign.ledger_receipt,
                &replay_subject.replay_receipts,
            ),
        );

        let denial = ComparePlanarBooleanLoopReplayParity::compare(
            PlanarBooleanLoopReplayParityInput::admit_from_ledger_and_evidence(
                &original.ledger_receipt,
                &foreign.ledger_receipt,
                &original.evidence_receipt,
                &foreign_evidence,
                &replay_subject.replay_receipts,
            )
            .expect("foreign loop evidence should still admit a typed parity input"),
        )
        .expect_err("foreign loop authority must deny phase sixteen replay closeout");

        assert!(
            matches!(
                denial.kind(),
                PlanarBooleanLoopReplayParityDenialKind::LoopEvidenceMismatch
                    | PlanarBooleanLoopReplayParityDenialKind::DecisionLogMismatch
                    | PlanarBooleanLoopReplayParityDenialKind::LoopLedgerMismatch
            ),
            "foreign authority should deny on a typed loop proof surface: {denial:?}"
        );
    });
}

#[test]
fn planar_boolean_loop_reconstruction_replay_closeout_rejects_foreign_retained_replay_authority() {
    run_with_large_stack(|| {
        let label = "phase7.4 loop reconstruction retained replay authority closeout";
        let original = real_loop_replay_closeout_products(label, ReplayBranch::Original);
        let replayed = real_loop_replay_closeout_products(label, ReplayBranch::Replayed);
        let foreign_replay_receipts =
            build_edge_split_replay_parity_subject(&MetabossEventExtractionSubject::certify(
                "phase7.4 foreign retained replay authority closeout",
            ))
            .replay_receipts;

        let denial = PlanarBooleanLoopReplayParityInput::admit_from_ledger_and_evidence(
            &original.ledger_receipt,
            &replayed.ledger_receipt,
            &original.evidence_receipt,
            &replayed.evidence_receipt,
            &foreign_replay_receipts,
        )
        .expect_err("foreign retained replay authority must fail phase sixteen admission");

        assert_eq!(
            denial.kind(),
            PlanarBooleanLoopReplayParityDenialKind::CheckpointAuthorityMismatch
        );
    });
}
