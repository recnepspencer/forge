use crate::runtime::{
    WorthUiActivationReadiness, WorthUiActivationStagingCounters, WorthUiActivationStagingDenial,
    WorthUiActivationStagingDenialReason, WorthUiActivationStagingReport,
    WorthUiActiveRuntimeObservation, WorthUiAdmittedReplacementCandidate,
    WorthUiCandidateAdmissionDenial, WorthUiDurableStateReconciliationPlan,
    WorthUiNodeReplacementPlan, WorthUiPendingActivation, WorthUiPendingExecutionPlanLoweringInput,
    WorthUiQueryLiveRebindPlan, WorthUiReplacementImpactClassification, WorthUiRuntimeFrameEpoch,
    WorthUiRuntimeImpactNarrowing, WorthUiStagedReplacement,
};

pub(crate) struct WorthUiActivationStager;

impl WorthUiActivationStager {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn stage(
        active_before: WorthUiActiveRuntimeObservation,
        active_after: WorthUiActiveRuntimeObservation,
        admitted: WorthUiAdmittedReplacementCandidate,
        impact: &WorthUiReplacementImpactClassification,
        narrowing: &WorthUiRuntimeImpactNarrowing,
        node_plan: &WorthUiNodeReplacementPlan,
        reconciliation_plan: Option<&WorthUiDurableStateReconciliationPlan>,
        query_rebind_plan: Option<&WorthUiQueryLiveRebindPlan>,
        pending_execution_plan_lowering_input: Option<&WorthUiPendingExecutionPlanLoweringInput>,
    ) -> Result<WorthUiPendingActivation, WorthUiActivationStagingDenial> {
        let active_artifact_digest = active_before.artifact_digest();
        let candidate_artifact_digest = admitted.artifact_bundle().artifact_digest().raw();
        let frame_epoch = active_before.frame_epoch();
        let mut counters = WorthUiActivationStagingCounters::default();

        reject_active_mutation(
            active_before,
            active_after,
            candidate_artifact_digest,
            &mut counters,
        )?;
        reject_changed_query_support_receipt(
            &admitted,
            active_artifact_digest,
            candidate_artifact_digest,
            frame_epoch,
            &mut counters,
        )?;
        verify_required_digest_pair(
            active_artifact_digest,
            candidate_artifact_digest,
            impact.active_artifact_digest(),
            impact.candidate_artifact_digest(),
            frame_epoch,
            &mut counters,
        )?;
        verify_required_digest_pair(
            active_artifact_digest,
            candidate_artifact_digest,
            narrowing.active_artifact_digest(),
            narrowing.candidate_artifact_digest(),
            frame_epoch,
            &mut counters,
        )?;
        verify_required_digest_pair(
            active_artifact_digest,
            candidate_artifact_digest,
            node_plan.active_artifact_digest(),
            node_plan.candidate_artifact_digest(),
            frame_epoch,
            &mut counters,
        )?;
        let reconciliation_plan = require_reconciliation_plan(
            active_artifact_digest,
            candidate_artifact_digest,
            frame_epoch,
            reconciliation_plan,
            &mut counters,
        )?;
        verify_required_digest_pair(
            active_artifact_digest,
            candidate_artifact_digest,
            reconciliation_plan.active_artifact_digest(),
            reconciliation_plan.candidate_artifact_digest(),
            frame_epoch,
            &mut counters,
        )?;
        let query_rebind_plan = require_query_rebind_plan(
            active_artifact_digest,
            candidate_artifact_digest,
            frame_epoch,
            query_rebind_plan,
            &mut counters,
        )?;
        verify_required_digest_pair(
            active_artifact_digest,
            candidate_artifact_digest,
            query_rebind_plan.active_artifact_digest(),
            query_rebind_plan.candidate_artifact_digest(),
            frame_epoch,
            &mut counters,
        )?;
        let pending_execution_plan_lowering_input = require_pending_execution_plan_lowering_input(
            active_artifact_digest,
            candidate_artifact_digest,
            frame_epoch,
            pending_execution_plan_lowering_input,
            &mut counters,
        )?;
        verify_required_digest_pair(
            active_artifact_digest,
            candidate_artifact_digest,
            pending_execution_plan_lowering_input.active_artifact_digest(),
            pending_execution_plan_lowering_input.candidate_artifact_digest(),
            frame_epoch,
            &mut counters,
        )?;
        reject_mismatched_plan_lowering_input(
            active_artifact_digest,
            candidate_artifact_digest,
            frame_epoch,
            node_plan,
            reconciliation_plan,
            query_rebind_plan,
            pending_execution_plan_lowering_input,
            &mut counters,
        )?;

        counters.record_staged_reconciliation_receipts(reconciliation_plan.receipts().len());
        counters.record_staged_query_bindings(query_rebind_plan.entries().len());
        counters.record_staged_plan_lowering_input();
        let readiness = WorthUiActivationReadiness::ready_for_execution_plan_input();
        let report = WorthUiActivationStagingReport::new(
            active_artifact_digest,
            candidate_artifact_digest,
            readiness,
            counters,
        );
        let staged_replacement = WorthUiStagedReplacement::new(
            frame_epoch,
            active_artifact_digest,
            candidate_artifact_digest,
            admitted,
            impact.clone(),
            narrowing.clone(),
            node_plan.clone(),
            reconciliation_plan.clone(),
            query_rebind_plan.clone(),
            pending_execution_plan_lowering_input.clone(),
        );
        Ok(WorthUiPendingActivation::new(
            frame_epoch,
            staged_replacement,
            readiness,
            report,
        ))
    }
}

