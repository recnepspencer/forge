use crate::failure::StoreError;
use rusqlite::Connection;

use super::super::super::helpers::sqlite_error;

pub(super) fn create_subscription_support_tables(
    connection: &Connection,
) -> Result<(), StoreError> {
    connection
        .execute_batch(
            "
            CREATE TABLE IF NOT EXISTS subscription_support_record_sets (
                storage_key TEXT PRIMARY KEY,
                family_id TEXT NOT NULL,
                artifact_id TEXT NOT NULL,
                declaration_digest TEXT NOT NULL DEFAULT '',
                basis_digest TEXT NOT NULL DEFAULT '',
                cursor_digest TEXT NOT NULL DEFAULT '',
                checkpoint_digest TEXT NOT NULL DEFAULT '',
                compatibility_digest TEXT NOT NULL DEFAULT '',
                initial_classification TEXT,
                restart_shard TEXT,
                payload_json TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS subscription_support_counter_snapshot (
                counter_id TEXT PRIMARY KEY,
                payload_json TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS subscription_support_action_records (
                action_id TEXT PRIMARY KEY,
                artifact_id TEXT NOT NULL DEFAULT '',
                action_origin TEXT NOT NULL DEFAULT '',
                publication_state TEXT NOT NULL DEFAULT '',
                payload_json TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS subscription_support_maintenance_descriptor_records (
                record_key TEXT PRIMARY KEY,
                family_id TEXT NOT NULL,
                support_role TEXT NOT NULL,
                maintenance_key TEXT NOT NULL,
                declaration_id TEXT NOT NULL,
                descriptor_digest TEXT NOT NULL,
                payload_json TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS subscription_support_maintenance_debt_records (
                record_key TEXT PRIMARY KEY,
                action_id TEXT NOT NULL DEFAULT '',
                family_id TEXT NOT NULL DEFAULT '',
                support_role TEXT NOT NULL DEFAULT '',
                verdict TEXT NOT NULL DEFAULT '',
                payload_json TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS subscription_support_access_structure_state (
                state_id TEXT PRIMARY KEY,
                verified INTEGER NOT NULL CHECK (verified IN (0, 1)),
                debted_json TEXT NOT NULL DEFAULT '[]'
            );
            INSERT OR IGNORE INTO subscription_support_access_structure_state
                (state_id, verified, debted_json) VALUES ('first_ship', 1, '[]');
            ",
        )
        .map_err(sqlite_error)?;
    Ok(())
}
