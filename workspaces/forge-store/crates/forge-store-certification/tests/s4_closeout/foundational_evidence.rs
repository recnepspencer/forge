use forge_foundational::{BoundaryArtifactId, BoundaryEpoch};
use forge_store_recovery_physics::{
    BoundedRecoveryReceipt, FoundationalRecoveryEvidenceBundle, FreshRuntimeRecoveryDriver,
    OfflineRecoveryVerificationReport, PersistedRecoveryArtifactDigest, RecoveryOfflineVerifier,
    RecoveryPhysicsEvidenceSource, RecoveryProfileId, ReopenedRecoveryArtifactAdmission,
};
use forge_store_test_support::deterministic_s4_recovery_artifacts;

pub fn from_receipt(receipt: &BoundedRecoveryReceipt) -> FoundationalRecoveryEvidenceBundle {
    let source = RecoveryPhysicsEvidenceSource::from_executed_recovery(
        receipt,
        &verified_report(),
        BoundaryArtifactId::new(1301),
        BoundaryEpoch::new(13),
    )
    .unwrap();
    FoundationalRecoveryEvidenceBundle::from_source(&source).unwrap()
}

pub fn verified_report() -> OfflineRecoveryVerificationReport {
    RecoveryOfflineVerifier::for_profile(
        "s4-format-v1",
        "strict-posix-fsync-dir-fsync",
        RecoveryProfileId::strict_s4(),
    )
    .verify_persisted_artifacts(&deterministic_s4_recovery_artifacts())
    .unwrap()
}

pub fn verified_reopened_artifact_admission() -> ReopenedRecoveryArtifactAdmission {
    let artifacts = deterministic_s4_recovery_artifacts();
    let report = RecoveryOfflineVerifier::for_profile(
        "s4-format-v1",
        "strict-posix-fsync-dir-fsync",
        RecoveryProfileId::strict_s4(),
    )
    .verify_persisted_artifacts(&artifacts)
    .unwrap();
    ReopenedRecoveryArtifactAdmission::admit(report, &artifacts).unwrap()
}

pub fn verified_fresh_runtime_driver(
    admission: &ReopenedRecoveryArtifactAdmission,
) -> FreshRuntimeRecoveryDriver {
    let artifacts = deterministic_s4_recovery_artifacts();
    assert_eq!(
        admission.artifact_digest(),
        &PersistedRecoveryArtifactDigest::from_artifacts(&artifacts)
    );
    let evidence = RecoveryOfflineVerifier::for_profile(
        "s4-format-v1",
        "strict-posix-fsync-dir-fsync",
        RecoveryProfileId::strict_s4(),
    )
    .verify_fresh_runtime_reopen(&artifacts)
    .unwrap();
    FreshRuntimeRecoveryDriver::from_reopen_harness_evidence(evidence)
}
