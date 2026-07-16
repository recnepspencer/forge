use crate::application::{WorthQueryDeclarationInput, WorthQueryDomainEntryMarker};
use crate::recovery_boundary::foundational::{
    diagnostic_context_for_stop_kind, support_context_for_basis_mismatch,
    support_context_for_stale_basis,
};
use crate::recovery_boundary::ordinary::worth_query_recovery_brief_from_ordinary_outcome;
use crate::recovery_boundary::{
    WorthQueryRecoveryAspectPosture, WorthQueryRecoveryBasisPosture, WorthQueryRecoveryBrief,
    WorthQueryRecoveryEvidenceStrength, WorthQueryRecoveryExplanation,
    WorthQueryRecoveryFoundationalSupportContext, WorthQueryRecoverySourceFamily,
    WorthQueryRecoveryStopKind,
};
use crate::signal_compatibility_orchestration::{
    ordinary_outcome_from_signal_compatibility_orchestration_checked,
    WorthQuerySignalCompatibilityOrchestrationChecked,
    WorthQuerySignalCompatibilityOrchestrationOutcome,
    WorthQuerySignalCompatibilityOrchestrationTranscript,
};

pub fn worth_query_recovery_brief_from_signal_compatibility_checked<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    checked: WorthQuerySignalCompatibilityOrchestrationChecked<D, I>,
) -> Option<WorthQueryRecoveryBrief> {
    let explanation = signal_explanation(
        checked.outcome(),
        checked.linked_artifacts().clone(),
        WorthQueryRecoveryEvidenceStrength::CheckedRetained,
    );
    worth_query_recovery_brief_from_ordinary_outcome(
        &ordinary_outcome_from_signal_compatibility_orchestration_checked(checked),
    )
    .map(|brief| {
        let merged = merge_signal_explanation(&brief, explanation);
        brief.with_explanation(merged)
    })
}

pub fn worth_query_recovery_brief_from_signal_compatibility_proof<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    proof: WorthQuerySignalCompatibilityOrchestrationTranscript<D, I>,
) -> Option<WorthQueryRecoveryBrief> {
    let explanation = signal_explanation(
        proof.outcome(),
        proof.linked_artifacts().clone(),
        WorthQueryRecoveryEvidenceStrength::ProofRetained,
    );
    worth_query_recovery_brief_from_ordinary_outcome(
        &ordinary_outcome_from_signal_compatibility_orchestration_checked(proof.into_checked()),
    )
    .map(|brief| {
        let merged = merge_signal_explanation(&brief, explanation);
        brief.with_explanation(merged)
    })
}

fn merge_signal_explanation(
    brief: &WorthQueryRecoveryBrief,
    explanation: WorthQueryRecoveryExplanation,
) -> WorthQueryRecoveryExplanation {
    let mut merged = brief
        .explanation()
        .clone()
        .with_source_family(explanation.source_family())
        .with_evidence_strength(explanation.evidence_strength())
        .with_basis_posture(explanation.basis_posture())
        .with_aspect_posture(explanation.aspect_posture())
        .with_diagnostic_context(diagnostic_context_for_stop_kind(brief.stop_kind()));
    if let (Some(truth_kind), Some(basis_disclosure)) = (
        explanation.support_truth_kind(),
        explanation.basis_disclosure(),
    ) {
        merged = merged.with_support_context(WorthQueryRecoveryFoundationalSupportContext::new(
            truth_kind,
            basis_disclosure,
            explanation.degraded_recovery_posture(),
        ));
    }
    if let Some(drift) = explanation.installed_domain_execution_drift() {
        merged = merged.with_installed_domain_execution_drift(drift.clone());
    }
    merged
}

