use crate::runtime::{
    WorthUiActivationReadiness, WorthUiActivationStagingCounters, WorthUiActivationStagingDenial,
    WorthUiActivationStagingDenialReason, WorthUiActivationStagingReport,
    WorthUiActiveRuntimeObservation, WorthUiAdmittedReplacementCandidate,
    WorthUiDurableStateReconciliationPlan, WorthUiNodeReplacementPlan, WorthUiPendingActivation,
    WorthUiQueryLiveRebindPlan, WorthUiReplacementImpactClassification, WorthUiRuntimeFrameEpoch,
    WorthUiRuntimeImpactNarrowing, WorthUiStagedReplacement,
};

use super::WorthUiStagedReplacementInput;

pub(crate) struct WorthUiActivationStagingInput<'a> {
    pub candidate_application_authority:
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationLoweringAuthority,
    pub active_before: WorthUiActiveRuntimeObservation,
    pub active_after: WorthUiActiveRuntimeObservation,
    pub admitted: WorthUiAdmittedReplacementCandidate,
    pub impact: &'a WorthUiReplacementImpactClassification,
    pub narrowing: &'a WorthUiRuntimeImpactNarrowing,
    pub node_plan: &'a WorthUiNodeReplacementPlan,
    pub reconciliation_plan: Option<&'a WorthUiDurableStateReconciliationPlan>,
    pub query_rebind_plan: Option<&'a WorthUiQueryLiveRebindPlan>,
}

pub(crate) struct WorthUiActivationStager;

impl WorthUiActivationStager {
    pub(crate) fn stage(
        input: WorthUiActivationStagingInput<'_>,
    ) -> Result<WorthUiPendingActivation, WorthUiActivationStagingDenial> {
        let WorthUiActivationStagingInput {
            candidate_application_authority,
            active_before,
            active_after,
            admitted,
            impact,
            narrowing,
            node_plan,
            reconciliation_plan,
            query_rebind_plan,
        } = input;
        let active_artifact_digest = active_before.artifact_digest();
        let candidate_artifact_digest = admitted.artifact_bundle().artifact_digest().raw();
        let frame_epoch = active_before.frame_epoch();
        let mut counters = WorthUiActivationStagingCounters::default();

        reject_foreign_candidate_application_authority(
            &candidate_application_authority,
            &admitted,
            active_artifact_digest,
            candidate_artifact_digest,
            frame_epoch,
            counters,
        )?;

        reject_active_mutation(
            active_before,
            active_after,
            candidate_artifact_digest,
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
        counters.record_staged_reconciliation_receipts(reconciliation_plan.receipts().len());
        counters.record_staged_query_bindings(query_rebind_plan.entries().len());
        let readiness = WorthUiActivationReadiness::ready_for_execution_plan_input();
        let report = WorthUiActivationStagingReport::new(
            active_artifact_digest,
            candidate_artifact_digest,
            readiness,
            counters,
        );
        let staged_replacement = WorthUiStagedReplacement::new(WorthUiStagedReplacementInput {
            frame_epoch,
            active_artifact_digest,
            candidate_artifact_digest,
            admitted_candidate: admitted,
            impact: impact.clone(),
            narrowing: narrowing.clone(),
            node_plan: node_plan.clone(),
            reconciliation_plan: reconciliation_plan.clone(),
            query_rebind_plan: query_rebind_plan.clone(),
        });
        Ok(WorthUiPendingActivation::new(
            frame_epoch,
            candidate_application_authority,
            staged_replacement,
            readiness,
            report,
        ))
    }
}

fn reject_foreign_candidate_application_authority(
    authority: &crate::facade::prepared_application_authority::WorthUiPreparedApplicationLoweringAuthority,
    admitted: &WorthUiAdmittedReplacementCandidate,
    active_artifact_digest: u64,
    candidate_artifact_digest: u64,
    frame_epoch: WorthUiRuntimeFrameEpoch,
    counters: WorthUiActivationStagingCounters,
) -> Result<(), WorthUiActivationStagingDenial> {
    if authority.admits_candidate(admitted) {
        return Ok(());
    }
    Err(WorthUiActivationStagingDenial::new(
        active_artifact_digest,
        candidate_artifact_digest,
        frame_epoch,
        WorthUiActivationStagingDenialReason::CandidateApplicationAuthorityMismatch,
        counters,
    ))
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
