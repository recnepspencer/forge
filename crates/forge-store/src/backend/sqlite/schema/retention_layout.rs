use crate::failure::StoreError;
use rusqlite::Connection;

use super::super::helpers::sqlite_error;

pub(super) fn create_retention_layout_schema(connection: &Connection) -> Result<(), StoreError> {
    connection
        .execute_batch(
            "
            CREATE TABLE IF NOT EXISTS stable_basis_records (
                artifact_id TEXT PRIMARY KEY,
                payload_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS compaction_product_records (
                artifact_id TEXT PRIMARY KEY,
                payload_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS retention_basis_records (
                artifact_id TEXT PRIMARY KEY,
                payload_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS retention_closure_records (
                artifact_id TEXT PRIMARY KEY,
                payload_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS rebuild_debt_records (
                artifact_id TEXT PRIMARY KEY,
                payload_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS maintenance_declaration_records (
                artifact_id TEXT PRIMARY KEY,
                payload_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS maintenance_execution_records (
                artifact_id TEXT PRIMARY KEY,
                payload_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS maintenance_batch_records (
                artifact_id TEXT PRIMARY KEY,
                payload_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS maintenance_checkpoint_records (
                artifact_id TEXT PRIMARY KEY,
                payload_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS branch_shared_base_records (
                branch_id TEXT PRIMARY KEY,
                source_branch_id TEXT NOT NULL,
                source_frontier_commit_id INTEGER,
                delta_family_version INTEGER NOT NULL,
                authority_basis_digest TEXT NOT NULL,
                FOREIGN KEY(branch_id) REFERENCES branch_records(branch_id)
            );

            CREATE TABLE IF NOT EXISTS branch_delta_layer_records (
                branch_delta_layer_id INTEGER PRIMARY KEY,
                branch_id TEXT NOT NULL,
                base_frontier_commit_id INTEGER,
                target_frontier_commit_id INTEGER NOT NULL,
                commit_ids_payload TEXT NOT NULL,
                delta_family_version INTEGER NOT NULL,
                authority_basis_digest TEXT NOT NULL,
                artifacts_payload TEXT NOT NULL,
                replacement_of_layer_ids_payload TEXT NOT NULL,
                replacement_lineage_proof_payload TEXT NOT NULL,
                FOREIGN KEY(branch_id) REFERENCES branch_records(branch_id)
            );

            CREATE TABLE IF NOT EXISTS embedded_checkpoint_records (
                checkpoint_id TEXT PRIMARY KEY,
                source_runtime_id TEXT NOT NULL,
                basis_branch_id TEXT,
                basis_commit_id INTEGER,
                classification TEXT NOT NULL,
                contained_commit_ids_payload TEXT NOT NULL,
                metadata_payload TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS milestone_6_layout_materialization_records (
                artifact_id TEXT PRIMARY KEY,
                payload_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS milestone_6_commit_coupled_layout_seed_records (
                artifact_id TEXT PRIMARY KEY,
                branch_id TEXT NOT NULL,
                frontier_commit_id INTEGER NOT NULL,
                scope_class TEXT NOT NULL,
                payload_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS milestone_6_scope_slice_membership_records (
                artifact_id TEXT PRIMARY KEY,
                branch_id TEXT NOT NULL,
                frontier_commit_id INTEGER NOT NULL,
                scope_class TEXT NOT NULL,
                projection_digest TEXT NOT NULL,
                payload_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS milestone_6_chunk_membership_records (
                artifact_id TEXT PRIMARY KEY,
                physical_chunk_id TEXT NOT NULL,
                chunk_shape_version INTEGER NOT NULL,
                determinism_digest TEXT NOT NULL,
                payload_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS milestone_6_structural_block_records (
                artifact_id TEXT PRIMARY KEY,
                structural_block_id TEXT NOT NULL,
                scope_class TEXT NOT NULL,
                equivalence_contract_version INTEGER NOT NULL,
                supporting_layout_materialization_count INTEGER NOT NULL,
                payload_json TEXT NOT NULL
            );
            ",
        )
        .map_err(sqlite_error)?;
    Ok(())
}
