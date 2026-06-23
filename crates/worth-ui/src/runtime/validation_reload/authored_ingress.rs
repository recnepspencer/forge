use crate::capability::CapabilitySnapshot;
use crate::runtime::validation_reload::activation_guard::reject_stale_prepared_reload_activation;
use crate::runtime::validation_reload::changed_facts::derive_validation_changed_fact_mapping_receipt;
use crate::runtime::validation_reload::driver_support::{
    active_authoring_snapshot_digest, denied_reload, denied_reload_with_detail, finish_evidence,
    platform_inventory,
};
use crate::runtime::validation_reload::evidence::WorthUiValidationReloadEvidenceBuilder;
use crate::runtime::{
    WorthUiCandidateAdmission, WorthUiCandidateRuntimeAuthoringSnapshot,
    WorthUiObservedAuthoredEditResultDenial, WorthUiReplacementCandidate,
    WorthUiRuntimeArtifactComparison, WorthUiRuntimeArtifactComparisonOutcome, WorthUiRuntimeHost,
    WorthUiSourceAuthoredCandidateSubmission, WorthUiValidationPreparedReload,
    WorthUiValidationReloadEvidence, WorthUiValidationReloadRequest, WorthUiValidationReloadStage,
    WorthUiValidationReloadStatus,
};

impl WorthUiRuntimeHost {
    pub fn prepare_validation_reload_from_authored_submission(
        &self,
        submission: WorthUiSourceAuthoredCandidateSubmission,
    ) -> WorthUiValidationPreparedReload {
        let before = self.inspect_active();
        let evidence = WorthUiValidationReloadEvidence::builder(
            self.instance_id().raw(),
            before.artifact_digest(),
            before.active_plan_digest(),
        )
        .record_active_authoring_snapshot_before(active_authoring_snapshot_digest(self));
        self.prepare_validation_reload_from_submission_with_evidence(submission, evidence)
    }

    pub(super) fn lower_validation_reload_request_to_submission(
        &self,
        snapshot: &CapabilitySnapshot,
        request: WorthUiValidationReloadRequest,
        evidence: WorthUiValidationReloadEvidenceBuilder,
    ) -> Result<
        (
            WorthUiSourceAuthoredCandidateSubmission,
            WorthUiValidationReloadEvidenceBuilder,
        ),
        (
            WorthUiValidationReloadStage,
            WorthUiValidationReloadEvidenceBuilder,
        ),
    > {
        let observed_edit = request.into_observed_authored_edit().map_err(|_| {
            (
                WorthUiValidationReloadStage::SourceIngress,
                evidence.clone(),
            )
        })?;
        let submission = self
            .observe_authored_edit(snapshot, observed_edit)
            .map_err(|denial| match denial {
                WorthUiObservedAuthoredEditResultDenial::SourceIngress(_)
                | WorthUiObservedAuthoredEditResultDenial::InvalidObservedEdit(_) => (
                    WorthUiValidationReloadStage::SourceIngress,
                    evidence.clone(),
                ),
                WorthUiObservedAuthoredEditResultDenial::CandidateSubmission(_) => (
                    WorthUiValidationReloadStage::CandidateSubmission,
                    evidence.clone(),
                ),
            })?;
        let source_authored_submission =
            submission.into_source_authored_submission().map_err(|_| {
                (
                    WorthUiValidationReloadStage::CandidateSubmission,
                    evidence.clone(),
                )
            })?;
        Ok((source_authored_submission, evidence))
    }

