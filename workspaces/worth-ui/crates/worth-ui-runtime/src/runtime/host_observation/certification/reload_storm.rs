use crate::capability::CapabilitySnapshot;
use crate::runtime::WorthUiRuntime;
use crate::runtime::{
    WorthUiCandidateAdmission, WorthUiCounterCaptureRichness,
    WorthUiFileRustReplacementParityCounters, WorthUiFoundationalCounterBridge,
    WorthUiFoundationalCounterEvidence, WorthUiFrameCostCounter, WorthUiMeasurementBoundary,
    WorthUiMeasurementCertificationDenial, WorthUiReloadCertificationBundle, WorthUiReloadDenial,
    WorthUiReloadFailure, WorthUiReloadFailureStage, WorthUiReloadLatencyCounters,
    WorthUiReloadStormCandidateDenialReason, WorthUiReloadStormCandidateStep,
    WorthUiReloadStormCandidateStepKind, WorthUiReloadStormCertification,
    WorthUiReloadStormCertificationDenial, WorthUiReloadStormCertificationDenialReason,
    WorthUiReloadStormDeniedIteration, WorthUiReloadStormIterationOutcome,
    WorthUiReloadStormNoOpIteration, WorthUiReloadStormOrderedTruth,
    WorthUiReloadStormReceiptBinding, WorthUiReloadStormScenario,
    WorthUiReloadStormSuccessfulIteration, WorthUiReplacementCandidate,
    WorthUiRuntimeArtifactComparisonOutcome, WorthUiRuntimeCounterFamily,
};

struct WorthUiReloadStormCandidateLoweringFailure {
    candidate_denial_reason: WorthUiReloadStormCandidateDenialReason,
    failure: WorthUiReloadFailure,
    counters: WorthUiReloadLatencyCounters,
}

impl WorthUiRuntime {
    pub fn certify_reload_storm_against_snapshot(
        &mut self,
        scenario: WorthUiReloadStormScenario,
        snapshot: &CapabilitySnapshot,
    ) -> Result<WorthUiReloadStormCertification, WorthUiReloadStormCertificationDenial> {
        let mut counters = WorthUiReloadLatencyCounters::default();
        validate_scenario(&scenario, counters)?;

        let initial_active = self.inspect_active();
        let scenario_digest = scenario.scenario_digest();
        let scenario_name = scenario.name().to_owned();
        let mut outcomes = Vec::new();
        let mut previous_binding: Option<WorthUiReloadStormReceiptBinding> = None;

        for step in scenario.steps() {
            counters.record_iteration();
            if !step.expected_provider_kind_matches() {
                return Err(denial(
                    WorthUiReloadStormCertificationDenialReason::ProviderKindDoesNotMatchStepKind {
                        label: step.label().to_owned(),
                    },
                    counters,
                ));
            }

            let candidate = match self.lower_step_candidate(step, snapshot, counters) {
                Ok(candidate) => candidate,
                Err(lowering_failure) => {
                    counters = lowering_failure.counters;
                    outcomes.push(WorthUiReloadStormIterationOutcome::DeniedPreserved(
                        WorthUiReloadStormDeniedIteration::new(
                            step.label(),
                            lowering_failure.candidate_denial_reason,
                            lowering_failure.failure,
                            self.inspect_active().active_plan_digest(),
                            self.last_valid().active_plan_digest(),
                        ),
                    ));
                    continue;
                }
            };

            let lane = candidate.authoring_lane();
            counters.record_candidate_lane(lane);
            if !lane_matches_step_kind(lane, step.kind()) {
                return Err(denial(
                    WorthUiReloadStormCertificationDenialReason::ProviderKindDoesNotMatchStepKind {
                        label: step.label().to_owned(),
                    },
                    counters,
                ));
            }

            if step.reuse_previous_receipt_probe() {
                let Some(previous) = previous_binding else {
                    return Err(denial(
                        WorthUiReloadStormCertificationDenialReason::ForgedReceiptReuseAcrossCandidates,
                        counters,
                    ));
                };
                if !previous.reusable_for_candidate(
                    lane,
                    candidate.basis(),
                    step.provider().final_package_digest(),
                ) {
                    counters.record_forged_receipt_reuse_denial();
                    return Err(denial(
                        WorthUiReloadStormCertificationDenialReason::ForgedReceiptReuseAcrossCandidates,
                        counters,
                    ));
                }
            }

            let outcome = self.classify_or_activate_step(
                step,
                candidate,
                &mut counters,
                &mut previous_binding,
            )?;
            outcomes.push(outcome);
        }

        let final_active = self.inspect_active();
        let final_last_valid = self.last_valid();
        let ordered_truth = WorthUiReloadStormOrderedTruth::from_outcomes(
            initial_active,
            final_active,
            final_last_valid,
            &outcomes,
        );
        let foundational_evidence =
            lower_storm_counters_to_foundational(counters, final_active.active_plan_digest())
                .map_err(|reason| denial(reason, counters))?;
        counters.record_foundational_receipts(foundational_evidence.len());
        let bundle =
            WorthUiReloadCertificationBundle::new(outcomes, foundational_evidence, counters);

        Ok(WorthUiReloadStormCertification::new(
            scenario_name,
            scenario_digest,
            ordered_truth,
            bundle,
        ))
    }

