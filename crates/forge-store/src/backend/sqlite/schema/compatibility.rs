use crate::failure::StoreError;
use rusqlite::Connection;

use super::super::helpers::sqlite_error;

pub(super) fn create_compatibility_schema(connection: &Connection) -> Result<(), StoreError> {
    connection
        .execute_batch(
            "
            CREATE TABLE IF NOT EXISTS compatibility_manifest_records (
                artifact_id TEXT PRIMARY KEY,
                payload_json TEXT NOT NULL
            );
            ",
        )
        .map_err(sqlite_error)?;
    Ok(())
}
