use worth_store_recovery_physics::{
    CheckpointManifestBudgetMaterialization, CheckpointManifestMaterialization,
    CheckpointManifestRecoveryBasisMaterialization, CheckpointManifestSourceMaterialization,
    CheckpointPageImageMaterialization, PersistedRecoveryArtifactDenial,
    PersistedRecoveryArtifactMaterialization, PersistedRecoveryArtifacts, RecoveryPersistedRecord,
    RecoveryProfileId, WalRedoFrameMaterialization,
};

pub(super) const RECOVERY_ARTIFACT_FORMAT_VERSION: &str = "s4-format-v1";
pub(super) const RECOVERY_ARTIFACT_BACKEND_PROFILE: &str = "strict-posix-fsync-dir-fsync";

pub fn deterministic_recovery_artifacts() -> PersistedRecoveryArtifacts {
    materialized_artifacts("op-20")
}

pub fn recovery_artifacts_with_operation_digest(
    operation_digest: &str,
) -> PersistedRecoveryArtifacts {
    materialized_artifacts(operation_digest)
}

pub fn reordered_recovery_artifacts() -> PersistedRecoveryArtifacts {
    let artifacts = deterministic_recovery_artifacts();
    let mut records = artifacts.records().to_vec();
    records.reverse();
    artifacts_from_records(records)
}

pub fn runtime_disagreement_recovery_artifacts() -> PersistedRecoveryArtifacts {
    materialized_artifacts("corrupt-op-20")
}

pub fn runtime_state_mismatch_recovery_artifacts() -> PersistedRecoveryArtifacts {
    materialized_artifacts("op-21")
}

pub fn incomplete_recovery_artifacts() -> PersistedRecoveryArtifacts {
    let records = deterministic_recovery_artifacts().records().to_vec();
    artifacts_from_records(vec![records[0].clone(), records[1].clone()])
}

pub fn duplicate_role_recovery_artifacts() -> PersistedRecoveryArtifacts {
    let records = deterministic_recovery_artifacts().records().to_vec();
    artifacts_from_records(vec![
        records[0].clone(),
        record("wal-tail-20-a", b"wal:lsn=20;page=2;op=op-20;idem=idem-20"),
        record("wal-tail-20-b", b"wal:lsn=21;page=2;op=op-21;idem=idem-21"),
        records[1].clone(),
    ])
}

pub fn malformed_recovery_record(
) -> Result<RecoveryPersistedRecord, PersistedRecoveryArtifactDenial> {
    RecoveryPersistedRecord::from_persisted_bytes(
        "malformed-wal-tail",
        b"wal:lsn=not-a-number;page=2;op=op-20;idem=idem-20".to_vec(),
    )
}

fn materialized_artifacts(operation_digest: &str) -> PersistedRecoveryArtifacts {
    materialization(operation_digest)
        .materialize()
        .expect("test support emits valid materialized recovery artifacts")
}

fn materialization(operation_digest: &str) -> PersistedRecoveryArtifactMaterialization {
    PersistedRecoveryArtifactMaterialization::new(
        RECOVERY_ARTIFACT_FORMAT_VERSION,
        RECOVERY_ARTIFACT_BACKEND_PROFILE,
        RecoveryProfileId::strict_offline_recovery_artifacts(),
        CheckpointManifestMaterialization::new(
            "checkpoint-manifest",
            CheckpointManifestRecoveryBasisMaterialization::new(1, 1, 10, 20),
            CheckpointManifestSourceMaterialization::new("strict-test-profile", 2),
            CheckpointManifestBudgetMaterialization::new(128, 0, 128, 0),
        ),
        WalRedoFrameMaterialization::new("wal-tail-20", 20, 2, operation_digest, "idem-20"),
        CheckpointPageImageMaterialization::new("page-2", 2, 7, 19, "checkpoint-page"),
    )
}

fn artifacts_from_records(records: Vec<RecoveryPersistedRecord>) -> PersistedRecoveryArtifacts {
    PersistedRecoveryArtifacts::admit(
        RECOVERY_ARTIFACT_FORMAT_VERSION,
        RECOVERY_ARTIFACT_BACKEND_PROFILE,
        RecoveryProfileId::strict_offline_recovery_artifacts(),
        records,
    )
    .expect("test support emits valid persisted recovery artifacts")
}

fn record(record_id: &str, bytes: &[u8]) -> RecoveryPersistedRecord {
    RecoveryPersistedRecord::from_persisted_bytes(record_id, bytes.to_vec())
        .expect("test support emits valid persisted records")
}
