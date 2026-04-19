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

    pub fn open_for_durable_recovery(path: PathBuf) -> Result<Self, StoreError> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let connection = Connection::open(path).map_err(sqlite_error)?;
        configure_connection(&connection)?;
        create_schema(&connection)?;
        StateBackedStoreBackend::open_with_persistence_for_durable_recovery(SqlitePersistence {
            connection,
        })
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

            CREATE TABLE IF NOT EXISTS commit_support_summaries (
                commit_id INTEGER PRIMARY KEY,
                branch_id TEXT NOT NULL,
                schema_support_artifact_id TEXT,
                lineage_support_artifact_id TEXT,
                milestone_6_published_layout_request_artifact_ids_payload TEXT NOT NULL,
                emitted_schema_artifact INTEGER NOT NULL,
                emitted_lineage_artifact INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS schema_support_records (
                artifact_id TEXT PRIMARY KEY,
                commit_id INTEGER NOT NULL UNIQUE,
                branch_id TEXT NOT NULL,
                schema_version_id INTEGER NOT NULL,
                descriptor_semantics_version INTEGER NOT NULL,
                schema_transition_payload TEXT,
                schema_continuation_descriptor_payload TEXT,
                schema_reconciliation_descriptor_payload TEXT,
                FOREIGN KEY(commit_id) REFERENCES commit_envelopes(commit_id)
            );

            CREATE TABLE IF NOT EXISTS lineage_support_records (
                artifact_id TEXT PRIMARY KEY,
                commit_id INTEGER NOT NULL UNIQUE,
                branch_id TEXT NOT NULL,
                lineage_event_ids_payload TEXT NOT NULL,
                lineage_events_payload TEXT NOT NULL,
                lineage_digest_basis_payload TEXT NOT NULL,
                event_batch_digest_basis_payload TEXT NOT NULL,
                decision_log_digest_basis_payload TEXT NOT NULL,
                lineage_artifact_counters_payload TEXT NOT NULL,
                FOREIGN KEY(commit_id) REFERENCES commit_envelopes(commit_id)
            );

            CREATE TABLE IF NOT EXISTS durable_cursor_identity_records (
                artifact_id TEXT PRIMARY KEY,
                cursor_id TEXT NOT NULL UNIQUE,
                subscriber_id TEXT NOT NULL,
                branch_id TEXT NOT NULL,
                feed_shape_id TEXT NOT NULL,
                schema_interpretation_id TEXT NOT NULL,
                cursor_semantics_version INTEGER NOT NULL,
                latest_checkpoint_sequence INTEGER NOT NULL,
                latest_basis_commit_id INTEGER NOT NULL,
                latest_schema_support_artifact_id TEXT,
                FOREIGN KEY(latest_basis_commit_id) REFERENCES commit_envelopes(commit_id)
            );

            CREATE TABLE IF NOT EXISTS subscriber_checkpoint_records (
                artifact_id TEXT PRIMARY KEY,
                cursor_id TEXT NOT NULL,
                subscriber_id TEXT NOT NULL,
                branch_id TEXT NOT NULL,
                feed_shape_id TEXT NOT NULL,
                schema_interpretation_id TEXT NOT NULL,
                cursor_semantics_version INTEGER NOT NULL,
                checkpoint_sequence INTEGER NOT NULL,
                basis_commit_id INTEGER NOT NULL,
                schema_support_artifact_id TEXT,
                UNIQUE(cursor_id, checkpoint_sequence),
                FOREIGN KEY(basis_commit_id) REFERENCES commit_envelopes(commit_id)
            );

            CREATE TABLE IF NOT EXISTS branch_shared_base_records (
                branch_id TEXT PRIMARY KEY,
                source_branch_id TEXT NOT NULL,
                source_frontier_commit_id INTEGER,
                delta_family_version INTEGER NOT NULL,
                authority_basis_digest TEXT NOT NULL,
                FOREIGN KEY(branch_id) REFERENCES branch_records(branch_id)
            );

            CREATE TABLE IF NOT EXISTS branch_delta_layer_records (
                branch_delta_layer_id INTEGER PRIMARY KEY,
                branch_id TEXT NOT NULL,
                base_frontier_commit_id INTEGER,
                target_frontier_commit_id INTEGER NOT NULL,
                commit_ids_payload TEXT NOT NULL,
                delta_family_version INTEGER NOT NULL,
                authority_basis_digest TEXT NOT NULL,
                artifacts_payload TEXT NOT NULL,
                replacement_of_layer_ids_payload TEXT NOT NULL,
                replacement_lineage_proof_payload TEXT NOT NULL,
                FOREIGN KEY(branch_id) REFERENCES branch_records(branch_id)
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

            CREATE TABLE IF NOT EXISTS milestone_6_layout_materialization_records (
                artifact_id TEXT PRIMARY KEY,
                payload_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS milestone_6_commit_coupled_layout_seed_records (
                artifact_id TEXT PRIMARY KEY,
                branch_id TEXT NOT NULL,
                frontier_commit_id INTEGER NOT NULL,
                scope_class TEXT NOT NULL,
                payload_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS milestone_6_scope_slice_membership_records (
                artifact_id TEXT PRIMARY KEY,
                branch_id TEXT NOT NULL,
                frontier_commit_id INTEGER NOT NULL,
                scope_class TEXT NOT NULL,
                projection_digest TEXT NOT NULL,
                payload_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS milestone_6_chunk_membership_records (
                artifact_id TEXT PRIMARY KEY,
                physical_chunk_id TEXT NOT NULL,
                chunk_shape_version INTEGER NOT NULL,
                determinism_digest TEXT NOT NULL,
                payload_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS milestone_6_structural_block_records (
                artifact_id TEXT PRIMARY KEY,
                structural_block_id TEXT NOT NULL,
                scope_class TEXT NOT NULL,
                equivalence_contract_version INTEGER NOT NULL,
                supporting_layout_materialization_count INTEGER NOT NULL,
                payload_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS bulk_program_identity_records (
                artifact_id TEXT PRIMARY KEY,
                program_id TEXT NOT NULL UNIQUE,
                kind TEXT NOT NULL,
                payload_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS frozen_bulk_manifest_records (
                artifact_id TEXT PRIMARY KEY,
                program_id TEXT NOT NULL,
                manifest_digest TEXT NOT NULL,
                payload_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS frozen_transform_basis_records (
                artifact_id TEXT PRIMARY KEY,
                program_id TEXT NOT NULL,
                basis_digest TEXT NOT NULL,
                payload_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS frozen_transform_partition_records (
                artifact_id TEXT PRIMARY KEY,
                program_id TEXT NOT NULL,
                partition_digest TEXT NOT NULL,
                payload_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS bulk_deterministic_plan_records (
                artifact_id TEXT PRIMARY KEY,
                program_id TEXT NOT NULL,
                plan_id TEXT NOT NULL,
                payload_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS bulk_progress_checkpoint_records (
                artifact_id TEXT PRIMARY KEY,
                program_id TEXT NOT NULL,
                plan_id TEXT NOT NULL,
                checkpoint_sequence INTEGER NOT NULL,
                payload_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS bulk_chunk_witness_records (
                artifact_id TEXT PRIMARY KEY,
                program_id TEXT NOT NULL,
                plan_id TEXT NOT NULL,
                chunk_ordinal INTEGER NOT NULL,
                payload_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS program_chunk_witness_index_records (
                artifact_id TEXT PRIMARY KEY,
                program_id TEXT NOT NULL,
                plan_id TEXT NOT NULL,
                payload_json TEXT NOT NULL
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

            CREATE INDEX IF NOT EXISTS idx_commit_support_summaries_branch_commit
            ON commit_support_summaries(branch_id, commit_id);

            CREATE INDEX IF NOT EXISTS idx_schema_support_records_branch_commit
            ON schema_support_records(branch_id, commit_id);

            CREATE INDEX IF NOT EXISTS idx_lineage_support_records_branch_commit
            ON lineage_support_records(branch_id, commit_id);

            CREATE INDEX IF NOT EXISTS idx_durable_cursor_identity_records_cursor
            ON durable_cursor_identity_records(cursor_id);

            CREATE INDEX IF NOT EXISTS idx_subscriber_checkpoint_records_cursor_sequence
            ON subscriber_checkpoint_records(cursor_id, checkpoint_sequence);

            CREATE INDEX IF NOT EXISTS idx_branch_shared_base_records_source_branch
            ON branch_shared_base_records(source_branch_id);

            CREATE INDEX IF NOT EXISTS idx_branch_delta_layer_records_branch_target
            ON branch_delta_layer_records(branch_id, target_frontier_commit_id);

            CREATE INDEX IF NOT EXISTS idx_embedded_checkpoint_records_basis_commit
            ON embedded_checkpoint_records(basis_commit_id);

            CREATE INDEX IF NOT EXISTS idx_milestone_6_layout_materialization_records_artifact
            ON milestone_6_layout_materialization_records(artifact_id);

            CREATE INDEX IF NOT EXISTS idx_milestone_6_commit_coupled_layout_seed_records_scope
            ON milestone_6_commit_coupled_layout_seed_records(branch_id, frontier_commit_id, scope_class);

            CREATE INDEX IF NOT EXISTS idx_milestone_6_scope_slice_membership_records_scope
            ON milestone_6_scope_slice_membership_records(branch_id, frontier_commit_id, scope_class, projection_digest);

            CREATE INDEX IF NOT EXISTS idx_milestone_6_chunk_membership_records_chunk
            ON milestone_6_chunk_membership_records(physical_chunk_id, chunk_shape_version, determinism_digest);

            CREATE INDEX IF NOT EXISTS idx_milestone_6_structural_block_records_block
            ON milestone_6_structural_block_records(structural_block_id, scope_class, equivalence_contract_version);

            CREATE INDEX IF NOT EXISTS idx_bulk_manifest_program
            ON frozen_bulk_manifest_records(program_id, manifest_digest);

            CREATE INDEX IF NOT EXISTS idx_bulk_transform_basis_program
            ON frozen_transform_basis_records(program_id, basis_digest);

            CREATE INDEX IF NOT EXISTS idx_bulk_transform_partition_program
            ON frozen_transform_partition_records(program_id, partition_digest);

            CREATE INDEX IF NOT EXISTS idx_bulk_plan_program
            ON bulk_deterministic_plan_records(program_id, plan_id);

            CREATE INDEX IF NOT EXISTS idx_bulk_checkpoint_program
            ON bulk_progress_checkpoint_records(program_id, plan_id, checkpoint_sequence);

            CREATE INDEX IF NOT EXISTS idx_bulk_witness_program
            ON bulk_chunk_witness_records(program_id, plan_id, chunk_ordinal);

            CREATE INDEX IF NOT EXISTS idx_snapshot_basis_branch_frontier
            ON snapshot_basis_records(snapshot_branch_id, snapshot_frontier_commit_id);

            CREATE INDEX IF NOT EXISTS idx_wal_records_mutation_sequence
            ON wal_records(durable_mutation_id, wal_sequence);
            ",
        )
        .map_err(sqlite_error)?;
    migrate_milestone_6_commit_coupled_layout_seed_storage(connection)?;
    ensure_branch_delta_layer_artifacts_column(connection)?;
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
                        "BulkCheckpointPublicationIntent" => {
                            crate::wal::WalRecordFamily::BulkCheckpointPublicationIntent
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
                    "CommitSupportSummary" => {
                        super::records::AuthoritativeArtifactFamily::CommitSupportSummary
                    }
                    "SchemaSupportRecord" => {
                        super::records::AuthoritativeArtifactFamily::SchemaSupportRecord
                    }
                    "LineageSupportRecord" => {
                        super::records::AuthoritativeArtifactFamily::LineageSupportRecord
                    }
                    "DurableCursorIdentityRecord" => {
                        super::records::AuthoritativeArtifactFamily::DurableCursorIdentityRecord
                    }
                    "SubscriberCheckpointRecord" => {
                        super::records::AuthoritativeArtifactFamily::SubscriberCheckpointRecord
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
                SELECT commit_id, branch_id, schema_support_artifact_id, lineage_support_artifact_id,
                       milestone_6_published_layout_request_artifact_ids_payload,
                       emitted_schema_artifact, emitted_lineage_artifact
                FROM commit_support_summaries
                ORDER BY commit_id
                ",
            )
            .map_err(sqlite_error)?;
        let rows = statement
            .query_map([], |row| {
                Ok(super::records::CommitSupportSummaryRecord {
                    commit_id: forge_relational::facade::history::CommitId(
                        row.get::<_, i64>(0)? as u64
                    ),
                    branch_id: forge_relational::facade::history::BranchId(
                        row.get::<_, String>(1)?,
                    ),
                    schema_support_artifact_id: row.get(2)?,
                    lineage_support_artifact_id: row.get(3)?,
                    milestone_6_published_layout_request_artifact_ids: serde_json::from_str(
                        &row.get::<_, String>(4)?,
                    )
                    .map_err(|error| {
                        rusqlite::Error::FromSqlConversionFailure(
                            4,
                            rusqlite::types::Type::Text,
                            Box::new(error),
                        )
                    })?,
                    emitted_schema_artifact: row.get::<_, i64>(5)? != 0,
                    emitted_lineage_artifact: row.get::<_, i64>(6)? != 0,
                })
            })
            .map_err(sqlite_error)?;
        for row in rows {
            let record = row.map_err(sqlite_error)?;
            state
                .commit_support_summaries
                .insert(record.commit_id.0, record);
        }
    }

    {
        let mut statement = connection
            .prepare(
                "
                SELECT artifact_id, commit_id, branch_id, schema_version_id, descriptor_semantics_version,
                       schema_transition_payload, schema_continuation_descriptor_payload,
                       schema_reconciliation_descriptor_payload
                FROM schema_support_records
                ORDER BY commit_id
                ",
            )
            .map_err(sqlite_error)?;
        let rows = statement
            .query_map([], |row| {
                let schema_transition = deserialize_optional_json::<
                    forge_relational::facade::schema::SchemaTransitionArtifact,
                >(row.get(5)?)?;
                let schema_continuation_descriptor = deserialize_optional_json::<
                    forge_relational::facade::schema::SchemaContinuationDescriptor,
                >(row.get(6)?)?;
                let schema_reconciliation_descriptor = deserialize_optional_json::<
                    forge_relational::facade::schema::SchemaReconciliationDescriptor,
                >(row.get(7)?)?;
                Ok(super::records::SchemaSupportRecord {
                    artifact_id: row.get(0)?,
                    commit_id: forge_relational::facade::history::CommitId(
                        row.get::<_, i64>(1)? as u64
                    ),
                    branch_id: forge_relational::facade::history::BranchId(
                        row.get::<_, String>(2)?,
                    ),
                    schema_version_id: forge_relational::facade::schema::SchemaVersionId(
                        row.get::<_, i64>(3)? as u32,
                    ),
                    descriptor_semantics_version:
                        forge_relational::facade::schema::DescriptorSemanticsVersion(
                            row.get::<_, i64>(4)? as u32,
                        ),
                    schema_transition,
                    schema_continuation_descriptor,
                    schema_reconciliation_descriptor,
                })
            })
            .map_err(sqlite_error)?;
        for row in rows {
            let record = row.map_err(sqlite_error)?;
            state
                .schema_support_records
                .insert(record.artifact_id.clone(), record);
        }
    }

    {
        let mut statement = connection
            .prepare(
                "
                SELECT artifact_id, commit_id, branch_id, lineage_event_ids_payload, lineage_events_payload,
                       lineage_digest_basis_payload, event_batch_digest_basis_payload,
                       decision_log_digest_basis_payload, lineage_artifact_counters_payload
                FROM lineage_support_records
                ORDER BY commit_id
                ",
            )
            .map_err(sqlite_error)?;
        let rows = statement
            .query_map([], |row| {
                Ok(super::records::LineageSupportRecord {
                    artifact_id: row.get(0)?,
                    commit_id: forge_relational::facade::history::CommitId(
                        row.get::<_, i64>(1)? as u64
                    ),
                    branch_id: forge_relational::facade::history::BranchId(
                        row.get::<_, String>(2)?,
                    ),
                    lineage_event_ids: deserialize_json(row.get(3)?)?,
                    lineage_events: deserialize_json(row.get(4)?)?,
                    lineage_digest_basis: deserialize_json(row.get(5)?)?,
                    event_batch_digest_basis: deserialize_json(row.get(6)?)?,
                    decision_log_digest_basis: deserialize_json(row.get(7)?)?,
                    lineage_artifact_counters: deserialize_json(row.get(8)?)?,
                })
            })
            .map_err(sqlite_error)?;
        for row in rows {
            let record = row.map_err(sqlite_error)?;
            state
                .lineage_support_records
                .insert(record.artifact_id.clone(), record);
        }
    }

    {
        let mut statement = connection
            .prepare(
                "
                SELECT artifact_id, cursor_id, subscriber_id, branch_id, feed_shape_id,
                       schema_interpretation_id, cursor_semantics_version,
                       latest_checkpoint_sequence, latest_basis_commit_id,
                       latest_schema_support_artifact_id
                FROM durable_cursor_identity_records
                ORDER BY cursor_id
                ",
            )
            .map_err(sqlite_error)?;
        let rows = statement
            .query_map([], |row| {
                Ok(super::records::DurableCursorIdentityRecord {
                    artifact_id: row.get(0)?,
                    cursor_id: row.get(1)?,
                    subscriber_id: row.get(2)?,
                    branch_id: forge_relational::facade::history::BranchId(
                        row.get::<_, String>(3)?,
                    ),
                    feed_shape_id: row.get(4)?,
                    schema_interpretation_id: row.get(5)?,
                    cursor_semantics_version: row.get::<_, i64>(6)? as u32,
                    latest_checkpoint_sequence: row.get::<_, i64>(7)? as u64,
                    latest_basis_commit_id: forge_relational::facade::history::CommitId(
                        row.get::<_, i64>(8)? as u64,
                    ),
                    latest_schema_support_artifact_id: row.get(9)?,
                })
            })
            .map_err(sqlite_error)?;
        for row in rows {
            let record = row.map_err(sqlite_error)?;
            state
                .durable_cursor_identity_records
                .insert(record.artifact_id.clone(), record);
        }
    }

    {
        let mut statement = connection
            .prepare(
                "
                SELECT artifact_id, cursor_id, subscriber_id, branch_id, feed_shape_id,
                       schema_interpretation_id, cursor_semantics_version,
                       checkpoint_sequence, basis_commit_id, schema_support_artifact_id
                FROM subscriber_checkpoint_records
                ORDER BY cursor_id, checkpoint_sequence
                ",
            )
            .map_err(sqlite_error)?;
        let rows = statement
            .query_map([], |row| {
                Ok(super::records::SubscriberCheckpointRecord {
                    artifact_id: row.get(0)?,
                    cursor_id: row.get(1)?,
                    subscriber_id: row.get(2)?,
                    branch_id: forge_relational::facade::history::BranchId(
                        row.get::<_, String>(3)?,
                    ),
                    feed_shape_id: row.get(4)?,
                    schema_interpretation_id: row.get(5)?,
                    cursor_semantics_version: row.get::<_, i64>(6)? as u32,
                    checkpoint_sequence: row.get::<_, i64>(7)? as u64,
                    basis_commit_id: forge_relational::facade::history::CommitId(
                        row.get::<_, i64>(8)? as u64,
                    ),
                    schema_support_artifact_id: row.get(9)?,
                })
            })
            .map_err(sqlite_error)?;
        for row in rows {
            let record = row.map_err(sqlite_error)?;
            state
                .subscriber_checkpoint_records
                .insert(record.artifact_id.clone(), record);
        }
    }

    {
        let mut statement = connection
            .prepare(
                "
                SELECT branch_id, source_branch_id, source_frontier_commit_id, delta_family_version, authority_basis_digest
                FROM branch_shared_base_records
                ORDER BY branch_id
                ",
            )
            .map_err(sqlite_error)?;
        let rows = statement
            .query_map([], |row| {
                Ok(super::records::BranchSharedBaseRecord {
                    branch_id: forge_relational::facade::history::BranchId(
                        row.get::<_, String>(0)?,
                    ),
                    source_branch_id: forge_relational::facade::history::BranchId(
                        row.get::<_, String>(1)?,
                    ),
                    source_frontier_commit_id: row
                        .get::<_, Option<i64>>(2)?
                        .map(|value| forge_relational::facade::history::CommitId(value as u64)),
                    delta_family_version: row.get::<_, i64>(3)? as u32,
                    authority_basis_digest: row.get(4)?,
                })
            })
            .map_err(sqlite_error)?;
        for row in rows {
            let record = row.map_err(sqlite_error)?;
            state
                .branch_shared_base_records
                .insert(record.branch_id.0.clone(), record);
        }
    }

    {
        let mut statement = connection
            .prepare(
                "
                SELECT branch_delta_layer_id, branch_id, base_frontier_commit_id, target_frontier_commit_id,
                       commit_ids_payload, delta_family_version, authority_basis_digest, artifacts_payload,
                       replacement_of_layer_ids_payload, replacement_lineage_proof_payload
                FROM branch_delta_layer_records
                ORDER BY branch_delta_layer_id
                ",
            )
            .map_err(sqlite_error)?;
        let rows = statement
            .query_map([], |row| {
                let commit_ids: Vec<u64> = deserialize_json(row.get(4)?)?;
                let artifacts = deserialize_json(row.get(7)?)?;
                let replacement_of_layer_ids: Vec<u64> = deserialize_json(row.get(8)?)?;
                let replacement_lineage_proof = deserialize_json(row.get(9)?)?;
                Ok(super::records::BranchDeltaLayerRecord {
                    branch_delta_layer_id: crate::delta::BranchDeltaLayerId(
                        row.get::<_, i64>(0)? as u64
                    ),
                    branch_id: forge_relational::facade::history::BranchId(
                        row.get::<_, String>(1)?,
                    ),
                    base_frontier_commit_id: row
                        .get::<_, Option<i64>>(2)?
                        .map(|value| forge_relational::facade::history::CommitId(value as u64)),
                    target_frontier_commit_id: forge_relational::facade::history::CommitId(
                        row.get::<_, i64>(3)? as u64,
                    ),
                    commit_ids: commit_ids
                        .into_iter()
                        .map(forge_relational::facade::history::CommitId)
                        .collect(),
                    delta_family_version: row.get::<_, i64>(5)? as u32,
                    authority_basis_digest: row.get(6)?,
                    artifacts,
                    replacement_of_layer_ids: replacement_of_layer_ids
                        .into_iter()
                        .map(crate::delta::BranchDeltaLayerId)
                        .collect(),
                    replacement_lineage_proof,
                })
            })
            .map_err(sqlite_error)?;
        for row in rows {
            let record = row.map_err(sqlite_error)?;
            state
                .branch_delta_layer_records
                .insert(record.branch_delta_layer_id.0, record);
        }
    }
    state.backfill_missing_branch_delta_layer_artifacts()?;

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
                SELECT payload_json
                FROM milestone_6_layout_materialization_records
                ORDER BY artifact_id
                ",
            )
            .map_err(sqlite_error)?;
        let rows = statement
            .query_map([], |row| deserialize_json(row.get(0)?))
            .map_err(sqlite_error)?;
        for row in rows {
            let record: super::records::Milestone6LayoutMaterializationRecord =
                row.map_err(sqlite_error)?;
            state
                .milestone_6_layout_materialization_records
                .insert(record.artifact_id.clone(), record);
        }
    }

    {
        let mut statement = connection
            .prepare(
                "
                SELECT payload_json
                FROM milestone_6_commit_coupled_layout_seed_records
                ORDER BY artifact_id
                ",
            )
            .map_err(sqlite_error)?;
        let rows = statement
            .query_map([], |row| deserialize_json(row.get(0)?))
            .map_err(sqlite_error)?;
        for row in rows {
            let record: super::records::Milestone6CommitCoupledLayoutSeedRecord =
                row.map_err(sqlite_error)?;
            state
                .milestone_6_commit_coupled_layout_seed_records
                .insert(record.artifact_id.clone(), record);
        }
    }

    {
        let mut statement = connection
            .prepare(
                "
                SELECT payload_json
                FROM milestone_6_scope_slice_membership_records
                ORDER BY artifact_id
                ",
            )
            .map_err(sqlite_error)?;
        let rows = statement
            .query_map([], |row| deserialize_json(row.get(0)?))
            .map_err(sqlite_error)?;
        for row in rows {
            let record: super::records::Milestone6ScopeSliceMembershipRecord =
                row.map_err(sqlite_error)?;
            state
                .milestone_6_scope_slice_membership_records
                .insert(record.artifact_id.clone(), record);
        }
    }

    {
        let mut statement = connection
            .prepare(
                "
                SELECT payload_json
                FROM milestone_6_chunk_membership_records
                ORDER BY artifact_id
                ",
            )
            .map_err(sqlite_error)?;
        let rows = statement
            .query_map([], |row| deserialize_json(row.get(0)?))
            .map_err(sqlite_error)?;
        for row in rows {
            let record: super::records::Milestone6ChunkMembershipRecord =
                row.map_err(sqlite_error)?;
            state
                .milestone_6_chunk_membership_records
                .insert(record.artifact_id.clone(), record);
        }
    }

    {
        let mut statement = connection
            .prepare(
                "
                SELECT payload_json
                FROM milestone_6_structural_block_records
                ORDER BY artifact_id
                ",
            )
            .map_err(sqlite_error)?;
        let rows = statement
            .query_map([], |row| deserialize_json(row.get(0)?))
            .map_err(sqlite_error)?;
        for row in rows {
            let record: super::records::Milestone6StructuralBlockRecord =
                row.map_err(sqlite_error)?;
            state
                .milestone_6_structural_block_records
                .insert(record.artifact_id.clone(), record);
        }
    }

    {
        let mut statement = connection
            .prepare(
                "
                SELECT payload_json
                FROM bulk_program_identity_records
                ORDER BY artifact_id
                ",
            )
            .map_err(sqlite_error)?;
        let rows = statement
            .query_map([], |row| deserialize_json(row.get(0)?))
            .map_err(sqlite_error)?;
        for row in rows {
            let record: super::records::BulkProgramIdentityRecord = row.map_err(sqlite_error)?;
            state
                .bulk_program_identity_records
                .insert(record.artifact_id.clone(), record);
        }
    }

    {
        let mut statement = connection
            .prepare(
                "
                SELECT payload_json
                FROM frozen_bulk_manifest_records
                ORDER BY artifact_id
                ",
            )
            .map_err(sqlite_error)?;
        let rows = statement
            .query_map([], |row| deserialize_json(row.get(0)?))
            .map_err(sqlite_error)?;
        for row in rows {
            let record: super::records::FrozenBulkManifestRecord = row.map_err(sqlite_error)?;
            state
                .frozen_bulk_manifest_records
                .insert(record.artifact_id.clone(), record);
        }
    }

    {
        let mut statement = connection
            .prepare(
                "
                SELECT payload_json
                FROM frozen_transform_basis_records
                ORDER BY artifact_id
                ",
            )
            .map_err(sqlite_error)?;
        let rows = statement
            .query_map([], |row| deserialize_json(row.get(0)?))
            .map_err(sqlite_error)?;
        for row in rows {
            let record: super::records::FrozenTransformBasisRecord = row.map_err(sqlite_error)?;
            state
                .frozen_transform_basis_records
                .insert(record.artifact_id.clone(), record);
        }
    }

    {
        let mut statement = connection
            .prepare(
                "
                SELECT payload_json
                FROM frozen_transform_partition_records
                ORDER BY artifact_id
                ",
            )
            .map_err(sqlite_error)?;
        let rows = statement
            .query_map([], |row| deserialize_json(row.get(0)?))
            .map_err(sqlite_error)?;
        for row in rows {
            let record: super::records::FrozenTransformPartitionRecord =
                row.map_err(sqlite_error)?;
            state
                .frozen_transform_partition_records
                .insert(record.artifact_id.clone(), record);
        }
    }

    {
        let mut statement = connection
            .prepare(
                "
                SELECT payload_json
                FROM bulk_deterministic_plan_records
                ORDER BY artifact_id
                ",
            )
            .map_err(sqlite_error)?;
        let rows = statement
            .query_map([], |row| deserialize_json(row.get(0)?))
            .map_err(sqlite_error)?;
        for row in rows {
            let record: super::records::BulkDeterministicPlanRecord = row.map_err(sqlite_error)?;
            state
                .bulk_deterministic_plan_records
                .insert(record.artifact_id.clone(), record);
        }
    }

    {
        let mut statement = connection
            .prepare(
                "
                SELECT payload_json
                FROM bulk_progress_checkpoint_records
                ORDER BY artifact_id
                ",
            )
            .map_err(sqlite_error)?;
        let rows = statement
            .query_map([], |row| deserialize_json(row.get(0)?))
            .map_err(sqlite_error)?;
        for row in rows {
            let record: super::records::BulkProgressCheckpointRecord = row.map_err(sqlite_error)?;
            state
                .bulk_progress_checkpoint_records
                .insert(record.artifact_id.clone(), record);
        }
    }

    {
        let mut statement = connection
            .prepare(
                "
                SELECT payload_json
                FROM bulk_chunk_witness_records
                ORDER BY artifact_id
                ",
            )
            .map_err(sqlite_error)?;
        let rows = statement
            .query_map([], |row| deserialize_json(row.get(0)?))
            .map_err(sqlite_error)?;
        for row in rows {
            let record: super::records::BulkChunkWitnessRecord = row.map_err(sqlite_error)?;
            state
                .bulk_chunk_witness_records
                .insert(record.artifact_id.clone(), record);
        }
    }

    {
        let mut statement = connection
            .prepare(
                "
                SELECT payload_json
                FROM program_chunk_witness_index_records
                ORDER BY artifact_id
                ",
            )
            .map_err(sqlite_error)?;
        let rows = statement
            .query_map([], |row| deserialize_json(row.get(0)?))
            .map_err(sqlite_error)?;
        for row in rows {
            let record: super::records::ProgramChunkWitnessIndexRecord =
                row.map_err(sqlite_error)?;
            state
                .program_chunk_witness_index_records
                .insert(record.artifact_id.clone(), record);
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
    state.next_branch_delta_layer_id = load_meta_u64(connection, "next_branch_delta_layer_id")?
        .unwrap_or_else(|| {
            state
                .branch_delta_layer_records
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
    persist_commit_support_summaries(&transaction, state)?;
    persist_schema_support_records(&transaction, state)?;
    persist_lineage_support_records(&transaction, state)?;
    persist_durable_cursor_identity_records(&transaction, state)?;
    persist_subscriber_checkpoint_records(&transaction, state)?;
    persist_branch_shared_base_records(&transaction, state)?;
    persist_branch_delta_layer_records(&transaction, state)?;
    persist_embedded_checkpoint_records(&transaction, state)?;
    persist_milestone_6_layout_materialization_records(&transaction, state)?;
    persist_milestone_6_commit_coupled_layout_seed_records(&transaction, state)?;
    persist_milestone_6_scope_slice_membership_records(&transaction, state)?;
    persist_milestone_6_chunk_membership_records(&transaction, state)?;
    persist_milestone_6_structural_block_records_impl(&transaction, state)?;
    persist_bulk_program_identity_records(&transaction, state)?;
    persist_frozen_bulk_manifest_records(&transaction, state)?;
    persist_frozen_transform_basis_records(&transaction, state)?;
    persist_frozen_transform_partition_records(&transaction, state)?;
    persist_bulk_deterministic_plan_records(&transaction, state)?;
    persist_bulk_progress_checkpoint_records(&transaction, state)?;
    persist_bulk_chunk_witness_records(&transaction, state)?;
    persist_program_chunk_witness_index_records(&transaction, state)?;
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
            DELETE FROM commit_support_summaries;
            DELETE FROM schema_support_records;
            DELETE FROM lineage_support_records;
            DELETE FROM durable_cursor_identity_records;
            DELETE FROM subscriber_checkpoint_records;
            DELETE FROM branch_delta_layer_records;
            DELETE FROM branch_shared_base_records;
            DELETE FROM commit_parent_records;
            DELETE FROM commit_envelopes;
            DELETE FROM branch_records;
            DELETE FROM embedded_checkpoint_records;
            DELETE FROM milestone_6_layout_materialization_records;
            DELETE FROM milestone_6_commit_coupled_layout_seed_records;
            DELETE FROM milestone_6_scope_slice_membership_records;
            DELETE FROM milestone_6_chunk_membership_records;
            DELETE FROM milestone_6_structural_block_records;
            DELETE FROM bulk_program_identity_records;
            DELETE FROM frozen_bulk_manifest_records;
            DELETE FROM frozen_transform_basis_records;
            DELETE FROM frozen_transform_partition_records;
            DELETE FROM bulk_deterministic_plan_records;
            DELETE FROM bulk_progress_checkpoint_records;
            DELETE FROM bulk_chunk_witness_records;
            DELETE FROM program_chunk_witness_index_records;
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
        "next_branch_delta_layer_id",
        state.next_branch_delta_layer_id.to_string(),
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

fn persist_commit_support_summaries(
    transaction: &Transaction<'_>,
    state: &StoreState,
) -> Result<(), StoreError> {
    for record in state.commit_support_summaries.values() {
        transaction
            .execute(
                "
                INSERT INTO commit_support_summaries(
                    commit_id,
                    branch_id,
                    schema_support_artifact_id,
                    lineage_support_artifact_id,
                    milestone_6_published_layout_request_artifact_ids_payload,
                    emitted_schema_artifact,
                    emitted_lineage_artifact
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
                ",
                params![
                    as_i64(record.commit_id),
                    record.branch_id.0,
                    record.schema_support_artifact_id,
                    record.lineage_support_artifact_id,
                    serde_json::to_string(
                        &record.milestone_6_published_layout_request_artifact_ids
                    )
                    .map_err(StoreError::from)?,
                    if record.emitted_schema_artifact { 1 } else { 0 },
                    if record.emitted_lineage_artifact {
                        1
                    } else {
                        0
                    },
                ],
            )
            .map_err(sqlite_error)?;
    }
    Ok(())
}

fn persist_schema_support_records(
    transaction: &Transaction<'_>,
    state: &StoreState,
) -> Result<(), StoreError> {
    for record in state.schema_support_records.values() {
        transaction
            .execute(
                "
                INSERT INTO schema_support_records(
                    artifact_id,
                    commit_id,
                    branch_id,
                    schema_version_id,
                    descriptor_semantics_version,
                    schema_transition_payload,
                    schema_continuation_descriptor_payload,
                    schema_reconciliation_descriptor_payload
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
                ",
                params![
                    record.artifact_id,
                    as_i64(record.commit_id),
                    record.branch_id.0,
                    record.schema_version_id.0 as i64,
                    record.descriptor_semantics_version.0 as i64,
                    serialize_optional_json(&record.schema_transition)?,
                    serialize_optional_json(&record.schema_continuation_descriptor)?,
                    serialize_optional_json(&record.schema_reconciliation_descriptor)?,
                ],
            )
            .map_err(sqlite_error)?;
    }
    Ok(())
}

fn persist_lineage_support_records(
    transaction: &Transaction<'_>,
    state: &StoreState,
) -> Result<(), StoreError> {
    for record in state.lineage_support_records.values() {
        transaction
            .execute(
                "
                INSERT INTO lineage_support_records(
                    artifact_id,
                    commit_id,
                    branch_id,
                    lineage_event_ids_payload,
                    lineage_events_payload,
                    lineage_digest_basis_payload,
                    event_batch_digest_basis_payload,
                    decision_log_digest_basis_payload,
                    lineage_artifact_counters_payload
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
                ",
                params![
                    record.artifact_id,
                    as_i64(record.commit_id),
                    record.branch_id.0,
                    serde_json::to_string(&record.lineage_event_ids)?,
                    serde_json::to_string(&record.lineage_events)?,
                    serde_json::to_string(&record.lineage_digest_basis)?,
                    serde_json::to_string(&record.event_batch_digest_basis)?,
                    serde_json::to_string(&record.decision_log_digest_basis)?,
                    serde_json::to_string(&record.lineage_artifact_counters)?,
                ],
            )
            .map_err(sqlite_error)?;
    }
    Ok(())
}

fn persist_durable_cursor_identity_records(
    transaction: &Transaction<'_>,
    state: &StoreState,
) -> Result<(), StoreError> {
    for record in state.durable_cursor_identity_records.values() {
        transaction
            .execute(
                "
                INSERT INTO durable_cursor_identity_records(
                    artifact_id,
                    cursor_id,
                    subscriber_id,
                    branch_id,
                    feed_shape_id,
                    schema_interpretation_id,
                    cursor_semantics_version,
                    latest_checkpoint_sequence,
                    latest_basis_commit_id,
                    latest_schema_support_artifact_id
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
                ",
                params![
                    record.artifact_id,
                    record.cursor_id,
                    record.subscriber_id,
                    record.branch_id.0,
                    record.feed_shape_id,
                    record.schema_interpretation_id,
                    record.cursor_semantics_version as i64,
                    record.latest_checkpoint_sequence as i64,
                    as_i64(record.latest_basis_commit_id),
                    record.latest_schema_support_artifact_id,
                ],
            )
            .map_err(sqlite_error)?;
    }
    Ok(())
}

fn persist_subscriber_checkpoint_records(
    transaction: &Transaction<'_>,
    state: &StoreState,
) -> Result<(), StoreError> {
    for record in state.subscriber_checkpoint_records.values() {
        transaction
            .execute(
                "
                INSERT INTO subscriber_checkpoint_records(
                    artifact_id,
                    cursor_id,
                    subscriber_id,
                    branch_id,
                    feed_shape_id,
                    schema_interpretation_id,
                    cursor_semantics_version,
                    checkpoint_sequence,
                    basis_commit_id,
                    schema_support_artifact_id
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
                ",
                params![
                    record.artifact_id,
                    record.cursor_id,
                    record.subscriber_id,
                    record.branch_id.0,
                    record.feed_shape_id,
                    record.schema_interpretation_id,
                    record.cursor_semantics_version as i64,
                    record.checkpoint_sequence as i64,
                    as_i64(record.basis_commit_id),
                    record.schema_support_artifact_id,
                ],
            )
            .map_err(sqlite_error)?;
    }
    Ok(())
}

fn persist_branch_shared_base_records(
    transaction: &Transaction<'_>,
    state: &StoreState,
) -> Result<(), StoreError> {
    for record in state.branch_shared_base_records.values() {
        transaction
            .execute(
                "
                INSERT INTO branch_shared_base_records(
                    branch_id,
                    source_branch_id,
                    source_frontier_commit_id,
                    delta_family_version,
                    authority_basis_digest
                ) VALUES (?1, ?2, ?3, ?4, ?5)
                ",
                params![
                    record.branch_id.0,
                    record.source_branch_id.0,
                    record.source_frontier_commit_id.map(as_i64),
                    record.delta_family_version as i64,
                    record.authority_basis_digest,
                ],
            )
            .map_err(sqlite_error)?;
    }
    Ok(())
}

fn persist_branch_delta_layer_records(
    transaction: &Transaction<'_>,
    state: &StoreState,
) -> Result<(), StoreError> {
    for record in state.branch_delta_layer_records.values() {
        transaction
            .execute(
                "
                INSERT INTO branch_delta_layer_records(
                    branch_delta_layer_id,
                    branch_id,
                    base_frontier_commit_id,
                    target_frontier_commit_id,
                    commit_ids_payload,
                    delta_family_version,
                    authority_basis_digest,
                    artifacts_payload,
                    replacement_of_layer_ids_payload,
                    replacement_lineage_proof_payload
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
                ",
                params![
                    as_i64_u64(record.branch_delta_layer_id.0),
                    record.branch_id.0,
                    record.base_frontier_commit_id.map(as_i64),
                    as_i64(record.target_frontier_commit_id),
                    serde_json::to_string(
                        &record
                            .commit_ids
                            .iter()
                            .map(|commit_id| commit_id.0)
                            .collect::<Vec<_>>()
                    )?,
                    record.delta_family_version as i64,
                    record.authority_basis_digest,
                    serde_json::to_string(&record.artifacts)?,
                    serde_json::to_string(
                        &record
                            .replacement_of_layer_ids
                            .iter()
                            .map(|layer_id| layer_id.0)
                            .collect::<Vec<_>>()
                    )?,
                    serde_json::to_string(&record.replacement_lineage_proof)?,
                ],
            )
            .map_err(sqlite_error)?;
    }
    Ok(())
}

fn ensure_branch_delta_layer_artifacts_column(connection: &Connection) -> Result<(), StoreError> {
    let mut statement = connection
        .prepare("PRAGMA table_info(branch_delta_layer_records)")
        .map_err(sqlite_error)?;
    let rows = statement
        .query_map([], |row| row.get::<_, String>(1))
        .map_err(sqlite_error)?;
    let mut has_artifacts_column = false;
    for row in rows {
        if row.map_err(sqlite_error)? == "artifacts_payload" {
            has_artifacts_column = true;
            break;
        }
    }
    if !has_artifacts_column {
        let default_payload =
            serde_json::to_string(&super::records::BranchDeltaLayerArtifacts::default())?;
        connection
            .execute(
                &format!(
                    "ALTER TABLE branch_delta_layer_records ADD COLUMN artifacts_payload TEXT NOT NULL DEFAULT '{}'",
                    default_payload.replace('\'', "''")
                ),
                [],
            )
            .map_err(sqlite_error)?;
    }
    Ok(())
}

fn migrate_milestone_6_commit_coupled_layout_seed_storage(
    connection: &Connection,
) -> Result<(), StoreError> {
    if !table_exists(connection, "milestone_6_published_layout_request_records")? {
        return Ok(());
    }
    if table_row_count(connection, "milestone_6_commit_coupled_layout_seed_records")? > 0 {
        return Ok(());
    }
    connection
        .execute(
            "
            INSERT INTO milestone_6_commit_coupled_layout_seed_records(
                artifact_id,
                branch_id,
                frontier_commit_id,
                scope_class,
                payload_json
            )
            SELECT artifact_id, branch_id, frontier_commit_id, scope_class, payload_json
            FROM milestone_6_published_layout_request_records
            ",
            [],
        )
        .map_err(sqlite_error)?;
    Ok(())
}

fn table_exists(connection: &Connection, table_name: &str) -> Result<bool, StoreError> {
    connection
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = ?1)",
            [table_name],
            |row| row.get::<_, i64>(0),
        )
        .map(|value| value != 0)
        .map_err(sqlite_error)
}

fn table_row_count(connection: &Connection, table_name: &str) -> Result<i64, StoreError> {
    let sql = format!("SELECT COUNT(*) FROM {table_name}");
    connection
        .query_row(&sql, [], |row| row.get::<_, i64>(0))
        .map_err(sqlite_error)
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

fn persist_milestone_6_layout_materialization_records(
    transaction: &Transaction<'_>,
    state: &StoreState,
) -> Result<(), StoreError> {
    for record in state.milestone_6_layout_materialization_records.values() {
        persist_bulk_json_record(
            transaction,
            "milestone_6_layout_materialization_records",
            &record.artifact_id,
            Vec::new(),
            record,
        )?;
    }
    Ok(())
}

fn persist_milestone_6_commit_coupled_layout_seed_records(
    transaction: &Transaction<'_>,
    state: &StoreState,
) -> Result<(), StoreError> {
    for record in state
        .milestone_6_commit_coupled_layout_seed_records
        .values()
    {
        persist_bulk_json_record(
            transaction,
            "milestone_6_commit_coupled_layout_seed_records",
            &record.artifact_id,
            vec![
                (
                    "branch_id".to_string(),
                    record.request.target().branch_id().0.clone(),
                ),
                (
                    "frontier_commit_id".to_string(),
                    record.request.target().frontier_commit_id().0.to_string(),
                ),
                (
                    "scope_class".to_string(),
                    record.request.scope_class().label().to_string(),
                ),
            ],
            record,
        )?;
    }
    Ok(())
}

fn persist_milestone_6_scope_slice_membership_records(
    transaction: &Transaction<'_>,
    state: &StoreState,
) -> Result<(), StoreError> {
    for record in state.milestone_6_scope_slice_membership_records.values() {
        persist_bulk_json_record(
            transaction,
            "milestone_6_scope_slice_membership_records",
            &record.artifact_id,
            vec![
                ("branch_id".to_string(), record.branch_id.0.clone()),
                (
                    "frontier_commit_id".to_string(),
                    record.frontier_commit_id.0.to_string(),
                ),
                ("scope_class".to_string(), record.scope_class.clone()),
                (
                    "projection_digest".to_string(),
                    record.projection_digest.clone(),
                ),
            ],
            record,
        )?;
    }
    Ok(())
}

fn persist_milestone_6_chunk_membership_records(
    transaction: &Transaction<'_>,
    state: &StoreState,
) -> Result<(), StoreError> {
    for record in state.milestone_6_chunk_membership_records.values() {
        persist_bulk_json_record(
            transaction,
            "milestone_6_chunk_membership_records",
            &record.artifact_id,
            vec![
                (
                    "physical_chunk_id".to_string(),
                    record.physical_chunk_id.as_str().to_string(),
                ),
                (
                    "chunk_shape_version".to_string(),
                    record.chunk_shape_version.value().to_string(),
                ),
                (
                    "determinism_digest".to_string(),
                    record.determinism_digest.clone(),
                ),
            ],
            record,
        )?;
    }
    Ok(())
}

fn persist_milestone_6_structural_block_records_impl(
    transaction: &Transaction<'_>,
    state: &StoreState,
) -> Result<(), StoreError> {
    for record in state.milestone_6_structural_block_records.values() {
        persist_bulk_json_record(
            transaction,
            "milestone_6_structural_block_records",
            &record.artifact_id,
            vec![
                (
                    "structural_block_id".to_string(),
                    record.structural_block_id.as_str().to_string(),
                ),
                ("scope_class".to_string(), record.scope_class.clone()),
                (
                    "equivalence_contract_version".to_string(),
                    record.equivalence_contract_version.value().to_string(),
                ),
                (
                    "supporting_layout_materialization_count".to_string(),
                    record
                        .supporting_layout_materialization_artifact_ids
                        .len()
                        .to_string(),
                ),
            ],
            record,
        )?;
    }
    Ok(())
}

fn persist_bulk_program_identity_records(
    transaction: &Transaction<'_>,
    state: &StoreState,
) -> Result<(), StoreError> {
    for record in state.bulk_program_identity_records.values() {
        persist_bulk_json_record(
            transaction,
            "bulk_program_identity_records",
            &record.artifact_id,
            vec![
                ("program_id".to_string(), record.program_id.clone()),
                ("kind".to_string(), format!("{:?}", record.kind)),
            ],
            record,
        )?;
    }
    Ok(())
}

fn persist_frozen_bulk_manifest_records(
    transaction: &Transaction<'_>,
    state: &StoreState,
) -> Result<(), StoreError> {
    for record in state.frozen_bulk_manifest_records.values() {
        persist_bulk_json_record(
            transaction,
            "frozen_bulk_manifest_records",
            &record.artifact_id,
            vec![
                ("program_id".to_string(), record.program_id.clone()),
                (
                    "manifest_digest".to_string(),
                    record.manifest.manifest_digest().to_string(),
                ),
            ],
            record,
        )?;
    }
    Ok(())
}

fn persist_frozen_transform_basis_records(
    transaction: &Transaction<'_>,
    state: &StoreState,
) -> Result<(), StoreError> {
    for record in state.frozen_transform_basis_records.values() {
        persist_bulk_json_record(
            transaction,
            "frozen_transform_basis_records",
            &record.artifact_id,
            vec![
                ("program_id".to_string(), record.program_id.clone()),
                (
                    "basis_digest".to_string(),
                    record.basis.basis_digest().to_string(),
                ),
            ],
            record,
        )?;
    }
    Ok(())
}

fn persist_frozen_transform_partition_records(
    transaction: &Transaction<'_>,
    state: &StoreState,
) -> Result<(), StoreError> {
    for record in state.frozen_transform_partition_records.values() {
        persist_bulk_json_record(
            transaction,
            "frozen_transform_partition_records",
            &record.artifact_id,
            vec![
                ("program_id".to_string(), record.program_id.clone()),
                (
                    "partition_digest".to_string(),
                    record.partition.partition_digest().to_string(),
                ),
            ],
            record,
        )?;
    }
    Ok(())
}

fn persist_bulk_deterministic_plan_records(
    transaction: &Transaction<'_>,
    state: &StoreState,
) -> Result<(), StoreError> {
    for record in state.bulk_deterministic_plan_records.values() {
        persist_bulk_json_record(
            transaction,
            "bulk_deterministic_plan_records",
            &record.artifact_id,
            vec![
                ("program_id".to_string(), record.program_id.clone()),
                ("plan_id".to_string(), record.plan.plan_id().to_string()),
            ],
            record,
        )?;
    }
    Ok(())
}

fn persist_bulk_progress_checkpoint_records(
    transaction: &Transaction<'_>,
    state: &StoreState,
) -> Result<(), StoreError> {
    for record in state.bulk_progress_checkpoint_records.values() {
        persist_bulk_json_record(
            transaction,
            "bulk_progress_checkpoint_records",
            &record.artifact_id,
            vec![
                ("program_id".to_string(), record.program_id.clone()),
                ("plan_id".to_string(), record.plan_id.clone()),
                (
                    "checkpoint_sequence".to_string(),
                    record.checkpoint.checkpoint_sequence().to_string(),
                ),
            ],
            record,
        )?;
    }
    Ok(())
}

fn persist_bulk_chunk_witness_records(
    transaction: &Transaction<'_>,
    state: &StoreState,
) -> Result<(), StoreError> {
    for record in state.bulk_chunk_witness_records.values() {
        persist_bulk_json_record(
            transaction,
            "bulk_chunk_witness_records",
            &record.artifact_id,
            vec![
                ("program_id".to_string(), record.program_id.clone()),
                ("plan_id".to_string(), record.plan_id.clone()),
                (
                    "chunk_ordinal".to_string(),
                    record.witness.chunk_ordinal().value().to_string(),
                ),
            ],
            record,
        )?;
    }
    Ok(())
}

fn persist_program_chunk_witness_index_records(
    transaction: &Transaction<'_>,
    state: &StoreState,
) -> Result<(), StoreError> {
    for record in state.program_chunk_witness_index_records.values() {
        persist_bulk_json_record(
            transaction,
            "program_chunk_witness_index_records",
            &record.artifact_id,
            vec![
                ("program_id".to_string(), record.program_id.clone()),
                ("plan_id".to_string(), record.plan_id.clone()),
            ],
            record,
        )?;
    }
    Ok(())
}

fn persist_bulk_json_record<T: serde::Serialize>(
    transaction: &Transaction<'_>,
    table: &str,
    artifact_id: &str,
    indexed_columns: Vec<(String, String)>,
    record: &T,
) -> Result<(), StoreError> {
    let payload = serde_json::to_string(record)?;
    let mut columns = vec!["artifact_id".to_string()];
    let mut placeholders = vec!["?1".to_string()];
    let mut values = vec![rusqlite::types::Value::Text(artifact_id.to_string())];
    let mut payload_index = 2usize;
    for (idx, (name, value)) in indexed_columns.iter().enumerate() {
        columns.push(name.clone());
        placeholders.push(format!("?{}", idx + 2));
        values.push(rusqlite::types::Value::Text(value.clone()));
        payload_index = idx + 3;
    }
    columns.push("payload_json".to_string());
    placeholders.push(format!("?{}", payload_index));
    values.push(rusqlite::types::Value::Text(payload));
    let sql = format!(
        "INSERT INTO {table}({}) VALUES ({})",
        columns.join(", "),
        placeholders.join(", ")
    );
    transaction
        .execute(&sql, rusqlite::params_from_iter(values))
        .map_err(sqlite_error)?;
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

fn deserialize_json<T: serde::de::DeserializeOwned>(payload: String) -> rusqlite::Result<T> {
    serde_json::from_str(&payload).map_err(|error| {
        rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(error))
    })
}

fn deserialize_optional_json<T: serde::de::DeserializeOwned>(
    payload: Option<String>,
) -> rusqlite::Result<Option<T>> {
    payload.map(deserialize_json).transpose()
}

fn serialize_optional_json<T: serde::Serialize>(
    value: &Option<T>,
) -> Result<Option<String>, StoreError> {
    value
        .as_ref()
        .map(serde_json::to_string)
        .transpose()
        .map_err(Into::into)
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
