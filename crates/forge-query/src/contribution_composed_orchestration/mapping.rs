use crate::application::{
    ForgeQueryDeclarationAdmissionError, ForgeQueryDeclarationAdmissionOrLegalityError,
    ForgeQueryDeclarationEntryOrchestrationStage, ForgeQueryDeclarationEntryProgressionError,
    ForgeQueryDeclarationEnvelope, ForgeQueryDeclarationEnvelopeTerminalError,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationProgressionTerminalError,
    ForgeQueryDomainEntryMarker,
};
use crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts;

use super::composition::ForgeQueryContributionComposedStop;
use super::intent_result::ForgeQueryContributionComposedIntentRequestDescriptor;
use super::outcome::{
    ForgeQueryContributionComposedOrchestrationCheckedKind,
    ForgeQueryContributionComposedOrchestrationOutcome,
    ForgeQueryContributionComposedOrchestrationPosture,
};
use super::ForgeQueryContributionComposedDeclarationAspectRecord;

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
            ForgeQueryDeclarationAdmissionOrLegalityError::Legality(_) => declaration_outcome(
                ForgeQueryContributionComposedOrchestrationCheckedKind::DeclarationDenied,
                ForgeQueryContributionComposedStop::DeclarationDenied,
                ForgeQueryDeclarationEntryOrchestrationStage::LegalityEstablished,
                "declaration legality denied contribution-composed orchestration",
                ForgeQueryBindingLinkedArtifacts::new(),
                None,
                None,
                None,
            ),
        },
        ForgeQueryDeclarationEntryProgressionError::Progression(value) => match value {
            ForgeQueryDeclarationProgressionTerminalError::Deferred(progress) => {
                declaration_outcome(
                    ForgeQueryContributionComposedOrchestrationCheckedKind::Deferred,
                    ForgeQueryContributionComposedStop::Deferred,
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
                    None,
                    None,
                )
            }
            ForgeQueryDeclarationProgressionTerminalError::Denied(progress) => declaration_outcome(
                ForgeQueryContributionComposedOrchestrationCheckedKind::DeclarationDenied,
                ForgeQueryContributionComposedStop::DeclarationDenied,
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
                None,
                None,
            ),
            ForgeQueryDeclarationProgressionTerminalError::Stale(progress) => declaration_outcome(
                ForgeQueryContributionComposedOrchestrationCheckedKind::Stale,
                ForgeQueryContributionComposedStop::Stale,
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
                None,
                None,
            ),
            ForgeQueryDeclarationProgressionTerminalError::RebindRequired(progress) => {
                declaration_outcome(
                    ForgeQueryContributionComposedOrchestrationCheckedKind::RebindRequired,
                    ForgeQueryContributionComposedStop::RebindRequired,
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
                    None,
                    None,
                )
            }
            ForgeQueryDeclarationProgressionTerminalError::Failed(progress) => declaration_outcome(
                ForgeQueryContributionComposedOrchestrationCheckedKind::Failed,
                ForgeQueryContributionComposedStop::Failed,
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
                None,
                None,
            ),
        },
    }
}

