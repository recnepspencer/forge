use crate::{
    failure::{StoreError, StoreErrorKind},
    layout::{
        AdmittedAspectLayoutReadPlan, AspectLayoutReadPlanDecision, AspectLayoutReadRequest,
        ChunkModelFrozenPhysicalLayout, DedupAdmittedBlockReuse, Milestone6LayoutSupportLane,
        Milestone6LayoutSupportPolicy, Milestone6LayoutSupportPublicationDisposition,
        Milestone6PreparedLayoutSupport, Milestone6ResolvedLayoutSupportLane,
        Milestone7IndependentLayoutReference, Milestone9PhysicalChunkReference,
    },
};

use super::WORTHStore;

impl WORTHStore {
    pub fn plan_aspect_layout_read(
        &self,
        request: AspectLayoutReadRequest,
    ) -> Result<AspectLayoutReadPlanDecision, StoreError> {
        self.backend.plan_aspect_layout_read(request)
    }

    pub fn admit_structural_block_reuse(
        &self,
        plan: AdmittedAspectLayoutReadPlan,
    ) -> Result<DedupAdmittedBlockReuse, StoreError> {
        self.backend.admit_structural_block_reuse(plan)
    }

    pub fn freeze_chunk_model(
        &self,
        plan: AdmittedAspectLayoutReadPlan,
    ) -> Result<ChunkModelFrozenPhysicalLayout, StoreError> {
        self.backend.freeze_chunk_model(plan)
    }

    pub fn admit_milestone_7_independent_layout_reference(
        &self,
        plan: AdmittedAspectLayoutReadPlan,
    ) -> Result<Milestone7IndependentLayoutReference, StoreError> {
        self.backend
            .admit_milestone_7_independent_layout_reference(plan)
    }

    pub fn admit_milestone_9_physical_chunk_reference(
        &self,
        frozen: ChunkModelFrozenPhysicalLayout,
    ) -> Result<Milestone9PhysicalChunkReference, StoreError> {
        self.backend
            .admit_milestone_9_physical_chunk_reference(frozen)
    }

    pub fn materialize_milestone_6_layout_support(
        &mut self,
        request: AspectLayoutReadRequest,
    ) -> Result<crate::Milestone6LayoutMaterialization, StoreError> {
        self.backend.materialize_milestone_6_layout_support(request)
    }

    pub fn fetch_milestone_6_layout_support(
        &self,
        request: AspectLayoutReadRequest,
    ) -> Result<crate::Milestone6LayoutMaterialization, StoreError> {
        self.backend.fetch_milestone_6_layout_support(request)
    }

    pub fn prepare_milestone_6_layout_support(
        &mut self,
        request: AspectLayoutReadRequest,
        lane: Milestone6LayoutSupportLane,
    ) -> Result<Milestone6PreparedLayoutSupport, StoreError> {
        self.prepare_milestone_6_layout_support_with_policy(
            request,
            lane,
            Milestone6LayoutSupportPolicy::new(false, false, 0),
        )
    }

