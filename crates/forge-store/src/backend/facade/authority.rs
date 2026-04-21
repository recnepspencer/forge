use crate::authority::{
    AuthoritativeBranchHeadRecord, CanonicalizedCommitEnvelope, FetchedAuthoritativeCommit,
    PersistedAuthoritativeCommit, VerifiedAuthoritativeAppend,
};
use crate::delta::{
    SharedBaseBranchCreationReceipt, SharedBaseBranchCreationRequest,
    SharedBaseBranchCreationWitness,
};
use crate::failure::StoreError;
use forge_relational::facade::history::{BranchId, CommitId};

use super::{dispatch_mut, dispatch_ref, StoreBackend};

impl StoreBackend {
    pub fn create_branch(
        &mut self,
        new_branch: BranchId,
        from_branch: Option<&BranchId>,
    ) -> Result<AuthoritativeBranchHeadRecord, StoreError> {
        dispatch_mut!(self, |backend| backend
            .create_branch(new_branch, from_branch))
    }

    pub fn create_shared_base_branch(
        &mut self,
        request: SharedBaseBranchCreationRequest,
    ) -> Result<SharedBaseBranchCreationReceipt, StoreError> {
        dispatch_mut!(self, |backend| backend.create_shared_base_branch(request))
    }

    pub fn admit_shared_base_branch_creation(
        &self,
        request: SharedBaseBranchCreationRequest,
    ) -> Result<SharedBaseBranchCreationWitness, StoreError> {
        dispatch_ref!(self, |backend| backend
            .admit_shared_base_branch_creation(request))
    }

    pub fn verify_append(
        &self,
        append: CanonicalizedCommitEnvelope,
    ) -> Result<VerifiedAuthoritativeAppend, StoreError> {
        dispatch_ref!(self, |backend| backend.verify_append(append))
    }

    pub fn append(
        &mut self,
        verified: VerifiedAuthoritativeAppend,
    ) -> Result<PersistedAuthoritativeCommit, StoreError> {
        dispatch_mut!(self, |backend| backend.append(verified))
    }

    pub fn execute_rolling_commit_publication(
        &mut self,
        request: crate::CompatibilityRollingPublicationRequest,
        verified: VerifiedAuthoritativeAppend,
    ) -> Result<crate::CompatibilityRollingPublicationOutcome, StoreError> {
        dispatch_mut!(self, |backend| backend
            .execute_rolling_commit_publication(request, verified))
    }

    pub fn fetch_commit(
        &self,
        commit_id: CommitId,
    ) -> Result<FetchedAuthoritativeCommit, StoreError> {
        dispatch_ref!(self, |backend| backend.fetch_commit(commit_id))
    }

    pub fn fetch_branch_head(
        &self,
        branch_id: &BranchId,
    ) -> Result<AuthoritativeBranchHeadRecord, StoreError> {
        dispatch_ref!(self, |backend| backend.fetch_branch_head(branch_id))
    }
}
