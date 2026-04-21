use crate::failure::StoreError;
use rusqlite::{params, Transaction};

use super::super::super::records::StoreState;
use super::super::helpers::{as_i64, serialize_optional_json, sqlite_error};

pub(super) fn persist_authority_support(
    transaction: &Transaction<'_>,
    state: &StoreState,
) -> Result<(), StoreError> {
    persist_commit_support_summaries(transaction, state)?;
    persist_schema_support_records(transaction, state)?;
    persist_lineage_support_records(transaction, state)?;
    persist_durable_cursor_identity_records(transaction, state)?;
    persist_subscriber_checkpoint_records(transaction, state)?;
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
                    )?,
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
