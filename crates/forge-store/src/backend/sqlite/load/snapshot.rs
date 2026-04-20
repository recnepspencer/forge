use crate::backend::records;
use crate::failure::StoreError;
use rusqlite::Connection;

use super::super::super::records::StoreState;
use super::super::helpers::sqlite_error;

pub(super) fn load_snapshot(
    connection: &Connection,
    state: &mut StoreState,
) -> Result<(), StoreError> {
    load_snapshot_basis_records(connection, state)?;
    load_snapshot_image_records(connection, state)?;
    Ok(())
}

fn load_snapshot_basis_records(
    connection: &Connection,
    state: &mut StoreState,
) -> Result<(), StoreError> {
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
            let history_range = serde_json::from_str::<Vec<u64>>(&history_range_payload).map_err(
                |error| {
                    rusqlite::Error::FromSqlConversionFailure(
                        6,
                        rusqlite::types::Type::Text,
                        Box::new(error),
                    )
                },
            )?;
            Ok(records::SnapshotBasisRecord {
                snapshot_id: crate::snapshot::SnapshotId(row.get::<_, i64>(0)? as u64),
                snapshot_family_version: row.get::<_, i64>(1)? as u32,
                snapshot_basis_version: row.get::<_, i64>(2)? as u32,
                snapshot_image_format_version: row.get::<_, i64>(3)? as u32,
                snapshot_branch_id: forge_relational::facade::history::BranchId(row.get::<_, String>(4)?),
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
        state.snapshot_basis_records.insert(record.snapshot_id.0, record);
    }
    Ok(())
}

fn load_snapshot_image_records(
    connection: &Connection,
    state: &mut StoreState,
) -> Result<(), StoreError> {
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
            Ok(records::SnapshotImageRecord {
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
        state.snapshot_image_records.insert(record.snapshot_id.0, record);
    }
    Ok(())
}
