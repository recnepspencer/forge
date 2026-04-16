use crate::{
    authority::AuthoritativeExportBundle,
    delta::{
        stable_branch_delta_layer_authority_digest, stable_shared_base_authority_digest,
        BranchDeltaFallbackClass, BranchDeltaLayerId, BranchDeltaLocality,
        BranchDeltaPerformanceEnvelope, BranchDeltaReadPlan, BranchDeltaReadRegime,
        BranchDeltaReadRequest, BranchDeltaReadResult, BranchDeltaReadStrategy,
        BranchDeltaRebuildReceipt, BranchDeltaRewritePlan, BranchDeltaRewritePolicyDecision,
        BranchDeltaRewriteReceipt, BranchDeltaRewriteRecommendation, BranchDeltaRewriteRequest,
        BranchDeltaRewriteStrategy, ComplexityStatus, Milestone7IndependentReference,
        RewriteEligibleDeltaSegment, SameBranchDescendantWitness, SharedBaseBranchCreationReceipt,
        SharedBaseBranchCreationRequest, SharedBaseBranchCreationWitness,
        BRANCH_DELTA_FAMILY_VERSION, MAX_DIRECT_LAYER_READ_DEPTH, MAX_DIRECT_LAYER_READ_RECORDS,
        MAX_REWRITE_LAYER_WIDTH, RECOMMENDED_REWRITE_LAYER_WIDTH,
    },
    evidence::{Milestone5DeltaStorageReport, Milestone5ReadPathReport},
    failure::{StoreError, StoreErrorKind},
};
use forge_relational::facade::history::{BranchId, CommitId};
use std::collections::{BTreeMap, BTreeSet};

use crate::backend::{
    integrity::{
        branch_key, commit_artifact_id, commit_support_summary_artifact_id,
        durable_cursor_identity_artifact_id, lineage_support_artifact_id, parent_artifact_id,
        schema_support_artifact_id, stable_structural_digest, subscriber_checkpoint_artifact_id,
    },
    records::{
        AuthoritativeArtifactDigestRecord, AuthoritativeArtifactFamily, BranchDeltaLayerArtifacts,
        BranchDeltaLayerRecord, BranchDeltaReplacementProofEntry, BranchHeadRecord,
        BranchSharedBaseRecord, CommitParentRecord, StoreState,
    },
    state::branch_lifecycle::AppliedBranchCreation,
};

#[derive(Debug)]
pub(crate) struct AppliedSharedBaseBranchCreation {
    branch_creation: AppliedBranchCreation,
    branch_identity: String,
}

#[derive(Debug)]
pub(crate) struct AppliedBranchDeltaRewrite {
    replacement_layer_id: Option<u64>,
    removed_layers: Vec<BranchDeltaLayerRecord>,
    previous_next_branch_delta_layer_id: u64,
}

