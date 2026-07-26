use std::sync::Arc;

use worth_query_installation::facade::WorthQueryConvergenceIncumbentPosture;
use worth_query_installation::facade::WorthQueryConvergenceOscillationPosture;

use crate::domain_computation::{
    WorthQueryBoundGraphExecutionReceipt, WorthQueryConvergenceAssessment,
    WorthQueryConvergenceDisposition, WorthQueryConvergenceDomainAssessmentOutcome,
    WorthQueryConvergenceDomainDecision, WorthQueryConvergenceDomainFailure,
    WorthQueryConvergenceDomainProvider, WorthQueryConvergenceFeasibility,
    WorthQueryConvergenceIncumbentUpdate, WorthQueryConvergenceProgress,
    WorthQueryConvergenceRepeatedState, WorthQueryDomainEvidenceExecutionBinding,
};
use crate::execution_digest::hash_parts;

use super::core::WorthQueryConvergenceEpochCore;
use super::{
    WorthQueryBoundConvergenceReport, WorthQueryConvergenceEpochDenial,
    WorthQueryConvergenceEpochDenialKind, WorthQueryRetainedConvergenceCandidateEvidence,
};

pub(super) enum WorthQueryConvergenceReportAdmissionFailure {
    Epoch(WorthQueryConvergenceEpochDenial),
}

pub(super) fn assess_domain_report(
    core: &mut WorthQueryConvergenceEpochCore,
    provider: &dyn WorthQueryConvergenceDomainProvider,
    receipt: &WorthQueryBoundGraphExecutionReceipt,
) -> Result<WorthQueryConvergenceDomainAssessmentOutcome, WorthQueryConvergenceDomainFailure> {
    let iteration_ordinal = core.counters().iteration_count();
    core.counters_mut()
        .recorded_provider_work(receipt.work_report().completed_work_units());
    let outcome = provider.assess(WorthQueryConvergenceAssessment::new(
        core.contract(),
        receipt,
        iteration_ordinal,
        core.incumbents(),
    ));
    match outcome {
        Ok(outcome) => {
            core.counters_mut().recorded_domain_work(outcome.work());
            Ok(outcome)
        }
        Err(failure) => {
            core.counters_mut().recorded_domain_work(failure.work());
            Err(failure)
        }
    }
}

pub(super) fn admit_assessed_domain_report(
    core: &mut WorthQueryConvergenceEpochCore,
    receipt: &WorthQueryBoundGraphExecutionReceipt,
    outcome: WorthQueryConvergenceDomainAssessmentOutcome,
    domain_evidence: WorthQueryDomainEvidenceExecutionBinding,
) -> Result<WorthQueryConvergenceDisposition, WorthQueryConvergenceReportAdmissionFailure> {
    let (decision, domain_work) = outcome.into_parts();
    validate_domain_evidence(core, &decision, &domain_evidence)?;
    validate_domain_decision(core, &decision)?;
    let iteration_ordinal = core.counters().iteration_count();
    let report_identity = hash_parts(&[
        "worth_query_bound_convergence_report_v1".into(),
        format!("epoch:{}", core.identity()),
        format!("iteration:{iteration_ordinal}"),
        format!("graph-evidence:{}", receipt.evidence_identity()),
        format!("provider-receipt:{}", receipt.provider_receipt()),
        format!("candidate:{}", decision.candidate_occurrence_identity()),
        format!("state:{}", decision.state_identity()),
        format!("disposition:{}", decision.disposition().canonical_name()),
        format!("feasibility:{}", decision.feasibility().canonical_name()),
        format!("progress:{}", decision.progress().canonical_name()),
        format!(
            "repeated-state:{}",
            decision.repeated_state().canonical_name()
        ),
        format!(
            "incumbent:{}",
            decision.incumbent_update().canonical_identity()
        ),
        format!(
            "domain-work:{}:{}:{}",
            domain_work.comparator_call_count(),
            domain_work.progress_check_count(),
            domain_work.repeated_state_probe_count()
        ),
    ]);
    let disposition = decision.disposition();
    let provider_work = receipt.work_report();
    let candidate = WorthQueryRetainedConvergenceCandidateEvidence::new(
        decision.candidate_occurrence_identity(),
        decision.state_identity(),
        report_identity.as_str(),
        domain_evidence,
    );
    apply_incumbent_update(core, &decision, candidate)?;
    core.replace_latest_report(WorthQueryBoundConvergenceReport::new(
        report_identity,
        receipt.provider_receipt(),
        receipt.evidence_identity(),
        iteration_ordinal,
        decision,
        domain_work,
        provider_work,
    ));
    Ok(disposition)
}

