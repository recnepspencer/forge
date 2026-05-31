use crate::application::{ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker};
use crate::contribution_composed_orchestration::{
    ordinary_outcome_from_contribution_composed_checked,
    ForgeQueryContributionComposedClassification,
    ForgeQueryContributionComposedIntentClassification,
    ForgeQueryContributionComposedIntentRequestDescriptor,
    ForgeQueryContributionComposedOrchestrationChecked,
    ForgeQueryContributionComposedOrchestrationCheckedKind,
    ForgeQueryContributionComposedOrchestrationOutcome,
    ForgeQueryContributionComposedOrchestrationTranscript,
};
use crate::recovery_boundary::foundational::{
    diagnostic_context_for_stop_kind, support_context_for_stale_basis,
};
use crate::recovery_boundary::ordinary::forge_query_recovery_brief_from_ordinary_outcome;
use crate::recovery_boundary::{
    ForgeQueryRecoveryAspectPosture, ForgeQueryRecoveryBasisPosture, ForgeQueryRecoveryBrief,
    ForgeQueryRecoveryConflictPosture, ForgeQueryRecoveryEvidenceStrength,
    ForgeQueryRecoveryExplanation, ForgeQueryRecoveryFoundationalSupportContext,
    ForgeQueryRecoverySourceFamily, ForgeQueryRecoveryStopKind,
};

pub fn forge_query_recovery_brief_from_contribution_composed_checked<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    checked: ForgeQueryContributionComposedOrchestrationChecked<D, I>,
) -> Option<ForgeQueryRecoveryBrief> {
    let explanation = contribution_checked_explanation(
        &checked,
        ForgeQueryRecoveryEvidenceStrength::CheckedRetained,
    );
    forge_query_recovery_brief_from_ordinary_outcome(
        &ordinary_outcome_from_contribution_composed_checked(checked),
    )
    .map(|brief| {
        let merged = merge_contribution_explanation(&brief, explanation);
        brief.with_explanation(merged)
    })
}

pub fn forge_query_recovery_brief_from_contribution_composed_proof<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    proof: ForgeQueryContributionComposedOrchestrationTranscript<D, I>,
) -> Option<ForgeQueryRecoveryBrief> {
    let explanation =
        contribution_proof_explanation(&proof, ForgeQueryRecoveryEvidenceStrength::ProofRetained);
    forge_query_recovery_brief_from_ordinary_outcome(
        &ordinary_outcome_from_contribution_composed_checked(proof.into_checked()),
    )
    .map(|brief| {
        let merged = merge_contribution_explanation(&brief, explanation);
        brief.with_explanation(merged)
    })
}

fn merge_contribution_explanation(
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
        .with_conflict_posture(explanation.conflict_posture())
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
    if let Some(descriptor) = explanation.contribution_intent_descriptor() {
        merged = merged.with_contribution_intent_descriptor(descriptor.clone());
    }
    merged
}

fn contribution_checked_explanation<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    checked: &ForgeQueryContributionComposedOrchestrationChecked<D, I>,
    evidence_strength: ForgeQueryRecoveryEvidenceStrength,
) -> ForgeQueryRecoveryExplanation {
    let (kind, linked) = match checked {
        ForgeQueryContributionComposedOrchestrationOutcome::Bound(_) => {
            return base_contribution_explanation(
                evidence_strength,
                ForgeQueryRecoveryBasisPosture::Unknown,
                ForgeQueryRecoveryConflictPosture::None,
            );
        }
        ForgeQueryContributionComposedOrchestrationOutcome::Deferred(value)
        | ForgeQueryContributionComposedOrchestrationOutcome::DeclarationDenied(value)
        | ForgeQueryContributionComposedOrchestrationOutcome::ContributionDenied(value)
        | ForgeQueryContributionComposedOrchestrationOutcome::Stale(value)
        | ForgeQueryContributionComposedOrchestrationOutcome::RebindRequired(value)
        | ForgeQueryContributionComposedOrchestrationOutcome::Unsupported(value)
        | ForgeQueryContributionComposedOrchestrationOutcome::Failed(value) => {
            (value.kind(), value.linked_artifacts().clone())
        }
    };
    let mut explanation = ForgeQueryRecoveryExplanation::new_with_source_family(
        crate::ordinary_outcome::ForgeQueryOrdinaryCheckedTopology::contribution_composed(
            map_contribution_kind(kind),
            linked,
            None,
        ),
        ForgeQueryRecoverySourceFamily::ContributionComposed,
    )
    .with_evidence_strength(evidence_strength)
    .with_diagnostic_context(diagnostic_context_for_stop_kind(contribution_stop_kind(
        kind,
    )));
    explanation = explanation.with_aspect_posture(if value_has_retained_aspect_record(checked) {
        ForgeQueryRecoveryAspectPosture::RetainedContractAndCoverage
    } else {
        ForgeQueryRecoveryAspectPosture::CategoryScopedAspectComposition
    });
    if kind == ForgeQueryContributionComposedOrchestrationCheckedKind::Stale {
        explanation = explanation
            .with_basis_posture(ForgeQueryRecoveryBasisPosture::StaleBasis)
            .with_support_context(support_context_for_stale_basis());
    }
    if let Some(descriptor) = primary_checked_intent_descriptor(checked) {
        explanation = explanation.with_contribution_intent_descriptor(descriptor.clone());
    }
    explanation
}

