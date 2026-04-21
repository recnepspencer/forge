use crate::backend::records;
use crate::failure::StoreError;
use rusqlite::Connection;

use super::super::super::records::StoreState;
use super::super::helpers::{deserialize_json, sqlite_error};

pub(super) fn load_delta(
    connection: &Connection,
    state: &mut StoreState,
) -> Result<(), StoreError> {
    load_branch_shared_base_records(connection, state)?;
    load_branch_delta_layer_records(connection, state)?;
    state.backfill_missing_branch_delta_layer_artifacts()?;
    Ok(())
}

fn load_branch_shared_base_records(
    connection: &Connection,
    state: &mut StoreState,
) -> Result<(), StoreError> {
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
            Ok(records::BranchSharedBaseRecord {
                branch_id: forge_relational::facade::history::BranchId(row.get::<_, String>(0)?),
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
    Ok(())
}

fn load_branch_delta_layer_records(
    connection: &Connection,
    state: &mut StoreState,
) -> Result<(), StoreError> {
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
            Ok(records::BranchDeltaLayerRecord {
                branch_delta_layer_id: crate::delta::BranchDeltaLayerId(
                    row.get::<_, i64>(0)? as u64
                ),
                branch_id: forge_relational::facade::history::BranchId(row.get::<_, String>(1)?),
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
    Ok(())
}