fn validate_domain_evidence(
    core: &WorthQueryConvergenceEpochCore,
    decision: &WorthQueryConvergenceDomainDecision,
    evidence: &WorthQueryDomainEvidenceExecutionBinding,
) -> Result<(), WorthQueryConvergenceReportAdmissionFailure> {
    let exact_contract = evidence.contract().is_some_and(|contract| {
        contract.admission_identity() == core.contract().artifact_admission_identity()
            && contract.contract().identity().as_str()
                == core.contract().artifact_contract_identity()
    });
    if !exact_contract
        || evidence.output_occurrence_identity() != decision.candidate_occurrence_identity()
    {
        return Err(domain_report_failure(
            core,
            "candidate evidence does not bind the exact installed artifact contract and occurrence",
        ));
    }
    Ok(())
}

fn validate_domain_decision(
    core: &WorthQueryConvergenceEpochCore,
    decision: &WorthQueryConvergenceDomainDecision,
) -> Result<(), WorthQueryConvergenceReportAdmissionFailure> {
    if !oscillation_policy_is_coherent(core, decision) {
        return Err(domain_report_failure(
            core,
            "domain oscillation evidence contradicts the installed oscillation posture",
        ));
    }
    let coherent = match decision.disposition() {
        WorthQueryConvergenceDisposition::StableWithoutProof => {
            decision.progress() == WorthQueryConvergenceProgress::Stable
        }
        WorthQueryConvergenceDisposition::FeasibleIncumbent => {
            decision.feasibility() == WorthQueryConvergenceFeasibility::Feasible
                && update_leaves_incumbent(core, decision.incumbent_update())
        }
        WorthQueryConvergenceDisposition::Oscillating => {
            decision.repeated_state() == WorthQueryConvergenceRepeatedState::Repeated
                || core.contract().oscillation_posture()
                    == WorthQueryConvergenceOscillationPosture::DomainClassified
        }
        _ => true,
    };
    if !coherent {
        return Err(domain_report_failure(
            core,
            "domain terminal disposition contradicts its accompanying semantic evidence",
        ));
    }
    Ok(())
}

fn oscillation_policy_is_coherent(
    core: &WorthQueryConvergenceEpochCore,
    decision: &WorthQueryConvergenceDomainDecision,
) -> bool {
    let repeated = decision.repeated_state() == WorthQueryConvergenceRepeatedState::Repeated;
    let oscillating = decision.disposition() == WorthQueryConvergenceDisposition::Oscillating;
    match core.contract().oscillation_posture() {
        WorthQueryConvergenceOscillationPosture::Impossible => !repeated && !oscillating,
        WorthQueryConvergenceOscillationPosture::DetectAndDeny => {
            repeated == oscillating
                && (!oscillating || update_does_not_select_candidate(decision.incumbent_update()))
        }
        WorthQueryConvergenceOscillationPosture::DetectAndSelectIncumbent => {
            repeated == oscillating
                && (!oscillating || update_leaves_incumbent(core, decision.incumbent_update()))
        }
        WorthQueryConvergenceOscillationPosture::DomainClassified => true,
    }
}

fn update_does_not_select_candidate(update: &WorthQueryConvergenceIncumbentUpdate) -> bool {
    matches!(
        update,
        WorthQueryConvergenceIncumbentUpdate::Retain | WorthQueryConvergenceIncumbentUpdate::Clear
    )
}

fn update_leaves_incumbent(
    core: &WorthQueryConvergenceEpochCore,
    update: &WorthQueryConvergenceIncumbentUpdate,
) -> bool {
    match update {
        WorthQueryConvergenceIncumbentUpdate::Retain => !core.incumbents().is_empty(),
        WorthQueryConvergenceIncumbentUpdate::ReplaceWithCandidate
        | WorthQueryConvergenceIncumbentUpdate::AddCandidate
        | WorthQueryConvergenceIncumbentUpdate::RemoveCandidatesAndAdd { .. } => true,
        WorthQueryConvergenceIncumbentUpdate::Clear => false,
    }
}

fn apply_incumbent_update(
    core: &mut WorthQueryConvergenceEpochCore,
    decision: &crate::domain_computation::WorthQueryConvergenceDomainDecision,
    candidate: WorthQueryRetainedConvergenceCandidateEvidence,
) -> Result<(), WorthQueryConvergenceReportAdmissionFailure> {
    validate_incumbent_update(core, decision)?;
    commit_incumbent_update(core, decision.incumbent_update(), candidate);
    Ok(())
}

