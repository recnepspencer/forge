use crate::capability::CapabilitySnapshot;
use crate::runtime::validation_reload::evidence::WorthUiValidationReloadEvidenceBuilder;
use crate::runtime::{
    WorthUiCandidateAdmission, WorthUiDurableStateFamily, WorthUiExecutionPlan,
    WorthUiReadyActivation, WorthUiReplacementCandidate, WorthUiRuntimeArtifactComparison,
    WorthUiRuntimeArtifactComparisonOutcome, WorthUiRuntimeAuthoringSnapshot, WorthUiRuntimeHost,
    WorthUiRuntimeInstanceId, WorthUiSourceProvider, WorthUiValidationReloadEvidence,
    WorthUiValidationReloadRequest, WorthUiValidationReloadStage, WorthUiValidationReloadStatus,
    WorthUiWatcherEvent,
};

pub struct WorthUiValidationPreparedReload {
    runtime_instance_id: WorthUiRuntimeInstanceId,
    evidence: WorthUiValidationReloadEvidence,
    ready: Option<WorthUiReadyActivation>,
    candidate_plan: Option<WorthUiExecutionPlan>,
    candidate_authoring_snapshot: Option<WorthUiRuntimeAuthoringSnapshot>,
}

impl WorthUiRuntimeHost {
    pub fn prepare_validation_reload(
        &self,
        snapshot: &CapabilitySnapshot,
        request: WorthUiValidationReloadRequest,
    ) -> WorthUiValidationPreparedReload {
        let before = self.inspect_active();
        let evidence = WorthUiValidationReloadEvidence::builder(
            self.instance_id().raw(),
            before.artifact_digest(),
            before.active_plan_digest(),
        );
        if request.is_empty() {
            return denied_reload(evidence, self, WorthUiValidationReloadStage::EmptyRequest);
        }

        let (candidate, candidate_authoring_snapshot, evidence) =
            match self.lower_validation_reload_request(snapshot, request, evidence) {
                Ok(lowered) => lowered,
                Err((stage, evidence)) => return denied_reload(evidence, self, stage),
            };
        let admitted =
            match WorthUiCandidateAdmission::for_active_basis(self.replacement_admission_basis())
                .admit(candidate)
            {
                Ok(admitted) => admitted,
                Err(_) => {
                    return denied_reload(
                        evidence,
                        self,
                        WorthUiValidationReloadStage::CandidateAdmission,
                    );
                }
            };
        let comparison = match self.compare_admitted_replacement(&admitted) {
            Ok(comparison) => comparison,
            Err(_) => {
                return denied_reload(
                    evidence,
                    self,
                    WorthUiValidationReloadStage::ArtifactComparison,
                );
            }
        };
        let evidence = evidence.record_candidate_artifact(comparison.candidate_artifact_digest());
        if comparison.outcome() == WorthUiRuntimeArtifactComparisonOutcome::EquivalentNoOp {
            return WorthUiValidationPreparedReload {
                runtime_instance_id: self.instance_id(),
                evidence: finish_evidence(
                    evidence,
                    self,
                    WorthUiValidationReloadStatus::EquivalentNoOp,
                ),
                ready: None,
                candidate_plan: None,
                candidate_authoring_snapshot: None,
            };
        }

        self.plan_validation_reload_replacement(
            admitted,
            candidate_authoring_snapshot,
            comparison,
            evidence,
        )
    }

    fn lower_validation_reload_request(
        &self,
        snapshot: &CapabilitySnapshot,
        request: WorthUiValidationReloadRequest,
        evidence: WorthUiValidationReloadEvidenceBuilder,
    ) -> Result<
        (
            WorthUiReplacementCandidate,
            Option<WorthUiRuntimeAuthoringSnapshot>,
            WorthUiValidationReloadEvidenceBuilder,
        ),
        (
            WorthUiValidationReloadStage,
            WorthUiValidationReloadEvidenceBuilder,
        ),
    > {
        let provider = provider_from_request(request);
        let provider_id = provider.id().to_owned();
        let mut session = self.source_ingress(provider).start();
        let batch = session
            .ingest([WorthUiWatcherEvent::provider_revision(provider_id)])
            .map_err(|_| {
                (
                    WorthUiValidationReloadStage::SourceIngress,
                    evidence.clone(),
                )
            })?;
        let submission = batch.lower_to_candidate_submission(snapshot).map_err(|_| {
            (
                WorthUiValidationReloadStage::CandidateSubmission,
                evidence.clone(),
            )
        })?;
        let (candidate, candidate_authoring_snapshot, revision, ordering_receipt, counters) =
            submission.into_parts();
        let evidence = evidence.record_source_ingress(
            revision.final_package_digest(),
            ordering_receipt.receipt_digest(),
            counters,
        );
        let evidence = evidence.record_candidate_submission(counters);
        Ok((candidate, candidate_authoring_snapshot, evidence))
    }

