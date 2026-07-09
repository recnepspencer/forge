use crate::failure::{StoreError, StoreErrorKind};
use worth_relational::facade::history::BranchId;

use crate::backend::{
    integrity::{branch_key, digest_artifact_key, stable_structural_digest},
    records::{AuthoritativeArtifactFamily, BranchHeadRecord, BranchRecord, StoreState},
};

#[derive(Debug)]
pub(crate) struct AppliedBranchCreation {
    branch_identity: String,
}

impl StoreState {
    pub(crate) fn branch_head_record(&self, branch_id: &BranchId) -> Option<&BranchHeadRecord> {
        self.branch_head_records.get(&branch_key(branch_id))
    }

    pub fn apply_branch_creation_in_place(
        &mut self,
        new_branch: BranchId,
        from_branch: Option<&BranchId>,
    ) -> Result<AppliedBranchCreation, StoreError> {
        let new_branch_key = branch_key(&new_branch);
        if self.branch_records.contains_key(&new_branch_key) {
            return Err(StoreError::new(
                StoreErrorKind::DuplicateArtifactIdentity,
                format!("branch `{}` already exists", new_branch.0),
            ));
        }

        let source_head = match from_branch {
            Some(source) => self
                .branch_head_records
                .get(&branch_key(source))
                .cloned()
                .ok_or_else(|| StoreError::unknown_branch(source))?,
            None => BranchHeadRecord {
                branch_id: new_branch.clone(),
                head_commit_id: None,
                head_commit_digest: None,
                head_update_sequence: 0,
            },
        };

        let created_at_commit_sequence = source_head
            .head_commit_id
            .and_then(|commit_id| self.commit_envelopes.get(&commit_id.0))
            .map(|record| record.commit_sequence);

        self.branch_records.insert(
            new_branch_key.clone(),
            BranchRecord {
                branch_id: new_branch.clone(),
                created_from_branch: from_branch.cloned(),
                created_from_commit_id: source_head.head_commit_id,
                created_at_commit_sequence,
            },
        );
        self.branch_head_records.insert(
            new_branch_key.clone(),
            BranchHeadRecord {
                branch_id: new_branch.clone(),
                head_commit_id: source_head.head_commit_id,
                head_commit_digest: source_head.head_commit_digest,
                head_update_sequence: source_head.head_update_sequence,
            },
        );
        self.upsert_digest_record(
            AuthoritativeArtifactFamily::BranchRecord,
            new_branch_key.clone(),
            stable_structural_digest(&self.branch_records[&new_branch_key])?,
        );
        self.upsert_digest_record(
            AuthoritativeArtifactFamily::BranchHeadRecord,
            new_branch_key.clone(),
            stable_structural_digest(&self.branch_head_records[&new_branch_key])?,
        );
        Ok(AppliedBranchCreation {
            branch_identity: new_branch_key,
        })
    }

    pub fn rollback_branch_creation(&mut self, applied: AppliedBranchCreation) {
        self.branch_records.remove(&applied.branch_identity);
        self.branch_head_records.remove(&applied.branch_identity);
        self.authoritative_artifact_digests
            .remove(&digest_artifact_key(
                &AuthoritativeArtifactFamily::BranchRecord,
                &applied.branch_identity,
                self.canonicalization_version,
            ));
        self.authoritative_artifact_digests
            .remove(&digest_artifact_key(
                &AuthoritativeArtifactFamily::BranchHeadRecord,
                &applied.branch_identity,
                self.canonicalization_version,
            ));
    }

    pub fn verify_applied_branch_creation(
        &self,
        applied: &AppliedBranchCreation,
    ) -> Result<(), StoreError> {
        self.verify_branch_record(&applied.branch_identity)?;
        self.verify_branch_head_record(&applied.branch_identity)?;
        Ok(())
    }
}