fn contribution_proof_explanation<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    proof: &ForgeQueryContributionComposedOrchestrationTranscript<D, I>,
    evidence_strength: ForgeQueryRecoveryEvidenceStrength,
) -> ForgeQueryRecoveryExplanation {
    let basis_posture = match proof.outcome() {
        ForgeQueryContributionComposedOrchestrationOutcome::Stale(_) => {
            ForgeQueryRecoveryBasisPosture::StaleBasis
        }
        ForgeQueryContributionComposedOrchestrationOutcome::RebindRequired(_) => {
            ForgeQueryRecoveryBasisPosture::Unknown
        }
        _ => ForgeQueryRecoveryBasisPosture::Unknown,
    };
    let conflict_posture = match proof.composition_classification() {
        Some(ForgeQueryContributionComposedClassification::PartiallyAdmitted)
        | Some(ForgeQueryContributionComposedClassification::MaterializationFailedAfterAdmission) => {
            ForgeQueryRecoveryConflictPosture::MixedContributionFailure
        }
        _ => ForgeQueryRecoveryConflictPosture::None,
    };
    let mut explanation =
        base_contribution_explanation(evidence_strength, basis_posture, conflict_posture)
            .with_aspect_posture(if proof.declaration().aspect_record().is_some() {
                ForgeQueryRecoveryAspectPosture::RetainedContractAndCoverage
            } else {
                ForgeQueryRecoveryAspectPosture::CategoryScopedAspectComposition
            });
    if let Some(descriptor) = primary_intent_descriptor(proof.intent_results()) {
        explanation = explanation.with_contribution_intent_descriptor(descriptor.clone());
    }
    if basis_posture == ForgeQueryRecoveryBasisPosture::StaleBasis {
        explanation = explanation.with_support_context(support_context_for_stale_basis());
    }
    explanation
}

fn base_contribution_explanation(
    evidence_strength: ForgeQueryRecoveryEvidenceStrength,
    basis_posture: ForgeQueryRecoveryBasisPosture,
    conflict_posture: ForgeQueryRecoveryConflictPosture,
) -> ForgeQueryRecoveryExplanation {
    ForgeQueryRecoveryExplanation::new_with_source_family(
        crate::ordinary_outcome::ForgeQueryOrdinaryCheckedTopology::contribution_composed(
            crate::ordinary_outcome::ForgeQueryOrdinaryContributionComposedCheckedTopologyKind::Failed,
            crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts::new(),
            None,
        ),
        ForgeQueryRecoverySourceFamily::ContributionComposed,
    )
    .with_evidence_strength(evidence_strength)
    .with_basis_posture(basis_posture)
    .with_conflict_posture(conflict_posture)
}

fn primary_intent_descriptor(
    intent_results: &[crate::contribution_composed_orchestration::ForgeQueryContributionComposedIntentResult],
) -> Option<&ForgeQueryContributionComposedIntentRequestDescriptor> {
    intent_results
        .iter()
        .find(|value| {
            value.classification() != ForgeQueryContributionComposedIntentClassification::Admitted
        })
        .or_else(|| intent_results.first())
        .map(|value| value.request())
}

fn value_has_retained_aspect_record<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    checked: &ForgeQueryContributionComposedOrchestrationChecked<D, I>,
) -> bool {
    checked_posture(checked)
        .and_then(|value| value.declaration_aspect_record())
        .is_some()
}

