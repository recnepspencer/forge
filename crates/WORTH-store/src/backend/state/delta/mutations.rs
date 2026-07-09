use crate::{
    delta::{
        stable_branch_delta_layer_authority_digest, BranchDeltaLayerId, BranchDeltaRebuildReceipt,
        BranchDeltaRewritePlan, BranchDeltaRewriteReceipt, BRANCH_DELTA_FAMILY_VERSION,
    },
    failure::{StoreError, StoreErrorKind},
};
use worth_relational::facade::history::{BranchId, CommitId};

use crate::backend::{
    integrity::branch_key,
    records::{
        BranchDeltaLayerArtifacts, BranchDeltaLayerRecord, BranchDeltaReplacementProofEntry,
        StoreState,
    },
};

use super::{
    artifacts::{combine_branch_delta_layer_artifacts, empty_branch_delta_layer_artifacts},
    AppliedBranchDeltaRebuild, AppliedBranchDeltaRewrite,
};

impl StoreState {
    pub fn apply_delta_rewrite_plan_in_place(
        &mut self,
        plan: BranchDeltaRewritePlan,
    ) -> Result<(AppliedBranchDeltaRewrite, BranchDeltaRewriteReceipt), StoreError> {
        let segment = plan.segment().cloned().ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::BranchDeltaRewriteTargetIllegal,
                "branch delta rewrite execution requires an admitted contiguous segment",
            )
        })?;
        let previous_next_branch_delta_layer_id = self.next_branch_delta_layer_id;
        let removed_layers = self.remove_branch_delta_layers(segment.layer_ids())?;
        let replacement_lineage_proof = removed_layers
            .iter()
            .map(|record| BranchDeltaReplacementProofEntry {
                layer_id: record.branch_delta_layer_id,
                branch_id: record.branch_id.clone(),
                base_frontier_commit_id: record.base_frontier_commit_id,
                target_frontier_commit_id: record.target_frontier_commit_id,
                commit_ids: record.commit_ids.clone(),
                delta_family_version: record.delta_family_version,
                authority_basis_digest: record.authority_basis_digest.clone(),
            })
            .collect();
        let replacement_artifacts = combine_branch_delta_layer_artifacts(&removed_layers);
        let replacement_layer_id = self.publish_branch_delta_layer(
            segment.branch_id().clone(),
            segment.base_frontier_commit_id(),
            segment.target_frontier_commit_id(),
            segment.commit_ids().to_vec(),
            replacement_artifacts,
            segment.layer_ids().to_vec(),
            replacement_lineage_proof,
        );
        Ok((
            AppliedBranchDeltaRewrite {
                replacement_layer_id: Some(replacement_layer_id),
                removed_layers,
                previous_next_branch_delta_layer_id,
            },
            BranchDeltaRewriteReceipt {
                branch_id: segment.branch_id().clone(),
                target_frontier_commit_id: segment.target_frontier_commit_id(),
                replacement_layer_id: Some(BranchDeltaLayerId(replacement_layer_id)),
                replaced_layer_ids: segment.layer_ids().to_vec(),
            },
        ))
    }

    pub fn rollback_delta_rewrite(&mut self, applied: AppliedBranchDeltaRewrite) {
        if let Some(layer_id) = applied.replacement_layer_id {
            self.branch_delta_layer_records.remove(&layer_id);
        }
        self.next_branch_delta_layer_id = applied.previous_next_branch_delta_layer_id;
        for record in applied.removed_layers {
            self.branch_delta_layer_records
                .insert(record.branch_delta_layer_id.0, record);
        }
    }

    pub fn verify_applied_delta_rewrite(
        &self,
        applied: &AppliedBranchDeltaRewrite,
    ) -> Result<(), StoreError> {
        let _ = applied;
        self.verify_delta_record_family()
    }

    pub fn apply_branch_delta_rebuild_in_place(
        &mut self,
        branch_id: BranchId,
    ) -> Result<(AppliedBranchDeltaRebuild, BranchDeltaRebuildReceipt), StoreError> {
        let basis = self
            .branch_shared_base_records
            .get(&branch_key(&branch_id))
            .cloned()
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BranchDeltaBasisUnsupported,
                    format!(
                        "branch `{}` does not publish a shared-base branch delta basis yet",
                        branch_id.0
                    ),
                )
            })?;
        let head_commit_id = self
            .branch_head_records
            .get(&branch_key(&branch_id))
            .cloned()
            .ok_or_else(|| StoreError::unknown_branch(&branch_id))?
            .head_commit_id;
        let commit_ids = match head_commit_id {
            Some(target_commit_id) => self.trace_linear_branch_segment(
                &branch_id,
                basis.source_frontier_commit_id,
                target_commit_id,
            )?,
            None => Vec::new(),
        };
        let previous_next_branch_delta_layer_id = self.next_branch_delta_layer_id;
        let removed_layers = self.remove_all_branch_delta_layers(&branch_id);
        let mut inserted_layer_ids = Vec::new();
        let mut current_base = basis.source_frontier_commit_id;
        for commit_id in &commit_ids {
            let artifacts = self.build_branch_delta_layer_artifacts(&branch_id, &[*commit_id])?;
            let layer_id = self.publish_branch_delta_layer(
                branch_id.clone(),
                current_base,
                *commit_id,
                vec![*commit_id],
                artifacts,
                Vec::new(),
                Vec::new(),
            );
            inserted_layer_ids.push(layer_id);
            current_base = Some(*commit_id);
        }
        Ok((
            AppliedBranchDeltaRebuild {
                branch_id: branch_id.clone(),
                inserted_layer_ids,
                removed_layers,
                previous_next_branch_delta_layer_id,
            },
            BranchDeltaRebuildReceipt {
                branch_id,
                rebuilt_layer_count: commit_ids.len(),
            },
        ))
    }

    pub fn rollback_branch_delta_rebuild(&mut self, applied: AppliedBranchDeltaRebuild) {
        for layer_id in applied.inserted_layer_ids {
            self.branch_delta_layer_records.remove(&layer_id);
        }
        self.next_branch_delta_layer_id = applied.previous_next_branch_delta_layer_id;
        for record in applied.removed_layers {
            self.branch_delta_layer_records
                .insert(record.branch_delta_layer_id.0, record);
        }
    }

    pub fn verify_applied_branch_delta_rebuild(
        &self,
        applied: &AppliedBranchDeltaRebuild,
    ) -> Result<(), StoreError> {
        let _ = &applied.branch_id;
        self.verify_delta_record_family()
    }
}