#[derive(Debug)]
pub(crate) struct AppliedBranchDeltaRebuild {
    branch_id: BranchId,
    inserted_layer_ids: Vec<u64>,
    removed_layers: Vec<BranchDeltaLayerRecord>,
    previous_next_branch_delta_layer_id: u64,
}

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

    pub fn admit_same_branch_descendant(
        &self,
        request: BranchDeltaReadRequest,
    ) -> Result<SameBranchDescendantWitness, StoreError> {
        let branch_identity = branch_key(&request.branch_id);
        let basis = self
            .branch_shared_base_records
            .get(&branch_identity)
            .cloned()
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BranchDeltaBasisUnsupported,
                    format!(
                        "branch `{}` does not publish a shared-base branch delta basis yet",
                        request.branch_id.0
                    ),
                )
            })?;
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

    fn plan_branch_delta_read_from_witness(
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

    pub fn read_branch_delta(
        &self,
        witness: SameBranchDescendantWitness,
    ) -> Result<BranchDeltaReadResult, StoreError> {
        let plan = self.plan_branch_delta_read_from_witness(&witness)?;
        match plan.strategy {
            BranchDeltaReadStrategy::DirectLayerRead => {
                let export = self.materialize_branch_delta_export(&plan)?;
                let parity = self
                    .read_branch_delta_control(witness.clone())?
                    .authoritative_export()
                    .clone()
                    .into_canonicalized();
                let direct = export.clone().into_canonicalized();
                if direct.canonical_json() != parity.canonical_json() {
                    return Err(StoreError::new(
                        StoreErrorKind::BranchDeltaReplayParityViolation,
                        format!(
                            "branch delta direct-layer read for branch `{}` target {} diverged from authoritative replay parity",
                            plan.locality.branch_id.0, plan.locality.target_commit_id.0
                        ),
                    ));
                }
                Ok(BranchDeltaReadResult::new(plan, export))
            }
            BranchDeltaReadStrategy::AuthorityReplayControl => Err(StoreError::new(
                StoreErrorKind::BranchDeltaBasisUnsupported,
                format!(
                    "branch delta read for branch `{}` target {} requires the authority replay control lane and is not admitted on the direct-layer path",
                    plan.locality.branch_id.0, plan.locality.target_commit_id.0
                ),
            )),
            BranchDeltaReadStrategy::EmptyBranchReuse => {
                let export = self.materialize_branch_delta_export(&plan)?;
                Ok(BranchDeltaReadResult::new(plan, export))
            }
        }
    }

    pub fn read_branch_delta_control(
        &self,
        witness: SameBranchDescendantWitness,
    ) -> Result<BranchDeltaReadResult, StoreError> {
        let plan = self.plan_branch_delta_control_from_witness(&witness);
        let export = self.materialize_authority_replay_control_export(&witness)?;
        Ok(BranchDeltaReadResult::new(plan, export))
    }

    pub fn read_branch_delta_control_from_milestone_7_reference(
        &self,
        reference: Milestone7IndependentReference,
    ) -> Result<BranchDeltaReadResult, StoreError> {
        let witness = self.admit_same_branch_descendant(BranchDeltaReadRequest::new(
            reference.branch_id().clone(),
            reference.target_commit_id(),
        ))?;
        self.read_branch_delta_control(witness)
    }

    pub fn plan_delta_rewrite(
        &self,
        request: BranchDeltaRewriteRequest,
    ) -> Result<BranchDeltaRewritePlan, StoreError> {
        let read_plan = self.plan_branch_delta_read(BranchDeltaReadRequest::new(
            request.branch_id,
            request.target_commit_id,
        ))?;
        match read_plan.strategy {
            BranchDeltaReadStrategy::EmptyBranchReuse => Ok(BranchDeltaRewritePlan::new(
                BranchDeltaRewriteStrategy::NotNeeded,
                None,
                0,
            )),
            BranchDeltaReadStrategy::AuthorityReplayControl => Ok(BranchDeltaRewritePlan::new(
                BranchDeltaRewriteStrategy::RejectAsTooBroad,
                None,
                read_plan.performance.replay_commit_count,
            )),
            BranchDeltaReadStrategy::DirectLayerRead => {
                if read_plan.used_layer_ids.len() <= 1 {
                    return Ok(BranchDeltaRewritePlan::new(
                        BranchDeltaRewriteStrategy::NotNeeded,
                        None,
                        read_plan.used_layer_ids.len(),
                    ));
                }
                if read_plan.used_layer_ids.len() > MAX_REWRITE_LAYER_WIDTH {
                    return Err(StoreError::new(
                        StoreErrorKind::BranchDeltaRewriteBudgetExceeded,
                        format!(
                            "branch delta rewrite planning for branch `{}` target {} exceeded the admitted rewrite width budget (width={}, max_width={})",
                            read_plan.locality.branch_id.0,
                            read_plan.locality.target_commit_id.0,
                            read_plan.used_layer_ids.len(),
                            MAX_REWRITE_LAYER_WIDTH
                        ),
                    ));
                }
                Ok(BranchDeltaRewritePlan::new(
                    BranchDeltaRewriteStrategy::ReplaceContiguousSegment,
                    Some(RewriteEligibleDeltaSegment::new(
                        read_plan.locality.branch_id.clone(),
                        read_plan.locality.base_frontier_commit_id,
                        read_plan.locality.target_commit_id,
                        read_plan.used_layer_ids.clone(),
                        read_plan.commit_ids.clone(),
                    )),
                    read_plan.used_layer_ids.len(),
                ))
            }
        }
    }

    pub fn recommend_delta_rewrite(
        &self,
        request: BranchDeltaRewriteRequest,
    ) -> Result<BranchDeltaRewriteRecommendation, StoreError> {
        let plan = self.plan_delta_rewrite(request)?;
        let decision = match plan.strategy() {
            BranchDeltaRewriteStrategy::NotNeeded => BranchDeltaRewritePolicyDecision::NoAction,
            BranchDeltaRewriteStrategy::RejectAsTooBroad => {
                BranchDeltaRewritePolicyDecision::RejectAsTooBroad
            }
            BranchDeltaRewriteStrategy::ReplaceContiguousSegment => {
                if plan.rewrite_breadth() >= RECOMMENDED_REWRITE_LAYER_WIDTH {
                    BranchDeltaRewritePolicyDecision::CompactNow
                } else {
                    BranchDeltaRewritePolicyDecision::Defer
                }
            }
        };
        Ok(BranchDeltaRewriteRecommendation {
            decision,
            plan,
            recommended_layer_width: RECOMMENDED_REWRITE_LAYER_WIDTH,
        })
    }

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

    fn trace_linear_branch_segment(
        &self,
        branch_id: &BranchId,
        base_frontier_commit_id: Option<CommitId>,
        target_commit_id: CommitId,
    ) -> Result<Vec<CommitId>, StoreError> {
        if Some(target_commit_id) == base_frontier_commit_id {
            return Ok(Vec::new());
        }

        let mut reversed = Vec::new();
        let mut current = target_commit_id;
        loop {
            let record = self.commit_record(current).ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BranchDeltaPublicationGap,
                    format!("branch delta traversal missing commit {}", current.0),
                )
            })?;
            if record.envelope.branch_context != *branch_id {
                return Err(StoreError::new(
                    StoreErrorKind::BranchDeltaReadTargetIllegal,
                    format!(
                        "commit {} drifted onto branch `{}` during branch delta planning",
                        current.0, record.envelope.branch_context.0
                    ),
                ));
            }
            reversed.push(current);

            match record.envelope.commit.parents.as_slice() {
                [] => {
                    if base_frontier_commit_id.is_none() {
                        break;
                    }
                    return Err(StoreError::new(
                        StoreErrorKind::BranchDeltaReadTargetIllegal,
                        format!(
                            "target commit {} does not descend from the branch basis",
                            target_commit_id.0
                        ),
                    ));
                }
                [parent] => {
                    if Some(*parent) == base_frontier_commit_id {
                        break;
                    }
                    current = *parent;
                }
                _ => {
                    return Err(StoreError::new(
                        StoreErrorKind::BranchDeltaTargetRequiresMergeAwareWidening,
                        format!(
                            "target commit {} requires merge-aware target widening, which milestone 5 does not admit",
                            target_commit_id.0
                        ),
                    ));
                }
            }
        }

        reversed.reverse();
        Ok(reversed)
    }

    pub(crate) fn find_covering_branch_delta_layer(
        &self,
        branch_id: &BranchId,
        base_frontier_commit_id: Option<CommitId>,
        remaining_commit_ids: &[CommitId],
    ) -> Option<&BranchDeltaLayerRecord> {
        self.branch_delta_layer_records
            .values()
            .filter(|record| {
                record.branch_id == *branch_id
                    && record.base_frontier_commit_id == base_frontier_commit_id
                    && remaining_commit_ids.starts_with(&record.commit_ids)
            })
            .max_by_key(|record| record.commit_ids.len())
    }

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

    fn materialize_branch_delta_export(
        &self,
        plan: &BranchDeltaReadPlan,
    ) -> Result<AuthoritativeExportBundle, StoreError> {
        let basis = self
            .branch_shared_base_records
            .get(&branch_key(&plan.locality.branch_id))
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BranchDeltaBasisUnsupported,
                    format!(
                        "branch `{}` does not publish a shared-base branch delta basis yet",
                        plan.locality.branch_id.0
                    ),
                )
            })?;
        let mut export = if let Some(source_frontier_commit_id) = basis.source_frontier_commit_id {
            self.build_snapshot_image(&basis.source_branch_id, source_frontier_commit_id)?
                .authoritative_export()
                .clone()
        } else {
            empty_authoritative_export(self.canonicalization_version)
        };

        let branch_record = self
            .branch_records
            .get(&branch_key(&plan.locality.branch_id))
            .cloned()
            .ok_or_else(|| StoreError::unknown_branch(&plan.locality.branch_id))?;
        export.branch_records = vec![branch_record];

        for layer_id in &plan.used_layer_ids {
            let layer = self
                .branch_delta_layer_records
                .get(&layer_id.0)
                .ok_or_else(|| {
                    StoreError::new(
                        StoreErrorKind::BranchDeltaPublicationGap,
                        format!(
                            "branch delta direct-layer read missing published layer {} during materialization",
                            layer_id.0
                        ),
                    )
                })?;
            let layer_artifacts = if branch_delta_layer_artifacts_empty(&layer.artifacts) {
                self.build_branch_delta_layer_artifacts(&layer.branch_id, &layer.commit_ids)?
            } else {
                layer.artifacts.clone()
            };
            export
                .commit_envelopes
                .extend(layer_artifacts.commit_envelopes.iter().cloned());
            export
                .commit_parent_records
                .extend(layer_artifacts.commit_parent_records.iter().cloned());
        }

        let final_commit_set = export
            .commit_envelopes
            .iter()
            .map(|record| record.envelope.commit.commit_id)
            .collect::<BTreeSet<_>>();
        export.commit_support_summaries = self
            .commit_support_summaries
            .values()
            .filter(|record| final_commit_set.contains(&record.commit_id))
            .cloned()
            .collect();
        export.schema_support_records = self
            .schema_support_records
            .values()
            .filter(|record| final_commit_set.contains(&record.commit_id))
            .cloned()
            .collect();
        export.lineage_support_records = self
            .lineage_support_records
            .values()
            .filter(|record| final_commit_set.contains(&record.commit_id))
            .cloned()
            .collect();
        let target_record = self
            .commit_record(plan.locality.target_commit_id)
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BranchDeltaReadTargetIllegal,
                    format!(
                        "target commit {} not found during branch delta materialization",
                        plan.locality.target_commit_id.0
                    ),
                )
            })?;
        export.branch_head_records = vec![BranchHeadRecord {
            branch_id: plan.locality.branch_id.clone(),
            head_commit_id: Some(plan.locality.target_commit_id),
            head_commit_digest: Some(target_record.envelope_digest.clone()),
            head_update_sequence: target_record.commit_sequence,
        }];
        export.durable_cursor_identity_records = self
            .durable_cursor_identity_records
            .values()
            .filter(|record| {
                record.branch_id == plan.locality.branch_id
                    && final_commit_set.contains(&record.latest_basis_commit_id)
            })
            .cloned()
            .collect();
        export.subscriber_checkpoint_records = self
            .subscriber_checkpoint_records
            .values()
            .filter(|record| {
                record.branch_id == plan.locality.branch_id
                    && final_commit_set.contains(&record.basis_commit_id)
            })
            .cloned()
            .collect();
        export.authoritative_artifact_digests = self
            .rebuild_authoritative_export_digests(&export)?
            .into_values()
            .collect();
        export.canonicalize_order();
        Ok(export)
    }

    fn plan_branch_delta_control_from_witness(
        &self,
        witness: &SameBranchDescendantWitness,
    ) -> BranchDeltaReadPlan {
        let commit_ids = witness.commit_ids().to_vec();
        let locality = BranchDeltaLocality {
            branch_id: witness.branch_id().clone(),
            base_frontier_commit_id: witness.base_frontier_commit_id(),
            target_commit_id: witness.target_commit_id(),
            commit_span: commit_ids.len(),
        };
        BranchDeltaReadPlan {
            strategy: BranchDeltaReadStrategy::AuthorityReplayControl,
            regime: regime_for_commit_span(commit_ids.len()),
            locality,
            used_layer_ids: Vec::new(),
            commit_ids: commit_ids.clone(),
            performance: BranchDeltaPerformanceEnvelope {
                layers_traversed: 0,
                records_decoded: commit_ids.len(),
                replay_commit_count: commit_ids.len(),
                fallback_class: BranchDeltaFallbackClass::None,
                complexity_status: ComplexityStatus::Verified,
            },
        }
    }

    fn materialize_authority_replay_control_export(
        &self,
        witness: &SameBranchDescendantWitness,
    ) -> Result<AuthoritativeExportBundle, StoreError> {
        let basis = self
            .branch_shared_base_records
            .get(&branch_key(witness.branch_id()))
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BranchDeltaBasisUnsupported,
                    format!(
                        "branch `{}` does not publish a shared-base branch delta basis yet",
                        witness.branch_id().0
                    ),
                )
            })?;
        let mut export = if let Some(source_frontier_commit_id) = basis.source_frontier_commit_id {
            self.build_snapshot_image(&basis.source_branch_id, source_frontier_commit_id)?
                .authoritative_export()
                .clone()
        } else {
            empty_authoritative_export(self.canonicalization_version)
        };

        let branch_record = self
            .branch_records
            .get(&branch_key(witness.branch_id()))
            .cloned()
            .ok_or_else(|| StoreError::unknown_branch(witness.branch_id()))?;
        export.branch_records = vec![branch_record];

        for commit_id in witness.commit_ids() {
            let commit_record = self.commit_record(*commit_id).cloned().ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BranchDeltaPublicationGap,
                    format!(
                        "authority replay control read missing commit {} for branch `{}`",
                        commit_id.0,
                        witness.branch_id().0
                    ),
                )
            })?;
            export.commit_envelopes.push(commit_record.clone());
            export.commit_parent_records.extend(
                commit_record
                    .envelope
                    .commit
                    .parents
                    .iter()
                    .copied()
                    .enumerate()
                    .map(|(parent_position, parent_commit_id)| CommitParentRecord {
                        commit_id: *commit_id,
                        parent_position,
                        parent_commit_id,
                    }),
            );
        }

        let final_commit_set = export
            .commit_envelopes
            .iter()
            .map(|record| record.envelope.commit.commit_id)
            .collect::<BTreeSet<_>>();
        export.commit_support_summaries = self
            .commit_support_summaries
            .values()
            .filter(|record| final_commit_set.contains(&record.commit_id))
            .cloned()
            .collect();
        export.schema_support_records = self
            .schema_support_records
            .values()
            .filter(|record| final_commit_set.contains(&record.commit_id))
            .cloned()
            .collect();
        export.lineage_support_records = self
            .lineage_support_records
            .values()
            .filter(|record| final_commit_set.contains(&record.commit_id))
            .cloned()
            .collect();

        let target_record = self
            .commit_record(witness.target_commit_id())
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BranchDeltaReadTargetIllegal,
                    format!(
                    "target commit {} not found during authority replay control materialization",
                    witness.target_commit_id().0
                ),
                )
            })?;
        export.branch_head_records = vec![BranchHeadRecord {
            branch_id: witness.branch_id().clone(),
            head_commit_id: Some(witness.target_commit_id()),
            head_commit_digest: Some(target_record.envelope_digest.clone()),
            head_update_sequence: target_record.commit_sequence,
        }];
        export.durable_cursor_identity_records = self
            .durable_cursor_identity_records
            .values()
            .filter(|record| {
                record.branch_id == *witness.branch_id()
                    && final_commit_set.contains(&record.latest_basis_commit_id)
            })
            .cloned()
            .collect();
        export.subscriber_checkpoint_records = self
            .subscriber_checkpoint_records
            .values()
            .filter(|record| {
                record.branch_id == *witness.branch_id()
                    && final_commit_set.contains(&record.basis_commit_id)
            })
            .cloned()
            .collect();
        export.authoritative_artifact_digests = self
            .rebuild_authoritative_export_digests(&export)?
            .into_values()
            .collect();
        export.canonicalize_order();
        Ok(export)
    }

    fn rebuild_authoritative_export_digests(
        &self,
        export: &AuthoritativeExportBundle,
    ) -> Result<BTreeMap<String, AuthoritativeArtifactDigestRecord>, StoreError> {
        let mut digests = BTreeMap::new();
        for branch_record in &export.branch_records {
            insert_digest_record(
                &mut digests,
                AuthoritativeArtifactFamily::BranchRecord,
                branch_record.branch_id.0.clone(),
                self.canonicalization_version,
                stable_structural_digest(branch_record)?,
            );
        }
        for branch_head_record in &export.branch_head_records {
            insert_digest_record(
                &mut digests,
                AuthoritativeArtifactFamily::BranchHeadRecord,
                branch_head_record.branch_id.0.clone(),
                self.canonicalization_version,
                stable_structural_digest(branch_head_record)?,
            );
        }
        for commit_record in &export.commit_envelopes {
            insert_digest_record(
                &mut digests,
                AuthoritativeArtifactFamily::CommitEnvelope,
                commit_artifact_id(commit_record.envelope.commit.commit_id),
                self.canonicalization_version,
                commit_record.envelope_digest.clone(),
            );
        }
        for parent_record in &export.commit_parent_records {
            insert_digest_record(
                &mut digests,
                AuthoritativeArtifactFamily::CommitParentRecord,
                parent_artifact_id(parent_record.commit_id, parent_record.parent_position),
                self.canonicalization_version,
                stable_structural_digest(parent_record)?,
            );
        }
        for summary in &export.commit_support_summaries {
            insert_digest_record(
                &mut digests,
                AuthoritativeArtifactFamily::CommitSupportSummary,
                commit_support_summary_artifact_id(summary.commit_id),
                self.canonicalization_version,
                stable_structural_digest(summary)?,
            );
        }
        for record in &export.schema_support_records {
            insert_digest_record(
                &mut digests,
                AuthoritativeArtifactFamily::SchemaSupportRecord,
                record.artifact_id.clone(),
                self.canonicalization_version,
                stable_structural_digest(record)?,
            );
        }
        for record in &export.lineage_support_records {
            insert_digest_record(
                &mut digests,
                AuthoritativeArtifactFamily::LineageSupportRecord,
                record.artifact_id.clone(),
                self.canonicalization_version,
                stable_structural_digest(record)?,
            );
        }
        for record in &export.durable_cursor_identity_records {
            insert_digest_record(
                &mut digests,
                AuthoritativeArtifactFamily::DurableCursorIdentityRecord,
                durable_cursor_identity_artifact_id(&record.cursor_id),
                self.canonicalization_version,
                stable_structural_digest(record)?,
            );
        }
        for record in &export.subscriber_checkpoint_records {
            insert_digest_record(
                &mut digests,
                AuthoritativeArtifactFamily::SubscriberCheckpointRecord,
                subscriber_checkpoint_artifact_id(&record.cursor_id, record.checkpoint_sequence),
                self.canonicalization_version,
                stable_structural_digest(record)?,
            );
        }
        Ok(digests)
    }
}

