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
            embedded_checkpoint_records: std::collections::BTreeMap::new(),
            next_snapshot_id: 1,
            snapshot_basis_records: std::collections::BTreeMap::new(),
            snapshot_image_records: std::collections::BTreeMap::new(),
            next_durable_mutation_id: 1,
            next_wal_sequence: 1,
            wal_records: std::collections::BTreeMap::new(),
        }
    }
}
