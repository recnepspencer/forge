use worth_foundational::{BoundaryArtifactId, BoundaryEpoch};
use worth_store_recovery_physics::{
    BoundedRecoveryReceipt, FoundationalRecoveryEvidenceBundle, FreshRuntimeRecoveryDriver,
    OfflineRecoveryVerificationReport, PersistedRecoveryArtifactDigest, PersistedRecoveryArtifacts,
    RecoveryOfflineVerifier, RecoveryPhysicsEvidenceSource, RecoveryProfileId,
    ReopenedRecoveryArtifactAdmission,
};
use worth_store_test_support::deterministic_s4_recovery_artifacts;

pub fn from_receipt_and_artifacts(
    receipt: &BoundedRecoveryReceipt,
    artifacts: &PersistedRecoveryArtifacts,
) -> FoundationalRecoveryEvidenceBundle {
    let source = RecoveryPhysicsEvidenceSource::from_executed_recovery(
        receipt,
        &verified_report_for_artifacts(artifacts),
        BoundaryArtifactId::new(1301),
        BoundaryEpoch::new(13),
    )
    .unwrap();
    FoundationalRecoveryEvidenceBundle::from_source(&source).unwrap()
}

pub fn verified_report_for_artifacts(
    artifacts: &PersistedRecoveryArtifacts,
) -> OfflineRecoveryVerificationReport {
    RecoveryOfflineVerifier::for_profile(
        "s4-format-v1",
        "strict-posix-fsync-dir-fsync",
        RecoveryProfileId::strict_s4(),
    )
    .verify_persisted_artifacts(artifacts)
    .unwrap()
}

pub fn verified_reopened_artifact_admission() -> ReopenedRecoveryArtifactAdmission {
    let artifacts = deterministic_s4_recovery_artifacts();
    verified_reopened_artifact_admission_for_artifacts(&artifacts)
}

pub fn verified_reopened_artifact_admission_for_artifacts(
    artifacts: &PersistedRecoveryArtifacts,
) -> ReopenedRecoveryArtifactAdmission {
    let report = verified_report_for_artifacts(artifacts);
    ReopenedRecoveryArtifactAdmission::admit(report, &artifacts).unwrap()
}

pub fn verified_fresh_runtime_driver(
    admission: &ReopenedRecoveryArtifactAdmission,
) -> FreshRuntimeRecoveryDriver {
    let artifacts = deterministic_s4_recovery_artifacts();
    verified_fresh_runtime_driver_for_artifacts(admission, &artifacts)
}

pub fn verified_fresh_runtime_driver_for_artifacts(
    admission: &ReopenedRecoveryArtifactAdmission,
    artifacts: &PersistedRecoveryArtifacts,
) -> FreshRuntimeRecoveryDriver {
    assert_eq!(
        admission.artifact_digest(),
        &PersistedRecoveryArtifactDigest::from_artifacts(artifacts)
    );
    let evidence = RecoveryOfflineVerifier::for_profile(
        "s4-format-v1",
        "strict-posix-fsync-dir-fsync",
        RecoveryProfileId::strict_s4(),
    )
    .verify_fresh_runtime_reopen(artifacts)
    .unwrap();
    FreshRuntimeRecoveryDriver::from_reopen_harness_evidence(evidence)
}
