use crate::{backend::records::StoreState, failure::StoreError};

use super::receipt::AppliedAuthoritativeAppend;

impl StoreState {
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
        self.verify_support_record_family()?;
        self.verify_delta_record_family()?;
        self.verify_branch_head_record(&applied.branch_identity)?;
        Ok(())
    }
}
