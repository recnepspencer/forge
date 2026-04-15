use crate::{
    failure::{StoreError, StoreErrorKind},
    media::{DurabilityBarrierClass, DurableBackendFamily, DurableMediaReport},
};
use rusqlite::{params, Connection, OptionalExtension, Transaction};
use std::path::PathBuf;

use super::{
    engine::{StateBackedStoreBackend, StatePersistence},
    records::StoreState,
};

#[derive(Debug)]
pub struct SqlitePersistence {
    connection: Connection,
}

pub type SqliteStoreBackend = StateBackedStoreBackend<SqlitePersistence>;

impl SqliteStoreBackend {
    pub fn open(path: PathBuf) -> Result<Self, StoreError> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let connection = Connection::open(path).map_err(sqlite_error)?;
        configure_connection(&connection)?;
        create_schema(&connection)?;
        StateBackedStoreBackend::open_with_persistence(SqlitePersistence { connection })
    }
}

impl StatePersistence for SqlitePersistence {
    fn load_state(&mut self) -> Result<StoreState, StoreError> {
        load_state(&self.connection)
    }

    fn persist_state(&mut self, state: &StoreState) -> Result<DurableMediaReport, StoreError> {
        persist_state(&mut self.connection, state)?;
        Ok(self.durable_media_report())
    }

    fn durable_media_report(&self) -> DurableMediaReport {
        DurableMediaReport::new(
            DurableBackendFamily::SqliteTransactional,
            DurabilityBarrierClass::TransactionalCommitDurable,
            DurabilityBarrierClass::TransactionalCommitDurable,
            DurabilityBarrierClass::TransactionalCommitDurable,
        )
    }
}

fn configure_connection(connection: &Connection) -> Result<(), StoreError> {
    connection
        .execute_batch(
            "
            PRAGMA foreign_keys = ON;
            PRAGMA journal_mode = WAL;
            PRAGMA synchronous = FULL;
            ",
        )
        .map_err(sqlite_error)?;
    Ok(())
}