impl StoreState {
    pub(crate) fn publish_branch_delta_layer_for_append(
        &mut self,
        branch_id: BranchId,
        base_frontier_commit_id: Option<CommitId>,
        target_frontier_commit_id: CommitId,
        commit_ids: Vec<CommitId>,
    ) -> Option<u64> {
        if commit_ids.is_empty() {
            return None;
        }
        let artifacts = self
            .build_branch_delta_layer_artifacts(&branch_id, &commit_ids)
            .unwrap_or_else(|_| empty_branch_delta_layer_artifacts());
        Some(self.publish_branch_delta_layer(
            branch_id,
            base_frontier_commit_id,
            target_frontier_commit_id,
            commit_ids,
            artifacts,
            Vec::new(),
            Vec::new(),
        ))
    }

    fn publish_branch_delta_layer(
        &mut self,
        branch_id: BranchId,
        base_frontier_commit_id: Option<CommitId>,
        target_frontier_commit_id: CommitId,
        commit_ids: Vec<CommitId>,
        artifacts: BranchDeltaLayerArtifacts,
        replacement_of_layer_ids: Vec<BranchDeltaLayerId>,
        replacement_lineage_proof: Vec<BranchDeltaReplacementProofEntry>,
    ) -> u64 {
        let layer_id = self.next_branch_delta_layer_id;
        self.next_branch_delta_layer_id += 1;
        let record = BranchDeltaLayerRecord {
            branch_delta_layer_id: BranchDeltaLayerId(layer_id),
            branch_id: branch_id.clone(),
            base_frontier_commit_id,
            target_frontier_commit_id,
            commit_ids: commit_ids.clone(),
            delta_family_version: BRANCH_DELTA_FAMILY_VERSION,
            authority_basis_digest: stable_branch_delta_layer_authority_digest(
                &branch_id,
                base_frontier_commit_id,
                target_frontier_commit_id,
                &commit_ids,
                self.canonicalization_version,
            ),
            artifacts,
            replacement_of_layer_ids,
            replacement_lineage_proof,
        };
        self.branch_delta_layer_records.insert(layer_id, record);
        layer_id
    }

    fn remove_branch_delta_layers(
        &mut self,
        layer_ids: &[BranchDeltaLayerId],
    ) -> Result<Vec<BranchDeltaLayerRecord>, StoreError> {
        let mut removed_layers = Vec::with_capacity(layer_ids.len());
        for layer_id in layer_ids {
            let record = self
                .branch_delta_layer_records
                .remove(&layer_id.0)
                .ok_or_else(|| {
                    StoreError::new(
                        StoreErrorKind::BranchDeltaReplacementGap,
                        format!("branch delta layer {} missing during rewrite", layer_id.0),
                    )
                })?;
            removed_layers.push(record);
        }
        Ok(removed_layers)
    }

    fn remove_all_branch_delta_layers(
        &mut self,
        branch_id: &BranchId,
    ) -> Vec<BranchDeltaLayerRecord> {
        let layer_ids = self
            .branch_delta_layer_records
            .iter()
            .filter_map(|(layer_id, record)| (record.branch_id == *branch_id).then_some(*layer_id))
            .collect::<Vec<_>>();
        let mut removed_layers = Vec::with_capacity(layer_ids.len());
        for layer_id in layer_ids {
            if let Some(record) = self.branch_delta_layer_records.remove(&layer_id) {
                removed_layers.push(record);
            }
        }
        removed_layers
    }
}
