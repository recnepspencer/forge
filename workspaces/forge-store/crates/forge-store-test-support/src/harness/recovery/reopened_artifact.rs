use forge_store_recovery_physics::{
    CheckpointManifestMaterialization, CheckpointPageImageMaterialization,
    PersistedRecoveryArtifactMaterialization, RecoveryOfflineVerifier, RecoveryProfileId,
    ReopenedRecoveryArtifactAdmission, WalRedoFrameMaterialization,
};

pub fn reopened_recovery_artifact_fixture(seed: &str) -> ReopenedRecoveryArtifactAdmission {
    let profile = RecoveryProfileId::strict_offline_recovery_artifacts();
    let artifacts = PersistedRecoveryArtifactMaterialization::new(
        seed,
        "posix",
        profile.clone(),
        CheckpointManifestMaterialization::new(
            &format!("checkpoint-{seed}"),
            &format!("root-{seed}"),
            19,
            "checkpoint",
            1,
            4096,
            1,
            4096,
            1,
        ),
        WalRedoFrameMaterialization::new(
            &format!("wal-{seed}"),
            20,
            1,
            &format!("sha256:op-{seed}"),
            &format!("sha256:idem-{seed}"),
        ),
        CheckpointPageImageMaterialization::new(
            &format!("page-{seed}"),
            1,
            7,
            19,
            &format!("sha256:page-{seed}"),
        ),
    )
    .materialize()
    .unwrap();
    let report = RecoveryOfflineVerifier::for_profile(seed, "posix", profile)
        .verify_persisted_artifacts(&artifacts)
        .unwrap();
    ReopenedRecoveryArtifactAdmission::admit(report, &artifacts).unwrap()
}
