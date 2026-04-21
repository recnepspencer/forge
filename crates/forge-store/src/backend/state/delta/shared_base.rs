use crate::{
    delta::{
        stable_shared_base_authority_digest, SharedBaseBranchCreationReceipt,
        SharedBaseBranchCreationRequest, SharedBaseBranchCreationWitness,
        BRANCH_DELTA_FAMILY_VERSION,
    },
    failure::{StoreError, StoreErrorKind},
};

use crate::backend::{
    integrity::branch_key,
    records::{BranchSharedBaseRecord, StoreState},
};

use super::AppliedSharedBaseBranchCreation;

impl StoreState {
    pub fn admit_shared_base_branch_creation(
        &self,
        request: SharedBaseBranchCreationRequest,
    ) -> Result<SharedBaseBranchCreationWitness, StoreError> {
        if request.new_branch_id == request.source_branch_id {
            return Err(StoreError::new(
                StoreErrorKind::BranchDeltaBasisAmbiguous,
                format!(
                    "shared-base branch `{}` cannot cite itself as the source branch",
                    request.new_branch_id.0
                ),
            ));
        }
        let source_head = self
            .branch_head_records
            .get(&branch_key(&request.source_branch_id))
            .cloned()
            .ok_or_else(|| StoreError::unknown_branch(&request.source_branch_id))?;
        Ok(SharedBaseBranchCreationWitness::new(
            request.clone(),
            source_head.head_commit_id,
            stable_shared_base_authority_digest(
                &request.source_branch_id,
                source_head.head_commit_id,
                self.canonicalization_version,
            ),
        ))
    }

    pub fn apply_shared_base_branch_creation_in_place(
        &mut self,
        request: SharedBaseBranchCreationRequest,
    ) -> Result<
        (
            AppliedSharedBaseBranchCreation,
            SharedBaseBranchCreationReceipt,
        ),
        StoreError,
    > {
        let witness = self.admit_shared_base_branch_creation(request)?;
        let request = witness.request().clone();
        let source_frontier_commit_id = witness.source_frontier_commit_id();
        let branch_creation = self.apply_branch_creation_in_place(
            request.new_branch_id.clone(),
            Some(&request.source_branch_id),
        )?;
        let branch_identity = branch_key(&request.new_branch_id);
        let authority_basis_digest = witness.authority_basis_digest().to_string();
        self.branch_shared_base_records.insert(
            branch_identity.clone(),
            BranchSharedBaseRecord {
                branch_id: request.new_branch_id.clone(),
                source_branch_id: request.source_branch_id.clone(),
                source_frontier_commit_id,
                delta_family_version: BRANCH_DELTA_FAMILY_VERSION,
                authority_basis_digest: authority_basis_digest.clone(),
            },
        );

        Ok((
            AppliedSharedBaseBranchCreation {
                branch_creation,
                branch_identity,
            },
            SharedBaseBranchCreationReceipt {
                branch_id: request.new_branch_id,
                source_branch_id: request.source_branch_id,
                source_frontier_commit_id,
                delta_family_version: BRANCH_DELTA_FAMILY_VERSION,
                authority_basis_digest,
            },
        ))
    }

    pub fn rollback_shared_base_branch_creation(
        &mut self,
        applied: AppliedSharedBaseBranchCreation,
    ) {
        self.branch_shared_base_records
            .remove(&applied.branch_identity);
        self.rollback_branch_creation(applied.branch_creation);
    }

    pub fn verify_applied_shared_base_branch_creation(
        &self,
        applied: &AppliedSharedBaseBranchCreation,
    ) -> Result<(), StoreError> {
        self.verify_applied_branch_creation(&applied.branch_creation)?;
        self.verify_delta_record_family()
    }
}
