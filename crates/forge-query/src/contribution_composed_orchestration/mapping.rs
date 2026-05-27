use crate::application::{
    ForgeQueryDeclarationAdmissionError, ForgeQueryDeclarationAdmissionOrLegalityError,
    ForgeQueryDeclarationEntryOrchestrationStage, ForgeQueryDeclarationEntryProgressionError,
    ForgeQueryDeclarationEnvelope, ForgeQueryDeclarationEnvelopeTerminalError,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationProgressionTerminalError,
    ForgeQueryDomainEntryMarker,
};
use crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts;

use super::outcome::{
    ForgeQueryContributionComposedOrchestrationCheckedKind,
    ForgeQueryContributionComposedOrchestrationOutcome,
    ForgeQueryContributionComposedOrchestrationPosture,
};

pub(super) fn declaration_error_outcome<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    error: ForgeQueryDeclarationEntryProgressionError<D, I>,
) -> ForgeQueryContributionComposedOrchestrationOutcome<D, I> {
    match error {
        ForgeQueryDeclarationEntryProgressionError::Entry(value) => match value {
            ForgeQueryDeclarationAdmissionOrLegalityError::Admission(admission) => {
                declaration_admission_outcome(admission)
            }
            ForgeQueryDeclarationAdmissionOrLegalityError::Legality(_) => contribution_outcome(
                ForgeQueryContributionComposedOrchestrationCheckedKind::DeclarationDenied,
                ForgeQueryDeclarationEntryOrchestrationStage::LegalityEstablished,
                "declaration legality denied contribution-composed orchestration",
                ForgeQueryBindingLinkedArtifacts::new(),
                None,
            ),
        },
        ForgeQueryDeclarationEntryProgressionError::Progression(value) => match value {
            ForgeQueryDeclarationProgressionTerminalError::Deferred(progress) => {
                contribution_outcome(
                    ForgeQueryContributionComposedOrchestrationCheckedKind::Deferred,
                    ForgeQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                    "declaration progression remained deferred",
                    ForgeQueryBindingLinkedArtifacts::new().with_declaration_digest(format!(
                        "{:?}",
                        progress
                            .legality_evidence()
                            .canonical_declaration()
                            .declaration_digest()
                    )),
                    None,
                )
            }
            ForgeQueryDeclarationProgressionTerminalError::Denied(progress) => {
                contribution_outcome(
                    ForgeQueryContributionComposedOrchestrationCheckedKind::DeclarationDenied,
                    ForgeQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                    "declaration progression was denied",
                    ForgeQueryBindingLinkedArtifacts::new().with_declaration_digest(format!(
                        "{:?}",
                        progress
                            .legality_evidence()
                            .canonical_declaration()
                            .declaration_digest()
                    )),
                    None,
                )
            }
            ForgeQueryDeclarationProgressionTerminalError::Stale(progress) => contribution_outcome(
                ForgeQueryContributionComposedOrchestrationCheckedKind::Stale,
                ForgeQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                "declaration progression is stale",
                ForgeQueryBindingLinkedArtifacts::new()
                    .with_declaration_digest(format!(
                        "{:?}",
                        progress
                            .legality_evidence()
                            .canonical_declaration()
                            .declaration_digest()
                    ))
                    .with_progression_digest(progress.progression_digest().to_string()),
                None,
            ),
            ForgeQueryDeclarationProgressionTerminalError::RebindRequired(progress) => {
                contribution_outcome(
                    ForgeQueryContributionComposedOrchestrationCheckedKind::RebindRequired,
                    ForgeQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                    "declaration progression requires rebind",
                    ForgeQueryBindingLinkedArtifacts::new()
                        .with_declaration_digest(format!(
                            "{:?}",
                            progress
                                .legality_evidence()
                                .canonical_declaration()
                                .declaration_digest()
                        ))
                        .with_progression_digest(progress.progression_digest().to_string()),
                    None,
                )
            }
            ForgeQueryDeclarationProgressionTerminalError::Failed(progress) => {
                contribution_outcome(
                    ForgeQueryContributionComposedOrchestrationCheckedKind::Failed,
                    ForgeQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                    "declaration progression failed",
                    ForgeQueryBindingLinkedArtifacts::new()
                        .with_declaration_digest(format!(
                            "{:?}",
                            progress
                                .legality_evidence()
                                .canonical_declaration()
                                .declaration_digest()
                        ))
                        .with_progression_digest(progress.progression_digest().to_string()),
                    None,
                )
            }
        },
    }
}