    fn classify_or_activate_step(
        &mut self,
        step: &WorthUiReloadStormCandidateStep,
        candidate: WorthUiReplacementCandidate,
        counters: &mut WorthUiReloadLatencyCounters,
        previous_binding: &mut Option<WorthUiReloadStormReceiptBinding>,
    ) -> Result<WorthUiReloadStormIterationOutcome, WorthUiReloadStormCertificationDenial> {
        let lane = candidate.authoring_lane();
        let candidate_basis = candidate.basis();
        let admitted =
            WorthUiCandidateAdmission::for_active_basis(self.replacement_admission_basis())
                .admit(candidate)
                .map_err(|_| {
                    denial(
                        WorthUiReloadStormCertificationDenialReason::CandidateAdmissionDenied,
                        *counters,
                    )
                })?;
        let comparison = self.compare_admitted_replacement(&admitted).map_err(|_| {
            denial(
                WorthUiReloadStormCertificationDenialReason::ArtifactComparisonDenied,
                *counters,
            )
        })?;

        if comparison.outcome() == WorthUiRuntimeArtifactComparisonOutcome::EquivalentNoOp {
            counters.record_candidate_screening();
            counters.record_no_op();
            let binding = WorthUiReloadStormReceiptBinding::from_no_op(
                lane,
                candidate_basis,
                &comparison,
                step.provider().final_package_digest(),
            );
            *previous_binding = Some(binding);
            return Ok(WorthUiReloadStormIterationOutcome::EquivalentNoOp(
                WorthUiReloadStormNoOpIteration::new(
                    step.label(),
                    binding,
                    self.inspect_active().active_plan_digest(),
                ),
            ));
        }

        let parity_counters = replacement_parity_counters_after_screening(lane);
        let report = self
            .activate_admitted_replacement_for_file_rust_parity_report(
                admitted,
                comparison,
                parity_counters,
            )
            .map_err(|activation_denial| {
                denial(
                    WorthUiReloadStormCertificationDenialReason::ActivationDenied(
                        activation_denial,
                    ),
                    *counters,
                )
            })?;
        counters.record_activated_pipeline(report.counters());
        let binding = WorthUiReloadStormReceiptBinding::from_activation(
            &report,
            step.provider().final_package_digest(),
        );
        *previous_binding = Some(binding);
        Ok(WorthUiReloadStormIterationOutcome::Activated(Box::new(
            WorthUiReloadStormSuccessfulIteration::new(
                step.label(),
                binding,
                report,
                self.inspect_active().active_plan_digest(),
            ),
        )))
    }

    fn lower_step_candidate(
        &self,
        step: &WorthUiReloadStormCandidateStep,
        snapshot: &CapabilitySnapshot,
        mut counters: WorthUiReloadLatencyCounters,
    ) -> Result<WorthUiReplacementCandidate, Box<WorthUiReloadStormCandidateLoweringFailure>> {
        let provider = step.provider().clone();
        let mut session = self.source_ingress(provider).start();
        let batch = match session.ingest(step.events()) {
            Ok(batch) => batch,
            Err(ingress_denial) => {
                counters.record_denied_preservation();
                let candidate_denial_reason =
                    WorthUiReloadStormCandidateDenialReason::SourceIngressDenied(ingress_denial);
                let failure = self.preserve_failed_reload(WorthUiReloadDenial::ordinary(
                    WorthUiReloadFailureStage::InvalidCandidate,
                    Some(step.provider().final_package_digest()),
                ));
                return Err(Box::new(WorthUiReloadStormCandidateLoweringFailure {
                    candidate_denial_reason,
                    failure,
                    counters,
                }));
            }
        };
        match batch.lower_to_candidate_submission(snapshot) {
            Ok(submission) => Ok(submission.into_candidate()),
            Err(submission_denial) => {
                counters.record_denied_preservation();
                let candidate_denial_reason =
                    WorthUiReloadStormCandidateDenialReason::CandidateSubmissionDenied(
                        submission_denial,
                    );
                let failure = self.preserve_failed_reload(WorthUiReloadDenial::ordinary(
                    WorthUiReloadFailureStage::InvalidCandidate,
                    Some(step.provider().final_package_digest()),
                ));
                Err(Box::new(WorthUiReloadStormCandidateLoweringFailure {
                    candidate_denial_reason,
                    failure,
                    counters,
                }))
            }
        }
    }
}

