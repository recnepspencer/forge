use forge_store_recovery_physics::{
    PersistedRecoveryArtifactDenial, PersistedRecoveryArtifacts, RecoveryPersistedRecord,
    RecoveryProfileId, S4CheckpointManifestMaterialization, S4CheckpointPageImageMaterialization,
    S4PersistedRecoveryArtifactMaterialization, S4WalRedoFrameMaterialization,
};

pub fn deterministic_s4_recovery_artifacts() -> PersistedRecoveryArtifacts {
    materialized_artifacts("op-20")
}

pub fn s4_recovery_artifacts_with_operation_digest(
    operation_digest: &str,
) -> PersistedRecoveryArtifacts {
    materialized_artifacts(operation_digest)
}

pub fn reordered_s4_recovery_artifacts() -> PersistedRecoveryArtifacts {
    let artifacts = deterministic_s4_recovery_artifacts();
    let mut records = artifacts.records().to_vec();
    records.reverse();
    artifacts_from_records(records)
}

pub fn runtime_disagreement_s4_recovery_artifacts() -> PersistedRecoveryArtifacts {
    materialized_artifacts("corrupt-op-20")
}

pub fn runtime_state_mismatch_s4_recovery_artifacts() -> PersistedRecoveryArtifacts {
    materialized_artifacts("op-21")
}

pub fn incomplete_s4_recovery_artifacts() -> PersistedRecoveryArtifacts {
    let records = deterministic_s4_recovery_artifacts().records().to_vec();
    artifacts_from_records(vec![records[0].clone(), records[1].clone()])
}

pub fn duplicate_role_s4_recovery_artifacts() -> PersistedRecoveryArtifacts {
    let records = deterministic_s4_recovery_artifacts().records().to_vec();
    artifacts_from_records(vec![
        records[0].clone(),
        record("wal-tail-20-a", b"wal:lsn=20;page=2;op=op-20;idem=idem-20"),
        record("wal-tail-20-b", b"wal:lsn=21;page=2;op=op-21;idem=idem-21"),
        records[1].clone(),
    ])
}

pub fn malformed_s4_recovery_record()
-> Result<RecoveryPersistedRecord, PersistedRecoveryArtifactDenial> {
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

fn materialization(operation_digest: &str) -> S4PersistedRecoveryArtifactMaterialization {
    S4PersistedRecoveryArtifactMaterialization::new(
        "s4-format-v1",
        "strict-posix-fsync-dir-fsync",
        RecoveryProfileId::strict_s4(),
        S4CheckpointManifestMaterialization::new(
            "checkpoint-manifest",
            "alpha",
            19,
            "strict-test-profile",
            2,
            128,
            1,
            128,
            0,
        ),
        S4WalRedoFrameMaterialization::new("wal-tail-20", 20, 2, operation_digest, "idem-20"),
        S4CheckpointPageImageMaterialization::new("page-2", 2, 7, 19, "checkpoint-page"),
    )
}

fn artifacts_from_records(records: Vec<RecoveryPersistedRecord>) -> PersistedRecoveryArtifacts {
    PersistedRecoveryArtifacts::admit(
        "s4-format-v1",
        "strict-posix-fsync-dir-fsync",
        RecoveryProfileId::strict_s4(),
        records,
    )
    .expect("test support emits valid persisted recovery artifacts")
}

fn record(record_id: &str, bytes: &[u8]) -> RecoveryPersistedRecord {
    RecoveryPersistedRecord::from_persisted_bytes(record_id, bytes.to_vec())
        .expect("test support emits valid persisted records")
}
