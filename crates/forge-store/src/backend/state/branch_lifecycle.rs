use crate::failure::{StoreError, StoreErrorKind};
use forge_relational::facade::history::BranchId;

use crate::backend::{
    integrity::{branch_key, stable_structural_digest},
    records::{AuthoritativeArtifactFamily, BranchHeadRecord, BranchRecord, StoreState},
};

impl StoreState {
    pub fn stage_branch_creation(
        &self,
        new_branch: BranchId,
        from_branch: Option<&BranchId>,
    ) -> Result<Self, StoreError> {
        let new_branch_key = branch_key(&new_branch);
        if self.branch_records.contains_key(&new_branch_key) {
            return Err(StoreError::new(
                StoreErrorKind::DuplicateArtifactIdentity,
                format!("branch `{}` already exists", new_branch.0),
            ));
        }

        let mut next = self.clone();
        let source_head = match from_branch {
            Some(source) => next
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
            .and_then(|commit_id| next.commit_envelopes.get(&commit_id.0))
            .map(|record| record.commit_sequence);

        next.branch_records.insert(
            new_branch_key.clone(),
            BranchRecord {
                branch_id: new_branch.clone(),
                created_from_branch: from_branch.cloned(),
                created_from_commit_id: source_head.head_commit_id,
                created_at_commit_sequence,
            },
        );
        next.branch_head_records.insert(
            new_branch_key,
            BranchHeadRecord {
                branch_id: new_branch.clone(),
                head_commit_id: source_head.head_commit_id,
                head_commit_digest: source_head.head_commit_digest,
                head_update_sequence: source_head.head_update_sequence,
            },
        );
        next.upsert_digest_record(
            AuthoritativeArtifactFamily::BranchRecord,
            branch_key(&new_branch),
            stable_structural_digest(&next.branch_records[&branch_key(&new_branch)])?,
        );
        next.upsert_digest_record(
            AuthoritativeArtifactFamily::BranchHeadRecord,
            branch_key(&new_branch),
            stable_structural_digest(&next.branch_head_records[&branch_key(&new_branch)])?,
        );
        Ok(next)
    }
}
