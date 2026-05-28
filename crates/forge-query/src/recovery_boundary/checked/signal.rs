use crate::application::{ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker};
use crate::recovery_boundary::foundational::{
    diagnostic_context_for_stop_kind, support_context_for_basis_mismatch,
    support_context_for_stale_basis,
};
use crate::recovery_boundary::ordinary::forge_query_recovery_brief_from_ordinary_outcome;
use crate::recovery_boundary::{
    ForgeQueryRecoveryAspectPosture, ForgeQueryRecoveryBasisPosture, ForgeQueryRecoveryBrief,
    ForgeQueryRecoveryEvidenceStrength, ForgeQueryRecoveryExplanation,
    ForgeQueryRecoveryFoundationalSupportContext, ForgeQueryRecoverySourceFamily,
    ForgeQueryRecoveryStopKind,
};
use crate::signal_compatibility_orchestration::{
    ordinary_outcome_from_signal_compatibility_orchestration_checked,
    ForgeQuerySignalCompatibilityOrchestrationChecked,
    ForgeQuerySignalCompatibilityOrchestrationOutcome,
    ForgeQuerySignalCompatibilityOrchestrationTranscript,
};

pub fn forge_query_recovery_brief_from_signal_compatibility_checked<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    checked: ForgeQuerySignalCompatibilityOrchestrationChecked<D, I>,
) -> Option<ForgeQueryRecoveryBrief> {
    let explanation = signal_explanation(
        checked.outcome(),
        checked.linked_artifacts().clone(),
        ForgeQueryRecoveryEvidenceStrength::CheckedRetained,
    );
    forge_query_recovery_brief_from_ordinary_outcome(
        &ordinary_outcome_from_signal_compatibility_orchestration_checked(checked),
    )
    .map(|brief| {
        let merged = merge_signal_explanation(&brief, explanation);
        brief.with_explanation(merged)
    })
}

pub fn forge_query_recovery_brief_from_signal_compatibility_proof<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    proof: ForgeQuerySignalCompatibilityOrchestrationTranscript<D, I>,
) -> Option<ForgeQueryRecoveryBrief> {
    let explanation = signal_explanation(
        proof.outcome(),
        proof.linked_artifacts().clone(),
        ForgeQueryRecoveryEvidenceStrength::ProofRetained,
    );
    forge_query_recovery_brief_from_ordinary_outcome(
        &ordinary_outcome_from_signal_compatibility_orchestration_checked(proof.into_checked()),
    )
    .map(|brief| {
        let merged = merge_signal_explanation(&brief, explanation);
        brief.with_explanation(merged)
    })
}

fn merge_signal_explanation(
    brief: &ForgeQueryRecoveryBrief,
    explanation: ForgeQueryRecoveryExplanation,
) -> ForgeQueryRecoveryExplanation {
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
        merged = merged.with_support_context(ForgeQueryRecoveryFoundationalSupportContext::new(
            truth_kind,
            basis_disclosure,
            explanation.degraded_recovery_posture(),
        ));
    }
    merged
}

fn signal_explanation<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    outcome: &ForgeQuerySignalCompatibilityOrchestrationOutcome<D, I>,
    linked_artifacts: crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts,
    evidence_strength: ForgeQueryRecoveryEvidenceStrength,
) -> ForgeQueryRecoveryExplanation {
    let stop_kind = match outcome {
        ForgeQuerySignalCompatibilityOrchestrationOutcome::Bound(_) => {
            ForgeQueryRecoveryStopKind::Deferred
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::Ambiguous(_) => {
            ForgeQueryRecoveryStopKind::Ambiguous
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::Unavailable(_) => {
            ForgeQueryRecoveryStopKind::Unavailable
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::WrongWorld(_) => {
            ForgeQueryRecoveryStopKind::WrongWorld
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::WrongHandle(_) => {
            ForgeQueryRecoveryStopKind::WrongHandle
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::Stale(_) => {
            ForgeQueryRecoveryStopKind::Stale
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::RebindRequired(_) => {
            ForgeQueryRecoveryStopKind::RebindRequired
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::MissingRequiredAspect(_) => {
            ForgeQueryRecoveryStopKind::MissingRequiredAspect
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::AspectConflict(_) => {
            ForgeQueryRecoveryStopKind::AspectConflict
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::AuthorityMismatch(_) => {
            ForgeQueryRecoveryStopKind::AuthorityMismatch
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::BasisMismatch(_) => {
            ForgeQueryRecoveryStopKind::BasisMismatch
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::Deferred(_) => {
            ForgeQueryRecoveryStopKind::Deferred
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::Denied(_) => {
            ForgeQueryRecoveryStopKind::DeclarationDenied
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::Unsupported(_) => {
            ForgeQueryRecoveryStopKind::Unsupported
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::Failed(_) => {
            ForgeQueryRecoveryStopKind::Failed
        }
    };
    let mut explanation = ForgeQueryRecoveryExplanation::new_with_source_family(
        crate::ordinary_outcome::ForgeQueryOrdinaryCheckedTopology::signal_compatibility_orchestration(
            crate::ordinary_outcome::ForgeQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind::Unsupported,
            linked_artifacts,
        ),
        ForgeQueryRecoverySourceFamily::SignalCompatibility,
    )
    .with_evidence_strength(evidence_strength)
    .with_aspect_posture(ForgeQueryRecoveryAspectPosture::RequiredContract)
    .with_diagnostic_context(diagnostic_context_for_stop_kind(stop_kind));
    match outcome {
        ForgeQuerySignalCompatibilityOrchestrationOutcome::Stale(_) => {
            explanation = explanation
                .with_basis_posture(ForgeQueryRecoveryBasisPosture::StaleBasis)
                .with_support_context(support_context_for_stale_basis());
        }
        ForgeQuerySignalCompatibilityOrchestrationOutcome::BasisMismatch(_) => {
            explanation = explanation
                .with_basis_posture(ForgeQueryRecoveryBasisPosture::BasisMismatch)
                .with_support_context(support_context_for_basis_mismatch());
        }
        _ => {}
    }
    explanation
}
