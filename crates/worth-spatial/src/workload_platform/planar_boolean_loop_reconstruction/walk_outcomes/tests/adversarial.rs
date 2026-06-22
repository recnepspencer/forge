use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::{
    adversarial_loop_reconstruction_subject, AdversarialLoopReconstructionScenario,
};
use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanWalkOutcomeCause, PlanarBooleanWalkOutcomeKind,
};

#[test]
fn walk_outcomes_localize_open_walks_before_loop_promotion() {
    let subject =
        adversarial_loop_reconstruction_subject(AdversarialLoopReconstructionScenario::OpenWalk);

    let outcomes = subject.classify();

    assert!(outcomes.counters().walks_classified() >= 1);
    assert!(outcomes.counters().open_walks() >= 1);
    assert!(outcomes.rows().iter().any(|row| {
        row.kind() == PlanarBooleanWalkOutcomeKind::Open
            && row.cause() == PlanarBooleanWalkOutcomeCause::OpenInsufficientSlots
    }));
}

#[test]
fn walk_outcomes_localize_denied_proof_mismatches_before_loop_promotion() {
    let subject = adversarial_loop_reconstruction_subject(
        AdversarialLoopReconstructionScenario::DeniedProofMismatch,
    );

    let outcomes = subject.classify();

    let row = outcomes
        .rows()
        .first()
        .expect("fixture should produce one walk row");
    assert_eq!(row.kind(), PlanarBooleanWalkOutcomeKind::Denied);
    assert_eq!(
        row.cause(),
        PlanarBooleanWalkOutcomeCause::DeniedProofMismatch
    );
}

#[test]
fn walk_outcomes_localize_residual_fragment_claims_before_loop_promotion() {
    let subject = adversarial_loop_reconstruction_subject(
        AdversarialLoopReconstructionScenario::ResidualFragmentClaim,
    );

    let outcomes = subject.classify();

    assert_eq!(outcomes.counters().walks_classified(), 1);
    assert_eq!(outcomes.counters().residual_walks(), 1);
    let row = outcomes
        .rows()
        .first()
        .expect("fixture should produce one walk row");
    assert_eq!(row.kind(), PlanarBooleanWalkOutcomeKind::Residual);
    assert_eq!(
        row.cause(),
        PlanarBooleanWalkOutcomeCause::ResidualCoverageMismatch
    );
}
