use crate::failure::StoreError;
use rusqlite::Connection;

use super::super::helpers::sqlite_error;

pub(super) fn create_indexes(connection: &Connection) -> Result<(), StoreError> {
    connection
        .execute_batch(
            "
            CREATE INDEX IF NOT EXISTS idx_commit_envelopes_branch_sequence
            ON commit_envelopes(branch_id, commit_sequence);
            CREATE INDEX IF NOT EXISTS idx_commit_parent_records_commit_position
            ON commit_parent_records(commit_id, parent_position);
            CREATE INDEX IF NOT EXISTS idx_authoritative_artifact_digests_family_id
            ON authoritative_artifact_digests(artifact_family, artifact_id);
            CREATE INDEX IF NOT EXISTS idx_commit_support_summaries_branch_commit
            ON commit_support_summaries(branch_id, commit_id);
            CREATE INDEX IF NOT EXISTS idx_schema_support_records_branch_commit
            ON schema_support_records(branch_id, commit_id);
            CREATE INDEX IF NOT EXISTS idx_lineage_support_records_branch_commit
            ON lineage_support_records(branch_id, commit_id);
            CREATE INDEX IF NOT EXISTS idx_durable_cursor_identity_records_cursor
            ON durable_cursor_identity_records(cursor_id);
            CREATE INDEX IF NOT EXISTS idx_subscriber_checkpoint_records_cursor_sequence
            ON subscriber_checkpoint_records(cursor_id, checkpoint_sequence);
            CREATE INDEX IF NOT EXISTS idx_branch_shared_base_records_source_branch
            ON branch_shared_base_records(source_branch_id);
            CREATE INDEX IF NOT EXISTS idx_branch_delta_layer_records_branch_target
            ON branch_delta_layer_records(branch_id, target_frontier_commit_id);
            CREATE INDEX IF NOT EXISTS idx_embedded_checkpoint_records_basis_commit
            ON embedded_checkpoint_records(basis_commit_id);
            CREATE INDEX IF NOT EXISTS idx_milestone_6_layout_materialization_records_artifact
            ON milestone_6_layout_materialization_records(artifact_id);
            CREATE INDEX IF NOT EXISTS idx_milestone_6_commit_coupled_layout_seed_records_scope
            ON milestone_6_commit_coupled_layout_seed_records(branch_id, frontier_commit_id, scope_class);
            CREATE INDEX IF NOT EXISTS idx_milestone_6_scope_slice_membership_records_scope
            ON milestone_6_scope_slice_membership_records(branch_id, frontier_commit_id, scope_class, projection_digest);
            CREATE INDEX IF NOT EXISTS idx_milestone_6_chunk_membership_records_chunk
            ON milestone_6_chunk_membership_records(physical_chunk_id, chunk_shape_version, determinism_digest);
            CREATE INDEX IF NOT EXISTS idx_milestone_6_structural_block_records_block
            ON milestone_6_structural_block_records(structural_block_id, scope_class, equivalence_contract_version);
            CREATE INDEX IF NOT EXISTS idx_bulk_manifest_program
            ON frozen_bulk_manifest_records(program_id, manifest_digest);
            CREATE INDEX IF NOT EXISTS idx_bulk_transform_basis_program
            ON frozen_transform_basis_records(program_id, basis_digest);
            CREATE INDEX IF NOT EXISTS idx_bulk_transform_partition_program
            ON frozen_transform_partition_records(program_id, partition_digest);
            CREATE INDEX IF NOT EXISTS idx_bulk_plan_program
            ON bulk_deterministic_plan_records(program_id, plan_id);
            CREATE INDEX IF NOT EXISTS idx_bulk_checkpoint_program
            ON bulk_progress_checkpoint_records(program_id, plan_id, checkpoint_sequence);
            CREATE INDEX IF NOT EXISTS idx_bulk_witness_program
            ON bulk_chunk_witness_records(program_id, plan_id, chunk_ordinal);
            CREATE INDEX IF NOT EXISTS idx_snapshot_basis_branch_frontier
            ON snapshot_basis_records(snapshot_branch_id, snapshot_frontier_commit_id);
            CREATE INDEX IF NOT EXISTS idx_tier_residency_records_family
            ON tier_residency_records(artifact_family);
            CREATE INDEX IF NOT EXISTS idx_tier_transfer_records_family_origin
            ON tier_transfer_records(artifact_family, execution_origin);
            CREATE INDEX IF NOT EXISTS idx_tier_recall_records_family_scope
            ON tier_recall_records(artifact_family, scope_class, execution_origin);
            CREATE INDEX IF NOT EXISTS idx_wal_records_mutation_sequence
            ON wal_records(durable_mutation_id, wal_sequence);
            ",
        )
        .map_err(sqlite_error)?;
    Ok(())
}
