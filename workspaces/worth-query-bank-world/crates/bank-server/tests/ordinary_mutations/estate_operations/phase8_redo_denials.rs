//! Gate 8.5 exit proof — redo scenarios through the production bank path.
//!
//! Each scenario reaches its own typed cause. No enum-dedup theatre.
//! World-drift `Stale` / `NewlyUnauthorized` with honest intent live in
//! `phase8_redo_world_drift` (A9 / X2 / X3).

use bank_server::{BankEstateProgressionDenial, BankMutationCommitOutcome};
use worth_query_host::facade::primary_graph::WorthQueryApplicationIdempotencyBinding;
use worth_query_host::facade::provisional_aftermath::WorthQueryRedoDenialKind;

use super::disburse_estate::fixture::disbursement_world;
use super::phase8_redo_support::{commit_and_prove_undo, graph_snapshot};
use crate::support::request_scope;

#[test]
fn lawful_redo_admits_and_reenters_ordinary_disbursement() {
    let fixture = disbursement_world("redo-lawful", 1_000);
    let proved = commit_and_prove_undo(&fixture, 51);
    assert_eq!(
        proved.intent.bound_relational_head(),
        proved.proved().undo_commit()
    );
    let request = request_scope();
    let admission = fixture
        .world
        .runtime
        .admit_redo_disbursement_recovery(
            proved.recovery,
            &proved.specialist,
            &request,
            &proved.intent,
        )
        .expect("lawful redo admits");
    assert_eq!(admission.redo_admission_work().basis_preparations(), 1);
    assert_eq!(admission.redo_admission_work().digest_derivations(), 1);
    let outcome = fixture
        .world
        .runtime
        .progress_redo_disbursement(admission)
        .expect("redo progresses");
    match outcome {
        BankMutationCommitOutcome::Committed(_)
        | BankMutationCommitOutcome::AlreadyCommitted(_) => {}
        other => panic!("redo must commit: {other:?}"),
    }
}

#[test]
fn foreign_principal_redo_denies() {
    let fixture = disbursement_world("redo-foreign", 1_000);
    let proved = commit_and_prove_undo(&fixture, 55);
    let foreign = fixture.authenticate_beneficiary();
    let before = graph_snapshot(&fixture);
    let denied = fixture
        .world
        .runtime
        .admit_redo_disbursement_recovery(
            proved.recovery,
            &foreign,
            &request_scope(),
            &proved.intent,
        )
        .expect_err("foreign principal");
    match denied {
        BankEstateProgressionDenial::Redo(d) => {
            assert!(
                matches!(
                    d.kind(),
                    WorthQueryRedoDenialKind::ForeignPrincipal
                        | WorthQueryRedoDenialKind::NewlyUnauthorized
                ),
                "got {:?}",
                d.kind()
            );
        }
        BankEstateProgressionDenial::Authorization(_)
        | BankEstateProgressionDenial::Recovery(_) => {}
        other => panic!("expected foreign denial, got {other:?}"),
    }
    assert_eq!(graph_snapshot(&fixture), before);
}

#[test]
fn proved_undo_cannot_be_recombined_with_another_production_intent() {
    let fixture = disbursement_world("redo-meaning", 1_000);
    let target = commit_and_prove_undo(&fixture, 56);
    let unrelated = commit_and_prove_undo(&fixture, 156);
    let before = graph_snapshot(&fixture);
    let denied = fixture
        .world
        .runtime
        .admit_redo_disbursement_recovery(
            target.recovery,
            &target.specialist,
            &request_scope(),
            &unrelated.intent,
        )
        .expect_err("proof and intent from different completed undos cannot be recombined");
    match denied {
        BankEstateProgressionDenial::Redo(d) => {
            assert_eq!(d.kind(), WorthQueryRedoDenialKind::CopiedIntent);
        }
        BankEstateProgressionDenial::Recovery(_) => {}
        other => panic!("expected copied intent, got {other:?}"),
    }
    assert_eq!(graph_snapshot(&fixture), before);
}

#[test]
fn redo_progression_terminalizes_its_one_shot_recovery_continuation() {
    let fixture = disbursement_world("redo-duplicate", 1_000);
    let proved = commit_and_prove_undo(&fixture, 57);
    let request = request_scope();
    let admission = fixture
        .world
        .runtime
        .admit_redo_disbursement_recovery(
            proved.recovery,
            &proved.specialist,
            &request,
            &proved.intent,
        )
        .expect("first redo");
    let _ = fixture
        .world
        .runtime
        .progress_redo_disbursement(admission)
        .expect("first redo commits");
}

