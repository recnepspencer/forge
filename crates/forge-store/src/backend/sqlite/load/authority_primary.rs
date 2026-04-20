use crate::backend::{integrity, records};
use crate::failure::StoreError;
use rusqlite::Connection;

use super::super::super::records::StoreState;
use super::super::helpers::sqlite_error;

pub(super) fn load_authority_primary(
    connection: &Connection,
    state: &mut StoreState,
) -> Result<(), StoreError> {
    load_wal_records(connection, state)?;
    load_branch_records(connection, state)?;
    load_branch_head_records(connection, state)?;
    load_commit_envelopes(connection, state)?;
    load_commit_parent_records(connection, state)?;
    load_digest_records(connection, state)?;
    Ok(())
}

fn load_wal_records(connection: &Connection, state: &mut StoreState) -> Result<(), StoreError> {
    let mut statement = connection
        .prepare(
            "
            SELECT wal_sequence, family, durable_mutation_id, runtime_session_id, wal_version, record_digest, payload_json
            FROM wal_records
            ORDER BY wal_sequence
            ",
        )
        .map_err(sqlite_error)?;
    let rows = statement
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
                "DurableMutationIntent" => crate::wal::WalRecordFamily::DurableMutationIntent,
                "HostedRuntimeCommitResult" => crate::wal::WalRecordFamily::HostedRuntimeCommitResult,
                "BulkCheckpointPublicationIntent" => {
                    crate::wal::WalRecordFamily::BulkCheckpointPublicationIntent
                }
                "DurablePublicationProgress" => crate::wal::WalRecordFamily::DurablePublicationProgress,
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
                durable_mutation_id: crate::wal::DurableMutationId(row.get::<_, i64>(2)? as u64),
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
    Ok(())
}

fn load_branch_records(connection: &Connection, state: &mut StoreState) -> Result<(), StoreError> {
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
            Ok(records::BranchRecord {
                branch_id: forge_relational::facade::history::BranchId(row.get::<_, String>(0)?),
                created_from_branch: row
                    .get::<_, Option<String>>(1)?
                    .map(forge_relational::facade::history::BranchId),
                created_from_commit_id: row
                    .get::<_, Option<i64>>(2)?
                    .map(|value| forge_relational::facade::history::CommitId(value as u64)),
                created_at_commit_sequence: row.get::<_, Option<i64>>(3)?.map(|value| value as u64),
            })
        })
        .map_err(sqlite_error)?;
    for row in rows {
        let record = row.map_err(sqlite_error)?;
        state.branch_records.insert(record.branch_id.0.clone(), record);
    }
    Ok(())
}

fn load_branch_head_records(
    connection: &Connection,
    state: &mut StoreState,
) -> Result<(), StoreError> {
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
            Ok(records::BranchHeadRecord {
                branch_id: forge_relational::facade::history::BranchId(row.get::<_, String>(0)?),
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
        state.branch_head_records.insert(record.branch_id.0.clone(), record);
    }
    Ok(())
}

fn load_commit_envelopes(
    connection: &Connection,
    state: &mut StoreState,
) -> Result<(), StoreError> {
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
            Ok(records::StoredCommitEnvelope {
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
    Ok(())
}

fn load_commit_parent_records(
    connection: &Connection,
    state: &mut StoreState,
) -> Result<(), StoreError> {
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
            Ok(records::CommitParentRecord {
                commit_id: forge_relational::facade::history::CommitId(row.get::<_, i64>(0)? as u64),
                parent_position: row.get::<_, i64>(1)? as usize,
                parent_commit_id: forge_relational::facade::history::CommitId(
                    row.get::<_, i64>(2)? as u64,
                ),
            })
        })
        .map_err(sqlite_error)?;
    for row in rows {
        let record = row.map_err(sqlite_error)?;
        let artifact_id = integrity::parent_artifact_id(record.commit_id, record.parent_position);
        state.commit_parent_records.insert(artifact_id, record);
    }
    Ok(())
}

fn load_digest_records(connection: &Connection, state: &mut StoreState) -> Result<(), StoreError> {
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
                "BranchRecord" => records::AuthoritativeArtifactFamily::BranchRecord,
                "BranchHeadRecord" => records::AuthoritativeArtifactFamily::BranchHeadRecord,
                "CommitEnvelope" => records::AuthoritativeArtifactFamily::CommitEnvelope,
                "CommitParentRecord" => records::AuthoritativeArtifactFamily::CommitParentRecord,
                "CommitSupportSummary" => records::AuthoritativeArtifactFamily::CommitSupportSummary,
                "SchemaSupportRecord" => records::AuthoritativeArtifactFamily::SchemaSupportRecord,
                "LineageSupportRecord" => records::AuthoritativeArtifactFamily::LineageSupportRecord,
                "DurableCursorIdentityRecord" => {
                    records::AuthoritativeArtifactFamily::DurableCursorIdentityRecord
                }
                "SubscriberCheckpointRecord" => {
                    records::AuthoritativeArtifactFamily::SubscriberCheckpointRecord
                }
                "StableBasisRecord" => records::AuthoritativeArtifactFamily::StableBasisRecord,
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
            Ok(records::AuthoritativeArtifactDigestRecord {
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
    Ok(())
}
