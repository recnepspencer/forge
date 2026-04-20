use crate::failure::StoreError;
use rusqlite::{params, Transaction};

use super::super::super::records::StoreState;
use super::super::helpers::{as_i64, as_i64_u64, sqlite_error};

pub(super) fn persist_delta(
    transaction: &Transaction<'_>,
    state: &StoreState,
) -> Result<(), StoreError> {
    persist_branch_shared_base_records(transaction, state)?;
    persist_branch_delta_layer_records(transaction, state)?;
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
                        &record.commit_ids.iter().map(|commit_id| commit_id.0).collect::<Vec<_>>()
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