fn reject_active_mutation(
    active_before: WorthUiActiveRuntimeObservation,
    active_after: WorthUiActiveRuntimeObservation,
    candidate_artifact_digest: u64,
    counters: &mut WorthUiActivationStagingCounters,
) -> Result<(), WorthUiActivationStagingDenial> {
    if active_before == active_after {
        Ok(())
    } else {
        counters.record_active_mutation_observed();
        Err(WorthUiActivationStagingDenial::new(
            active_before.artifact_digest(),
            candidate_artifact_digest,
            active_before.frame_epoch(),
            WorthUiActivationStagingDenialReason::ActiveRuntimeMutatedDuringStaging,
            *counters,
        ))
    }
}

fn reject_changed_query_support_receipt(
    admitted: &WorthUiAdmittedReplacementCandidate,
    active_artifact_digest: u64,
    candidate_artifact_digest: u64,
    frame_epoch: WorthUiRuntimeFrameEpoch,
    counters: &mut WorthUiActivationStagingCounters,
) -> Result<(), WorthUiActivationStagingDenial> {
    counters.record_receipt_verification();
    match admitted.verify_receipts_unchanged() {
        Ok(()) => Ok(()),
        Err(WorthUiCandidateAdmissionDenial::QuerySupportReceiptChanged { .. }) => {
            Err(WorthUiActivationStagingDenial::new(
                active_artifact_digest,
                candidate_artifact_digest,
                frame_epoch,
                WorthUiActivationStagingDenialReason::AdmittedQuerySupportReceiptChanged,
                *counters,
            ))
        }
        Err(_) => Err(WorthUiActivationStagingDenial::new(
            active_artifact_digest,
            candidate_artifact_digest,
            frame_epoch,
            WorthUiActivationStagingDenialReason::AdmittedQuerySupportReceiptChanged,
            *counters,
        )),
    }
}

fn require_reconciliation_plan<'a>(
    active_artifact_digest: u64,
    candidate_artifact_digest: u64,
    frame_epoch: WorthUiRuntimeFrameEpoch,
    reconciliation_plan: Option<&'a WorthUiDurableStateReconciliationPlan>,
    counters: &mut WorthUiActivationStagingCounters,
) -> Result<&'a WorthUiDurableStateReconciliationPlan, WorthUiActivationStagingDenial> {
    match reconciliation_plan {
        Some(plan) => Ok(plan),
        None => {
            counters.record_rejected_missing_input();
            Err(WorthUiActivationStagingDenial::new(
                active_artifact_digest,
                candidate_artifact_digest,
                frame_epoch,
                WorthUiActivationStagingDenialReason::MissingDurableStateReconciliation,
                *counters,
            ))
        }
    }
}

