use crate::authority::CURRENT_CANONICALIZATION_VERSION;

use super::super::records::StoreState;

impl Default for StoreState {
    fn default() -> Self {
        Self {
            canonicalization_version: CURRENT_CANONICALIZATION_VERSION,
            next_commit_sequence: 1,
            next_head_update_sequence: 1,
            branch_records: std::collections::BTreeMap::new(),
            branch_head_records: std::collections::BTreeMap::new(),
            commit_envelopes: std::collections::BTreeMap::new(),
            commit_parent_records: std::collections::BTreeMap::new(),
            authoritative_artifact_digests: std::collections::BTreeMap::new(),
            commit_support_summaries: std::collections::BTreeMap::new(),
            schema_support_records: std::collections::BTreeMap::new(),
            lineage_support_records: std::collections::BTreeMap::new(),
            durable_cursor_identity_records: std::collections::BTreeMap::new(),
            subscriber_checkpoint_records: std::collections::BTreeMap::new(),
            branch_shared_base_records: std::collections::BTreeMap::new(),
            next_branch_delta_layer_id: 1,
            branch_delta_layer_records: std::collections::BTreeMap::new(),
            embedded_checkpoint_records: std::collections::BTreeMap::new(),
            milestone_6_layout_materialization_records: std::collections::BTreeMap::new(),
            milestone_6_scope_slice_membership_records: std::collections::BTreeMap::new(),
            milestone_6_chunk_membership_records: std::collections::BTreeMap::new(),
            milestone_6_structural_block_records: std::collections::BTreeMap::new(),
            bulk_program_identity_records: std::collections::BTreeMap::new(),
            frozen_bulk_manifest_records: std::collections::BTreeMap::new(),
            frozen_transform_basis_records: std::collections::BTreeMap::new(),
            frozen_transform_partition_records: std::collections::BTreeMap::new(),
            bulk_deterministic_plan_records: std::collections::BTreeMap::new(),
            bulk_progress_checkpoint_records: std::collections::BTreeMap::new(),
            bulk_chunk_witness_records: std::collections::BTreeMap::new(),
            program_chunk_witness_index_records: std::collections::BTreeMap::new(),
            next_snapshot_id: 1,
            snapshot_basis_records: std::collections::BTreeMap::new(),
            snapshot_image_records: std::collections::BTreeMap::new(),
            next_durable_mutation_id: 1,
            next_wal_sequence: 1,
            wal_records: std::collections::BTreeMap::new(),
        }
    }
}
