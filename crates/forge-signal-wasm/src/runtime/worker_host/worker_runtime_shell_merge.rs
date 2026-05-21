use crate::boundary::errors::ForgeSignalJsError;
use crate::runtime::adapters::{
    MergePlanArtifactSummary, MergePlanProofEnvelope, MergeResultArtifactSummary,
    MergeResultProofEnvelope,
};
use crate::runtime::core::MergePolicyPreviewRequest;

use super::WorkerRuntimeShell;

impl WorkerRuntimeShell {
    pub fn merge_branches(
        &mut self,
        source_branch_id: u64,
        target_branch_id: u64,
    ) -> Result<MergeResultArtifactSummary, ForgeSignalJsError> {
        let result = self
            .core
            .merge_branches(source_branch_id, target_branch_id)?;
        self.clear_worker_boundary_certification_evidence();
        Ok(result)
    }

    pub fn merge_branches_with_proof(
        &mut self,
        source_branch_id: u64,
        target_branch_id: u64,
    ) -> Result<MergeResultProofEnvelope, ForgeSignalJsError> {
        let envelope = self
            .core
            .merge_branches_with_proof(source_branch_id, target_branch_id)?;
        self.clear_worker_boundary_certification_evidence();
        Ok(envelope)
    }

    pub fn plan_merge_branches(
        &mut self,
        source_branch_id: u64,
        target_branch_id: u64,
    ) -> Result<MergePlanArtifactSummary, ForgeSignalJsError> {
        self.core
            .plan_merge_branches(source_branch_id, target_branch_id)
    }

    pub fn plan_merge_branches_with_proof(
        &mut self,
        source_branch_id: u64,
        target_branch_id: u64,
    ) -> Result<MergePlanProofEnvelope, ForgeSignalJsError> {
        self.core
            .plan_merge_branches_with_proof(source_branch_id, target_branch_id)
    }

    pub fn plan_merge_policy_preview(
        &mut self,
        request: MergePolicyPreviewRequest,
    ) -> Result<MergePlanArtifactSummary, ForgeSignalJsError> {
        self.core.plan_merge_policy_preview(request)
    }

    pub fn plan_merge_policy_preview_with_proof(
        &mut self,
        request: MergePolicyPreviewRequest,
    ) -> Result<MergePlanProofEnvelope, ForgeSignalJsError> {
        self.core.plan_merge_policy_preview_with_proof(request)
    }

    pub fn merge_branches_policy_preview(
        &mut self,
        request: MergePolicyPreviewRequest,
    ) -> Result<MergeResultArtifactSummary, ForgeSignalJsError> {
        self.core.merge_branches_policy_preview(request)
    }

    pub fn merge_branches_policy_preview_with_proof(
        &mut self,
        request: MergePolicyPreviewRequest,
    ) -> Result<MergeResultProofEnvelope, ForgeSignalJsError> {
        self.core.merge_branches_policy_preview_with_proof(request)
    }
}