fn replacement_parity_counters_after_screening(
    lane: crate::runtime::WorthUiCandidateAuthoringLane,
) -> WorthUiFileRustReplacementParityCounters {
    let mut counters = WorthUiFileRustReplacementParityCounters::default();
    counters.record_candidate(lane);
    counters.record_candidate_admission();
    counters.record_artifact_comparison();
    counters
}

fn validate_scenario(
    scenario: &WorthUiReloadStormScenario,
    counters: WorthUiReloadLatencyCounters,
) -> Result<(), WorthUiReloadStormCertificationDenial> {
    if scenario.steps().is_empty() {
        return Err(denial(
            WorthUiReloadStormCertificationDenialReason::EmptyStorm,
            counters,
        ));
    }
    if !scenario.consumes_file_and_rust_candidates() {
        return Err(denial(
            WorthUiReloadStormCertificationDenialReason::MissingFileOrRustAuthoredCoverage,
            counters,
        ));
    }
    Ok(())
}

fn lane_matches_step_kind(
    lane: crate::runtime::WorthUiCandidateAuthoringLane,
    kind: WorthUiReloadStormCandidateStepKind,
) -> bool {
    matches!(
        (lane, kind),
        (
            crate::runtime::WorthUiCandidateAuthoringLane::FileAuthored,
            WorthUiReloadStormCandidateStepKind::FileAuthored
        ) | (
            crate::runtime::WorthUiCandidateAuthoringLane::RustAuthored,
            WorthUiReloadStormCandidateStepKind::RustAuthored
        )
    )
}

fn lower_storm_counters_to_foundational(
    counters: WorthUiReloadLatencyCounters,
    active_plan_digest: u64,
) -> Result<Vec<WorthUiFoundationalCounterEvidence>, WorthUiReloadStormCertificationDenialReason> {
    let family = WorthUiRuntimeCounterFamily::ReloadCandidateAdmission;
    let boundary = WorthUiMeasurementBoundary::ReloadCandidateAdmission;
    let packet = family
        .at_boundary(boundary)
        .with_capture_richness(WorthUiCounterCaptureRichness::Full)
        .with_active_plan_digest(active_plan_digest)
        .record(WorthUiFrameCostCounter::count(
            "reload.candidate_admission.candidate_proof_checks",
            counters.iteration_count() as u64,
        ))
        .record(WorthUiFrameCostCounter::count(
            "reload.candidate_admission.snapshot_compatibility_checks",
            counters.valid_candidate_count() as u64,
        ))
        .record(WorthUiFrameCostCounter::count(
            "reload.candidate_admission.runtime_posture_checks",
            counters.activated_candidate_count() as u64,
        ))
        .record(WorthUiFrameCostCounter::count(
            "reload.candidate_admission.query_support_checks",
            counters.no_op_candidate_count() as u64 + counters.activated_candidate_count() as u64,
        ))
        .seal()
        .map_err(WorthUiReloadStormCertificationDenialReason::FoundationalMeasurementDenied)?;
    let contract = crate::runtime::WorthUiComplexityContract::hot_path(boundary.token())
        .requires_boundary(boundary)
        .requires_counter_family(family)
        .foundational_boundary(boundary.foundational_boundary());
    let certified = packet
        .certify_against(contract)
        .map_err(WorthUiReloadStormCertificationDenialReason::FoundationalMeasurementDenied)?;
    let evidence = WorthUiFoundationalCounterBridge::lower_certified_packet(&certified)
        .map_err(foundational_denial)?;
    Ok(vec![evidence])
}

fn foundational_denial(
    denial: WorthUiMeasurementCertificationDenial,
) -> WorthUiReloadStormCertificationDenialReason {
    WorthUiReloadStormCertificationDenialReason::FoundationalLoweringDenied(denial)
}

fn denial(
    reason: WorthUiReloadStormCertificationDenialReason,
    counters: WorthUiReloadLatencyCounters,
) -> WorthUiReloadStormCertificationDenial {
    WorthUiReloadStormCertificationDenial::new(reason, counters)
}
