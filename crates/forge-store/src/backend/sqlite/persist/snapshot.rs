use crate::failure::StoreError;
use rusqlite::{params, Transaction};

use super::super::super::records::StoreState;
use super::super::helpers::{as_i64, as_i64_u64, sqlite_error};

pub(super) fn persist_snapshot(
    transaction: &Transaction<'_>,
    state: &StoreState,
) -> Result<(), StoreError> {
    persist_snapshot_basis_records(transaction, state)?;
    persist_snapshot_image_records(transaction, state)?;
    persist_wal_records(transaction, state)?;
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
