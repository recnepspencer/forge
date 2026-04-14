use crate::authority::AuthoritativeExportBundle;

use super::records::StoreState;

impl StoreState {
    pub fn authoritative_export_bundle(&self) -> AuthoritativeExportBundle {
        let mut bundle = AuthoritativeExportBundle {
            canonicalization_version: self.canonicalization_version,
            branch_records: self.branch_records.values().cloned().collect(),
            branch_head_records: self.branch_head_records.values().cloned().collect(),
            commit_envelopes: self.commit_envelopes.values().cloned().collect(),
            commit_parent_records: self.commit_parent_records.values().cloned().collect(),
            authoritative_artifact_digests: self
                .authoritative_artifact_digests
                .values()
                .cloned()
                .collect(),
        };
        bundle.canonicalize_order();
        bundle
    }
}