fn require_query_rebind_plan<'a>(
    active_artifact_digest: u64,
    candidate_artifact_digest: u64,
    frame_epoch: WorthUiRuntimeFrameEpoch,
    query_rebind_plan: Option<&'a WorthUiQueryLiveRebindPlan>,
    counters: &mut WorthUiActivationStagingCounters,
) -> Result<&'a WorthUiQueryLiveRebindPlan, WorthUiActivationStagingDenial> {
    match query_rebind_plan {
        Some(plan) => Ok(plan),
        None => {
            counters.record_rejected_missing_input();
            Err(WorthUiActivationStagingDenial::new(
                active_artifact_digest,
                candidate_artifact_digest,
                frame_epoch,
                WorthUiActivationStagingDenialReason::MissingQueryLiveRebindPlan,
                *counters,
            ))
        }
    }
}

fn require_pending_execution_plan_lowering_input<'a>(
    active_artifact_digest: u64,
    candidate_artifact_digest: u64,
    frame_epoch: WorthUiRuntimeFrameEpoch,
    pending_input: Option<&'a WorthUiPendingExecutionPlanLoweringInput>,
    counters: &mut WorthUiActivationStagingCounters,
) -> Result<&'a WorthUiPendingExecutionPlanLoweringInput, WorthUiActivationStagingDenial> {
    match pending_input {
        Some(input) => Ok(input),
        None => {
            counters.record_rejected_missing_input();
            Err(WorthUiActivationStagingDenial::new(
                active_artifact_digest,
                candidate_artifact_digest,
                frame_epoch,
                WorthUiActivationStagingDenialReason::MissingExecutionPlanLoweringInput,
                *counters,
            ))
        }
    }
}

fn verify_required_digest_pair(
    expected_active_digest: u64,
    expected_candidate_digest: u64,
    actual_active_digest: u64,
    actual_candidate_digest: u64,
    frame_epoch: WorthUiRuntimeFrameEpoch,
    counters: &mut WorthUiActivationStagingCounters,
) -> Result<(), WorthUiActivationStagingDenial> {
    counters.record_digest_comparison();
    if actual_active_digest != expected_active_digest {
        counters.record_rejected_mismatched_input();
        return Err(WorthUiActivationStagingDenial::new(
            expected_active_digest,
            expected_candidate_digest,
            frame_epoch,
            WorthUiActivationStagingDenialReason::ActiveArtifactDigestMismatch,
            *counters,
        ));
    }
    counters.record_digest_comparison();
    if actual_candidate_digest != expected_candidate_digest {
        counters.record_rejected_mismatched_input();
        return Err(WorthUiActivationStagingDenial::new(
            expected_active_digest,
            expected_candidate_digest,
            frame_epoch,
            WorthUiActivationStagingDenialReason::CandidateArtifactDigestMismatch,
            *counters,
        ));
    }
    counters.record_verified_input();
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn reject_mismatched_plan_lowering_input(
    active_artifact_digest: u64,
    candidate_artifact_digest: u64,
    frame_epoch: WorthUiRuntimeFrameEpoch,
    node_plan: &WorthUiNodeReplacementPlan,
    reconciliation_plan: &WorthUiDurableStateReconciliationPlan,
    query_rebind_plan: &WorthUiQueryLiveRebindPlan,
    pending_input: &WorthUiPendingExecutionPlanLoweringInput,
    counters: &mut WorthUiActivationStagingCounters,
) -> Result<(), WorthUiActivationStagingDenial> {
    let matches_staged_cardinality = pending_input.node_classification_count()
        == node_plan.classifications().len()
        && pending_input.reconciliation_receipt_count() == reconciliation_plan.receipts().len()
        && pending_input.query_rebind_entry_count() == query_rebind_plan.entries().len();
    if matches_staged_cardinality {
        Ok(())
    } else {
        counters.record_rejected_mismatched_input();
        Err(WorthUiActivationStagingDenial::new(
            active_artifact_digest,
            candidate_artifact_digest,
            frame_epoch,
            WorthUiActivationStagingDenialReason::ExecutionPlanLoweringInputMismatch,
            *counters,
        ))
    }
}
