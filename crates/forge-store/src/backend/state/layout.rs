use crate::{
    failure::{StoreError, StoreErrorKind},
    layout::{
        admit_milestone_7_reference_from_plan, admit_milestone_9_reference_from_frozen,
        classify_layout_request, freeze_chunk_model_from_plan, AdmittedAspectLayoutReadPlan,
        AspectLayoutReadExecutionDecision, AspectLayoutReadExecutionResult, AspectLayoutReadPlanDecision,
        AspectLayoutReadRequest, DedupAdmittedBlockReuse,
        Milestone7IndependentLayoutReference,
        Milestone9PhysicalChunkReference, StructuralBlockLookup,
        StructuralBlockLookupResult, EQUIVALENCE_CONTRACT_VERSION,
    },
};

use crate::backend::records::StoreState;

impl StoreState {
    pub fn plan_aspect_layout_read(
        &self,
        request: AspectLayoutReadRequest,
    ) -> Result<AspectLayoutReadPlanDecision, StoreError> {
        let target_commit_id = request.target().frontier_commit_id();
        let branch_id = request.target().branch_id().clone();
        let record = self.commit_record(target_commit_id).ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::AspectLayoutReadTargetIllegal,
                format!("layout target commit {} not found", target_commit_id.0),
            )
        })?;
        if record.envelope.branch_context != branch_id {
            return Err(StoreError::new(
                StoreErrorKind::AspectLayoutReadTargetIllegal,
                format!(
                    "layout target commit {} belongs to branch `{}` not branch `{}`",
                    target_commit_id.0,
                    record.envelope.branch_context.0,
                    branch_id.0
                ),
            ));
        }
        classify_layout_request(request)
    }

    pub fn admit_structural_block_reuse(
        &self,
        plan: AdmittedAspectLayoutReadPlan,
    ) -> Result<DedupAdmittedBlockReuse, StoreError> {
        if plan.slice_ids().is_empty() {
            return Err(StoreError::new(
                StoreErrorKind::StructuralBlockEquivalenceViolation,
                "admitted structural block reuse requires at least one canonical layout slice",
            ));
        }
        Ok(DedupAdmittedBlockReuse::new(
            &plan,
            EQUIVALENCE_CONTRACT_VERSION,
        ))
    }

    pub fn freeze_chunk_model(
        &self,
        plan: AdmittedAspectLayoutReadPlan,
    ) -> Result<crate::ChunkModelFrozenPhysicalLayout, StoreError> {
        freeze_chunk_model_from_plan(&plan)
    }

    pub fn admit_milestone_7_independent_layout_reference(
        &self,
        plan: AdmittedAspectLayoutReadPlan,
    ) -> Result<Milestone7IndependentLayoutReference, StoreError> {
        admit_milestone_7_reference_from_plan(&plan)
    }

    pub fn admit_milestone_9_physical_chunk_reference(
        &self,
        frozen: crate::ChunkModelFrozenPhysicalLayout,
    ) -> Result<Milestone9PhysicalChunkReference, StoreError> {
        if frozen.witness().ordered_slice_ids().is_empty() {
            return Err(StoreError::new(
                StoreErrorKind::ConcurrentBulkBoundaryViolation,
                "milestone 9 physical chunk references require a non-empty deterministic chunk witness",
            ));
        }
        Ok(admit_milestone_9_reference_from_frozen(&frozen))
    }

    pub fn structural_block_lookup(
        &self,
        lookup: StructuralBlockLookup,
    ) -> Result<StructuralBlockLookupResult, StoreError> {
        let artifact_id = crate::layout::structural_block_artifact_id(lookup.structural_block_id());
        let record = self
            .milestone_6_structural_block_records
            .get(&artifact_id)
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::AspectLayoutArtifactMissing,
                    format!("milestone 6 structural block `{artifact_id}` not found"),
                )
            })?;
        Ok(StructuralBlockLookupResult::new(
            record.structural_block_id.clone(),
            record.scope_class.clone(),
            record.equivalence_contract_version,
            record.slice_ids.clone(),
            record.supporting_layout_materialization_artifact_ids.clone(),
        ))
    }

    pub fn execute_aspect_layout_read(
        &self,
        request: AspectLayoutReadRequest,
    ) -> Result<AspectLayoutReadExecutionDecision, StoreError> {
        let decision = self.plan_aspect_layout_read(request)?;
        match decision {
            AspectLayoutReadPlanDecision::Admitted(plan) => {
                let scope_membership_artifact_id =
                    crate::layout::layout_scope_membership_artifact_id(plan.request())?;
                let scope_membership = self
                    .milestone_6_scope_slice_membership_records
                    .get(&scope_membership_artifact_id)
                    .ok_or_else(|| {
                        StoreError::new(
                            StoreErrorKind::AspectLayoutArtifactMissing,
                            format!(
                                "milestone 6 scope membership `{scope_membership_artifact_id}` not found"
                            ),
                        )
                    })?;
                let structural_block_artifact_id =
                    crate::layout::structural_block_artifact_id(plan.structural_block_id());
                let structural_block = self
                    .milestone_6_structural_block_records
                    .get(&structural_block_artifact_id)
                    .ok_or_else(|| {
                        StoreError::new(
                            StoreErrorKind::AspectLayoutArtifactMissing,
                            format!(
                                "milestone 6 structural block `{structural_block_artifact_id}` not found"
                            ),
                        )
                    })?;
                let frozen = freeze_chunk_model_from_plan(&plan)?;
                let chunk_membership_artifact_id =
                    crate::layout::chunk_membership_artifact_id(&frozen);
                let chunk_membership = self
                    .milestone_6_chunk_membership_records
                    .get(&chunk_membership_artifact_id)
                    .ok_or_else(|| {
                        StoreError::new(
                            StoreErrorKind::AspectLayoutArtifactMissing,
                            format!(
                                "milestone 6 chunk membership `{chunk_membership_artifact_id}` not found"
                            ),
                        )
                    })?;
                if scope_membership.slice_ids != plan.slice_ids()
                    || structural_block.slice_ids != plan.slice_ids()
                    || chunk_membership.slice_ids != plan.slice_ids()
                {
                    return Err(StoreError::backend_integrity(
                        "milestone 6 execution records drifted from the admitted aspect layout plan",
                    ));
                }
                let materialization = self
                    .milestone_6_layout_materialization_records
                    .get(&scope_membership.layout_materialization_artifact_id)
                    .ok_or_else(|| {
                        StoreError::new(
                            StoreErrorKind::AspectLayoutArtifactMissing,
                            format!(
                                "milestone 6 layout materialization `{}` not found",
                                scope_membership.layout_materialization_artifact_id
                            ),
                        )
                    })?;
                Ok(AspectLayoutReadExecutionDecision::Admitted(
                    AspectLayoutReadExecutionResult::new(
                        plan,
                        scope_membership_artifact_id,
                        structural_block_artifact_id,
                        chunk_membership_artifact_id,
                        scope_membership.layout_materialization_artifact_id.clone(),
                        materialization
                            .materialization
                            .semantic_truth_digest()
                            .to_string(),
                        materialization
                            .materialization
                            .authoritative_commit_count(),
                    ),
                ))
            }
            AspectLayoutReadPlanDecision::Fallback(plan) => {
                Ok(AspectLayoutReadExecutionDecision::Fallback(plan))
            }
            AspectLayoutReadPlanDecision::Rejected(plan) => {
                Ok(AspectLayoutReadExecutionDecision::Rejected(plan))
            }
        }
    }

}
