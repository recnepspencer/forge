use crate::authority::VerifiedAuthoritativeAppend;
use crate::failure::StoreError;
use forge_relational::facade::history::CommitId;

use crate::backend::{
    integrity::{
        branch_key, commit_artifact_id, digest_artifact_key, parent_artifact_id,
        stable_structural_digest,
    },
    records::{
        AuthoritativeArtifactFamily, BranchHeadRecord, BranchRecord, CommitParentRecord,
        StoreState, StoredCommitEnvelope,
    },
};

#[derive(Debug)]
pub(crate) struct AppliedAuthoritativeAppend {
    branch_identity: String,
    commit_id: CommitId,
    parent_count: usize,
    created_branch: bool,
    previous_next_commit_sequence: u64,
    previous_next_head_update_sequence: u64,
    previous_branch_record: Option<BranchRecord>,
    previous_branch_head_record: Option<BranchHeadRecord>,
}

impl StoreState {
    pub fn has_commit(&self, commit_id: CommitId) -> bool {
        self.commit_envelopes.contains_key(&commit_id.0)
    }

    pub fn apply_verified_append_in_place(
        &mut self,
        verified: &VerifiedAuthoritativeAppend,
    ) -> Result<AppliedAuthoritativeAppend, StoreError> {
        let envelope = verified.envelope();
        let commit_id = envelope.commit.commit_id;
        let branch_identity = branch_key(&envelope.branch_context);
        let previous_branch_record = self.branch_records.get(&branch_identity).cloned();
        let previous_branch_head_record = self.branch_head_records.get(&branch_identity).cloned();
        let created_branch = previous_branch_record.is_none();

        if created_branch {
            self.branch_records.insert(
                branch_identity.clone(),
                BranchRecord {
                    branch_id: envelope.branch_context.clone(),
                    created_from_branch: None,
                    created_from_commit_id: None,
                    created_at_commit_sequence: Some(self.next_commit_sequence),
                },
            );
            self.branch_head_records.insert(
                branch_identity.clone(),
                BranchHeadRecord {
                    branch_id: envelope.branch_context.clone(),
                    head_commit_id: None,
                    head_commit_digest: None,
                    head_update_sequence: 0,
                },
            );
            self.upsert_digest_record(
                AuthoritativeArtifactFamily::BranchRecord,
                branch_identity.clone(),
                stable_structural_digest(&self.branch_records[&branch_identity])?,
            );
            self.upsert_digest_record(
                AuthoritativeArtifactFamily::BranchHeadRecord,
                branch_identity.clone(),
                stable_structural_digest(&self.branch_head_records[&branch_identity])?,
            );
        }

        let previous_next_commit_sequence = self.next_commit_sequence;
        let previous_next_head_update_sequence = self.next_head_update_sequence;
        let commit_sequence = self.next_commit_sequence;
        self.next_commit_sequence += 1;
        let head_update_sequence = self.next_head_update_sequence;
        self.next_head_update_sequence += 1;

        self.commit_envelopes.insert(
            commit_id.0,
            StoredCommitEnvelope {
                envelope: envelope.clone(),
                envelope_digest: verified.digest().as_str().to_string(),
                canonicalization_version: verified.canonicalization_version(),
                commit_sequence,
            },
        );
        self.upsert_digest_record(
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
            self.commit_parent_records.insert(
                parent_artifact_id(commit_id, parent_position),
                parent_record.clone(),
            );
            self.upsert_digest_record(
                AuthoritativeArtifactFamily::CommitParentRecord,
                parent_artifact_id(commit_id, parent_position),
                stable_structural_digest(&parent_record)?,
            );
        }

        self.branch_head_records.insert(
            branch_identity.clone(),
            BranchHeadRecord {
                branch_id: envelope.branch_context.clone(),
                head_commit_id: Some(commit_id),
                head_commit_digest: Some(verified.digest().as_str().to_string()),
                head_update_sequence,
            },
        );
        self.upsert_digest_record(
            AuthoritativeArtifactFamily::BranchHeadRecord,
            branch_identity.clone(),
            stable_structural_digest(&self.branch_head_records[&branch_identity])?,
        );

        Ok(AppliedAuthoritativeAppend {
            branch_identity,
            commit_id,
            parent_count: envelope.commit.parents.len(),
            created_branch,
            previous_next_commit_sequence,
            previous_next_head_update_sequence,
            previous_branch_record,
            previous_branch_head_record,
        })
    }

    pub fn rollback_verified_append(&mut self, applied: AppliedAuthoritativeAppend) {
        self.next_commit_sequence = applied.previous_next_commit_sequence;
        self.next_head_update_sequence = applied.previous_next_head_update_sequence;

        self.commit_envelopes.remove(&applied.commit_id.0);
        self.authoritative_artifact_digests
            .remove(&digest_artifact_key(
                &AuthoritativeArtifactFamily::CommitEnvelope,
                &commit_artifact_id(applied.commit_id),
                self.canonicalization_version,
            ));

        for parent_position in 0..applied.parent_count {
            let artifact_id = parent_artifact_id(applied.commit_id, parent_position);
            self.commit_parent_records.remove(&artifact_id);
            self.authoritative_artifact_digests
                .remove(&digest_artifact_key(
                    &AuthoritativeArtifactFamily::CommitParentRecord,
                    &artifact_id,
                    self.canonicalization_version,
                ));
        }

        match applied.previous_branch_head_record {
            Some(record) => {
                let restored_digest = stable_structural_digest(&record)
                    .expect("restoring previous branch head digest should serialize");
                self.branch_head_records
                    .insert(applied.branch_identity.clone(), record);
                self.upsert_digest_record(
                    AuthoritativeArtifactFamily::BranchHeadRecord,
                    applied.branch_identity.clone(),
                    restored_digest,
                );
            }
            None => {
                self.branch_head_records.remove(&applied.branch_identity);
                self.authoritative_artifact_digests
                    .remove(&digest_artifact_key(
                        &AuthoritativeArtifactFamily::BranchHeadRecord,
                        &applied.branch_identity,
                        self.canonicalization_version,
                    ));
            }
        }

        match applied.previous_branch_record {
            Some(record) => {
                let restored_digest = stable_structural_digest(&record)
                    .expect("restoring previous branch digest should serialize");
                self.branch_records
                    .insert(applied.branch_identity.clone(), record);
                self.upsert_digest_record(
                    AuthoritativeArtifactFamily::BranchRecord,
                    applied.branch_identity.clone(),
                    restored_digest,
                );
            }
            None => {
                self.branch_records.remove(&applied.branch_identity);
                self.authoritative_artifact_digests
                    .remove(&digest_artifact_key(
                        &AuthoritativeArtifactFamily::BranchRecord,
                        &applied.branch_identity,
                        self.canonicalization_version,
                    ));
            }
        }
    }

    pub fn verify_applied_authoritative_append(
        &self,
        applied: &AppliedAuthoritativeAppend,
    ) -> Result<(), StoreError> {
        let commit_record = self
            .commit_envelopes
            .get(&applied.commit_id.0)
            .ok_or_else(|| {
                StoreError::backend_integrity(format!(
                    "commit {} missing after in-place authoritative append",
                    applied.commit_id.0
                ))
            })?;
        self.verify_commit_record(commit_record)?;
        if applied.created_branch {
            self.verify_branch_record(&applied.branch_identity)?;
        }
        self.verify_branch_head_record(&applied.branch_identity)?;
        Ok(())
    }
}