fn combine_branch_delta_layer_artifacts(
    removed_layers: &[BranchDeltaLayerRecord],
) -> BranchDeltaLayerArtifacts {
    let mut artifacts = BranchDeltaLayerArtifacts {
        commit_envelopes: Vec::new(),
        commit_parent_records: Vec::new(),
        commit_support_summaries: Vec::new(),
        schema_support_records: Vec::new(),
        lineage_support_records: Vec::new(),
    };
    for layer in removed_layers {
        artifacts
            .commit_envelopes
            .extend(layer.artifacts.commit_envelopes.iter().cloned());
        artifacts
            .commit_parent_records
            .extend(layer.artifacts.commit_parent_records.iter().cloned());
        artifacts
            .commit_support_summaries
            .extend(layer.artifacts.commit_support_summaries.iter().cloned());
        artifacts
            .schema_support_records
            .extend(layer.artifacts.schema_support_records.iter().cloned());
        artifacts
            .lineage_support_records
            .extend(layer.artifacts.lineage_support_records.iter().cloned());
    }
    artifacts.canonicalize_order();
    artifacts
}

fn regime_for_commit_span(commit_span: usize) -> BranchDeltaReadRegime {
    if commit_span <= 8 {
        BranchDeltaReadRegime::Sparse
    } else {
        BranchDeltaReadRegime::Dense
    }
}

