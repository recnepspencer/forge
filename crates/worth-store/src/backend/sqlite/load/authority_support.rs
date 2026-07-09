use crate::backend::records;
use crate::failure::StoreError;
use rusqlite::Connection;

use super::super::super::records::StoreState;
use super::super::helpers::{deserialize_json, deserialize_optional_json, sqlite_error};

pub(super) fn load_authority_support(
    connection: &Connection,
    state: &mut StoreState,
) -> Result<(), StoreError> {
    load_commit_support_summaries(connection, state)?;
    load_schema_support_records(connection, state)?;
    load_lineage_support_records(connection, state)?;
    load_durable_cursor_identity_records(connection, state)?;
    load_subscriber_checkpoint_records(connection, state)?;
    Ok(())
}

fn load_commit_support_summaries(
    connection: &Connection,
    state: &mut StoreState,
) -> Result<(), StoreError> {
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
    let rows =
        statement
            .query_map([], |row| {
                Ok(records::CommitSupportSummaryRecord {
                    commit_id: worth_relational::facade::history::CommitId(
                        row.get::<_, i64>(0)? as u64
                    ),
                    branch_id: worth_relational::facade::history::BranchId(
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
    Ok(())
}

fn load_schema_support_records(
    connection: &Connection,
    state: &mut StoreState,
) -> Result<(), StoreError> {
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
    let rows =
        statement
            .query_map([], |row| {
                let schema_transition = deserialize_optional_json::<
                    worth_relational::facade::schema::SchemaTransitionArtifact,
                >(row.get(5)?)?;
                let schema_continuation_descriptor = deserialize_optional_json::<
                    worth_relational::facade::schema::SchemaContinuationDescriptor,
                >(row.get(6)?)?;
                let schema_reconciliation_descriptor = deserialize_optional_json::<
                    worth_relational::facade::schema::SchemaReconciliationDescriptor,
                >(row.get(7)?)?;
                Ok(records::SchemaSupportRecord {
                    artifact_id: row.get(0)?,
                    commit_id: worth_relational::facade::history::CommitId(
                        row.get::<_, i64>(1)? as u64
                    ),
                    branch_id: worth_relational::facade::history::BranchId(
                        row.get::<_, String>(2)?,
                    ),
                    schema_version_id: worth_relational::facade::schema::SchemaVersionId(
                        row.get::<_, i64>(3)? as u32,
                    ),
                    descriptor_semantics_version:
                        worth_relational::facade::schema::DescriptorSemanticsVersion(
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
    Ok(())
}

fn load_lineage_support_records(
    connection: &Connection,
    state: &mut StoreState,
) -> Result<(), StoreError> {
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
    let rows =
        statement
            .query_map([], |row| {
                Ok(records::LineageSupportRecord {
                    artifact_id: row.get(0)?,
                    commit_id: worth_relational::facade::history::CommitId(
                        row.get::<_, i64>(1)? as u64
                    ),
                    branch_id: worth_relational::facade::history::BranchId(
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
    Ok(())
}

fn load_durable_cursor_identity_records(
    connection: &Connection,
    state: &mut StoreState,
) -> Result<(), StoreError> {
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
            Ok(records::DurableCursorIdentityRecord {
                artifact_id: row.get(0)?,
                cursor_id: row.get(1)?,
                subscriber_id: row.get(2)?,
                branch_id: worth_relational::facade::history::BranchId(row.get::<_, String>(3)?),
                feed_shape_id: row.get(4)?,
                schema_interpretation_id: row.get(5)?,
                cursor_semantics_version: row.get::<_, i64>(6)? as u32,
                latest_checkpoint_sequence: row.get::<_, i64>(7)? as u64,
                latest_basis_commit_id: worth_relational::facade::history::CommitId(
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
    Ok(())
}

fn load_subscriber_checkpoint_records(
    connection: &Connection,
    state: &mut StoreState,
) -> Result<(), StoreError> {
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
            Ok(records::SubscriberCheckpointRecord {
                artifact_id: row.get(0)?,
                cursor_id: row.get(1)?,
                subscriber_id: row.get(2)?,
                branch_id: worth_relational::facade::history::BranchId(row.get::<_, String>(3)?),
                feed_shape_id: row.get(4)?,
                schema_interpretation_id: row.get(5)?,
                cursor_semantics_version: row.get::<_, i64>(6)? as u32,
                checkpoint_sequence: row.get::<_, i64>(7)? as u64,
                basis_commit_id: worth_relational::facade::history::CommitId(
                    row.get::<_, i64>(8)? as u64
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
    Ok(())
}