pub(super) fn envelope_error_outcome<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    error: ForgeQueryDeclarationEnvelopeTerminalError<D, I>,
    declaration_digest: &str,
    progression_digest: &str,
) -> ForgeQueryContributionComposedOrchestrationOutcome<D, I> {
    match error {
        ForgeQueryDeclarationEnvelopeTerminalError::Deferred(value) => contribution_outcome(
            ForgeQueryContributionComposedOrchestrationCheckedKind::DeclarationDenied,
            ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
            value.reason(),
            linked_artifacts_for_envelope(value.envelope())
                .with_declaration_digest(declaration_digest.to_string())
                .with_progression_digest(progression_digest.to_string()),
            None,
        ),
        ForgeQueryDeclarationEnvelopeTerminalError::Denied(value) => contribution_outcome(
            ForgeQueryContributionComposedOrchestrationCheckedKind::DeclarationDenied,
            ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
            value.reason(),
            linked_artifacts_for_envelope(value.envelope())
                .with_declaration_digest(declaration_digest.to_string())
                .with_progression_digest(progression_digest.to_string()),
            None,
        ),
        ForgeQueryDeclarationEnvelopeTerminalError::Failed(value) => contribution_outcome(
            ForgeQueryContributionComposedOrchestrationCheckedKind::Failed,
            ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
            value.reason(),
            linked_artifacts_for_envelope(value.envelope())
                .with_declaration_digest(declaration_digest.to_string())
                .with_progression_digest(progression_digest.to_string()),
            None,
        ),
    }
}

pub(super) fn linked_artifacts_for_envelope<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    envelope: &ForgeQueryDeclarationEnvelope<D, I>,
) -> ForgeQueryBindingLinkedArtifacts {
    let mut linked = ForgeQueryBindingLinkedArtifacts::new()
        .with_declaration_digest(envelope.declaration_digest().to_string());
    if let Some(value) = envelope.progression_digest() {
        linked = linked.with_progression_digest(value.to_string());
    }
    if let Some(value) = envelope.route_plan_digest() {
        linked = linked.with_route_plan_digest(value.to_string());
    }
    linked
        .with_receipt_digest(format!("{:?}", envelope.receipt_digest()))
        .with_envelope_digest(format!("{:?}", envelope.envelope_digest()))
}

pub(super) fn linked_artifacts_from_outcome<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    outcome: &ForgeQueryContributionComposedOrchestrationOutcome<D, I>,
) -> ForgeQueryBindingLinkedArtifacts {
    match outcome {
        ForgeQueryContributionComposedOrchestrationOutcome::Bound(value) => {
            linked_artifacts_for_envelope(value.envelope())
        }
        ForgeQueryContributionComposedOrchestrationOutcome::Deferred(value)
        | ForgeQueryContributionComposedOrchestrationOutcome::DeclarationDenied(value)
        | ForgeQueryContributionComposedOrchestrationOutcome::ContributionDenied(value)
        | ForgeQueryContributionComposedOrchestrationOutcome::Stale(value)
        | ForgeQueryContributionComposedOrchestrationOutcome::RebindRequired(value)
        | ForgeQueryContributionComposedOrchestrationOutcome::Unsupported(value)
        | ForgeQueryContributionComposedOrchestrationOutcome::Failed(value) => {
            value.linked_artifacts().clone()
        }
    }
}