impl StoreState {
    fn build_branch_delta_layer_artifacts(
        &self,
        branch_id: &BranchId,
        commit_ids: &[CommitId],
    ) -> Result<BranchDeltaLayerArtifacts, StoreError> {
        let mut artifacts = BranchDeltaLayerArtifacts {
            commit_envelopes: Vec::new(),
            commit_parent_records: Vec::new(),
            commit_support_summaries: Vec::new(),
            schema_support_records: Vec::new(),
            lineage_support_records: Vec::new(),
        };
        for commit_id in commit_ids {
            let commit_record = self.commit_record(*commit_id).cloned().ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BranchDeltaPublicationGap,
                    format!(
                        "branch delta layer artifact build missing commit {} for branch `{}`",
                        commit_id.0, branch_id.0
                    ),
                )
            })?;
            artifacts.commit_envelopes.push(commit_record.clone());
            artifacts.commit_parent_records.extend(
                commit_record
                    .envelope
                    .commit
                    .parents
                    .iter()
                    .copied()
                    .enumerate()
                    .map(|(parent_position, parent_commit_id)| CommitParentRecord {
                        commit_id: *commit_id,
                        parent_position,
                        parent_commit_id,
                    }),
            );
            if let Some(summary) = self.commit_support_summaries.get(&commit_id.0).cloned() {
                artifacts.commit_support_summaries.push(summary);
            }
            if let Some(schema_record) = self
                .schema_support_records
                .get(&schema_support_artifact_id(*commit_id))
                .cloned()
            {
                artifacts.schema_support_records.push(schema_record);
            }
            if let Some(lineage_record) = self
                .lineage_support_records
                .get(&lineage_support_artifact_id(*commit_id))
                .cloned()
            {
                artifacts.lineage_support_records.push(lineage_record);
            }
        }
        artifacts.canonicalize_order();
        Ok(artifacts)
    }

    pub(crate) fn backfill_missing_branch_delta_layer_artifacts(
        &mut self,
    ) -> Result<(), StoreError> {
        let layer_ids = self
            .branch_delta_layer_records
            .iter()
            .filter_map(|(layer_id, record)| record.artifacts.is_empty().then_some(*layer_id))
            .collect::<Vec<_>>();
        for layer_id in layer_ids {
            let (branch_id, commit_ids) = {
                let record = self
                    .branch_delta_layer_records
                    .get(&layer_id)
                    .expect("selected branch delta layer should exist");
                (record.branch_id.clone(), record.commit_ids.clone())
            };
            let artifacts = self.build_branch_delta_layer_artifacts(&branch_id, &commit_ids)?;
            if let Some(record) = self.branch_delta_layer_records.get_mut(&layer_id) {
                record.artifacts = artifacts;
            }
        }
        Ok(())
    }

    pub(crate) fn milestone_5_delta_storage_report(
        &self,
        branch_id: BranchId,
        target_commit_id: CommitId,
        direct_plan: &BranchDeltaReadPlan,
        control_plan: &BranchDeltaReadPlan,
    ) -> Result<Milestone5DeltaStorageReport, StoreError> {
        let basis = self
            .branch_shared_base_records
            .get(&branch_key(&branch_id))
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BranchDeltaBasisUnsupported,
                    format!(
                        "branch `{}` does not publish a shared-base branch delta basis yet",
                        branch_id.0
                    ),
                )
            })?;
        let live_layers = self
            .branch_delta_layer_records
            .values()
            .filter(|record| record.branch_id == branch_id)
            .collect::<Vec<_>>();
        Ok(Milestone5DeltaStorageReport {
            branch_id,
            target_commit_id,
            shared_base_source_branch_id: basis.source_branch_id.clone(),
            shared_base_source_frontier_commit_id: basis.source_frontier_commit_id,
            live_layer_count: live_layers.len(),
            live_layer_commit_count: live_layers
                .iter()
                .map(|record| record.commit_ids.len())
                .sum(),
            replacement_layer_count: live_layers
                .iter()
                .filter(|record| !record.replacement_of_layer_ids.is_empty())
                .count(),
            direct_path: Milestone5ReadPathReport::from(direct_plan),
            control_path: Milestone5ReadPathReport::from(control_plan),
            control_reference_surface: "Milestone7IndependentReference".to_string(),
        })
    }
}