    pub(super) fn prepare_validation_reload_from_submission_with_evidence(
        &self,
        submission: WorthUiSourceAuthoredCandidateSubmission,
        evidence: WorthUiValidationReloadEvidenceBuilder,
    ) -> WorthUiValidationPreparedReload {
        let (candidate, candidate_authoring_snapshot, authored_delta_summary, evidence) =
            self.lower_validation_submission(submission, evidence);
        let candidate_authoring_snapshot_digest = candidate_authoring_snapshot
            .as_ref()
            .map(|snapshot| snapshot.digest().as_u64());
        let changed_fact_mapping_receipt =
            derive_validation_changed_fact_mapping_receipt(authored_delta_summary);
        let evidence = evidence.record_changed_facts(
            changed_fact_mapping_receipt
                .as_ref()
                .map_or_else(crate::runtime::WorthUiRuntimeFactSet::empty, |receipt| {
                    receipt.changed_facts().clone()
                }),
        );
        let admitted =
            match WorthUiCandidateAdmission::for_active_basis(self.replacement_admission_basis())
                .admit(candidate)
            {
                Ok(admitted) => admitted,
                Err(report) => {
                    return denied_reload_with_detail(
                        evidence,
                        self,
                        WorthUiValidationReloadStage::CandidateAdmission,
                        format!("{:?}", report.denial()),
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
            if candidate_authoring_snapshot_digest != active_authoring_snapshot_digest(self) {
                return WorthUiValidationPreparedReload {
                    runtime_instance_id: self.instance_id(),
                    evidence: finish_evidence(
                        evidence,
                        self,
                        WorthUiValidationReloadStatus::ReadyForFrameBoundary,
                    ),
                    changed_fact_mapping_receipt,
                    ready: None,
                    candidate_plan: None,
                    candidate_authoring_snapshot,
                };
            }
            return WorthUiValidationPreparedReload {
                runtime_instance_id: self.instance_id(),
                evidence: finish_evidence(
                    evidence,
                    self,
                    WorthUiValidationReloadStatus::EquivalentNoOp,
                ),
                changed_fact_mapping_receipt,
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
            changed_fact_mapping_receipt,
        )
    }

    fn lower_validation_submission(
        &self,
        submission: WorthUiSourceAuthoredCandidateSubmission,
        evidence: WorthUiValidationReloadEvidenceBuilder,
    ) -> (
        WorthUiReplacementCandidate,
        Option<WorthUiCandidateRuntimeAuthoringSnapshot>,
        Option<crate::runtime::WorthUiAuthoredDeltaSummary>,
        WorthUiValidationReloadEvidenceBuilder,
    ) {
        let (
            candidate,
            candidate_authoring_snapshot,
            authored_delta_summary,
            revision,
            ordering_receipt,
            counters,
        ) = submission.into_parts();
        let evidence = evidence.record_source_ingress(
            revision.final_package_digest(),
            ordering_receipt.receipt_digest(),
            counters,
        );
        let evidence = evidence.record_candidate_submission(counters);
        let evidence = evidence.record_authored_delta_summary(Some(&authored_delta_summary));
        let evidence = evidence.record_candidate_authoring_snapshot(Some(
            candidate_authoring_snapshot.digest().as_u64(),
        ));
        (
            candidate,
            Some(candidate_authoring_snapshot),
            Some(authored_delta_summary),
            evidence,
        )
    }

    pub(super) fn plan_validation_reload_replacement(
        &self,
        admitted: crate::runtime::WorthUiAdmittedReplacementCandidate,
        candidate_authoring_snapshot: Option<WorthUiCandidateRuntimeAuthoringSnapshot>,
        comparison: WorthUiRuntimeArtifactComparison,
        evidence: WorthUiValidationReloadEvidenceBuilder,
        changed_fact_mapping_receipt: Option<
            crate::runtime::WorthUiValidationChangedFactMappingReceipt,
        >,
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
            changed_fact_mapping_receipt,
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

    pub fn changed_fact_mapping_receipt(
        &self,
    ) -> Option<&crate::runtime::WorthUiValidationChangedFactMappingReceipt> {
        self.changed_fact_mapping_receipt.as_ref()
    }

    pub fn activate(
        self,
        runtime: &mut WorthUiRuntimeHost,
    ) -> Result<WorthUiValidationReloadEvidence, WorthUiValidationReloadStage> {
        if self.runtime_instance_id != runtime.instance_id() {
            return Err(WorthUiValidationReloadStage::RuntimeInstanceMismatch);
        }
        reject_stale_prepared_reload_activation(runtime, &self.evidence)?;
        if self.ready.is_none() && self.candidate_plan.is_none() {
            if self.candidate_authoring_snapshot.is_none() {
                return Err(WorthUiValidationReloadStage::MissingReadyActivation);
            }
            let _boundary = runtime.safe_frame_boundary();
            runtime.promote_authoring_snapshot_after_activation(self.candidate_authoring_snapshot);
            let after = runtime.inspect_active();
            return Ok(self
                .evidence
                .mark_activated(after.artifact_digest(), after.active_plan_digest()));
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
        runtime.promote_authoring_snapshot_after_activation(self.candidate_authoring_snapshot);
        let after = runtime.inspect_active();
        Ok(self
            .evidence
            .mark_activated(after.artifact_digest(), after.active_plan_digest()))
    }
}
