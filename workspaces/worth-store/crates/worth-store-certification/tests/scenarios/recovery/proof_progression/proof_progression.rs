use crate::foundational_evidence_support as evidence_support;

use worth_proof::TransitionOutcome;
use worth_store_recovery_physics::{
    ProofProgressionRecoveryTrace, RecoveryEvidenceDenial, RecoveryProofProgressionStep,
    RecoveryProofSourceFamily,
};

#[test]
fn recovery_proof_progression_uses_checked_outcomes_for_every_phase() {
    let source = evidence_support::verified_source();
    let trace = ProofProgressionRecoveryTrace::from_source(&source);

    assert!(matches!(trace.outcome(), TransitionOutcome::Success(())));
    assert!(trace.checked_recipe_admitted());
    assert!(matches!(
        trace.checked_recipe_outcome(),
        TransitionOutcome::Success(_)
    ));
    assert!(matches!(
        trace.checked_executed_replay(),
        TransitionOutcome::Success(_)
    ));
    assert_eq!(
        trace.steps().as_slice(),
        &[
            RecoveryProofProgressionStep::RecoveryEntry,
            RecoveryProofProgressionStep::LoweredRedoPlan,
            RecoveryProofProgressionStep::ExecutionReadyReplay,
            RecoveryProofProgressionStep::ExecutedReplay,
            RecoveryProofProgressionStep::StaleRestart,
            RecoveryProofProgressionStep::BoundaryReadmission,
        ]
    );
}

#[test]
fn proof_collections_require_canonical_order_and_unique_typed_source_families() {
    let ordered = ProofProgressionRecoveryTrace::admit_wal_replay_order(vec![1, 2, 3]).unwrap();
    let families = ProofProgressionRecoveryTrace::admit_source_families(vec![
        RecoveryProofSourceFamily::Checkpoint,
        RecoveryProofSourceFamily::Wal,
        RecoveryProofSourceFamily::RecoveredState,
        RecoveryProofSourceFamily::OfflineVerifier,
    ])
    .unwrap();

    assert_eq!(ordered.as_slice(), &[1, 2, 3]);
    assert_eq!(
        families.as_slice(),
        &[
            RecoveryProofSourceFamily::Checkpoint,
            RecoveryProofSourceFamily::Wal,
            RecoveryProofSourceFamily::RecoveredState,
            RecoveryProofSourceFamily::OfflineVerifier,
        ]
    );
    assert_eq!(
        ProofProgressionRecoveryTrace::admit_wal_replay_order(vec![2, 1]).unwrap_err(),
        RecoveryEvidenceDenial::NonCanonicalWalReplayOrder
    );
    assert_eq!(
        ProofProgressionRecoveryTrace::admit_source_families(vec![
            RecoveryProofSourceFamily::Wal,
            RecoveryProofSourceFamily::Wal,
        ])
        .unwrap_err(),
        RecoveryEvidenceDenial::DuplicateRecoverySourceFamily
    );
}
