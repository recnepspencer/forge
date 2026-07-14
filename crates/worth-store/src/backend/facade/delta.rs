use crate::delta::{
    BranchDeltaAutoCompactOutcome, BranchDeltaReadPlan, BranchDeltaReadRequest,
    BranchDeltaReadResult, BranchDeltaRebuildReceipt, BranchDeltaRewritePlan,
    BranchDeltaRewriteReceipt, BranchDeltaRewriteRecommendation, BranchDeltaRewriteRequest,
    SameBranchDescendantWitness,
};
use crate::failure::StoreError;
use worth_relational::facade::history::{BranchId, CommitId};

use super::{dispatch_mut, dispatch_ref, StoreBackend};

impl StoreBackend {
    pub fn plan_branch_delta_read(
        &self,
        request: BranchDeltaReadRequest,
    ) -> Result<BranchDeltaReadPlan, StoreError> {
        dispatch_ref!(self, |backend| backend.plan_branch_delta_read(request))
    }

    pub fn admit_same_branch_descendant(
        &self,
        request: BranchDeltaReadRequest,
    ) -> Result<SameBranchDescendantWitness, StoreError> {
        dispatch_ref!(self, |backend| backend
            .admit_same_branch_descendant(request))
    }

    pub fn admit_milestone_7_independent_reference(
        &self,
        request: BranchDeltaReadRequest,
    ) -> Result<crate::Milestone7IndependentReference, StoreError> {
        dispatch_ref!(self, |backend| backend
            .admit_milestone_7_independent_reference(request))
    }

    pub fn read_branch_delta(
        &self,
        witness: SameBranchDescendantWitness,
    ) -> Result<BranchDeltaReadResult, StoreError> {
        dispatch_ref!(self, |backend| backend.read_branch_delta(witness))
    }

    pub fn read_branch_delta_control(
        &self,
        witness: SameBranchDescendantWitness,
    ) -> Result<BranchDeltaReadResult, StoreError> {
        dispatch_ref!(self, |backend| backend.read_branch_delta_control(witness))
    }

    pub fn read_branch_delta_control_from_milestone_7_reference(
        &self,
        reference: crate::Milestone7IndependentReference,
    ) -> Result<BranchDeltaReadResult, StoreError> {
        dispatch_ref!(self, |backend| backend
            .read_branch_delta_control_from_milestone_7_reference(reference))
    }

    pub fn plan_delta_rewrite(
        &self,
        request: BranchDeltaRewriteRequest,
    ) -> Result<BranchDeltaRewritePlan, StoreError> {
        dispatch_ref!(self, |backend| backend.plan_delta_rewrite(request))
    }

    pub fn recommend_delta_rewrite(
        &self,
        request: BranchDeltaRewriteRequest,
    ) -> Result<BranchDeltaRewriteRecommendation, StoreError> {
        dispatch_ref!(self, |backend| backend.recommend_delta_rewrite(request))
    }

    pub fn auto_compact_branch_delta(
        &mut self,
        request: BranchDeltaRewriteRequest,
    ) -> Result<BranchDeltaAutoCompactOutcome, StoreError> {
        dispatch_mut!(self, |backend| backend.auto_compact_branch_delta(request))
    }

    pub fn rewrite_branch_delta(
        &mut self,
        plan: BranchDeltaRewritePlan,
    ) -> Result<BranchDeltaRewriteReceipt, StoreError> {
        dispatch_mut!(self, |backend| backend.rewrite_branch_delta(plan))
    }

    pub fn rebuild_branch_delta_artifacts(
        &mut self,
        branch_id: BranchId,
    ) -> Result<BranchDeltaRebuildReceipt, StoreError> {
        dispatch_mut!(self, |backend| backend
            .rebuild_branch_delta_artifacts(branch_id))
    }

    pub(crate) fn milestone_5_delta_storage_report(
        &self,
        branch_id: BranchId,
        target_commit_id: CommitId,
        direct_plan: &BranchDeltaReadPlan,
        control_plan: &BranchDeltaReadPlan,
    ) -> Result<crate::Milestone5DeltaStorageReport, StoreError> {
        dispatch_ref!(self, |backend| backend.milestone_5_delta_storage_report(
            branch_id,
            target_commit_id,
            direct_plan,
            control_plan,
        ))
    }
}
