use crate::failure::StoreError;

use crate::backend::records::{AuthoritativeArtifactFamily, StoreState};

impl StoreState {
    pub fn verify_branch_record_family(&self) -> Result<(), StoreError> {
        for (branch_id, record) in &self.branch_records {
            if *branch_id != record.branch_id.0 {
                return Err(StoreError::backend_integrity(
                    "branch record key does not match branch identity",
                ));
            }
            self.require_digest_record(
                AuthoritativeArtifactFamily::BranchRecord,
                branch_id.clone(),
                &super::stable_structural_digest(record)?,
            )?;
        }

        for (branch_id, head_record) in &self.branch_head_records {
            if !self.branch_records.contains_key(branch_id) {
                return Err(StoreError::backend_integrity(format!(
                    "branch head exists without branch record for `{branch_id}`"
                )));
            }
            if let Some(head_commit_id) = head_record.head_commit_id {
                let commit_record =
                    self.commit_envelopes
                        .get(&head_commit_id.0)
                        .ok_or_else(|| {
                            StoreError::backend_integrity(format!(
                                "branch `{branch_id}` points at missing commit {}",
                                head_commit_id.0
                            ))
                        })?;
                match &head_record.head_commit_digest {
                    Some(digest) if digest == &commit_record.envelope_digest => {}
                    _ => {
                        return Err(StoreError::backend_integrity(format!(
                            "branch head `{branch_id}` digest does not match commit digest"
                        )))
                    }
                }
            }
            self.require_digest_record(
                AuthoritativeArtifactFamily::BranchHeadRecord,
                branch_id.clone(),
                &super::stable_structural_digest(head_record)?,
            )?;
        }

        Ok(())
    }
}
