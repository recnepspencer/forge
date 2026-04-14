use crate::authority::VerifiedAuthoritativeAppend;
use crate::failure::StoreError;
use forge_relational::facade::history::CommitId;

use crate::backend::{
    integrity::{branch_key, commit_artifact_id, parent_artifact_id, stable_structural_digest},
    records::{
        AuthoritativeArtifactFamily, BranchHeadRecord, BranchRecord, CommitParentRecord,
        StoreState, StoredCommitEnvelope,
    },
};

impl StoreState {
    pub fn stage_verified_append(
        &self,
        verified: &VerifiedAuthoritativeAppend,
    ) -> Result<Self, StoreError> {
        let mut next = self.clone();
        let envelope = verified.envelope();
        let commit_id = envelope.commit.commit_id;
        let branch_identity = branch_key(&envelope.branch_context);

        if !next.branch_records.contains_key(&branch_identity) {
            next.branch_records.insert(
                branch_identity.clone(),
                BranchRecord {
                    branch_id: envelope.branch_context.clone(),
                    created_from_branch: None,
                    created_from_commit_id: None,
                    created_at_commit_sequence: Some(next.next_commit_sequence),
                },
            );
            next.branch_head_records.insert(
                branch_identity.clone(),
                BranchHeadRecord {
                    branch_id: envelope.branch_context.clone(),
                    head_commit_id: None,
                    head_commit_digest: None,
                    head_update_sequence: 0,
                },
            );
            next.upsert_digest_record(
                AuthoritativeArtifactFamily::BranchRecord,
                branch_identity.clone(),
                stable_structural_digest(&next.branch_records[&branch_identity])?,
            );
            next.upsert_digest_record(
                AuthoritativeArtifactFamily::BranchHeadRecord,
                branch_identity.clone(),
                stable_structural_digest(&next.branch_head_records[&branch_identity])?,
            );
        }

        let commit_sequence = next.next_commit_sequence;
        next.next_commit_sequence += 1;
        let head_update_sequence = next.next_head_update_sequence;
        next.next_head_update_sequence += 1;

        next.commit_envelopes.insert(
            commit_id.0,
            StoredCommitEnvelope {
                envelope: envelope.clone(),
                envelope_digest: verified.digest().as_str().to_string(),
                canonicalization_version: verified.canonicalization_version(),
                commit_sequence,
            },
        );
        next.upsert_digest_record(
            AuthoritativeArtifactFamily::CommitEnvelope,
            commit_artifact_id(commit_id),
            verified.digest().as_str().to_string(),
        );

        for (parent_position, parent_commit_id) in
            envelope.commit.parents.iter().copied().enumerate()
        {
            let parent_record = CommitParentRecord {
                commit_id,
                parent_position,
                parent_commit_id,
            };
            next.commit_parent_records.insert(
                parent_artifact_id(commit_id, parent_position),
                parent_record.clone(),
            );
            next.upsert_digest_record(
                AuthoritativeArtifactFamily::CommitParentRecord,
                parent_artifact_id(commit_id, parent_position),
                stable_structural_digest(&parent_record)?,
            );
        }

        next.branch_head_records.insert(
            branch_identity.clone(),
            BranchHeadRecord {
                branch_id: envelope.branch_context.clone(),
                head_commit_id: Some(commit_id),
                head_commit_digest: Some(verified.digest().as_str().to_string()),
                head_update_sequence,
            },
        );
        next.upsert_digest_record(
            AuthoritativeArtifactFamily::BranchHeadRecord,
            branch_identity.clone(),
            stable_structural_digest(&next.branch_head_records[&branch_identity])?,
        );

        Ok(next)
    }

    pub fn has_commit(&self, commit_id: CommitId) -> bool {
        self.commit_envelopes.contains_key(&commit_id.0)
    }
}