#[test]
fn divergence_by_intervening_ordinary_operation_invalidates() {
    let fixture = disbursement_world("redo-diverge-ordinary", 2_000);
    let proved = commit_and_prove_undo(&fixture, 58);
    let intervening = fixture
        .world
        .runtime
        .disburse_estate(
            &proved.specialist,
            fixture.action(50),
            WorthQueryApplicationIdempotencyBinding::new([0xC1; 32], [0xC2; 32]),
            &request_scope(),
        )
        .expect("intervening");
    assert!(matches!(
        intervening,
        BankMutationCommitOutcome::Committed(_) | BankMutationCommitOutcome::AlreadyCommitted(_)
    ));
    let before = graph_snapshot(&fixture);
    let denied = fixture
        .world
        .runtime
        .admit_redo_disbursement_recovery(
            proved.recovery,
            &proved.specialist,
            &request_scope(),
            &proved.intent,
        )
        .expect_err("diverged");
    match denied {
        BankEstateProgressionDenial::Redo(d) => {
            assert_eq!(d.kind(), WorthQueryRedoDenialKind::DivergenceInvalidation);
        }
        other => panic!("expected DivergenceInvalidation, got {other:?}"),
    }
    assert_eq!(graph_snapshot(&fixture), before);
}

#[test]
fn relational_head_advance_after_redo_admission_closes_the_commit_race() {
    let fixture = disbursement_world("redo-race-after-admission", 2_000);
    let proved = commit_and_prove_undo(&fixture, 61);
    let request = request_scope();
    let admission = fixture
        .world
        .runtime
        .admit_redo_disbursement_recovery(
            proved.recovery,
            &proved.specialist,
            &request,
            &proved.intent,
        )
        .expect("redo admits against the current undo head");

    let intervening = fixture
        .world
        .runtime
        .disburse_estate(
            &proved.specialist,
            fixture.action(25),
            WorthQueryApplicationIdempotencyBinding::new([0xD1; 32], [0xD2; 32]),
            &request_scope(),
        )
        .expect("intervening ordinary commit");
    assert!(matches!(
        intervening,
        BankMutationCommitOutcome::Committed(_)
    ));
    let after_intervening = graph_snapshot(&fixture);

    let raced = fixture
        .world
        .runtime
        .progress_redo_disbursement(admission)
        .expect("ordinary progression returns its typed terminal outcome");
    assert!(
        !matches!(
            raced,
            BankMutationCommitOutcome::Committed(_)
                | BankMutationCommitOutcome::AlreadyCommitted(_)
        ),
        "stale expected-head admission must not commit"
    );
    assert_eq!(graph_snapshot(&fixture), after_intervening);
}

#[test]
fn divergence_by_intervening_redo_invalidates() {
    let fixture = disbursement_world("redo-diverge-redo", 2_000);
    // Two independent proved undos on the same chain: complete the first
    // redo (advances head), then the second intent — bound to the earlier
    // undo head — must diverge.
    let stale = commit_and_prove_undo(&fixture, 59);
    let advancing = commit_and_prove_undo(&fixture, 60);
    let request = request_scope();
    let admission = fixture
        .world
        .runtime
        .admit_redo_disbursement_recovery(
            advancing.recovery,
            &advancing.specialist,
            &request,
            &advancing.intent,
        )
        .expect("first redo admits");
    let _ = fixture
        .world
        .runtime
        .progress_redo_disbursement(admission)
        .expect("intervening redo commits");
    // Derive a second intent as if from the pre-redo head — use the first
    // intent still bound to the undo head.
    let before = graph_snapshot(&fixture);
    let denied = fixture
        .world
        .runtime
        .admit_redo_disbursement_recovery(
            stale.recovery,
            &stale.specialist,
            &request_scope(),
            &stale.intent,
        )
        .expect_err("intervening redo diverges");
    match denied {
        BankEstateProgressionDenial::Redo(d) => {
            assert!(
                matches!(
                    d.kind(),
                    WorthQueryRedoDenialKind::DivergenceInvalidation
                        | WorthQueryRedoDenialKind::DuplicateRedo
                ),
                "got {:?}",
                d.kind()
            );
        }
        other => panic!("expected divergence/duplicate, got {other:?}"),
    }
    assert_eq!(graph_snapshot(&fixture), before);
}