fn signal_explanation<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>(
    outcome: &WorthQuerySignalCompatibilityOrchestrationOutcome<D, I>,
    linked_artifacts: crate::binding_pipeline::WorthQueryBindingLinkedArtifacts,
    evidence_strength: WorthQueryRecoveryEvidenceStrength,
) -> WorthQueryRecoveryExplanation {
    let stop_kind = match outcome {
        WorthQuerySignalCompatibilityOrchestrationOutcome::Bound(_) => {
            WorthQueryRecoveryStopKind::Deferred
        }
        WorthQuerySignalCompatibilityOrchestrationOutcome::Ambiguous(_) => {
            WorthQueryRecoveryStopKind::Ambiguous
        }
        WorthQuerySignalCompatibilityOrchestrationOutcome::Unavailable(_) => {
            WorthQueryRecoveryStopKind::Unavailable
        }
        WorthQuerySignalCompatibilityOrchestrationOutcome::WrongWorld(_) => {
            WorthQueryRecoveryStopKind::WrongWorld
        }
        WorthQuerySignalCompatibilityOrchestrationOutcome::WrongHandle(_) => {
            WorthQueryRecoveryStopKind::WrongHandle
        }
        WorthQuerySignalCompatibilityOrchestrationOutcome::InstalledAuthorityDrift(_) => {
            WorthQueryRecoveryStopKind::RebindRequired
        }
        WorthQuerySignalCompatibilityOrchestrationOutcome::Stale(_) => {
            WorthQueryRecoveryStopKind::Stale
        }
        WorthQuerySignalCompatibilityOrchestrationOutcome::RebindRequired(_) => {
            WorthQueryRecoveryStopKind::RebindRequired
        }
        WorthQuerySignalCompatibilityOrchestrationOutcome::MissingRequiredAspect(_) => {
            WorthQueryRecoveryStopKind::MissingRequiredAspect
        }
        WorthQuerySignalCompatibilityOrchestrationOutcome::AspectConflict(_) => {
            WorthQueryRecoveryStopKind::AspectConflict
        }
        WorthQuerySignalCompatibilityOrchestrationOutcome::AuthorityMismatch(_) => {
            WorthQueryRecoveryStopKind::AuthorityMismatch
        }
        WorthQuerySignalCompatibilityOrchestrationOutcome::BasisMismatch(_) => {
            WorthQueryRecoveryStopKind::BasisMismatch
        }
        WorthQuerySignalCompatibilityOrchestrationOutcome::Deferred(_) => {
            WorthQueryRecoveryStopKind::Deferred
        }
        WorthQuerySignalCompatibilityOrchestrationOutcome::Denied(_) => {
            WorthQueryRecoveryStopKind::DeclarationDenied
        }
        WorthQuerySignalCompatibilityOrchestrationOutcome::Unsupported(_) => {
            WorthQueryRecoveryStopKind::Unsupported
        }
        WorthQuerySignalCompatibilityOrchestrationOutcome::Failed(_) => {
            WorthQueryRecoveryStopKind::Failed
        }
    };
    let mut explanation = WorthQueryRecoveryExplanation::new_with_source_family(
        crate::ordinary_outcome::WorthQueryOrdinaryCheckedTopology::signal_compatibility_orchestration(
            crate::ordinary_outcome::WorthQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind::Unsupported,
            linked_artifacts,
        ),
        WorthQueryRecoverySourceFamily::SignalCompatibility,
    )
    .with_evidence_strength(evidence_strength)
    .with_aspect_posture(WorthQueryRecoveryAspectPosture::RequiredContract)
    .with_diagnostic_context(diagnostic_context_for_stop_kind(stop_kind));
    match outcome {
        WorthQuerySignalCompatibilityOrchestrationOutcome::InstalledAuthorityDrift(drift) => {
            explanation = explanation.with_installed_domain_execution_drift(drift.clone());
        }
        WorthQuerySignalCompatibilityOrchestrationOutcome::Stale(_) => {
            explanation = explanation
                .with_basis_posture(WorthQueryRecoveryBasisPosture::StaleBasis)
                .with_support_context(support_context_for_stale_basis());
        }
        WorthQuerySignalCompatibilityOrchestrationOutcome::BasisMismatch(_) => {
            explanation = explanation
                .with_basis_posture(WorthQueryRecoveryBasisPosture::BasisMismatch)
                .with_support_context(support_context_for_basis_mismatch());
        }
        _ => {}
    }
    explanation
}