fn create_schema(connection: &Connection) -> Result<(), StoreError> {
    connection
        .execute_batch(
            "
            CREATE TABLE IF NOT EXISTS store_meta (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS branch_records (
                branch_id TEXT PRIMARY KEY,
                created_from_branch TEXT,
                created_from_commit_id INTEGER,
                created_at_commit_sequence INTEGER
            );

            CREATE TABLE IF NOT EXISTS commit_envelopes (
                commit_id INTEGER PRIMARY KEY,
                branch_id TEXT NOT NULL,
                commit_sequence INTEGER NOT NULL UNIQUE,
                canonicalization_version INTEGER NOT NULL,
                envelope_payload TEXT NOT NULL,
                envelope_digest TEXT NOT NULL,
                FOREIGN KEY(branch_id) REFERENCES branch_records(branch_id)
            );

            CREATE TABLE IF NOT EXISTS commit_parent_records (
                commit_id INTEGER NOT NULL,
                parent_position INTEGER NOT NULL,
                parent_commit_id INTEGER NOT NULL,
                PRIMARY KEY(commit_id, parent_position),
                FOREIGN KEY(commit_id) REFERENCES commit_envelopes(commit_id),
                FOREIGN KEY(parent_commit_id) REFERENCES commit_envelopes(commit_id)
            );

            CREATE TABLE IF NOT EXISTS branch_head_records (
                branch_id TEXT PRIMARY KEY,
                head_commit_id INTEGER,
                head_commit_digest TEXT,
                head_update_sequence INTEGER NOT NULL,
                FOREIGN KEY(branch_id) REFERENCES branch_records(branch_id),
                FOREIGN KEY(head_commit_id) REFERENCES commit_envelopes(commit_id)
            );

            CREATE TABLE IF NOT EXISTS authoritative_artifact_digests (
                artifact_family TEXT NOT NULL,
                artifact_id TEXT NOT NULL,
                canonicalization_version INTEGER NOT NULL,
                digest_algorithm TEXT NOT NULL,
                artifact_digest TEXT NOT NULL,
                PRIMARY KEY(artifact_family, artifact_id, canonicalization_version)
            );

            CREATE TABLE IF NOT EXISTS embedded_checkpoint_records (
                checkpoint_id TEXT PRIMARY KEY,
                source_runtime_id TEXT NOT NULL,
                basis_branch_id TEXT,
                basis_commit_id INTEGER,
                classification TEXT NOT NULL,
                contained_commit_ids_payload TEXT NOT NULL,
                metadata_payload TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS snapshot_basis_records (
                snapshot_id INTEGER PRIMARY KEY,
                snapshot_family_version INTEGER NOT NULL,
                snapshot_basis_version INTEGER NOT NULL,
                snapshot_image_format_version INTEGER NOT NULL,
                snapshot_branch_id TEXT NOT NULL,
                snapshot_frontier_commit_id INTEGER NOT NULL,
                snapshot_history_range_payload TEXT NOT NULL,
                snapshot_canonicalization_version INTEGER NOT NULL,
                snapshot_authority_digest TEXT NOT NULL,
                snapshot_image_digest TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS snapshot_image_records (
                snapshot_id INTEGER PRIMARY KEY,
                image_payload TEXT NOT NULL,
                FOREIGN KEY(snapshot_id) REFERENCES snapshot_basis_records(snapshot_id)
            );

            CREATE TABLE IF NOT EXISTS wal_records (
                wal_sequence INTEGER PRIMARY KEY,
                family TEXT NOT NULL,
                durable_mutation_id INTEGER NOT NULL,
                runtime_session_id TEXT NOT NULL,
                wal_version INTEGER NOT NULL,
                record_digest TEXT NOT NULL,
                payload_json TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_commit_envelopes_branch_sequence
            ON commit_envelopes(branch_id, commit_sequence);

            CREATE INDEX IF NOT EXISTS idx_commit_parent_records_commit_position
            ON commit_parent_records(commit_id, parent_position);

            CREATE INDEX IF NOT EXISTS idx_authoritative_artifact_digests_family_id
            ON authoritative_artifact_digests(artifact_family, artifact_id);

            CREATE INDEX IF NOT EXISTS idx_embedded_checkpoint_records_basis_commit
            ON embedded_checkpoint_records(basis_commit_id);

            CREATE INDEX IF NOT EXISTS idx_snapshot_basis_branch_frontier
            ON snapshot_basis_records(snapshot_branch_id, snapshot_frontier_commit_id);

            CREATE INDEX IF NOT EXISTS idx_wal_records_mutation_sequence
            ON wal_records(durable_mutation_id, wal_sequence);
            ",
        )
        .map_err(sqlite_error)?;
    Ok(())
}

fn load_state(connection: &Connection) -> Result<StoreState, StoreError> {
    let mut state = StoreState::default();
    state.canonicalization_version = load_meta_u32(connection, "canonicalization_version")?
        .unwrap_or(state.canonicalization_version);

    {
        let mut statement = connection
            .prepare(
                "
                SELECT wal_sequence, family, durable_mutation_id, runtime_session_id, wal_version, record_digest, payload_json
                FROM wal_records
                ORDER BY wal_sequence
                ",
            )
            .map_err(sqlite_error)?;
        let rows =
            statement
                .query_map([], |row| {
                    let payload_json: String = row.get(6)?;
                    let payload = serde_json::from_str(&payload_json).map_err(|error| {
                        rusqlite::Error::FromSqlConversionFailure(
                            6,
                            rusqlite::types::Type::Text,
                            Box::new(error),
                        )
                    })?;
                    let family = match row.get::<_, String>(1)?.as_str() {
                        "DurableMutationIntent" => {
                            crate::wal::WalRecordFamily::DurableMutationIntent
                        }
                        "HostedRuntimeCommitResult" => {
                            crate::wal::WalRecordFamily::HostedRuntimeCommitResult
                        }
                        "DurablePublicationProgress" => {
                            crate::wal::WalRecordFamily::DurablePublicationProgress
                        }
                        "RecoveryDecision" => crate::wal::WalRecordFamily::RecoveryDecision,
                        other => {
                            return Err(rusqlite::Error::FromSqlConversionFailure(
                                1,
                                rusqlite::types::Type::Text,
                                Box::new(std::io::Error::other(format!(
                                    "unknown wal record family `{other}`"
                                ))),
                            ));
                        }
                    };
                    Ok(crate::wal::WalRecord {
                        wal_sequence: row.get::<_, i64>(0)? as u64,
                        family,
                        durable_mutation_id: crate::wal::DurableMutationId(
                            row.get::<_, i64>(2)? as u64
                        ),
                        runtime_session_id: row.get(3)?,
                        wal_version: row.get::<_, i64>(4)? as u32,
                        record_digest: row.get(5)?,
                        payload,
                    })
                })
                .map_err(sqlite_error)?;
        for row in rows {
            let record = row.map_err(sqlite_error)?;
            state.wal_records.insert(record.wal_sequence, record);
        }
    }

    {
        let mut statement = connection
            .prepare(
                "
                SELECT branch_id, created_from_branch, created_from_commit_id, created_at_commit_sequence
                FROM branch_records
                ORDER BY branch_id
                ",
            )
            .map_err(sqlite_error)?;
        let rows = statement
            .query_map([], |row| {
                Ok(super::records::BranchRecord {
                    branch_id: forge_relational::facade::history::BranchId(
                        row.get::<_, String>(0)?,
                    ),
                    created_from_branch: row
                        .get::<_, Option<String>>(1)?
                        .map(forge_relational::facade::history::BranchId),
                    created_from_commit_id: row
                        .get::<_, Option<i64>>(2)?
                        .map(|value| forge_relational::facade::history::CommitId(value as u64)),
                    created_at_commit_sequence: row
                        .get::<_, Option<i64>>(3)?
                        .map(|value| value as u64),
                })
            })
            .map_err(sqlite_error)?;
        for row in rows {
            let record = row.map_err(sqlite_error)?;
            state
                .branch_records
                .insert(record.branch_id.0.clone(), record);
        }
    }

    {
        let mut statement = connection
            .prepare(
                "
                SELECT branch_id, head_commit_id, head_commit_digest, head_update_sequence
                FROM branch_head_records
                ORDER BY branch_id
                ",
            )
            .map_err(sqlite_error)?;
        let rows = statement
            .query_map([], |row| {
                Ok(super::records::BranchHeadRecord {
                    branch_id: forge_relational::facade::history::BranchId(
                        row.get::<_, String>(0)?,
                    ),
                    head_commit_id: row
                        .get::<_, Option<i64>>(1)?
                        .map(|value| forge_relational::facade::history::CommitId(value as u64)),
                    head_commit_digest: row.get(2)?,
                    head_update_sequence: row.get::<_, i64>(3)? as u64,
                })
            })
            .map_err(sqlite_error)?;
        for row in rows {
            let record = row.map_err(sqlite_error)?;
            state
                .branch_head_records
                .insert(record.branch_id.0.clone(), record);
        }
    }

    {
        let mut statement = connection
            .prepare(
                "
                SELECT commit_id, envelope_payload, envelope_digest, canonicalization_version, commit_sequence
                FROM commit_envelopes
                ORDER BY commit_sequence
                ",
            )
            .map_err(sqlite_error)?;
        let rows = statement
            .query_map([], |row| {
                let payload: String = row.get(1)?;
                Ok(super::records::StoredCommitEnvelope {
                    envelope: serde_json::from_str(&payload).map_err(|error| {
                        rusqlite::Error::FromSqlConversionFailure(
                            1,
                            rusqlite::types::Type::Text,
                            Box::new(error),
                        )
                    })?,
                    envelope_digest: row.get(2)?,
                    canonicalization_version: row.get::<_, i64>(3)? as u32,
                    commit_sequence: row.get::<_, i64>(4)? as u64,
                })
            })
            .map_err(sqlite_error)?;
        for row in rows {
            let record = row.map_err(sqlite_error)?;
            state
                .commit_envelopes
                .insert(record.envelope.commit.commit_id.0, record);
        }
    }

    {
        let mut statement = connection
            .prepare(
                "
                SELECT commit_id, parent_position, parent_commit_id
                FROM commit_parent_records
                ORDER BY commit_id, parent_position
                ",
            )
            .map_err(sqlite_error)?;
        let rows = statement
            .query_map([], |row| {
                Ok(super::records::CommitParentRecord {
                    commit_id: forge_relational::facade::history::CommitId(
                        row.get::<_, i64>(0)? as u64
                    ),
                    parent_position: row.get::<_, i64>(1)? as usize,
                    parent_commit_id: forge_relational::facade::history::CommitId(
                        row.get::<_, i64>(2)? as u64,
                    ),
                })
            })
            .map_err(sqlite_error)?;
        for row in rows {
            let record = row.map_err(sqlite_error)?;
            let artifact_id =
                super::integrity::parent_artifact_id(record.commit_id, record.parent_position);
            state.commit_parent_records.insert(artifact_id, record);
        }
    }

    {
        let mut statement = connection
            .prepare(
                "
                SELECT artifact_family, artifact_id, canonicalization_version, digest_algorithm, artifact_digest
                FROM authoritative_artifact_digests
                ORDER BY artifact_family, artifact_id, canonicalization_version
                ",
            )
            .map_err(sqlite_error)?;
        let rows = statement
            .query_map([], |row| {
                let family = match row.get::<_, String>(0)?.as_str() {
                    "BranchRecord" => super::records::AuthoritativeArtifactFamily::BranchRecord,
                    "BranchHeadRecord" => {
                        super::records::AuthoritativeArtifactFamily::BranchHeadRecord
                    }
                    "CommitEnvelope" => super::records::AuthoritativeArtifactFamily::CommitEnvelope,
                    "CommitParentRecord" => {
                        super::records::AuthoritativeArtifactFamily::CommitParentRecord
                    }
                    other => {
                        return Err(rusqlite::Error::FromSqlConversionFailure(
                            0,
                            rusqlite::types::Type::Text,
                            Box::new(std::io::Error::other(format!(
                                "unknown artifact family `{other}`"
                            ))),
                        ));
                    }
                };
                Ok(super::records::AuthoritativeArtifactDigestRecord {
                    artifact_family: family,
                    artifact_id: row.get(1)?,
                    canonicalization_version: row.get::<_, i64>(2)? as u32,
                    digest_algorithm: row.get(3)?,
                    artifact_digest: row.get(4)?,
                })
            })
            .map_err(sqlite_error)?;
        for row in rows {
            let record = row.map_err(sqlite_error)?;
            let key = format!(
                "{:?}:{}:v{}",
                record.artifact_family, record.artifact_id, record.canonicalization_version
            );
            state.authoritative_artifact_digests.insert(key, record);
        }
    }

    {
        let mut statement = connection
            .prepare(
                "
                SELECT checkpoint_id, source_runtime_id, basis_branch_id, basis_commit_id, classification,
                       contained_commit_ids_payload, metadata_payload
                FROM embedded_checkpoint_records
                ORDER BY checkpoint_id
                ",
            )
            .map_err(sqlite_error)?;
        let rows = statement
            .query_map([], |row| {
                let classification = match row.get::<_, String>(4)?.as_str() {
                    "DerivedDurable" => {
                        super::records::EmbeddedCheckpointClassification::DerivedDurable
                    }
                    "Ephemeral" => super::records::EmbeddedCheckpointClassification::Ephemeral,
                    other => {
                        return Err(rusqlite::Error::FromSqlConversionFailure(
                            4,
                            rusqlite::types::Type::Text,
                            Box::new(std::io::Error::other(format!(
                                "unknown embedded checkpoint classification `{other}`"
                            ))),
                        ));
                    }
                };
                let contained_commit_ids_payload: String = row.get(5)?;
                let contained_commit_ids = serde_json::from_str::<Vec<u64>>(
                    &contained_commit_ids_payload,
                )
                .map_err(|error| {
                    rusqlite::Error::FromSqlConversionFailure(
                        5,
                        rusqlite::types::Type::Text,
                        Box::new(error),
                    )
                })?;
                let metadata_payload: String = row.get(6)?;
                let metadata = serde_json::from_str(&metadata_payload).map_err(|error| {
                    rusqlite::Error::FromSqlConversionFailure(
                        6,
                        rusqlite::types::Type::Text,
                        Box::new(error),
                    )
                })?;
                Ok(super::records::EmbeddedCheckpointRecord {
                    checkpoint_id: row.get(0)?,
                    source_runtime_id: row.get(1)?,
                    basis_branch_id: row
                        .get::<_, Option<String>>(2)?
                        .map(forge_relational::facade::history::BranchId),
                    basis_commit_id: row
                        .get::<_, Option<i64>>(3)?
                        .map(|value| forge_relational::facade::history::CommitId(value as u64)),
                    classification,
                    contained_commit_ids: contained_commit_ids
                        .into_iter()
                        .map(forge_relational::facade::history::CommitId)
                        .collect(),
                    metadata,
                })
            })
            .map_err(sqlite_error)?;
        for row in rows {
            let record = row.map_err(sqlite_error)?;
            state
                .embedded_checkpoint_records
                .insert(record.checkpoint_id.clone(), record);
        }
    }

    {
        let mut statement = connection
            .prepare(
                "
                SELECT snapshot_id, snapshot_family_version, snapshot_basis_version, snapshot_image_format_version,
                       snapshot_branch_id, snapshot_frontier_commit_id, snapshot_history_range_payload,
                       snapshot_canonicalization_version, snapshot_authority_digest, snapshot_image_digest
                FROM snapshot_basis_records
                ORDER BY snapshot_id
                ",
            )
            .map_err(sqlite_error)?;
        let rows = statement
            .query_map([], |row| {
                let history_range_payload: String = row.get(6)?;
                let history_range = serde_json::from_str::<Vec<u64>>(&history_range_payload)
                    .map_err(|error| {
                        rusqlite::Error::FromSqlConversionFailure(
                            6,
                            rusqlite::types::Type::Text,
                            Box::new(error),
                        )
                    })?;
                Ok(super::records::SnapshotBasisRecord {
                    snapshot_id: crate::snapshot::SnapshotId(row.get::<_, i64>(0)? as u64),
                    snapshot_family_version: row.get::<_, i64>(1)? as u32,
                    snapshot_basis_version: row.get::<_, i64>(2)? as u32,
                    snapshot_image_format_version: row.get::<_, i64>(3)? as u32,
                    snapshot_branch_id: forge_relational::facade::history::BranchId(
                        row.get::<_, String>(4)?,
                    ),
                    snapshot_frontier_commit_id: forge_relational::facade::history::CommitId(
                        row.get::<_, i64>(5)? as u64,
                    ),
                    snapshot_history_range: history_range
                        .into_iter()
                        .map(forge_relational::facade::history::CommitId)
                        .collect(),
                    snapshot_canonicalization_version: row.get::<_, i64>(7)? as u32,
                    snapshot_authority_digest: row.get(8)?,
                    snapshot_image_digest: row.get(9)?,
                })
            })
            .map_err(sqlite_error)?;
        for row in rows {
            let record = row.map_err(sqlite_error)?;
            state
                .snapshot_basis_records
                .insert(record.snapshot_id.0, record);
        }
    }

    {
        let mut statement = connection
            .prepare(
                "
                SELECT snapshot_id, image_payload
                FROM snapshot_image_records
                ORDER BY snapshot_id
                ",
            )
            .map_err(sqlite_error)?;
        let rows = statement
            .query_map([], |row| {
                let image_payload: String = row.get(1)?;
                Ok(super::records::SnapshotImageRecord {
                    snapshot_id: crate::snapshot::SnapshotId(row.get::<_, i64>(0)? as u64),
                    image: serde_json::from_str(&image_payload).map_err(|error| {
                        rusqlite::Error::FromSqlConversionFailure(
                            1,
                            rusqlite::types::Type::Text,
                            Box::new(error),
                        )
                    })?,
                })
            })
            .map_err(sqlite_error)?;
        for row in rows {
            let record = row.map_err(sqlite_error)?;
            state
                .snapshot_image_records
                .insert(record.snapshot_id.0, record);
        }
    }

    state.next_commit_sequence =
        load_meta_u64(connection, "next_commit_sequence")?.unwrap_or_else(|| {
            state
                .commit_envelopes
                .values()
                .map(|record| record.commit_sequence)
                .max()
                .map(|value| value + 1)
                .unwrap_or(1)
        });
    state.next_head_update_sequence = load_meta_u64(connection, "next_head_update_sequence")?
        .unwrap_or_else(|| {
            state
                .branch_head_records
                .values()
                .map(|record| record.head_update_sequence)
                .max()
                .map(|value| value + 1)
                .unwrap_or(1)
        });
    state.next_durable_mutation_id = load_meta_u64(connection, "next_durable_mutation_id")?
        .unwrap_or_else(|| {
            state
                .wal_records
                .values()
                .map(|record| record.durable_mutation_id.0)
                .max()
                .map(|value| value + 1)
                .unwrap_or(1)
        });
    state.next_snapshot_id = load_meta_u64(connection, "next_snapshot_id")?.unwrap_or_else(|| {
        state
            .snapshot_basis_records
            .keys()
            .max()
            .map(|value| value + 1)
            .unwrap_or(1)
    });
    state.next_wal_sequence =
        load_meta_u64(connection, "next_wal_sequence")?.unwrap_or_else(|| {
            state
                .wal_records
                .keys()
                .max()
                .map(|value| value + 1)
                .unwrap_or(1)
        });

    Ok(state)
}

fn persist_state(connection: &mut Connection, state: &StoreState) -> Result<(), StoreError> {
    let transaction = connection.transaction().map_err(sqlite_error)?;
    clear_tables(&transaction)?;
    persist_meta(&transaction, state)?;
    persist_branch_records(&transaction, state)?;
    persist_commit_envelopes(&transaction, state)?;
    persist_commit_parent_records(&transaction, state)?;
    persist_branch_head_records(&transaction, state)?;
    persist_digest_records(&transaction, state)?;
    persist_embedded_checkpoint_records(&transaction, state)?;
    persist_snapshot_basis_records(&transaction, state)?;
    persist_snapshot_image_records(&transaction, state)?;
    persist_wal_records(&transaction, state)?;
    transaction.commit().map_err(sqlite_error)?;
    Ok(())
}

fn clear_tables(transaction: &Transaction<'_>) -> Result<(), StoreError> {
    transaction
        .execute_batch(
            "
            DELETE FROM authoritative_artifact_digests;
            DELETE FROM branch_head_records;
            DELETE FROM commit_parent_records;
            DELETE FROM commit_envelopes;
            DELETE FROM branch_records;
            DELETE FROM embedded_checkpoint_records;
            DELETE FROM snapshot_image_records;
            DELETE FROM snapshot_basis_records;
            DELETE FROM wal_records;
            DELETE FROM store_meta;
            ",
        )
        .map_err(sqlite_error)?;
    Ok(())
}

fn persist_meta(transaction: &Transaction<'_>, state: &StoreState) -> Result<(), StoreError> {
    persist_meta_value(
        transaction,
        "canonicalization_version",
        state.canonicalization_version.to_string(),
    )?;
    persist_meta_value(
        transaction,
        "next_commit_sequence",
        state.next_commit_sequence.to_string(),
    )?;
    persist_meta_value(
        transaction,
        "next_head_update_sequence",
        state.next_head_update_sequence.to_string(),
    )?;
    persist_meta_value(
        transaction,
        "next_durable_mutation_id",
        state.next_durable_mutation_id.to_string(),
    )?;
    persist_meta_value(
        transaction,
        "next_snapshot_id",
        state.next_snapshot_id.to_string(),
    )?;
    persist_meta_value(
        transaction,
        "next_wal_sequence",
        state.next_wal_sequence.to_string(),
    )?;
    Ok(())
}

fn persist_meta_value(
    transaction: &Transaction<'_>,
    key: &str,
    value: String,
) -> Result<(), StoreError> {
    transaction
        .execute(
            "INSERT INTO store_meta(key, value) VALUES (?1, ?2)",
            params![key, value],
        )
        .map_err(sqlite_error)?;
    Ok(())
}

fn persist_branch_records(
    transaction: &Transaction<'_>,
    state: &StoreState,
) -> Result<(), StoreError> {
    for record in state.branch_records.values() {
        transaction
            .execute(
                "
                INSERT INTO branch_records(
                    branch_id,
                    created_from_branch,
                    created_from_commit_id,
                    created_at_commit_sequence
                ) VALUES (?1, ?2, ?3, ?4)
                ",
                params![
                    record.branch_id.0,
                    record
                        .created_from_branch
                        .as_ref()
                        .map(|value| value.0.clone()),
                    record.created_from_commit_id.map(as_i64),
                    record.created_at_commit_sequence.map(as_i64_u64),
                ],
            )
            .map_err(sqlite_error)?;
    }
    Ok(())
}

fn persist_commit_envelopes(
    transaction: &Transaction<'_>,
    state: &StoreState,
) -> Result<(), StoreError> {
    for record in state.commit_envelopes.values() {
        let payload = serde_json::to_string(&record.envelope)?;
        transaction
            .execute(
                "
                INSERT INTO commit_envelopes(
                    commit_id,
                    branch_id,
                    commit_sequence,
                    canonicalization_version,
                    envelope_payload,
                    envelope_digest
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                ",
                params![
                    as_i64(record.envelope.commit.commit_id),
                    record.envelope.branch_context.0,
                    as_i64_u64(record.commit_sequence),
                    record.canonicalization_version as i64,
                    payload,
                    record.envelope_digest,
                ],
            )
            .map_err(sqlite_error)?;
    }
    Ok(())
}

fn persist_commit_parent_records(
    transaction: &Transaction<'_>,
    state: &StoreState,
) -> Result<(), StoreError> {
    for record in state.commit_parent_records.values() {
        transaction
            .execute(
                "
                INSERT INTO commit_parent_records(
                    commit_id,
                    parent_position,
                    parent_commit_id
                ) VALUES (?1, ?2, ?3)
                ",
                params![
                    as_i64(record.commit_id),
                    record.parent_position as i64,
                    as_i64(record.parent_commit_id),
                ],
            )
            .map_err(sqlite_error)?;
    }
    Ok(())
}

fn persist_branch_head_records(
    transaction: &Transaction<'_>,
    state: &StoreState,
) -> Result<(), StoreError> {
    for record in state.branch_head_records.values() {
        transaction
            .execute(
                "
                INSERT INTO branch_head_records(
                    branch_id,
                    head_commit_id,
                    head_commit_digest,
                    head_update_sequence
                ) VALUES (?1, ?2, ?3, ?4)
                ",
                params![
                    record.branch_id.0,
                    record.head_commit_id.map(as_i64),
                    record.head_commit_digest,
                    as_i64_u64(record.head_update_sequence),
                ],
            )
            .map_err(sqlite_error)?;
    }
    Ok(())
}

fn persist_digest_records(
    transaction: &Transaction<'_>,
    state: &StoreState,
) -> Result<(), StoreError> {
    for record in state.authoritative_artifact_digests.values() {
        transaction
            .execute(
                "
                INSERT INTO authoritative_artifact_digests(
                    artifact_family,
                    artifact_id,
                    canonicalization_version,
                    digest_algorithm,
                    artifact_digest
                ) VALUES (?1, ?2, ?3, ?4, ?5)
                ",
                params![
                    format!("{:?}", record.artifact_family),
                    record.artifact_id,
                    record.canonicalization_version as i64,
                    record.digest_algorithm,
                    record.artifact_digest,
                ],
            )
            .map_err(sqlite_error)?;
    }
    Ok(())
}

fn persist_embedded_checkpoint_records(
    transaction: &Transaction<'_>,
    state: &StoreState,
) -> Result<(), StoreError> {
    for record in state.embedded_checkpoint_records.values() {
        let contained_commit_ids_payload = serde_json::to_string(
            &record
                .contained_commit_ids
                .iter()
                .map(|commit_id| commit_id.0)
                .collect::<Vec<_>>(),
        )?;
        let metadata_payload = serde_json::to_string(&record.metadata)?;
        transaction
            .execute(
                "
                INSERT INTO embedded_checkpoint_records(
                    checkpoint_id,
                    source_runtime_id,
                    basis_branch_id,
                    basis_commit_id,
                    classification,
                    contained_commit_ids_payload,
                    metadata_payload
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
                ",
                params![
                    record.checkpoint_id,
                    record.source_runtime_id,
                    record.basis_branch_id.as_ref().map(|value| value.0.clone()),
                    record.basis_commit_id.map(as_i64),
                    format!("{:?}", record.classification),
                    contained_commit_ids_payload,
                    metadata_payload,
                ],
            )
            .map_err(sqlite_error)?;
    }
    Ok(())
}

fn persist_snapshot_basis_records(
    transaction: &Transaction<'_>,
    state: &StoreState,
) -> Result<(), StoreError> {
    for record in state.snapshot_basis_records.values() {
        let history_range_payload = serde_json::to_string(
            &record
                .snapshot_history_range
                .iter()
                .map(|commit_id| commit_id.0)
                .collect::<Vec<_>>(),
        )?;
        transaction
            .execute(
                "
                INSERT INTO snapshot_basis_records(
                    snapshot_id,
                    snapshot_family_version,
                    snapshot_basis_version,
                    snapshot_image_format_version,
                    snapshot_branch_id,
                    snapshot_frontier_commit_id,
                    snapshot_history_range_payload,
                    snapshot_canonicalization_version,
                    snapshot_authority_digest,
                    snapshot_image_digest
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
                ",
                params![
                    as_i64_u64(record.snapshot_id.0),
                    record.snapshot_family_version as i64,
                    record.snapshot_basis_version as i64,
                    record.snapshot_image_format_version as i64,
                    record.snapshot_branch_id.0,
                    as_i64(record.snapshot_frontier_commit_id),
                    history_range_payload,
                    record.snapshot_canonicalization_version as i64,
                    record.snapshot_authority_digest,
                    record.snapshot_image_digest,
                ],
            )
            .map_err(sqlite_error)?;
    }
    Ok(())
}

fn persist_snapshot_image_records(
    transaction: &Transaction<'_>,
    state: &StoreState,
) -> Result<(), StoreError> {
    for record in state.snapshot_image_records.values() {
        let image_payload = serde_json::to_string(&record.image)?;
        transaction
            .execute(
                "
                INSERT INTO snapshot_image_records(
                    snapshot_id,
                    image_payload
                ) VALUES (?1, ?2)
                ",
                params![as_i64_u64(record.snapshot_id.0), image_payload],
            )
            .map_err(sqlite_error)?;
    }
    Ok(())
}

fn persist_wal_records(
    transaction: &Transaction<'_>,
    state: &StoreState,
) -> Result<(), StoreError> {
    for record in state.wal_records.values() {
        let payload_json = serde_json::to_string(&record.payload)?;
        transaction
            .execute(
                "
                INSERT INTO wal_records(
                    wal_sequence,
                    family,
                    durable_mutation_id,
                    runtime_session_id,
                    wal_version,
                    record_digest,
                    payload_json
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
                ",
                params![
                    as_i64_u64(record.wal_sequence),
                    format!("{:?}", record.family),
                    as_i64_u64(record.durable_mutation_id.0),
                    record.runtime_session_id,
                    record.wal_version as i64,
                    record.record_digest,
                    payload_json,
                ],
            )
            .map_err(sqlite_error)?;
    }
    Ok(())
}

fn load_meta_u64(connection: &Connection, key: &str) -> Result<Option<u64>, StoreError> {
    connection
        .query_row(
            "SELECT value FROM store_meta WHERE key = ?1",
            params![key],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(sqlite_error)?
        .map(|value| {
            value.parse::<u64>().map_err(|error| {
                StoreError::backend_integrity(format!("invalid u64 store_meta `{key}`: {error}"))
            })
        })
        .transpose()
}

fn load_meta_u32(connection: &Connection, key: &str) -> Result<Option<u32>, StoreError> {
    load_meta_u64(connection, key).map(|value| value.map(|value| value as u32))
}

fn as_i64(commit_id: forge_relational::facade::history::CommitId) -> i64 {
    commit_id.0 as i64
}

fn as_i64_u64(value: u64) -> i64 {
    value as i64
}

fn sqlite_error(error: rusqlite::Error) -> StoreError {
    match error {
        rusqlite::Error::SqliteFailure(code, message) => {
            if code.code == rusqlite::ErrorCode::ConstraintViolation {
                StoreError::new(
                    StoreErrorKind::DuplicateArtifactIdentity,
                    format!(
                        "sqlite constraint rejected authoritative write: {}",
                        message.unwrap_or_else(|| code.to_string())
                    ),
                )
            } else {
                StoreError::new(
                    StoreErrorKind::Io,
                    format!(
                        "sqlite backend failure {}: {}",
                        code,
                        message.unwrap_or_else(|| code.to_string())
                    ),
                )
            }
        }
        rusqlite::Error::FromSqlConversionFailure(_, _, _)
        | rusqlite::Error::IntegralValueOutOfRange(_, _)
        | rusqlite::Error::InvalidColumnType(_, _, _) => {
            StoreError::backend_integrity(format!("sqlite backend integrity failure: {error}"))
        }
        _ => StoreError::new(StoreErrorKind::Io, format!("sqlite backend error: {error}")),
    }
}
