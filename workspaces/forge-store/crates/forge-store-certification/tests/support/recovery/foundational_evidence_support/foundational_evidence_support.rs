#![allow(dead_code)]

#[path = "../recovery_offline_verifier/runtime_recovery_fixture.rs"]
mod runtime_recovery_fixture;

use forge_foundational::{BoundaryArtifactId, BoundaryEpoch};
use forge_store_recovery_physics::{
    FoundationalRecoveryEvidenceBundle, OfflineRecoveryVerificationReport, RecoveryOfflineVerifier,
    RecoveryPhysicsEvidenceSource, RecoveryProfileId,
};
use forge_store_test_support::{
    deterministic_s4_recovery_artifacts, runtime_disagreement_s4_recovery_artifacts,
};

pub fn bundle_from_source(
    source: &RecoveryPhysicsEvidenceSource,
) -> FoundationalRecoveryEvidenceBundle {
    FoundationalRecoveryEvidenceBundle::from_source(source).unwrap()
}

pub fn verified_source() -> RecoveryPhysicsEvidenceSource {
    let receipt = runtime_recovery_fixture::execute_bounded_recovery_fixture();
    RecoveryPhysicsEvidenceSource::from_executed_recovery(
        &receipt,
        &verified_report(),
        BoundaryArtifactId::new(91),
        BoundaryEpoch::new(4),
    )
    .unwrap()
}

pub fn verifier_disagreement_source() -> RecoveryPhysicsEvidenceSource {
    let receipt = runtime_recovery_fixture::execute_bounded_recovery_fixture();
    let report = RecoveryOfflineVerifier::for_profile(
        "s4-format-v1",
        "strict-posix-fsync-dir-fsync",
        RecoveryProfileId::strict_s4(),
    )
    .verify_persisted_artifacts(&runtime_disagreement_s4_recovery_artifacts())
    .unwrap();
    RecoveryPhysicsEvidenceSource::from_executed_recovery(
        &receipt,
        &report,
        BoundaryArtifactId::new(92),
        BoundaryEpoch::new(4),
    )
    .unwrap()
}

fn verified_report() -> OfflineRecoveryVerificationReport {
    RecoveryOfflineVerifier::for_profile(
        "s4-format-v1",
        "strict-posix-fsync-dir-fsync",
        RecoveryProfileId::strict_s4(),
    )
    .verify_persisted_artifacts(&deterministic_s4_recovery_artifacts())
    .unwrap()
}
