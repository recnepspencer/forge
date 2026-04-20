use crate::failure::StoreError;
use rusqlite::Connection;

use super::super::helpers::sqlite_error;

pub(super) fn create_bulk_snapshot_schema(connection: &Connection) -> Result<(), StoreError> {
    connection
        .execute_batch(
            "
            CREATE TABLE IF NOT EXISTS bulk_program_identity_records (
                artifact_id TEXT PRIMARY KEY,
                program_id TEXT NOT NULL UNIQUE,
                kind TEXT NOT NULL,
                payload_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS frozen_bulk_manifest_records (
                artifact_id TEXT PRIMARY KEY,
                program_id TEXT NOT NULL,
                manifest_digest TEXT NOT NULL,
                payload_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS frozen_transform_basis_records (
                artifact_id TEXT PRIMARY KEY,
                program_id TEXT NOT NULL,
                basis_digest TEXT NOT NULL,
                payload_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS frozen_transform_partition_records (
                artifact_id TEXT PRIMARY KEY,
                program_id TEXT NOT NULL,
                partition_digest TEXT NOT NULL,
                payload_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS bulk_deterministic_plan_records (
                artifact_id TEXT PRIMARY KEY,
                program_id TEXT NOT NULL,
                plan_id TEXT NOT NULL,
                payload_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS bulk_progress_checkpoint_records (
                artifact_id TEXT PRIMARY KEY,
                program_id TEXT NOT NULL,
                plan_id TEXT NOT NULL,
                checkpoint_sequence INTEGER NOT NULL,
                payload_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS bulk_chunk_witness_records (
                artifact_id TEXT PRIMARY KEY,
                program_id TEXT NOT NULL,
                plan_id TEXT NOT NULL,
                chunk_ordinal INTEGER NOT NULL,
                payload_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS program_chunk_witness_index_records (
                artifact_id TEXT PRIMARY KEY,
                program_id TEXT NOT NULL,
                plan_id TEXT NOT NULL,
                payload_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS snapshot_basis_records (
                snapshot_id INTEGER PRIMARY KEY,
                snapshot_family_version INTEGER NOT NULL,
                snapshot_basis_version INTEGER NOT NULL,
                snapshot_image_format_version INTEGER NOT NULL,
                snapshot_branch_id TEXT NOT NULL,
                snapshot_frontier_commit_id INTEGER NOT NULL,
                snapshot_history_range_payload TEXT NOT NULL,
                snapshot_canonicalization_version INTEGER NOT NULL,
                snapshot_authority_digest TEXT NOT NULL,
                snapshot_image_digest TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS snapshot_image_records (
                snapshot_id INTEGER PRIMARY KEY,
                image_payload TEXT NOT NULL,
                FOREIGN KEY(snapshot_id) REFERENCES snapshot_basis_records(snapshot_id)
            );

            CREATE TABLE IF NOT EXISTS wal_records (
                wal_sequence INTEGER PRIMARY KEY,
                family TEXT NOT NULL,
                durable_mutation_id INTEGER NOT NULL,
                runtime_session_id TEXT NOT NULL,
                wal_version INTEGER NOT NULL,
                record_digest TEXT NOT NULL,
                payload_json TEXT NOT NULL
            );
            ",
        )
        .map_err(sqlite_error)?;
    Ok(())
}
