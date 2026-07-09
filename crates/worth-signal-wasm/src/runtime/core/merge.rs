use std::collections::BTreeMap;

use worth_signal::facade::adapters::{
    ArtifactMergeAction, BranchStateDenseGridProofBasis, BranchStateProofBasis,
    BRANCH_STATE_PROOF_BASIS_VERSION,
};
use worth_signal::facade::adapters::{BranchMergePlan, BranchMergeResult};
use worth_signal::facade::history::RuntimeBranchId;

use crate::boundary::errors::WorthSignalJsError;
use crate::expression::model::SignalValue;
use crate::runtime::adapters::{
    MergePlanArtifactSummary, MergePlanProofEnvelope, MergeResultArtifactSummary,
    MergeResultProofEnvelope,
};
use crate::runtime::summaries::{AspectVersionSummary, RuntimeStoreSnapshot, StoredSourceSnapshot};

use super::state::{BranchRuntimeMetadata, BranchRuntimeState};
use super::MergePolicyPreviewRequest;
use super::RuntimeCore;

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

pub(super) fn build_branch_state_proof_basis(
    state: &BranchRuntimeState,
) -> BranchStateProofBasis<RuntimeStoreSnapshot> {
    let mut catalog_ids = state.metadata.catalog.keys().cloned().collect::<Vec<_>>();
    catalog_ids.sort();

    let mut dense_grids = state
        .metadata
        .dense_grids
        .iter()
        .map(|(family_id, grid)| BranchStateDenseGridProofBasis {
            family_id: family_id.clone(),
            width: grid.width,
            height: grid.height,
            key_count: grid.key_to_index.len(),
            ids: grid.ids.clone(),
        })
        .collect::<Vec<_>>();
    dense_grids.sort_by(|left, right| left.family_id.cmp(&right.family_id));
    for grid in &mut dense_grids {
        grid.ids.sort();
    }

    BranchStateProofBasis {
        proof_schema_version: BRANCH_STATE_PROOF_BASIS_VERSION.to_owned(),
        catalog_ids,
        dense_grids,
        store: state.store.clone(),
    }
}

pub(super) fn merge_branch_metadata(
    target: &BranchRuntimeMetadata,
    source: &BranchRuntimeMetadata,
) -> BranchRuntimeMetadata {
    let mut merged = target.clone();
    for (node, id) in &source.nodes_by_id {
        merged
            .nodes_by_id
            .entry(*node)
            .or_insert_with(|| id.clone());
    }
    for (id, entry) in &source.catalog {
        merged
            .catalog
            .entry(id.clone())
            .or_insert_with(|| entry.clone());
    }
    for (family_id, grid) in &source.dense_grids {
        merged
            .dense_grids
            .entry(family_id.clone())
            .or_insert_with(|| grid.clone());
    }
    merged
}

pub(super) fn merge_branch_store(
    target: &RuntimeStoreSnapshot,
    source: &RuntimeStoreSnapshot,
    source_metadata: &BranchRuntimeMetadata,
    merged_metadata: &BranchRuntimeMetadata,
    result: &BranchMergeResult,
) -> RuntimeStoreSnapshot {
    let mut merged = target.clone();
    let source_sources = source
        .sources
        .iter()
        .map(|entry| (entry.id.clone(), entry))
        .collect::<BTreeMap<_, _>>();
    let mut merged_sources = merged
        .sources
        .iter()
        .map(|entry| (entry.id.clone(), entry.clone()))
        .collect::<BTreeMap<_, _>>();

    for record in &result.records {
        let should_adopt = matches!(
            record.action,
            ArtifactMergeAction::Adopted
                | ArtifactMergeAction::Replaced
                | ArtifactMergeAction::IntroducedIntoTarget
                | ArtifactMergeAction::EquivalentUnchanged
        );
        if !should_adopt {
            continue;
        }
        let Some(source_id) = source_metadata.nodes_by_id.get(&record.source_node) else {
            continue;
        };
        let Some(source_value) = source_sources.get(source_id) else {
            continue;
        };
        let target_id = record
            .target_node
            .and_then(|node| merged_metadata.nodes_by_id.get(&node).cloned())
            .unwrap_or_else(|| source_id.clone());
        merged_sources.insert(
            target_id.clone(),
            StoredSourceSnapshot {
                id: target_id,
                value: source_value.value.clone(),
                version: source_value.version,
                produces_aspects: None,
                aspect_versions: vec![AspectVersionSummary {
                    aspect: super::DEFAULT_ASPECT.id(),
                    version: source_value.version,
                }],
            },
        );
    }

    merged.sources = merged_sources.into_values().collect();
    for recipe in &mut merged.recipes {
        recipe.value = SignalValue::Null;
        recipe.initialized = false;
        recipe.output_identity = None;
    }
    merged
}