fn validate_incumbent_update(
    core: &WorthQueryConvergenceEpochCore,
    decision: &WorthQueryConvergenceDomainDecision,
) -> Result<(), WorthQueryConvergenceReportAdmissionFailure> {
    let posture = core.contract().incumbent_posture();
    let update = decision.incumbent_update();
    let valid = match (posture, update) {
        (
            WorthQueryConvergenceIncumbentPosture::NoIncumbent,
            WorthQueryConvergenceIncumbentUpdate::Retain,
        )
        | (
            WorthQueryConvergenceIncumbentPosture::NoIncumbent,
            WorthQueryConvergenceIncumbentUpdate::Clear,
        ) => true,
        (
            WorthQueryConvergenceIncumbentPosture::FirstFeasible,
            WorthQueryConvergenceIncumbentUpdate::Retain,
        ) => true,
        (
            WorthQueryConvergenceIncumbentPosture::FirstFeasible,
            WorthQueryConvergenceIncumbentUpdate::ReplaceWithCandidate,
        ) => {
            core.incumbents().is_empty()
                && decision.feasibility() == WorthQueryConvergenceFeasibility::Feasible
        }
        (
            WorthQueryConvergenceIncumbentPosture::BestObserved,
            WorthQueryConvergenceIncumbentUpdate::Retain
            | WorthQueryConvergenceIncumbentUpdate::ReplaceWithCandidate
            | WorthQueryConvergenceIncumbentUpdate::Clear,
        ) => true,
        (
            WorthQueryConvergenceIncumbentPosture::ParetoFrontier,
            WorthQueryConvergenceIncumbentUpdate::Retain
            | WorthQueryConvergenceIncumbentUpdate::Clear,
        ) => true,
        (
            WorthQueryConvergenceIncumbentPosture::ParetoFrontier,
            WorthQueryConvergenceIncumbentUpdate::AddCandidate,
        ) => !contains_incumbent(core, decision.candidate_occurrence_identity()),
        (
            WorthQueryConvergenceIncumbentPosture::ParetoFrontier,
            WorthQueryConvergenceIncumbentUpdate::RemoveCandidatesAndAdd {
                removed_occurrence_identities,
            },
        ) => {
            let every_removal_exists = removed_occurrence_identities
                .iter()
                .all(|removed| contains_incumbent(core, removed));
            let candidate_collision = core.incumbents().iter().any(|incumbent| {
                incumbent.occurrence_identity() == decision.candidate_occurrence_identity()
                    && !removed_occurrence_identities
                        .iter()
                        .any(|removed| removed.as_ref() == decision.candidate_occurrence_identity())
            });
            every_removal_exists && !candidate_collision
        }
        _ => false,
    };
    if !valid {
        return Err(epoch_failure(
            core,
            "domain incumbent transition contradicts the installed incumbent posture",
        ));
    }
    Ok(())
}

fn commit_incumbent_update(
    core: &mut WorthQueryConvergenceEpochCore,
    update: &WorthQueryConvergenceIncumbentUpdate,
    candidate: WorthQueryRetainedConvergenceCandidateEvidence,
) {
    match update {
        WorthQueryConvergenceIncumbentUpdate::Retain => {
            core.counters_mut().retained_incumbent();
        }
        WorthQueryConvergenceIncumbentUpdate::ReplaceWithCandidate => {
            core.incumbents_mut().clear();
            core.incumbents_mut().push(candidate);
            core.counters_mut().replaced_incumbent();
        }
        WorthQueryConvergenceIncumbentUpdate::AddCandidate => {
            core.incumbents_mut().push(candidate);
            core.counters_mut().replaced_incumbent();
        }
        WorthQueryConvergenceIncumbentUpdate::RemoveCandidatesAndAdd {
            removed_occurrence_identities,
        } => {
            core.incumbents_mut().retain(|incumbent| {
                !removed_occurrence_identities
                    .iter()
                    .any(|removed| removed.as_ref() == incumbent.occurrence_identity())
            });
            core.incumbents_mut().push(candidate);
            core.counters_mut().replaced_incumbent();
        }
        WorthQueryConvergenceIncumbentUpdate::Clear => {
            core.incumbents_mut().clear();
            core.counters_mut().replaced_incumbent();
        }
    }
}

fn contains_incumbent(core: &WorthQueryConvergenceEpochCore, occurrence_identity: &str) -> bool {
    core.incumbents()
        .iter()
        .any(|incumbent| incumbent.occurrence_identity() == occurrence_identity)
}

fn epoch_failure(
    core: &WorthQueryConvergenceEpochCore,
    detail: &'static str,
) -> WorthQueryConvergenceReportAdmissionFailure {
    WorthQueryConvergenceReportAdmissionFailure::Epoch(WorthQueryConvergenceEpochDenial::new(
        WorthQueryConvergenceEpochDenialKind::InvalidIncumbentTransition,
        Arc::<str>::from(detail),
        core.counters().clone(),
    ))
}

fn domain_report_failure(
    core: &WorthQueryConvergenceEpochCore,
    detail: &'static str,
) -> WorthQueryConvergenceReportAdmissionFailure {
    WorthQueryConvergenceReportAdmissionFailure::Epoch(WorthQueryConvergenceEpochDenial::new(
        WorthQueryConvergenceEpochDenialKind::InvalidDomainReport,
        Arc::<str>::from(detail),
        core.counters().clone(),
    ))
}
