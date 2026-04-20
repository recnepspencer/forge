use crate::failure::{StoreError, StoreErrorKind};
use crate::layout::{
    AdmittedAspectLayoutReadPlan, AspectLayoutReadExecutionDecision, AspectLayoutReadPlanDecision,
    AspectLayoutReadRequest, ChunkModelFrozenPhysicalLayout, DedupAdmittedBlockReuse,
    DedupBackedReadResult, Milestone7IndependentLayoutReference, Milestone9PhysicalChunkReference,
    StructuralBlockLookup, StructuralBlockLookupResult,
};

use super::{StateBackedStoreBackend, StatePersistence};

impl<P: StatePersistence> StateBackedStoreBackend<P> {
    pub fn plan_aspect_layout_read(
        &self,
        request: AspectLayoutReadRequest,
    ) -> Result<AspectLayoutReadPlanDecision, StoreError> {
        let decision = self.state.plan_aspect_layout_read(request)?;
        match &decision {
            AspectLayoutReadPlanDecision::Admitted(plan) => self.counters.record_aspect_layout_plan(
                true,
                false,
                false,
                plan.performance().layout_slices_read,
                plan.performance().blocks_decoded,
                plan.performance().control_replay_breadth,
            ),
            AspectLayoutReadPlanDecision::Fallback(plan) => self.counters.record_aspect_layout_plan(
                false,
                true,
                false,
                plan.performance().layout_slices_read,
                plan.performance().blocks_decoded,
                plan.performance().control_replay_breadth,
            ),
            AspectLayoutReadPlanDecision::Rejected(_) => {
                self.counters
                    .record_aspect_layout_plan(false, false, true, 0, 0, 0)
            }
        }
        Ok(decision)
    }

    pub fn admit_structural_block_reuse(
        &self,
        plan: AdmittedAspectLayoutReadPlan,
    ) -> Result<DedupAdmittedBlockReuse, StoreError> {
        let admitted = self.state.admit_structural_block_reuse(plan)?;
        self.counters.record_structural_block_reuse_admission();
        Ok(admitted)
    }

    pub fn freeze_chunk_model(
        &self,
        plan: AdmittedAspectLayoutReadPlan,
    ) -> Result<ChunkModelFrozenPhysicalLayout, StoreError> {
        match self.state.freeze_chunk_model(plan) {
            Ok(frozen) => {
                self.counters.record_chunk_model_freeze();
                Ok(frozen)
            }
            Err(error) => {
                if matches!(
                    error.kind(),
                    StoreErrorKind::PhysicalChunkDeterminismViolation
                ) {
                    self.counters.record_physical_chunk_determinism_violation();
                }
                Err(error)
            }
        }
    }

    pub fn admit_milestone_7_independent_layout_reference(
        &self,
        plan: AdmittedAspectLayoutReadPlan,
    ) -> Result<Milestone7IndependentLayoutReference, StoreError> {
        let reference = self
            .state
            .admit_milestone_7_independent_layout_reference(plan)?;
        self.counters
            .record_milestone_7_layout_reference_admission();
        Ok(reference)
    }

    pub fn admit_milestone_9_physical_chunk_reference(
        &self,
        frozen: ChunkModelFrozenPhysicalLayout,
    ) -> Result<Milestone9PhysicalChunkReference, StoreError> {
        let reference = self
            .state
            .admit_milestone_9_physical_chunk_reference(frozen)?;
        self.counters
            .record_milestone_9_physical_chunk_reference_admission();
        Ok(reference)
    }

    pub fn structural_block_lookup(
        &self,
        lookup: StructuralBlockLookup,
    ) -> Result<StructuralBlockLookupResult, StoreError> {
        match self.state.structural_block_lookup(lookup) {
            Ok(result) => {
                self.counters.record_structural_block_lookup(true);
                Ok(result)
            }
            Err(error) => {
                if matches!(error.kind(), StoreErrorKind::AspectLayoutArtifactMissing) {
                    self.counters.record_structural_block_lookup(false);
                }
                Err(error)
            }
        }
    }

    pub fn execute_aspect_layout_read(
        &self,
        request: AspectLayoutReadRequest,
    ) -> Result<AspectLayoutReadExecutionDecision, StoreError> {
        self.state.execute_aspect_layout_read(request)
    }

    pub fn execute_dedup_backed_read(
        &self,
        request: AspectLayoutReadRequest,
    ) -> Result<DedupBackedReadResult, StoreError> {
        let read = match self.execute_aspect_layout_read(request)? {
            AspectLayoutReadExecutionDecision::Admitted(read) => read,
            AspectLayoutReadExecutionDecision::Fallback(plan) => {
                return Err(StoreError::new(
                    StoreErrorKind::AspectLayoutFallbackRequired,
                    plan.reason().to_string(),
                ))
            }
            AspectLayoutReadExecutionDecision::Rejected(plan) => {
                return Err(StoreError::new(
                    StoreErrorKind::AspectScopeUnsupported,
                    plan.reason().to_string(),
                ))
            }
        };
        let lookup = self.structural_block_lookup(StructuralBlockLookup::new(
            read.plan().structural_block_id().clone(),
        ))?;
        if lookup.slice_ids() != read.plan().slice_ids() {
            return Err(StoreError::backend_integrity(
                "dedup-backed read structural block lookup drifted from admitted plan slice ids",
            ));
        }
        Ok(DedupBackedReadResult::new(read, lookup))
    }
}