pub(super) fn envelope_error_outcome<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    error: ForgeQueryDeclarationEnvelopeTerminalError<D, I>,
) -> ForgeQueryContributionComposedOrchestrationOutcome<D, I> {
    match error {
        ForgeQueryDeclarationEnvelopeTerminalError::Deferred(value) => declaration_outcome(
            ForgeQueryContributionComposedOrchestrationCheckedKind::DeclarationDenied,
            ForgeQueryContributionComposedStop::DeclarationDenied,
            ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
            value.reason(),
            linked_artifacts_for_envelope(value.envelope()),
            None,
            None,
            None,
        ),
        ForgeQueryDeclarationEnvelopeTerminalError::Denied(value) => declaration_outcome(
            ForgeQueryContributionComposedOrchestrationCheckedKind::DeclarationDenied,
            ForgeQueryContributionComposedStop::DeclarationDenied,
            ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
            value.reason(),
            linked_artifacts_for_envelope(value.envelope()),
            None,
            None,
            None,
        ),
        ForgeQueryDeclarationEnvelopeTerminalError::Failed(value) => declaration_outcome(
            ForgeQueryContributionComposedOrchestrationCheckedKind::Failed,
            ForgeQueryContributionComposedStop::Failed,
            ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
            value.reason(),
            linked_artifacts_for_envelope(value.envelope()),
            None,
            None,
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
        ForgeQueryContributionComposedOrchestrationOutcome::Bound(value) => {
            Some(value.composition_for_reporting().to_string())
        }
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

pub(super) fn declaration_outcome<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    kind: ForgeQueryContributionComposedOrchestrationCheckedKind,
    stop: ForgeQueryContributionComposedStop,
    stop_stage: ForgeQueryDeclarationEntryOrchestrationStage,
    reason: impl Into<String>,
    linked_artifacts: ForgeQueryBindingLinkedArtifacts,
    contribution_digest: Option<String>,
    declaration_aspect_record: Option<ForgeQueryContributionComposedDeclarationAspectRecord>,
    primary_intent_descriptor: Option<ForgeQueryContributionComposedIntentRequestDescriptor>,
) -> ForgeQueryContributionComposedOrchestrationOutcome<D, I> {
    composed_outcome(
        kind,
        stop,
        stop_stage,
        reason,
        linked_artifacts,
        contribution_digest,
        declaration_aspect_record,
        primary_intent_descriptor,
    )
}

pub(super) fn composed_outcome<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    kind: ForgeQueryContributionComposedOrchestrationCheckedKind,
    stop: ForgeQueryContributionComposedStop,
    stop_stage: ForgeQueryDeclarationEntryOrchestrationStage,
    reason: impl Into<String>,
    linked_artifacts: ForgeQueryBindingLinkedArtifacts,
    contribution_digest: Option<String>,
    declaration_aspect_record: Option<ForgeQueryContributionComposedDeclarationAspectRecord>,
    primary_intent_descriptor: Option<ForgeQueryContributionComposedIntentRequestDescriptor>,
) -> ForgeQueryContributionComposedOrchestrationOutcome<D, I> {
    let posture = ForgeQueryContributionComposedOrchestrationPosture::new(
        kind,
        stop,
        stop_stage,
        reason,
        linked_artifacts,
        contribution_digest,
        declaration_aspect_record,
        primary_intent_descriptor,
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
        ForgeQueryDeclarationAdmissionError::Deferred(_) => declaration_outcome(
            ForgeQueryContributionComposedOrchestrationCheckedKind::Deferred,
            ForgeQueryContributionComposedStop::Deferred,
            ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
            "declaration admission is deferred for this handle",
            ForgeQueryBindingLinkedArtifacts::new(),
            None,
            None,
            None,
        ),
        ForgeQueryDeclarationAdmissionError::AsyncDeferred(_) => declaration_outcome(
            ForgeQueryContributionComposedOrchestrationCheckedKind::Deferred,
            ForgeQueryContributionComposedStop::Deferred,
            ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
            "async declaration support is deferred for this family",
            ForgeQueryBindingLinkedArtifacts::new(),
            None,
            None,
            None,
        ),
        ForgeQueryDeclarationAdmissionError::TemporalDeferred(_) => declaration_outcome(
            ForgeQueryContributionComposedOrchestrationCheckedKind::Deferred,
            ForgeQueryContributionComposedStop::Deferred,
            ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
            "temporal declaration support is deferred for this family",
            ForgeQueryBindingLinkedArtifacts::new(),
            None,
            None,
            None,
        ),
        ForgeQueryDeclarationAdmissionError::Unsupported(_) => declaration_outcome(
            ForgeQueryContributionComposedOrchestrationCheckedKind::Unsupported,
            ForgeQueryContributionComposedStop::Unsupported,
            ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
            "declaration family is unsupported for contribution-composed orchestration",
            ForgeQueryBindingLinkedArtifacts::new(),
            None,
            None,
            None,
        ),
        ForgeQueryDeclarationAdmissionError::AsyncUnsupported(_) => declaration_outcome(
            ForgeQueryContributionComposedOrchestrationCheckedKind::Unsupported,
            ForgeQueryContributionComposedStop::Unsupported,
            ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
            "async declaration clauses are unsupported for this family",
            ForgeQueryBindingLinkedArtifacts::new(),
            None,
            None,
            None,
        ),
        ForgeQueryDeclarationAdmissionError::TemporalUnsupported(_) => declaration_outcome(
            ForgeQueryContributionComposedOrchestrationCheckedKind::Unsupported,
            ForgeQueryContributionComposedStop::Unsupported,
            ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
            "temporal declaration clauses are unsupported for this family",
            ForgeQueryBindingLinkedArtifacts::new(),
            None,
            None,
            None,
        ),
        ForgeQueryDeclarationAdmissionError::InvalidContext(_) => declaration_outcome(
            ForgeQueryContributionComposedOrchestrationCheckedKind::Unsupported,
            ForgeQueryContributionComposedStop::Unsupported,
            ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
            "declaration family is invalid in the current admitted context",
            ForgeQueryBindingLinkedArtifacts::new(),
            None,
            None,
            None,
        ),
        ForgeQueryDeclarationAdmissionError::Canonicalization(value) => declaration_outcome(
            ForgeQueryContributionComposedOrchestrationCheckedKind::Failed,
            ForgeQueryContributionComposedStop::Failed,
            ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
            format!("{value:?}"),
            ForgeQueryBindingLinkedArtifacts::new(),
            None,
            None,
            None,
        ),
    }
}
