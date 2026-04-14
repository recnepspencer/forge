use crate::{
    authority::digest_from_string,
    authority::{CanonicalizedCommitEnvelope, VerifiedAuthoritativeAppend},
    failure::{StoreError, StoreErrorKind},
};
use forge_relational::facade::{
    history::{BranchId, CommitId},
    replay::CanonicalCommitEnvelope,
};

use super::{integrity::branch_key, records::StoreState};

impl StoreState {
    pub fn verify_authoritative_append(
        &self,
        append: CanonicalizedCommitEnvelope,
    ) -> Result<VerifiedAuthoritativeAppend, StoreError> {
        let envelope = append.envelope();
        if let Some(existing) = self.commit_envelopes.get(&envelope.commit.commit_id.0) {
            if existing.envelope_digest == append.digest().as_str() {
                return Ok(VerifiedAuthoritativeAppend::new(
                    existing.envelope.clone(),
                    digest_from_string(existing.envelope_digest.clone()),
                    existing.canonicalization_version,
                ));
            }
            return Err(StoreError::duplicate_conflict(envelope.commit.commit_id));
        }

        self.ensure_branch_identity(envelope)?;
        self.ensure_parent_closure(envelope)?;
        self.ensure_branch_head_legality(envelope)?;

        Ok(VerifiedAuthoritativeAppend::new(
            envelope.clone(),
            append.digest().clone(),
            append.canonicalization_version(),
        ))
    }

    fn ensure_branch_identity(&self, envelope: &CanonicalCommitEnvelope) -> Result<(), StoreError> {
        let branch_identity = branch_key(&envelope.branch_context);
        if self.branch_records.contains_key(&branch_identity) || envelope.commit.parents.is_empty()
        {
            return Ok(());
        }
        Err(StoreError::unknown_branch(&envelope.branch_context))
    }

    fn ensure_parent_closure(&self, envelope: &CanonicalCommitEnvelope) -> Result<(), StoreError> {
        for parent in &envelope.commit.parents {
            if !self.has_commit(*parent) {
                return Err(StoreError::orphan_parent(
                    envelope.commit.commit_id,
                    *parent,
                ));
            }
        }
        Ok(())
    }

    fn ensure_branch_head_legality(
        &self,
        envelope: &CanonicalCommitEnvelope,
    ) -> Result<(), StoreError> {
        let branch_identity = branch_key(&envelope.branch_context);
        let Some(head_record) = self.branch_head_records.get(&branch_identity) else {
            if envelope.commit.parents.is_empty() {
                return Ok(());
            }
            return Err(StoreError::unknown_branch(&envelope.branch_context));
        };

        match head_record.head_commit_id {
            None => {
                if envelope.commit.parents.is_empty() {
                    return Ok(());
                }
                Err(StoreError::new(
                    StoreErrorKind::IllegalBranchHeadTransition,
                    format!(
                        "empty branch `{}` only accepts a root commit in milestone 1",
                        envelope.branch_context.0
                    ),
                ))
            }
            Some(current_head) if envelope.commit.parents.contains(&current_head) => Ok(()),
            Some(current_head) => Err(StoreError::new(
                StoreErrorKind::IllegalBranchHeadTransition,
                format!(
                    "commit {} does not fast-forward branch `{}` from head {}",
                    envelope.commit.commit_id.0, envelope.branch_context.0, current_head.0
                ),
            )),
        }
    }

    pub fn commit_record(
        &self,
        commit_id: CommitId,
    ) -> Option<&super::records::StoredCommitEnvelope> {
        self.commit_envelopes.get(&commit_id.0)
    }

    pub fn branch_exists(&self, branch_id: &BranchId) -> bool {
        self.branch_records.contains_key(&branch_key(branch_id))
    }
}
