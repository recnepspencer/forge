use crate::failure::StoreError;
use rusqlite::Connection;

use super::super::helpers::sqlite_error;

pub(super) fn create_authority_schema(connection: &Connection) -> Result<(), StoreError> {
    connection
        .execute_batch(
            "
            CREATE TABLE IF NOT EXISTS store_meta (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS branch_records (
                branch_id TEXT PRIMARY KEY,
                created_from_branch TEXT,
                created_from_commit_id INTEGER,
                created_at_commit_sequence INTEGER
            );

            CREATE TABLE IF NOT EXISTS commit_envelopes (
                commit_id INTEGER PRIMARY KEY,
                branch_id TEXT NOT NULL,
                commit_sequence INTEGER NOT NULL UNIQUE,
                canonicalization_version INTEGER NOT NULL,
                envelope_payload TEXT NOT NULL,
                envelope_digest TEXT NOT NULL,
                FOREIGN KEY(branch_id) REFERENCES branch_records(branch_id)
            );

            CREATE TABLE IF NOT EXISTS commit_parent_records (
                commit_id INTEGER NOT NULL,
                parent_position INTEGER NOT NULL,
                parent_commit_id INTEGER NOT NULL,
                PRIMARY KEY(commit_id, parent_position),
                FOREIGN KEY(commit_id) REFERENCES commit_envelopes(commit_id),
                FOREIGN KEY(parent_commit_id) REFERENCES commit_envelopes(commit_id)
            );

            CREATE TABLE IF NOT EXISTS branch_head_records (
                branch_id TEXT PRIMARY KEY,
                head_commit_id INTEGER,
                head_commit_digest TEXT,
                head_update_sequence INTEGER NOT NULL,
                FOREIGN KEY(branch_id) REFERENCES branch_records(branch_id),
                FOREIGN KEY(head_commit_id) REFERENCES commit_envelopes(commit_id)
            );

            CREATE TABLE IF NOT EXISTS authoritative_artifact_digests (
                artifact_family TEXT NOT NULL,
                artifact_id TEXT NOT NULL,
                canonicalization_version INTEGER NOT NULL,
                digest_algorithm TEXT NOT NULL,
                artifact_digest TEXT NOT NULL,
                PRIMARY KEY(artifact_family, artifact_id, canonicalization_version)
            );

            CREATE TABLE IF NOT EXISTS commit_support_summaries (
                commit_id INTEGER PRIMARY KEY,
                branch_id TEXT NOT NULL,
                schema_support_artifact_id TEXT,
                lineage_support_artifact_id TEXT,
                milestone_6_published_layout_request_artifact_ids_payload TEXT NOT NULL,
                emitted_schema_artifact INTEGER NOT NULL,
                emitted_lineage_artifact INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS schema_support_records (
                artifact_id TEXT PRIMARY KEY,
                commit_id INTEGER NOT NULL UNIQUE,
                branch_id TEXT NOT NULL,
                schema_version_id INTEGER NOT NULL,
                descriptor_semantics_version INTEGER NOT NULL,
                schema_transition_payload TEXT,
                schema_continuation_descriptor_payload TEXT,
                schema_reconciliation_descriptor_payload TEXT,
                FOREIGN KEY(commit_id) REFERENCES commit_envelopes(commit_id)
            );

            CREATE TABLE IF NOT EXISTS lineage_support_records (
                artifact_id TEXT PRIMARY KEY,
                commit_id INTEGER NOT NULL UNIQUE,
                branch_id TEXT NOT NULL,
                lineage_event_ids_payload TEXT NOT NULL,
                lineage_events_payload TEXT NOT NULL,
                lineage_digest_basis_payload TEXT NOT NULL,
                event_batch_digest_basis_payload TEXT NOT NULL,
                decision_log_digest_basis_payload TEXT NOT NULL,
                lineage_artifact_counters_payload TEXT NOT NULL,
                FOREIGN KEY(commit_id) REFERENCES commit_envelopes(commit_id)
            );

            CREATE TABLE IF NOT EXISTS durable_cursor_identity_records (
                artifact_id TEXT PRIMARY KEY,
                cursor_id TEXT NOT NULL UNIQUE,
                subscriber_id TEXT NOT NULL,
                branch_id TEXT NOT NULL,
                feed_shape_id TEXT NOT NULL,
                schema_interpretation_id TEXT NOT NULL,
                cursor_semantics_version INTEGER NOT NULL,
                latest_checkpoint_sequence INTEGER NOT NULL,
                latest_basis_commit_id INTEGER NOT NULL,
                latest_schema_support_artifact_id TEXT,
                FOREIGN KEY(latest_basis_commit_id) REFERENCES commit_envelopes(commit_id)
            );

            CREATE TABLE IF NOT EXISTS subscriber_checkpoint_records (
                artifact_id TEXT PRIMARY KEY,
                cursor_id TEXT NOT NULL,
                subscriber_id TEXT NOT NULL,
                branch_id TEXT NOT NULL,
                feed_shape_id TEXT NOT NULL,
                schema_interpretation_id TEXT NOT NULL,
                cursor_semantics_version INTEGER NOT NULL,
                checkpoint_sequence INTEGER NOT NULL,
                basis_commit_id INTEGER NOT NULL,
                schema_support_artifact_id TEXT,
                UNIQUE(cursor_id, checkpoint_sequence),
                FOREIGN KEY(basis_commit_id) REFERENCES commit_envelopes(commit_id)
            );
            ",
        )
        .map_err(sqlite_error)?;
    Ok(())
}
