use crate::runtime::{
    WorthUiExecutionPlan, WorthUiExecutionPlanDigest, WorthUiLaneAdmission,
    WorthUiOrdinaryFrameTarget, WorthUiOrdinaryLaneFrameDenial, WorthUiOrdinaryLaneFrameReceipt,
    WorthUiOrdinaryLanePlanDenial, WorthUiOrdinaryPlanAvailability,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiSealedExecutionPlanBundle {
    generation_identity:
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity,
    generation_witness:
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationWitness,
    digest: WorthUiExecutionPlanDigest,
    cross_lane_receipt: super::WorthUiCrossLaneBundleReceipt,
    execution_plan: WorthUiExecutionPlan,
    lane_admission: WorthUiLaneAdmission,
    host_binding: crate::facade::WorthUiHostPlanBinding,
    ordinary: crate::runtime::execution::ordinary_lane::WorthUiActiveOrdinaryPlanPosture,
    virtualized:
        crate::runtime::execution::virtualized_data_lane::WorthUiActiveVirtualizedDataPlanPosture,
    canvas_spatial:
        crate::runtime::execution::canvas_spatial_lane::WorthUiActiveCanvasSpatialPlanPosture,
    realtime_overlay:
        crate::runtime::execution::realtime_overlay_lane::WorthUiActiveRealtimePlanPosture,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum WorthUiExecutionPlanBundleDenial {
    ForeignLoweringAuthority,
    OrdinaryPlan(WorthUiOrdinaryLanePlanDenial),
    VirtualizedPlan(crate::runtime::WorthUiVirtualizedDataPlanDenial),
    CanvasSpatialPlan(crate::runtime::WorthUiCanvasSpatialPlanDenial),
    RealtimeOverlayPlan(crate::runtime::WorthUiHudPlanDenial),
}

impl WorthUiSealedExecutionPlanBundle {
    pub(crate) fn seal(
        authority: &crate::runtime::planning::WorthUiExecutionPlanLoweringFacts,
        execution_plan: WorthUiExecutionPlan,
        lane_admission: &WorthUiLaneAdmission,
        host_binding: crate::facade::WorthUiHostPlanBinding,
    ) -> Result<Self, WorthUiExecutionPlanBundleDenial> {
        if !execution_plan.shares_lowering_authority_with(authority) {
            return Err(WorthUiExecutionPlanBundleDenial::ForeignLoweringAuthority);
        }
        let ordinary =
            crate::runtime::execution::ordinary_lane::WorthUiActiveOrdinaryPlanPosture::lower(
                &execution_plan,
                lane_admission,
            )
            .map_err(WorthUiExecutionPlanBundleDenial::OrdinaryPlan)?;
        let virtualized =
            crate::runtime::execution::virtualized_data_lane::WorthUiActiveVirtualizedDataPlanPosture::lower(
                &execution_plan,
                lane_admission,
            )
            .map_err(WorthUiExecutionPlanBundleDenial::VirtualizedPlan)?;
        let canvas_spatial =
            crate::runtime::execution::canvas_spatial_lane::WorthUiActiveCanvasSpatialPlanPosture::lower(
                &execution_plan,
                lane_admission,
                host_binding,
            )
            .map_err(WorthUiExecutionPlanBundleDenial::CanvasSpatialPlan)?;
        let realtime_overlay =
            crate::runtime::execution::realtime_overlay_lane::WorthUiActiveRealtimePlanPosture::lower(
                &execution_plan,
                lane_admission,
                host_binding,
            )
            .map_err(WorthUiExecutionPlanBundleDenial::RealtimeOverlayPlan)?;
        let digest =
            crate::runtime::planning::plan_equivalence::WorthUiExecutionPlanDigestor::regional_digest(
                &execution_plan,
            )
            .0;
        let cross_lane_receipt =
            super::WorthUiCrossLaneBundleReceipt::new(super::WorthUiCrossLaneBundleReceiptInput {
                plan_digest: digest,
                handle_allocation_basis_digest: execution_plan.handle_receipt().basis_digest(),
                lane_support_digest: lane_admission.support_digest(),
                lane_plan_input_basis_digest: lane_admission.plan_input_basis_digest(),
                construction_counters: execution_plan.construction_counters(),
                ordinary: ordinary.availability(),
                virtualized: virtualized.availability(),
                canvas_spatial: canvas_spatial.availability(),
                realtime_overlay: realtime_overlay.availability(),
            });
        Ok(Self {
            generation_identity: authority
                .candidate_application_authority()
                .generation_identity()
                .clone(),
            generation_witness: authority
                .candidate_application_authority()
                .generation_witness(),
            digest,
            cross_lane_receipt,
            execution_plan,
            lane_admission: lane_admission.clone(),
            host_binding,
            ordinary,
            virtualized,
            canvas_spatial,
            realtime_overlay,
        })
    }

    pub(crate) fn generation_identity(
        &self,
    ) -> &crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity
    {
        &self.generation_identity
    }

    pub(crate) fn digest(&self) -> WorthUiExecutionPlanDigest {
        self.digest
    }

    pub(crate) fn cross_lane_receipt(&self) -> super::WorthUiCrossLaneBundleReceipt {
        self.cross_lane_receipt
    }

    pub(crate) fn execution_plan(&self) -> &WorthUiExecutionPlan {
        &self.execution_plan
    }

    pub(crate) fn mounted_projection_rows(&self) -> Vec<(u64, u32)> {
        self.execution_plan.mounted_projection_rows()
    }

    pub(crate) fn classify_candidate(
        &self,
        candidate: &Self,
    ) -> crate::runtime::WorthUiExecutablePlanDecision {
        use crate::runtime::{
            WorthUiExecutablePlanDecision as Decision,
            WorthUiExecutablePlanEquivalenceDenial as Denial,
            WorthUiPlanEquivalenceEvidenceReference, WorthUiPlanEquivalenceSummary,
            WorthUiPlanRegionTransition,
        };

        if !self
            .host_binding
            .shares_session_with(candidate.host_binding)
        {
            return Decision::Denied(Denial::ForeignHostSession);
        }
        let regional = candidate.execution_plan.regional_evidence();
        let Some(predecessor_artifact_digest) = regional.predecessor_artifact_digest() else {
            return Decision::Denied(Denial::MissingPredecessorProof);
        };
        let Some(predecessor_plan_digest) = regional.predecessor_plan_digest() else {
            return Decision::Denied(Denial::MissingPredecessorProof);
        };
        if predecessor_artifact_digest
            != self
                .execution_plan
                .regional_evidence()
                .candidate_artifact_digest()
        {
            return Decision::Denied(Denial::PredecessorArtifactMismatch);
        }
        if predecessor_plan_digest != self.digest.raw() {
            return Decision::Denied(Denial::PredecessorPlanMismatch);
        }

        let changed_region_count = regional
            .transitions()
            .iter()
            .filter(|evidence| evidence.transition() != WorthUiPlanRegionTransition::Reused)
            .count();
        let region_counters = candidate.execution_plan.region_storage_counters();
        let regions_match = changed_region_count == 0 && self.digest == candidate.digest;
        let evidence_reference = WorthUiPlanEquivalenceEvidenceReference::new(
            predecessor_artifact_digest,
            regional.candidate_artifact_digest(),
            predecessor_plan_digest,
            candidate.digest.raw(),
            regional.transitions().len(),
        );
        let summary = WorthUiPlanEquivalenceSummary::new(
            self.digest.raw(),
            candidate.digest.raw(),
            changed_region_count,
            region_counters.exact_comparison_count(),
            evidence_reference,
        );
        let shared_contracts_match = self
            .lane_admission
            .executable_contract_matches(&candidate.lane_admission)
            && self
                .host_binding
                .executable_contract_matches(candidate.host_binding);
        if regions_match && shared_contracts_match {
            Decision::ExactSemanticNoOp(summary)
        } else if shared_contracts_match && changed_region_count > 0 {
            Decision::BoundedChangedRegions(summary)
        } else {
            Decision::RebuildRequired(summary)
        }
    }

    pub(crate) fn handle_arena_identity(&self) -> crate::runtime::WorthUiHandleArenaIdentity {
        self.execution_plan.handle_receipt().arena_identity()
    }

    pub(crate) fn shares_lowering_identity_with(
        &self,
        identity: &crate::runtime::planning::WorthUiExecutionPlanLoweringIdentity,
    ) -> bool {
        self.execution_plan.shares_lowering_identity_with(identity)
    }

    pub(crate) fn ordinary_availability(&self) -> WorthUiOrdinaryPlanAvailability {
        self.ordinary.availability()
    }

    pub(crate) fn virtualized_availability(
        &self,
    ) -> crate::runtime::WorthUiVirtualizedPlanAvailability {
        self.virtualized.availability()
    }

    pub(crate) fn query_fact_link_for_plan_index(
        &self,
        plan_index: u32,
    ) -> Option<crate::runtime::WorthUiQuerySettledFactLink> {
        self.virtualized
            .row_for_plan_index(plan_index)
            .map(|row| row.settled_fact_link().clone())
    }

    pub(crate) fn query_fact_link_for_binding_id(
        &self,
        binding_id: &crate::capability::ViewBindingId,
    ) -> Option<crate::runtime::WorthUiQueryLaneFactLink> {
        let identity = crate::runtime::WorthUiPlanRegionIdentity::from_exact_basis(
            binding_id.as_str().to_owned(),
        );
        let handle = self.execution_plan.region_store().handle_for(&identity)?;
        let plan_index = u32::try_from(handle.stable_slot()).ok()?;
        let executable = self
            .execution_plan
            .region_store()
            .executable_for(&identity)?;
        Some(crate::runtime::WorthUiQueryLaneFactLink::from_active_plan(
            plan_index,
            executable
                .query_binding_identity_reference()?
                .as_ref()
                .clone(),
            executable.query_settled_fact_link()?.as_ref().clone(),
            &self.generation_identity,
            self.generation_witness.clone(),
        ))
    }

    pub(crate) fn canvas_spatial_availability(
        &self,
    ) -> crate::runtime::WorthUiCanvasSpatialPlanAvailability {
        self.canvas_spatial.availability()
    }

    pub(crate) fn first_canvas_spatial_handle(&self) -> Option<crate::runtime::WorthUiLaneHandle> {
        self.canvas_spatial.first_handle()
    }

    pub(crate) fn execute_canvas_spatial(
        &self,
        target: crate::runtime::WorthUiCanvasSpatialFrameTarget,
    ) -> Result<
        crate::runtime::WorthUiCanvasSpatialFrameReceipt,
        crate::runtime::WorthUiCanvasSpatialFrameDenial,
    > {
        self.canvas_spatial.execute(target)
    }

    pub(crate) fn canvas_spatial_summary(
        &self,
        handle: crate::runtime::WorthUiLaneHandle,
    ) -> Result<
        crate::runtime::WorthUiCanvasSpatialTargetSummary,
        crate::runtime::WorthUiCanvasSpatialInspectionDenial,
    > {
        self.canvas_spatial.summary(handle)
    }

    pub(crate) fn realtime_availability(&self) -> crate::runtime::WorthUiRealtimePlanAvailability {
        self.realtime_overlay.availability()
    }

    pub(crate) fn first_realtime_handle(
        &self,
    ) -> Option<crate::runtime::WorthUiRendererSurfaceHandle> {
        self.realtime_overlay.first_handle()
    }

    pub(crate) fn execute_realtime(
        &self,
        target: crate::runtime::WorthUiRealtimeFrameTarget,
    ) -> Result<
        crate::runtime::WorthUiRealtimeFrameReceipt,
        crate::runtime::WorthUiRealtimeFrameDenial,
    > {
        self.realtime_overlay.execute(target)
    }

    pub(crate) fn realtime_summary(
        &self,
        handle: crate::runtime::WorthUiRendererSurfaceHandle,
    ) -> Result<
        crate::runtime::WorthUiRealtimeTargetSummary,
        crate::runtime::WorthUiRealtimeInspectionDenial,
    > {
        self.realtime_overlay.summary(handle)
    }

    pub(crate) fn query_succession_changes(
        &self,
        candidate: &Self,
    ) -> Vec<worth_ui_query_binding::WorthUiQueryBindingSuccessionChange> {
        candidate
            .execution_plan
            .regional_evidence()
            .transitions()
            .iter()
            .filter(|transition| {
                transition.transition() != crate::runtime::WorthUiPlanRegionTransition::Reused
            })
            .filter_map(|transition| {
                let identity = transition.region_identity();
                let predecessor = self
                    .execution_plan
                    .region_store()
                    .executable_for(identity)
                    .and_then(|executable| executable.query_settled_fact_link())
                    .map(|link| link.installed_reference().clone());
                let successor = candidate
                    .execution_plan
                    .region_store()
                    .executable_for(identity)
                    .and_then(|executable| executable.query_settled_fact_link())
                    .map(|link| link.installed_reference().clone());
                (predecessor.is_some() || successor.is_some()).then(|| {
                    worth_ui_query_binding::WorthUiQueryBindingSuccessionChange::new(
                        predecessor,
                        successor,
                    )
                })
            })
            .collect()
    }

    pub(crate) fn execute_virtualized(
        &self,
        query_binding: &worth_ui_query_binding::WorthUiRuntimeQueryBinding,
        target: crate::runtime::WorthUiVirtualizedDataFrameTarget,
    ) -> Result<
        crate::runtime::WorthUiVirtualizedDataFrameReceipt,
        crate::runtime::WorthUiVirtualizedDataFrameDenial,
    > {
        self.virtualized.execute(query_binding, target)
    }

    pub(crate) fn virtualized_summary(
        &self,
        query_binding: &worth_ui_query_binding::WorthUiRuntimeQueryBinding,
        request: crate::runtime::WorthUiVirtualizedPlanSummaryRequest,
    ) -> Result<
        crate::runtime::WorthUiVirtualizedPlanSummary,
        crate::runtime::WorthUiVirtualizedPlanSummaryDenial,
    > {
        self.virtualized.summary(query_binding, request)
    }

    pub(crate) fn execute_ordinary(
        &self,
        target: WorthUiOrdinaryFrameTarget,
    ) -> Result<WorthUiOrdinaryLaneFrameReceipt, WorthUiOrdinaryLaneFrameDenial> {
        self.ordinary.execute(target)
    }

    pub(crate) fn ordinary_summary(
        &self,
        request: crate::runtime::WorthUiOrdinaryPlanSummaryRequest,
    ) -> Result<
        crate::runtime::WorthUiOrdinaryPlanSummary,
        crate::runtime::WorthUiOrdinaryPlanSummaryDenial,
    > {
        self.ordinary.summary(request)
    }
}
