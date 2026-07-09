use crate::failure::StoreError;
use crate::layout::{
    AdmittedAspectLayoutReadPlan, AspectLayoutReadExecutionDecision, AspectLayoutReadPlanDecision,
    AspectLayoutReadRequest, ChunkModelFrozenPhysicalLayout, DedupAdmittedBlockReuse,
    DedupBackedReadResult, Milestone7IndependentLayoutReference, Milestone9PhysicalChunkReference,
    StructuralBlockLookup, StructuralBlockLookupResult,
};
use worth_relational::facade::history::BranchId;

use super::{dispatch_mut, dispatch_ref, StoreBackend};

impl StoreBackend {
    pub fn plan_aspect_layout_read(
        &self,
        request: AspectLayoutReadRequest,
    ) -> Result<AspectLayoutReadPlanDecision, StoreError> {
        dispatch_ref!(self, |backend| backend.plan_aspect_layout_read(request))
    }

    pub fn admit_structural_block_reuse(
        &self,
        plan: AdmittedAspectLayoutReadPlan,
    ) -> Result<DedupAdmittedBlockReuse, StoreError> {
        dispatch_ref!(self, |backend| backend.admit_structural_block_reuse(plan))
    }

    pub fn freeze_chunk_model(
        &self,
        plan: AdmittedAspectLayoutReadPlan,
    ) -> Result<ChunkModelFrozenPhysicalLayout, StoreError> {
        dispatch_ref!(self, |backend| backend.freeze_chunk_model(plan))
    }

    pub fn admit_milestone_7_independent_layout_reference(
        &self,
        plan: AdmittedAspectLayoutReadPlan,
    ) -> Result<Milestone7IndependentLayoutReference, StoreError> {
        dispatch_ref!(self, |backend| backend
            .admit_milestone_7_independent_layout_reference(plan))
    }

    pub fn admit_milestone_9_physical_chunk_reference(
        &self,
        frozen: ChunkModelFrozenPhysicalLayout,
    ) -> Result<Milestone9PhysicalChunkReference, StoreError> {
        dispatch_ref!(self, |backend| backend
            .admit_milestone_9_physical_chunk_reference(frozen))
    }

    pub fn materialize_milestone_6_layout_support(
        &mut self,
        request: AspectLayoutReadRequest,
    ) -> Result<crate::Milestone6LayoutMaterialization, StoreError> {
        dispatch_mut!(self, |backend| backend
            .materialize_milestone_6_layout_support(request))
    }

    pub fn fetch_milestone_6_layout_support(
        &self,
        request: AspectLayoutReadRequest,
    ) -> Result<crate::Milestone6LayoutMaterialization, StoreError> {
        dispatch_ref!(self, |backend| backend
            .fetch_milestone_6_layout_support(request))
    }

    pub(crate) fn note_milestone_6_scope_prepare(
        &mut self,
        request: &AspectLayoutReadRequest,
    ) -> Result<u64, StoreError> {
        dispatch_mut!(self, |backend| backend
            .note_milestone_6_scope_prepare(request))
    }

    pub(crate) fn record_milestone_6_proof_only_prepare(&self) {
        dispatch_ref!(self, |backend| backend
            .record_milestone_6_proof_only_prepare())
    }

    pub(crate) fn record_milestone_6_on_demand_materialize(&self) {
        dispatch_ref!(self, |backend| backend
            .record_milestone_6_on_demand_materialize())
    }

    pub(crate) fn record_milestone_6_policy_eager_resolution(&self) {
        dispatch_ref!(self, |backend| backend
            .record_milestone_6_policy_eager_resolution())
    }

    pub(crate) fn record_milestone_6_policy_eager_publish(&self) {
        dispatch_ref!(self, |backend| backend
            .record_milestone_6_policy_eager_publish())
    }

    pub(crate) fn record_milestone_6_policy_eager_reuse_existing(&self) {
        dispatch_ref!(self, |backend| backend
            .record_milestone_6_policy_eager_reuse_existing())
    }

    pub(crate) fn milestone_6_branch_has_materialized_support(&self, branch_id: &BranchId) -> bool {
        dispatch_ref!(self, |backend| backend
            .milestone_6_branch_has_materialized_support(branch_id))
    }

    pub(crate) fn fetch_existing_milestone_6_layout_support(
        &self,
        artifact_id: &str,
    ) -> Result<crate::Milestone6LayoutMaterialization, StoreError> {
        dispatch_ref!(self, |backend| backend
            .fetch_existing_milestone_6_layout_support(artifact_id))
    }

    pub fn rebuild_milestone_6_derived_artifacts_from_materializations(
        &mut self,
    ) -> Result<crate::Milestone6DerivedArtifactRebuildReport, StoreError> {
        dispatch_mut!(self, |backend| backend
            .rebuild_milestone_6_derived_artifacts_from_materializations())
    }

    pub fn rebuild_milestone_6_derived_artifacts_from_authority(
        &mut self,
    ) -> Result<crate::Milestone6DerivedArtifactRebuildReport, StoreError> {
        dispatch_mut!(self, |backend| backend
            .rebuild_milestone_6_derived_artifacts_from_authority())
    }

    pub fn structural_block_lookup(
        &self,
        lookup: StructuralBlockLookup,
    ) -> Result<StructuralBlockLookupResult, StoreError> {
        dispatch_ref!(self, |backend| backend.structural_block_lookup(lookup))
    }

    pub fn execute_aspect_layout_read(
        &self,
        request: AspectLayoutReadRequest,
    ) -> Result<AspectLayoutReadExecutionDecision, StoreError> {
        dispatch_ref!(self, |backend| backend.execute_aspect_layout_read(request))
    }

    pub fn execute_dedup_backed_read(
        &self,
        request: AspectLayoutReadRequest,
    ) -> Result<DedupBackedReadResult, StoreError> {
        dispatch_ref!(self, |backend| backend.execute_dedup_backed_read(request))
    }
}
