use crate::backend::records::{
    AuthoritativeArtifactDigestRecord, BranchHeadRecord, BranchRecord, CommitParentRecord,
    StoredCommitEnvelope,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthoritativeExportBundle {
    pub(crate) canonicalization_version: u32,
    pub(crate) branch_records: Vec<BranchRecord>,
    pub(crate) branch_head_records: Vec<BranchHeadRecord>,
    pub(crate) commit_envelopes: Vec<StoredCommitEnvelope>,
    pub(crate) commit_parent_records: Vec<CommitParentRecord>,
    pub(crate) authoritative_artifact_digests: Vec<AuthoritativeArtifactDigestRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthoritativeExportRestoreRequest {
    bundle: AuthoritativeExportBundle,
}

impl AuthoritativeExportBundle {
    pub(crate) fn canonicalize_order(&mut self) {
        self.branch_records
            .sort_by(|left, right| left.branch_id.cmp(&right.branch_id));
        self.branch_head_records
            .sort_by(|left, right| left.branch_id.cmp(&right.branch_id));
        self.commit_envelopes
            .sort_by_key(|record| record.commit_sequence);
        self.commit_parent_records.sort_by(|left, right| {
            left.commit_id
                .cmp(&right.commit_id)
                .then(left.parent_position.cmp(&right.parent_position))
                .then(left.parent_commit_id.cmp(&right.parent_commit_id))
        });
        self.authoritative_artifact_digests.sort();
    }

    pub(crate) fn into_canonicalized(mut self) -> Self {
        self.canonicalize_order();
        self
    }

    pub fn canonical_json(&self) -> String {
        serde_json::to_string(&self.clone().into_canonicalized())
            .expect("canonical authoritative export serialization")
    }

    pub fn admit_restore(self) -> AuthoritativeExportRestoreRequest {
        AuthoritativeExportRestoreRequest { bundle: self }
    }
}

impl AuthoritativeExportRestoreRequest {
    pub(crate) fn into_bundle(self) -> AuthoritativeExportBundle {
        self.bundle
    }
}