    fn plan_validation_reload_replacement(
        &self,
        admitted: crate::runtime::WorthUiAdmittedReplacementCandidate,
        candidate_authoring_snapshot: Option<WorthUiRuntimeAuthoringSnapshot>,
        comparison: WorthUiRuntimeArtifactComparison,
        evidence: WorthUiValidationReloadEvidenceBuilder,
    ) -> WorthUiValidationPreparedReload {
        let impact = match self.classify_replacement_impact(&comparison, &admitted) {
            Ok(impact) => impact,
            Err(_) => {
                return denied_reload(
                    evidence,
                    self,
                    WorthUiValidationReloadStage::ImpactClassification,
                );
            }
        };
        let narrowing = match self.narrow_replacement_impact(&impact, &admitted) {
            Ok(narrowing) => narrowing,
            Err(_) => {
                return denied_reload(
                    evidence,
                    self,
                    WorthUiValidationReloadStage::ImpactNarrowing,
                );
            }
        };
        let identity = match self.build_identity_match_graph(&narrowing, &admitted) {
            Ok(identity) => identity,
            Err(_) => {
                return denied_reload(
                    evidence,
                    self,
                    WorthUiValidationReloadStage::IdentityMatching,
                );
            }
        };
        let node_plan = match self.classify_node_replacements(&impact, &narrowing, &identity) {
            Ok(node_plan) => node_plan,
            Err(_) => {
                return denied_reload(
                    evidence,
                    self,
                    WorthUiValidationReloadStage::NodeReplacement,
                );
            }
        };
        let inventory = match platform_inventory(self).build_for_replacement(&node_plan) {
            Ok(inventory) => inventory,
            Err(_) => {
                return denied_reload(evidence, self, WorthUiValidationReloadStage::StateInventory)
            }
        };
        let reconciliation = match self.reconcile_durable_state(&node_plan, &inventory) {
            Ok(reconciliation) => reconciliation,
            Err(_) => {
                return denied_reload(
                    evidence,
                    self,
                    WorthUiValidationReloadStage::DurableStateReconciliation,
                );
            }
        };
        let query_comparison = match self.compare_query_bindings(&node_plan, &narrowing, &admitted)
        {
            Ok(comparison) => comparison,
            Err(_) => {
                return denied_reload(
                    evidence,
                    self,
                    WorthUiValidationReloadStage::QueryBindingComparison,
                );
            }
        };
        let query_rebind = match self.plan_query_live_rebinds(
            &query_comparison,
            &node_plan,
            &narrowing,
            &admitted,
        ) {
            Ok(query_rebind) => query_rebind,
            Err(_) => {
                return denied_reload(
                    evidence,
                    self,
                    WorthUiValidationReloadStage::QueryLiveRebind,
                );
            }
        };
        let evidence = evidence.record_query_and_state_planning(
            query_comparison.counters().bindings_compared(),
            query_rebind.entries().len(),
            reconciliation.receipts().len(),
        );
        let lowering_input = self.prepare_pending_execution_plan_lowering_input(
            &node_plan,
            &reconciliation,
            &query_rebind,
        );
        let pending = match self.stage_replacement_activation(
            admitted,
            &impact,
            &narrowing,
            &node_plan,
            Some(&reconciliation),
            Some(&query_rebind),
            Some(&lowering_input),
        ) {
            Ok(pending) => pending,
            Err(_) => {
                return denied_reload(
                    evidence,
                    self,
                    WorthUiValidationReloadStage::ActivationStaging,
                );
            }
        };
        let plan_input = match self.prepare_execution_plan_input(&pending) {
            Ok(plan_input) => plan_input,
            Err(_) => {
                return denied_reload(evidence, self, WorthUiValidationReloadStage::PlanLowering)
            }
        };
        let handles = match self.allocate_runtime_handles(&plan_input) {
            Ok(handles) => handles,
            Err(_) => {
                return denied_reload(
                    evidence,
                    self,
                    WorthUiValidationReloadStage::HandleAllocation,
                )
            }
        };
        let candidate_plan = match self.assemble_execution_plan_topology(&plan_input, &handles) {
            Ok(candidate_plan) => candidate_plan,
            Err(denial) => {
                return denied_reload_with_detail(
                    evidence,
                    self,
                    WorthUiValidationReloadStage::TopologyAssembly,
                    format!("{denial:?}"),
                );
            }
        };
        let candidate_plan_digest = self.digest_execution_plan(&candidate_plan).raw();
        let ready = match self.prepare_ready_activation(
            pending,
            &plan_input,
            &handles,
            &candidate_plan,
            None,
        ) {
            Ok(ready) => ready,
            Err(_) => {
                return denied_reload(
                    evidence,
                    self,
                    WorthUiValidationReloadStage::ReadyActivation,
                )
            }
        };
        let evidence = evidence.record_candidate_plan(candidate_plan_digest);

        WorthUiValidationPreparedReload {
            runtime_instance_id: self.instance_id(),
            evidence: finish_evidence(
                evidence,
                self,
                WorthUiValidationReloadStatus::ReadyForFrameBoundary,
            ),
            ready: Some(ready),
            candidate_plan: Some(candidate_plan),
            candidate_authoring_snapshot,
        }
    }
}

