use crate::failure::StoreError;
use rusqlite::{params, Transaction};

use super::super::super::records::StoreState;
use super::super::helpers::{as_i64, as_i64_u64, sqlite_error};

pub(super) fn persist_authority_primary(
    transaction: &Transaction<'_>,
    state: &StoreState,
) -> Result<(), StoreError> {
    persist_branch_records(transaction, state)?;
    persist_commit_envelopes(transaction, state)?;
    persist_commit_parent_records(transaction, state)?;
    persist_branch_head_records(transaction, state)?;
    persist_digest_records(transaction, state)?;
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
                    record.created_from_branch.as_ref().map(|value| value.0.clone()),
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
