use crate::{
    backend::records::{BranchSharedBaseRecord, StoreState},
    delta::{stable_shared_base_authority_digest, BRANCH_DELTA_FAMILY_VERSION},
    failure::{StoreError, StoreErrorKind},
};

impl StoreState {
    pub(super) fn verify_branch_shared_base_record(
        &self,
        record: &BranchSharedBaseRecord,
    ) -> Result<(), StoreError> {
        if record.delta_family_version != BRANCH_DELTA_FAMILY_VERSION {
            return Err(StoreError::new(
                StoreErrorKind::BranchDeltaFamilyVersionUnsupported,
                format!(
                    "branch `{}` used unsupported delta family version {}",
                    record.branch_id.0, record.delta_family_version
                ),
            ));
        }
        let branch = self
            .branch_records
            .get(&record.branch_id.0)
            .ok_or_else(|| StoreError::unknown_branch(&record.branch_id))?;
        if branch.created_from_branch.as_ref() != Some(&record.source_branch_id)
            || branch.created_from_commit_id != record.source_frontier_commit_id
        {
            return Err(StoreError::new(
                StoreErrorKind::BranchDeltaPublicationGap,
                format!(
                    "branch `{}` shared-base record drifted from authoritative branch creation basis",
                    record.branch_id.0
                ),
            ));
        }
        if !self.branch_records.contains_key(&record.source_branch_id.0) {
            return Err(StoreError::unknown_branch(&record.source_branch_id));
        }
        if let Some(frontier_commit_id) = record.source_frontier_commit_id {
            let frontier_record = self.commit_record(frontier_commit_id).ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BranchDeltaPublicationGap,
                    format!(
                        "shared-base branch `{}` references missing source frontier commit {}",
                        record.branch_id.0, frontier_commit_id.0
                    ),
                )
            })?;
            if frontier_record.envelope.branch_context != record.source_branch_id {
                return Err(StoreError::new(
                    StoreErrorKind::BranchDeltaPublicationGap,
                    format!(
                        "shared-base branch `{}` source frontier {} drifted off source branch `{}`",
                        record.branch_id.0, frontier_commit_id.0, record.source_branch_id.0
                    ),
                ));
            }
        }
        let expected_digest = stable_shared_base_authority_digest(
            &record.source_branch_id,
            record.source_frontier_commit_id,
            self.canonicalization_version,
        );
        if record.authority_basis_digest != expected_digest {
            return Err(StoreError::new(
                StoreErrorKind::BranchDeltaDigestMismatch,
                format!(
                    "branch `{}` shared-base digest drifted from authoritative basis",
                    record.branch_id.0
                ),
            ));
        }
        Ok(())
    }
}