impl WorthUiValidationPreparedReload {
    pub fn evidence(&self) -> &WorthUiValidationReloadEvidence {
        &self.evidence
    }

    pub fn is_ready(&self) -> bool {
        self.ready.is_some() && self.candidate_plan.is_some()
    }

    pub fn activate(
        self,
        runtime: &mut WorthUiRuntimeHost,
    ) -> Result<WorthUiValidationReloadEvidence, WorthUiValidationReloadStage> {
        if self.runtime_instance_id != runtime.instance_id() {
            return Err(WorthUiValidationReloadStage::RuntimeInstanceMismatch);
        }
        let ready = self
            .ready
            .ok_or(WorthUiValidationReloadStage::MissingReadyActivation)?;
        let candidate_plan = self
            .candidate_plan
            .ok_or(WorthUiValidationReloadStage::MissingReadyActivation)?;
        let boundary = runtime.safe_frame_boundary();
        runtime
            .swap_ready_activation_at_frame_boundary(ready, candidate_plan, boundary)
            .map_err(|_| WorthUiValidationReloadStage::PlanSwap)?;
        runtime
            .active_state_for_swap_mut()
            .replace_authoring_snapshot(self.candidate_authoring_snapshot);
        let after = runtime.inspect_active();
        Ok(self
            .evidence
            .mark_activated(after.artifact_digest(), after.active_plan_digest()))
    }
}

fn provider_from_request(request: WorthUiValidationReloadRequest) -> WorthUiSourceProvider {
    request.modules().iter().fold(
        WorthUiSourceProvider::in_memory("validation-app-reload"),
        |provider, module| provider.with_file(module.relative_path(), module.source_text()),
    )
}

fn platform_inventory(
    runtime: &WorthUiRuntimeHost,
) -> crate::runtime::WorthUiDurableStateInventoryBuilder {
    runtime
        .durable_state_inventory()
        .register_platform_family(WorthUiDurableStateFamily::focus_chain())
        .register_platform_family(WorthUiDurableStateFamily::scroll_anchor())
        .register_platform_family(WorthUiDurableStateFamily::selection_range())
        .register_platform_family(WorthUiDurableStateFamily::text_edit_buffer())
        .register_platform_family(WorthUiDurableStateFamily::splitter_position())
        .register_platform_family(WorthUiDurableStateFamily::tab_state())
        .register_platform_family(WorthUiDurableStateFamily::panel_visibility())
}

fn denied_reload(
    evidence: WorthUiValidationReloadEvidenceBuilder,
    runtime: &WorthUiRuntimeHost,
    stage: WorthUiValidationReloadStage,
) -> WorthUiValidationPreparedReload {
    WorthUiValidationPreparedReload {
        runtime_instance_id: runtime.instance_id(),
        evidence: finish_evidence(
            evidence,
            runtime,
            WorthUiValidationReloadStatus::Denied(stage),
        ),
        ready: None,
        candidate_plan: None,
        candidate_authoring_snapshot: None,
    }
}

fn denied_reload_with_detail(
    evidence: WorthUiValidationReloadEvidenceBuilder,
    runtime: &WorthUiRuntimeHost,
    stage: WorthUiValidationReloadStage,
    detail: impl Into<String>,
) -> WorthUiValidationPreparedReload {
    let after = runtime.inspect_active();
    WorthUiValidationPreparedReload {
        runtime_instance_id: runtime.instance_id(),
        evidence: evidence.finish_denied(
            stage,
            detail,
            after.artifact_digest(),
            after.active_plan_digest(),
        ),
        ready: None,
        candidate_plan: None,
        candidate_authoring_snapshot: None,
    }
}

fn finish_evidence(
    evidence: WorthUiValidationReloadEvidenceBuilder,
    runtime: &WorthUiRuntimeHost,
    status: WorthUiValidationReloadStatus,
) -> WorthUiValidationReloadEvidence {
    let after = runtime.inspect_active();
    evidence.finish(status, after.artifact_digest(), after.active_plan_digest())
}