pub(super) fn contribution_digest_from_outcome<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    outcome: &ForgeQueryContributionComposedOrchestrationOutcome<D, I>,
) -> Option<String> {
    match outcome {
        ForgeQueryContributionComposedOrchestrationOutcome::Bound(value) => Some(
            value
                .contribution_composition()
                .contribution_digest()
                .to_string(),
        ),
        ForgeQueryContributionComposedOrchestrationOutcome::Deferred(value)
        | ForgeQueryContributionComposedOrchestrationOutcome::DeclarationDenied(value)
        | ForgeQueryContributionComposedOrchestrationOutcome::ContributionDenied(value)
        | ForgeQueryContributionComposedOrchestrationOutcome::Stale(value)
        | ForgeQueryContributionComposedOrchestrationOutcome::RebindRequired(value)
        | ForgeQueryContributionComposedOrchestrationOutcome::Unsupported(value)
        | ForgeQueryContributionComposedOrchestrationOutcome::Failed(value) => {
            value.contribution_digest().map(str::to_string)
        }
    }
}

pub(super) fn contribution_outcome<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    kind: ForgeQueryContributionComposedOrchestrationCheckedKind,
    stop_stage: ForgeQueryDeclarationEntryOrchestrationStage,
    reason: impl Into<String>,
    linked_artifacts: ForgeQueryBindingLinkedArtifacts,
    contribution_digest: Option<String>,
) -> ForgeQueryContributionComposedOrchestrationOutcome<D, I> {
    let posture = ForgeQueryContributionComposedOrchestrationPosture::new(
        kind,
        stop_stage,
        reason,
        linked_artifacts,
        contribution_digest,
    );
    match kind {
        ForgeQueryContributionComposedOrchestrationCheckedKind::Deferred => {
            ForgeQueryContributionComposedOrchestrationOutcome::Deferred(posture)
        }
        ForgeQueryContributionComposedOrchestrationCheckedKind::DeclarationDenied => {
            ForgeQueryContributionComposedOrchestrationOutcome::DeclarationDenied(posture)
        }
        ForgeQueryContributionComposedOrchestrationCheckedKind::ContributionDenied => {
            ForgeQueryContributionComposedOrchestrationOutcome::ContributionDenied(posture)
        }
        ForgeQueryContributionComposedOrchestrationCheckedKind::Stale => {
            ForgeQueryContributionComposedOrchestrationOutcome::Stale(posture)
        }
        ForgeQueryContributionComposedOrchestrationCheckedKind::RebindRequired => {
            ForgeQueryContributionComposedOrchestrationOutcome::RebindRequired(posture)
        }
        ForgeQueryContributionComposedOrchestrationCheckedKind::Unsupported => {
            ForgeQueryContributionComposedOrchestrationOutcome::Unsupported(posture)
        }
        ForgeQueryContributionComposedOrchestrationCheckedKind::Failed => {
            ForgeQueryContributionComposedOrchestrationOutcome::Failed(posture)
        }
    }
}

fn declaration_admission_outcome<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    error: ForgeQueryDeclarationAdmissionError<D, I>,
) -> ForgeQueryContributionComposedOrchestrationOutcome<D, I> {
    match error {
        ForgeQueryDeclarationAdmissionError::Deferred(_) => contribution_outcome(
            ForgeQueryContributionComposedOrchestrationCheckedKind::Deferred,
            ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
            "declaration admission is deferred for this handle",
            ForgeQueryBindingLinkedArtifacts::new(),
            None,
        ),
        ForgeQueryDeclarationAdmissionError::Unsupported(_) => contribution_outcome(
            ForgeQueryContributionComposedOrchestrationCheckedKind::Unsupported,
            ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
            "declaration family is unsupported for contribution-composed orchestration",
            ForgeQueryBindingLinkedArtifacts::new(),
            None,
        ),
        ForgeQueryDeclarationAdmissionError::InvalidContext(_) => contribution_outcome(
            ForgeQueryContributionComposedOrchestrationCheckedKind::Unsupported,
            ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
            "declaration family is invalid in the current admitted context",
            ForgeQueryBindingLinkedArtifacts::new(),
            None,
        ),
        ForgeQueryDeclarationAdmissionError::Canonicalization(value) => contribution_outcome(
            ForgeQueryContributionComposedOrchestrationCheckedKind::Failed,
            ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
            format!("{value:?}"),
            ForgeQueryBindingLinkedArtifacts::new(),
            None,
        ),
    }
}
