use worth_signal::facade::adapters::{BranchMergePlan, BranchMergeResult};
use worth_signal::facade::history::RuntimeBranchId;

use super::merge_state::{merge_branch_metadata, merge_branch_store};
use super::state::BranchRuntimeState;
use super::MergePolicyPreviewRequest;
use super::RuntimeCore;
use crate::boundary::errors::WorthSignalJsError;
use crate::runtime::adapters::{
    MergePlanArtifactSummary, MergePlanProofEnvelope, MergeResultArtifactSummary,
    MergeResultProofEnvelope,
};

impl RuntimeCore {
    pub fn merge_branches(
        &mut self,
        source_branch_id: u64,
        target_branch_id: u64,
    ) -> Result<MergeResultArtifactSummary, WorthSignalJsError> {
        self.merge_branches_raw(source_branch_id, target_branch_id)
            .map(Into::into)
    }

    pub fn merge_branches_with_proof(
        &mut self,
        source_branch_id: u64,
        target_branch_id: u64,
    ) -> Result<MergeResultProofEnvelope, WorthSignalJsError> {
        let raw_result = self.merge_branches_raw(source_branch_id, target_branch_id)?;
        let proof = self.merge_result_proof_report(&raw_result)?;
        let result = raw_result.into();
        Ok(MergeResultProofEnvelope { result, proof })
    }

    pub fn plan_merge_branches(
        &mut self,
        source_branch_id: u64,
        target_branch_id: u64,
    ) -> Result<MergePlanArtifactSummary, WorthSignalJsError> {
        self.plan_merge_branches_raw(source_branch_id, target_branch_id)
            .map(Into::into)
    }

    fn plan_merge_branches_raw(
        &mut self,
        source_branch_id: u64,
        target_branch_id: u64,
    ) -> Result<BranchMergePlan, WorthSignalJsError> {
        let source = self
            .runtime
            .branch_handle(RuntimeBranchId(source_branch_id))
            .ok_or_else(|| {
                WorthSignalJsError::invalid_input(format!("unknown branch `{source_branch_id}`"))
            })?;
        let target = self
            .runtime
            .branch_handle(RuntimeBranchId(target_branch_id))
            .ok_or_else(|| {
                WorthSignalJsError::invalid_input(format!("unknown branch `{target_branch_id}`"))
            })?;
        self.runtime
            .merge()
            .from(source)
            .into_branch(target)
            .plan()
            .map(|planned| planned.plan().clone())
            .map_err(WorthSignalJsError::from)
    }

    pub fn plan_merge_branches_with_proof(
        &mut self,
        source_branch_id: u64,
        target_branch_id: u64,
    ) -> Result<MergePlanProofEnvelope, WorthSignalJsError> {
        let raw_plan = self.plan_merge_branches_raw(source_branch_id, target_branch_id)?;
        let proof = self.merge_plan_proof_report(&raw_plan)?;
        let plan = raw_plan.into();
        Ok(MergePlanProofEnvelope { plan, proof })
    }

    pub fn plan_merge_policy_preview(
        &mut self,
        request: MergePolicyPreviewRequest,
    ) -> Result<MergePlanArtifactSummary, WorthSignalJsError> {
        self.plan_merge_policy_preview_raw(request).map(Into::into)
    }

    fn plan_merge_policy_preview_raw(
        &mut self,
        request: MergePolicyPreviewRequest,
    ) -> Result<BranchMergePlan, WorthSignalJsError> {
        let source = self
            .runtime
            .branch_handle(RuntimeBranchId(request.source_branch_id))
            .ok_or_else(|| {
                WorthSignalJsError::invalid_input(format!(
                    "unknown branch `{}`",
                    request.source_branch_id
                ))
            })?;
        let target = self
            .runtime
            .branch_handle(RuntimeBranchId(request.target_branch_id))
            .ok_or_else(|| {
                WorthSignalJsError::invalid_input(format!(
                    "unknown branch `{}`",
                    request.target_branch_id
                ))
            })?;

        let mut merge = self.runtime.merge().from(source).into_branch(target);
        if let Some(policy_name) = request.conflict_policy_name {
            merge = merge.conflict_policy_named(policy_name);
        }
        if let Some(policy_name) = request.conflict_isolation_policy_name {
            merge = merge.conflict_isolation_policy_named(policy_name);
        }
        if let Some(matcher_name) = request.identity_matcher_name {
            merge = merge.identity_matcher_named(matcher_name);
        }
        if let Some(policy_name) = request.deletion_policy_name {
            merge = merge.deletion_policy_named(policy_name);
        }

        merge
            .plan()
            .map(|planned| planned.plan().clone())
            .map_err(WorthSignalJsError::from)
    }

