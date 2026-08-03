#[cfg(feature = "certification-world")]
use worth_store_recovery_physics::PageRedoEligibility;
use worth_store_recovery_physics::{
    CheckpointManifestBudgetMaterialization, CheckpointManifestMaterialization,
    CheckpointManifestRecoveryBasisMaterialization, CheckpointManifestSourceMaterialization,
    CheckpointPageImageMaterialization, PersistedRecoveryArtifactMaterialization,
    RecoveryOfflineVerifier, RecoveryProfileId, ReopenedRecoveryArtifactAdmission,
    WalRedoFrameMaterialization,
};

pub fn reopened_recovery_artifact_fixture(seed: &str) -> ReopenedRecoveryArtifactAdmission {
    let profile = RecoveryProfileId::strict_offline_recovery_artifacts();
    let artifacts = PersistedRecoveryArtifactMaterialization::new(
        seed,
        "posix",
        profile.clone(),
        CheckpointManifestMaterialization::new(
            format!("checkpoint-{seed}"),
            CheckpointManifestRecoveryBasisMaterialization::new(1, 1, 10, 20),
            CheckpointManifestSourceMaterialization::new("checkpoint", 1),
            CheckpointManifestBudgetMaterialization::new(4096, 0, 4096, 1),
        ),
        WalRedoFrameMaterialization::new(
            format!("wal-{seed}"),
            20,
            1,
            format!("sha256:op-{seed}"),
            format!("sha256:idem-{seed}"),
        ),
        CheckpointPageImageMaterialization::new(
            format!("page-{seed}"),
            1,
            7,
            19,
            format!("sha256:page-{seed}"),
        ),
    )
    .materialize()
    .unwrap();
    let report = RecoveryOfflineVerifier::for_profile(seed, "posix", profile)
        .verify_persisted_artifacts(&artifacts)
        .unwrap();
    ReopenedRecoveryArtifactAdmission::admit(report, &artifacts).unwrap()
}

pub fn reopened_recovery_artifact_for_operation_digest(
    operation_digest: &str,
) -> ReopenedRecoveryArtifactAdmission {
    let artifacts =
        super::s4_persisted_recovery::recovery_artifacts_with_operation_digest(operation_digest);
    let profile = RecoveryProfileId::strict_offline_recovery_artifacts();
    let report = RecoveryOfflineVerifier::for_profile(
        super::s4_persisted_recovery::RECOVERY_ARTIFACT_FORMAT_VERSION,
        super::s4_persisted_recovery::RECOVERY_ARTIFACT_BACKEND_PROFILE,
        profile,
    )
    .verify_persisted_artifacts(&artifacts)
    .unwrap();
    ReopenedRecoveryArtifactAdmission::admit(report, &artifacts).unwrap()
}

#[cfg(feature = "certification-world")]
pub fn reopened_redo_eligibility(
    current_lsn: u64,
    redo_lsn: u64,
    page_value: u64,
) -> PageRedoEligibility {
    let seed = format!("redo-{page_value}-{current_lsn}-{redo_lsn}");
    let profile = RecoveryProfileId::strict_offline_recovery_artifacts();
    let artifacts = PersistedRecoveryArtifactMaterialization::new(
        seed.clone(),
        "posix",
        profile.clone(),
        CheckpointManifestMaterialization::new(
            format!("checkpoint-{seed}"),
            CheckpointManifestRecoveryBasisMaterialization::new(
                1,
                1,
                current_lsn.saturating_sub(1),
                redo_lsn,
            ),
            CheckpointManifestSourceMaterialization::new("checkpoint", 1),
            CheckpointManifestBudgetMaterialization::new(4096, 0, 4096, 1),
        ),
        WalRedoFrameMaterialization::new(
            format!("wal-{seed}"),
            redo_lsn,
            page_value,
            format!("sha256:op-{seed}"),
            format!("sha256:idem-{seed}"),
        ),
        CheckpointPageImageMaterialization::new(
            format!("page-{seed}"),
            page_value,
            7,
            current_lsn,
            format!("sha256:page-{seed}"),
        ),
    )
    .materialize()
    .unwrap();
    let report = RecoveryOfflineVerifier::for_profile(seed, "posix", profile)
        .verify_persisted_artifacts(&artifacts)
        .unwrap();
    let reopened = ReopenedRecoveryArtifactAdmission::admit(report, &artifacts).unwrap();
    reopened.replay_cursor().pages()[0].eligibility().clone()
}

#[cfg(test)]
#[test]
fn operation_digest_artifact_reopens_under_its_persisted_profile() {
    let reopened = reopened_recovery_artifact_for_operation_digest("reopen-profile-regression");

    assert_eq!(
        reopened.report().artifact_digest().format_version(),
        super::s4_persisted_recovery::RECOVERY_ARTIFACT_FORMAT_VERSION
    );
    assert_eq!(
        reopened.checkpoint_base().covered_lsn_range(),
        super::redo_replay::wal_range(10, 20)
    );
}