    pub fn prepare_milestone_6_layout_support_with_policy(
        &mut self,
        request: AspectLayoutReadRequest,
        lane: Milestone6LayoutSupportLane,
        policy: Milestone6LayoutSupportPolicy,
    ) -> Result<Milestone6PreparedLayoutSupport, StoreError> {
        match lane {
            Milestone6LayoutSupportLane::ProofOnly => {
                self.backend.record_milestone_6_proof_only_prepare();
                self.require_admitted_aspect_layout_plan(
                    request.clone(),
                    "milestone 6 proof-only layout support",
                )?;
                Ok(Milestone6PreparedLayoutSupport::proof_only(request))
            }
            Milestone6LayoutSupportLane::OnDemandMaterialized => {
                self.backend.record_milestone_6_on_demand_materialize();
                let (materialization, publication_disposition) =
                    self.fetch_or_publish_milestone_6_layout_materialization(request.clone())?;
                Ok(Milestone6PreparedLayoutSupport::resolved(
                    lane,
                    Milestone6ResolvedLayoutSupportLane::OnDemandMaterialized,
                    publication_disposition,
                    request,
                    Some(materialization.artifact_id().to_string()),
                ))
            }
            Milestone6LayoutSupportLane::PolicyEagerMaterialized => {
                self.backend.record_milestone_6_policy_eager_resolution();
                let resolution = self.resolve_milestone_6_policy_eager_lane(&request, policy)?;
                match resolution {
                    Milestone6ResolvedLayoutSupportLane::ProofOnly => {
                        self.backend.record_milestone_6_proof_only_prepare();
                        self.require_admitted_aspect_layout_plan(
                            request.clone(),
                            "milestone 6 policy-eager proof-only layout support",
                        )?;
                        Ok(Milestone6PreparedLayoutSupport::resolved(
                            lane,
                            resolution,
                            Milestone6LayoutSupportPublicationDisposition::None,
                            request,
                            None,
                        ))
                    }
                    Milestone6ResolvedLayoutSupportLane::PolicyEagerMaterializedPublished
                    | Milestone6ResolvedLayoutSupportLane::PolicyEagerMaterializedReuseExisting => {
                        let (materialization, publication_disposition) = self
                            .fetch_or_publish_milestone_6_layout_materialization(request.clone())?;
                        let resolved_lane = match publication_disposition {
                            Milestone6LayoutSupportPublicationDisposition::PublishedThisOperation => {
                                self.backend.record_milestone_6_policy_eager_publish();
                                Milestone6ResolvedLayoutSupportLane::PolicyEagerMaterializedPublished
                            }
                            Milestone6LayoutSupportPublicationDisposition::ReusedExisting => {
                                self.backend
                                    .record_milestone_6_policy_eager_reuse_existing();
                                Milestone6ResolvedLayoutSupportLane::PolicyEagerMaterializedReuseExisting
                            }
                            Milestone6LayoutSupportPublicationDisposition::None => {
                                return Err(StoreError::backend_integrity(
                                    "policy-eager materialized lane completed without a publication disposition",
                                ))
                            }
                        };
                        Ok(Milestone6PreparedLayoutSupport::resolved(
                            lane,
                            resolved_lane,
                            publication_disposition,
                            request,
                            Some(materialization.artifact_id().to_string()),
                        ))
                    }
                    Milestone6ResolvedLayoutSupportLane::OnDemandMaterialized => {
                        Err(StoreError::backend_integrity(
                            "policy-eager lane resolved to illegal on-demand posture",
                        ))
                    }
                }
            }
        }
    }

    fn fetch_or_publish_milestone_6_layout_materialization(
        &mut self,
        request: AspectLayoutReadRequest,
    ) -> Result<
        (
            crate::Milestone6LayoutMaterialization,
            Milestone6LayoutSupportPublicationDisposition,
        ),
        StoreError,
    > {
        match self.fetch_milestone_6_layout_support(request.clone()) {
            Ok(materialization) => Ok((
                materialization,
                Milestone6LayoutSupportPublicationDisposition::ReusedExisting,
            )),
            Err(error) if matches!(error.kind(), StoreErrorKind::AspectLayoutArtifactMissing) => {
                Ok((
                    self.materialize_milestone_6_layout_support(request)?,
                    Milestone6LayoutSupportPublicationDisposition::PublishedThisOperation,
                ))
            }
            Err(error) => Err(error),
        }
    }

    fn resolve_milestone_6_policy_eager_lane(
        &mut self,
        request: &AspectLayoutReadRequest,
        policy: Milestone6LayoutSupportPolicy,
    ) -> Result<Milestone6ResolvedLayoutSupportLane, StoreError> {
        let repeated_scope_count = self.backend.note_milestone_6_scope_prepare(request)?;
        let branch_is_hot = policy.materialize_hot_branch_reads()
            && self
                .backend
                .milestone_6_branch_has_materialized_support(request.target().branch_id());
        let repeated_scope_is_hot = policy.materialize_repeated_scope_reads()
            && policy.repeated_scope_threshold() > 0
            && repeated_scope_count >= policy.repeated_scope_threshold();
        if !(branch_is_hot || repeated_scope_is_hot) {
            return Ok(Milestone6ResolvedLayoutSupportLane::ProofOnly);
        }
        match self.fetch_milestone_6_layout_support(request.clone()) {
            Ok(_) => Ok(Milestone6ResolvedLayoutSupportLane::PolicyEagerMaterializedReuseExisting),
            Err(error) if matches!(error.kind(), StoreErrorKind::AspectLayoutArtifactMissing) => {
                Ok(Milestone6ResolvedLayoutSupportLane::PolicyEagerMaterializedPublished)
            }
            Err(error) => Err(error),
        }
    }

    pub fn rebuild_milestone_6_derived_artifacts_from_materializations(
        &mut self,
    ) -> Result<crate::Milestone6DerivedArtifactRebuildReport, StoreError> {
        self.backend
            .rebuild_milestone_6_derived_artifacts_from_materializations()
    }

    pub fn rebuild_milestone_6_derived_artifacts_from_authority(
        &mut self,
    ) -> Result<crate::Milestone6DerivedArtifactRebuildReport, StoreError> {
        self.backend
            .rebuild_milestone_6_derived_artifacts_from_authority()
    }
}