    pub fn plan_merge_policy_preview_with_proof(
        &mut self,
        request: MergePolicyPreviewRequest,
    ) -> Result<MergePlanProofEnvelope, WorthSignalJsError> {
        let raw_plan = self.plan_merge_policy_preview_raw(request)?;
        let proof = self.merge_plan_proof_report(&raw_plan)?;
        let plan = raw_plan.into();
        Ok(MergePlanProofEnvelope { plan, proof })
    }

    pub fn merge_branches_policy_preview(
        &mut self,
        request: MergePolicyPreviewRequest,
    ) -> Result<MergeResultArtifactSummary, WorthSignalJsError> {
        self.merge_branches_policy_preview_raw(request)
            .map(Into::into)
    }

    fn merge_branches_policy_preview_raw(
        &mut self,
        request: MergePolicyPreviewRequest,
    ) -> Result<BranchMergeResult, WorthSignalJsError> {
        let source = self
            .runtime
            .branch_handle(RuntimeBranchId(request.source_branch_id))
            .ok_or_else(|| {
                WorthSignalJsError::invalid_input(format!(
                    "unknown branch `{}`",
                    request.source_branch_id
                ))
            })?;
        let target = self
            .runtime
            .branch_handle(RuntimeBranchId(request.target_branch_id))
            .ok_or_else(|| {
                WorthSignalJsError::invalid_input(format!(
                    "unknown branch `{}`",
                    request.target_branch_id
                ))
            })?;

        let mut merge = self.runtime.merge().from(source).into_branch(target);
        if let Some(policy_name) = request.conflict_policy_name {
            merge = merge.conflict_policy_named(policy_name);
        }
        if let Some(policy_name) = request.conflict_isolation_policy_name {
            merge = merge.conflict_isolation_policy_named(policy_name);
        }
        if let Some(matcher_name) = request.identity_matcher_name {
            merge = merge.identity_matcher_named(matcher_name);
        }
        if let Some(policy_name) = request.deletion_policy_name {
            merge = merge.deletion_policy_named(policy_name);
        }

        merge.run().map_err(WorthSignalJsError::from)
    }

    pub fn merge_branches_policy_preview_with_proof(
        &mut self,
        request: MergePolicyPreviewRequest,
    ) -> Result<MergeResultProofEnvelope, WorthSignalJsError> {
        let raw_result = self.merge_branches_policy_preview_raw(request)?;
        let proof = self.merge_result_proof_report(&raw_result)?;
        let result = raw_result.into();
        Ok(MergeResultProofEnvelope { result, proof })
    }
}

impl RuntimeCore {
    fn merge_branches_raw(
        &mut self,
        source_branch_id: u64,
        target_branch_id: u64,
    ) -> Result<BranchMergeResult, WorthSignalJsError> {
        let current_branch_id = self.runtime.current_branch().id.0;
        let current_state = self.snapshot_branch_state();
        self.branch_states
            .insert(current_branch_id, current_state.clone());

        if current_branch_id != source_branch_id {
            self.switch_branch(source_branch_id)?;
        }
        let source_state = self.snapshot_branch_state();
        self.branch_states
            .insert(source_branch_id, source_state.clone());

        if self.runtime.current_branch().id.0 != target_branch_id {
            self.switch_branch(target_branch_id)?;
        }
        let target_state = self.snapshot_branch_state();
        self.branch_states
            .insert(target_branch_id, target_state.clone());

        if self.runtime.current_branch().id.0 != source_branch_id {
            self.switch_branch(source_branch_id)?;
        }

        let merged_metadata = merge_branch_metadata(&target_state.metadata, &source_state.metadata);
        self.restore_branch_metadata(merged_metadata.clone());
        let source = self
            .runtime
            .branch_handle(RuntimeBranchId(source_branch_id))
            .ok_or_else(|| {
                WorthSignalJsError::invalid_input(format!("unknown branch `{source_branch_id}`"))
            })?;
        let target = self
            .runtime
            .branch_handle(RuntimeBranchId(target_branch_id))
            .ok_or_else(|| {
                WorthSignalJsError::invalid_input(format!("unknown branch `{target_branch_id}`"))
            })?;
        self.runtime
            .merge_branch(source, target)
            .map_err(WorthSignalJsError::from)
            .map(|result| {
                let merged_store = merge_branch_store(
                    &target_state.store,
                    &source_state.store,
                    &source_state.metadata,
                    &merged_metadata,
                    &result,
                );
                let merged_state = BranchRuntimeState {
                    metadata: merged_metadata,
                    store: merged_store,
                    authored_graph_generation: target_state
                        .authored_graph_generation
                        .max(source_state.authored_graph_generation)
                        .saturating_add(1),
                };
                self.branch_states
                    .insert(target_branch_id, merged_state.clone());
                let active_branch_id = self.runtime.current_branch().id.0;
                let restored = if active_branch_id == target_branch_id {
                    merged_state
                } else if active_branch_id == source_branch_id {
                    source_state
                } else {
                    current_state
                };
                let _ = self.restore_branch_state(restored);
                result
            })
    }
}