fn primary_checked_intent_descriptor<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    checked: &ForgeQueryContributionComposedOrchestrationChecked<D, I>,
) -> Option<&ForgeQueryContributionComposedIntentRequestDescriptor> {
    checked_posture(checked).and_then(|value| value.primary_intent_descriptor())
}

fn checked_posture<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    checked: &ForgeQueryContributionComposedOrchestrationChecked<D, I>,
) -> Option<
    &crate::contribution_composed_orchestration::ForgeQueryContributionComposedOrchestrationPosture<
        D,
        I,
    >,
> {
    match checked {
        ForgeQueryContributionComposedOrchestrationOutcome::Bound(_) => None,
        ForgeQueryContributionComposedOrchestrationOutcome::Deferred(value)
        | ForgeQueryContributionComposedOrchestrationOutcome::DeclarationDenied(value)
        | ForgeQueryContributionComposedOrchestrationOutcome::ContributionDenied(value)
        | ForgeQueryContributionComposedOrchestrationOutcome::Stale(value)
        | ForgeQueryContributionComposedOrchestrationOutcome::RebindRequired(value)
        | ForgeQueryContributionComposedOrchestrationOutcome::Unsupported(value)
        | ForgeQueryContributionComposedOrchestrationOutcome::Failed(value) => Some(value),
    }
}

fn map_contribution_kind(
    kind: ForgeQueryContributionComposedOrchestrationCheckedKind,
) -> crate::ordinary_outcome::ForgeQueryOrdinaryContributionComposedCheckedTopologyKind {
    match kind {
        ForgeQueryContributionComposedOrchestrationCheckedKind::Deferred => {
            crate::ordinary_outcome::ForgeQueryOrdinaryContributionComposedCheckedTopologyKind::Deferred
        }
        ForgeQueryContributionComposedOrchestrationCheckedKind::DeclarationDenied => {
            crate::ordinary_outcome::ForgeQueryOrdinaryContributionComposedCheckedTopologyKind::DeclarationDenied
        }
        ForgeQueryContributionComposedOrchestrationCheckedKind::ContributionDenied => {
            crate::ordinary_outcome::ForgeQueryOrdinaryContributionComposedCheckedTopologyKind::ContributionDenied
        }
        ForgeQueryContributionComposedOrchestrationCheckedKind::Stale => {
            crate::ordinary_outcome::ForgeQueryOrdinaryContributionComposedCheckedTopologyKind::Stale
        }
        ForgeQueryContributionComposedOrchestrationCheckedKind::RebindRequired => {
            crate::ordinary_outcome::ForgeQueryOrdinaryContributionComposedCheckedTopologyKind::RebindRequired
        }
        ForgeQueryContributionComposedOrchestrationCheckedKind::Unsupported => {
            crate::ordinary_outcome::ForgeQueryOrdinaryContributionComposedCheckedTopologyKind::Unsupported
        }
        ForgeQueryContributionComposedOrchestrationCheckedKind::Failed => {
            crate::ordinary_outcome::ForgeQueryOrdinaryContributionComposedCheckedTopologyKind::Failed
        }
    }
}

fn contribution_stop_kind(
    kind: ForgeQueryContributionComposedOrchestrationCheckedKind,
) -> ForgeQueryRecoveryStopKind {
    match kind {
        ForgeQueryContributionComposedOrchestrationCheckedKind::Deferred => {
            ForgeQueryRecoveryStopKind::Deferred
        }
        ForgeQueryContributionComposedOrchestrationCheckedKind::DeclarationDenied => {
            ForgeQueryRecoveryStopKind::DeclarationDenied
        }
        ForgeQueryContributionComposedOrchestrationCheckedKind::ContributionDenied => {
            ForgeQueryRecoveryStopKind::ContributionDenied
        }
        ForgeQueryContributionComposedOrchestrationCheckedKind::Stale => {
            ForgeQueryRecoveryStopKind::Stale
        }
        ForgeQueryContributionComposedOrchestrationCheckedKind::RebindRequired => {
            ForgeQueryRecoveryStopKind::RebindRequired
        }
        ForgeQueryContributionComposedOrchestrationCheckedKind::Unsupported => {
            ForgeQueryRecoveryStopKind::Unsupported
        }
        ForgeQueryContributionComposedOrchestrationCheckedKind::Failed => {
            ForgeQueryRecoveryStopKind::Failed
        }
    }
}
