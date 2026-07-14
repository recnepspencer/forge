use crate::failure::StoreError;
use rusqlite::Connection;

use super::super::helpers::sqlite_error;

pub(super) fn create_tiering_schema(connection: &Connection) -> Result<(), StoreError> {
    connection
        .execute_batch(
            "
            CREATE TABLE IF NOT EXISTS tier_residency_records (
                artifact_key TEXT PRIMARY KEY,
                artifact_family TEXT NOT NULL,
                canonical_residence TEXT NOT NULL,
                canonical_replica_locator TEXT NOT NULL,
                verification_label TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS tier_transfer_records (
                artifact_key TEXT PRIMARY KEY,
                artifact_family TEXT NOT NULL,
                source_residence TEXT NOT NULL,
                target_residence TEXT NOT NULL,
                execution_origin TEXT NOT NULL,
                source_replica_locator TEXT NOT NULL,
                transferred_replica_locator TEXT,
                verification_label TEXT,
                cutover_completed INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS tier_recall_records (
                coalescing_key TEXT PRIMARY KEY,
                artifact_family TEXT NOT NULL,
                scope_class TEXT NOT NULL,
                scope_key TEXT NOT NULL,
                execution_origin TEXT NOT NULL,
                artifact_key TEXT NOT NULL,
                recall_cost_class TEXT NOT NULL,
                amplification_budget TEXT NOT NULL,
                completion_state TEXT NOT NULL
            );
            ",
        )
        .map_err(sqlite_error)?;
    Ok(())
}