fn empty_authoritative_export(canonicalization_version: u32) -> AuthoritativeExportBundle {
    AuthoritativeExportBundle {
        canonicalization_version,
        branch_records: Vec::new(),
        branch_head_records: Vec::new(),
        commit_envelopes: Vec::new(),
        commit_parent_records: Vec::new(),
        commit_support_summaries: Vec::new(),
        schema_support_records: Vec::new(),
        lineage_support_records: Vec::new(),
        durable_cursor_identity_records: Vec::new(),
        subscriber_checkpoint_records: Vec::new(),
        authoritative_artifact_digests: Vec::new(),
    }
}

fn empty_branch_delta_layer_artifacts() -> BranchDeltaLayerArtifacts {
    BranchDeltaLayerArtifacts {
        commit_envelopes: Vec::new(),
        commit_parent_records: Vec::new(),
        commit_support_summaries: Vec::new(),
        schema_support_records: Vec::new(),
        lineage_support_records: Vec::new(),
    }
}

fn branch_delta_layer_artifacts_empty(artifacts: &BranchDeltaLayerArtifacts) -> bool {
    artifacts.commit_envelopes.is_empty()
        && artifacts.commit_parent_records.is_empty()
        && artifacts.commit_support_summaries.is_empty()
        && artifacts.schema_support_records.is_empty()
        && artifacts.lineage_support_records.is_empty()
}

fn insert_digest_record(
    digests: &mut BTreeMap<String, AuthoritativeArtifactDigestRecord>,
    artifact_family: AuthoritativeArtifactFamily,
    artifact_id: String,
    canonicalization_version: u32,
    artifact_digest: String,
) {
    let key = format!(
        "{:?}:{}:v{}",
        artifact_family, artifact_id, canonicalization_version
    );
    digests.insert(
        key,
        AuthoritativeArtifactDigestRecord {
            artifact_family,
            artifact_id,
            canonicalization_version,
            digest_algorithm: "sha256".to_string(),
            artifact_digest,
        },
    );
}
