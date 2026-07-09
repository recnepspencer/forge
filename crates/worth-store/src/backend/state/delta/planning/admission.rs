use crate::{
    delta::{
        stable_shared_base_authority_digest, BranchDeltaFallbackClass, BranchDeltaLocality,
        BranchDeltaPerformanceEnvelope, BranchDeltaReadPlan, BranchDeltaReadRegime,
        BranchDeltaReadRequest, BranchDeltaReadStrategy, ComplexityStatus,
        Milestone7IndependentReference, SameBranchDescendantWitness, BRANCH_DELTA_FAMILY_VERSION,
        MAX_DIRECT_LAYER_READ_DEPTH, MAX_DIRECT_LAYER_READ_RECORDS,
    },
    failure::{StoreError, StoreErrorKind},
};

use crate::backend::{
    integrity::branch_key,
    records::{BranchSharedBaseRecord, StoreState},
};

use super::regime_for_commit_span;

impl StoreState {
    pub fn admit_same_branch_descendant(
        &self,
        request: BranchDeltaReadRequest,
    ) -> Result<SameBranchDescendantWitness, StoreError> {
        let branch_identity = branch_key(&request.branch_id);
        if !self.branch_records.contains_key(&branch_identity) {
            return Err(StoreError::unknown_branch(&request.branch_id));
        }
        let basis = self
            .branch_shared_base_records
            .get(&branch_identity)
            .cloned()
            .unwrap_or_else(|| BranchSharedBaseRecord {
                branch_id: request.branch_id.clone(),
                source_branch_id: request.branch_id.clone(),
                source_frontier_commit_id: None,
                delta_family_version: BRANCH_DELTA_FAMILY_VERSION,
                authority_basis_digest: stable_shared_base_authority_digest(
                    &request.branch_id,
                    None,
                    self.canonicalization_version,
                ),
            });
        if Some(request.target_commit_id) == basis.source_frontier_commit_id {
            return Ok(SameBranchDescendantWitness::new(
                request.branch_id,
                basis.source_frontier_commit_id,
                request.target_commit_id,
                Vec::new(),
            ));
        }
        let target_record = self
            .commit_record(request.target_commit_id)
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BranchDeltaReadTargetIllegal,
                    format!("target commit {} not found", request.target_commit_id.0),
                )
            })?;
        if target_record.envelope.branch_context != request.branch_id {
            return Err(StoreError::new(
                StoreErrorKind::BranchDeltaReadTargetIllegal,
                format!(
                    "target commit {} belongs to branch `{}` not branch `{}`",
                    request.target_commit_id.0,
                    target_record.envelope.branch_context.0,
                    request.branch_id.0
                ),
            ));
        }
        let commit_ids = self.trace_linear_branch_segment(
            &request.branch_id,
            basis.source_frontier_commit_id,
            request.target_commit_id,
        )?;
        Ok(SameBranchDescendantWitness::new(
            request.branch_id,
            basis.source_frontier_commit_id,
            request.target_commit_id,
            commit_ids,
        ))
    }

    pub fn plan_branch_delta_read(
        &self,
        request: BranchDeltaReadRequest,
    ) -> Result<BranchDeltaReadPlan, StoreError> {
        let witness = self.admit_same_branch_descendant(request)?;
        self.plan_branch_delta_read_from_witness(&witness)
    }

    pub fn admit_milestone_7_independent_reference(
        &self,
        request: BranchDeltaReadRequest,
    ) -> Result<Milestone7IndependentReference, StoreError> {
        let witness = self.admit_same_branch_descendant(request)?;
        Ok(Milestone7IndependentReference::new(
            witness.branch_id().clone(),
            witness.target_commit_id(),
        ))
    }

    pub(crate) fn plan_branch_delta_read_from_witness(
        &self,
        witness: &SameBranchDescendantWitness,
    ) -> Result<BranchDeltaReadPlan, StoreError> {
        let commit_ids = witness.commit_ids().to_vec();
        let locality = BranchDeltaLocality {
            branch_id: witness.branch_id().clone(),
            base_frontier_commit_id: witness.base_frontier_commit_id(),
            target_commit_id: witness.target_commit_id(),
            commit_span: commit_ids.len(),
        };
        if commit_ids.is_empty() {
            return Ok(BranchDeltaReadPlan {
                strategy: BranchDeltaReadStrategy::EmptyBranchReuse,
                regime: BranchDeltaReadRegime::Sparse,
                locality,
                used_layer_ids: Vec::new(),
                commit_ids,
                performance: BranchDeltaPerformanceEnvelope {
                    layers_traversed: 0,
                    records_decoded: 0,
                    replay_commit_count: 0,
                    fallback_class: BranchDeltaFallbackClass::None,
                    complexity_status: ComplexityStatus::Verified,
                },
            });
        }

        let mut used_layer_ids = Vec::new();
        let mut current_base = witness.base_frontier_commit_id();
        let mut covered_commit_count = 0usize;
        while covered_commit_count < commit_ids.len() {
            let remaining = &commit_ids[covered_commit_count..];
            let Some(layer) =
                self.find_covering_branch_delta_layer(&locality.branch_id, current_base, remaining)
            else {
                return Ok(BranchDeltaReadPlan {
                    strategy: BranchDeltaReadStrategy::AuthorityReplayControl,
                    regime: regime_for_commit_span(commit_ids.len()),
                    locality,
                    used_layer_ids: Vec::new(),
                    commit_ids: commit_ids.clone(),
                    performance: BranchDeltaPerformanceEnvelope {
                        layers_traversed: 0,
                        records_decoded: 0,
                        replay_commit_count: commit_ids.len(),
                        fallback_class:
                            BranchDeltaFallbackClass::RequiresAuthorityReplayControlLane,
                        complexity_status: ComplexityStatus::Debt,
                    },
                });
            };
            used_layer_ids.push(layer.branch_delta_layer_id);
            covered_commit_count += layer.commit_ids.len();
            current_base = Some(layer.target_frontier_commit_id);
        }

        if used_layer_ids.len() > MAX_DIRECT_LAYER_READ_DEPTH
            || commit_ids.len() > MAX_DIRECT_LAYER_READ_RECORDS
        {
            return Err(StoreError::new(
                StoreErrorKind::BranchDeltaReadBudgetExceeded,
                format!(
                    "branch delta direct read for branch `{}` target {} exceeded the admitted local budget (layers={}, records={}, max_layers={}, max_records={})",
                    locality.branch_id.0,
                    locality.target_commit_id.0,
                    used_layer_ids.len(),
                    commit_ids.len(),
                    MAX_DIRECT_LAYER_READ_DEPTH,
                    MAX_DIRECT_LAYER_READ_RECORDS
                ),
            ));
        }

        Ok(BranchDeltaReadPlan {
            strategy: BranchDeltaReadStrategy::DirectLayerRead,
            regime: regime_for_commit_span(commit_ids.len()),
            locality,
            used_layer_ids: used_layer_ids.clone(),
            commit_ids: commit_ids.clone(),
            performance: BranchDeltaPerformanceEnvelope {
                layers_traversed: used_layer_ids.len(),
                records_decoded: used_layer_ids.len(),
                replay_commit_count: commit_ids.len(),
                fallback_class: BranchDeltaFallbackClass::None,
                complexity_status: ComplexityStatus::Verified,
            },
        })
    }
}
