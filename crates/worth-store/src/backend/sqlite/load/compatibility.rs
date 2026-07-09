use crate::backend::records::CompatibilityManifestRecord;
use crate::failure::StoreError;
use rusqlite::Connection;

use super::super::super::records::StoreState;
use super::super::helpers::{deserialize_json, sqlite_error};

pub(super) fn load_compatibility(
    connection: &Connection,
    state: &mut StoreState,
) -> Result<(), StoreError> {
    state.compatibility_manifest_records.clear();
    let mut statement = connection
        .prepare("SELECT payload_json FROM compatibility_manifest_records ORDER BY artifact_id")
        .map_err(sqlite_error)?;
    let rows = statement
        .query_map([], |row| deserialize_json(row.get(0)?))
        .map_err(sqlite_error)?;
    for row in rows {
        let record: CompatibilityManifestRecord = row.map_err(sqlite_error)?;
        state
            .compatibility_manifest_records
            .insert(record.artifact_id.clone(), record);
    }
    Ok(())
}
