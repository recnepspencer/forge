use crate::{
    failure::{StoreError, StoreErrorKind},
    layout::{
        admit_milestone_7_reference_from_plan, admit_milestone_9_reference_from_frozen,
        classify_layout_request, freeze_chunk_model_from_plan, AdmittedAspectLayoutReadPlan,
        AspectLayoutReadPlanDecision, AspectLayoutReadRequest, DedupAdmittedBlockReuse,
        Milestone7IndependentLayoutReference, Milestone9PhysicalChunkReference,
        EQUIVALENCE_CONTRACT_VERSION,
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
}
