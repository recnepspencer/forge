use crate::application::{
    WorthQueryDeclarationAdmissionError, WorthQueryDeclarationAdmissionOrLegalityError,
    WorthQueryDeclarationEntryOrchestrationStage, WorthQueryDeclarationEntryProgressionError,
    WorthQueryDeclarationEnvelope, WorthQueryDeclarationEnvelopeTerminalError,
    WorthQueryDeclarationInput, WorthQueryDeclarationProgressionTerminalError,
    WorthQueryDomainEntryMarker,
};
use crate::binding_pipeline::WorthQueryBindingLinkedArtifacts;

use super::composition::WorthQueryContributionComposedStop;
use super::intent_result::WorthQueryContributionComposedIntentRequestDescriptor;
use super::outcome::{
    WorthQueryContributionComposedOrchestrationCheckedKind,
    WorthQueryContributionComposedOrchestrationOutcome,
    WorthQueryContributionComposedOrchestrationPosture,
};
use super::WorthQueryContributionComposedDeclarationAspectRecord;

pub(super) fn declaration_error_outcome<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    error: WorthQueryDeclarationEntryProgressionError<D, I>,
) -> WorthQueryContributionComposedOrchestrationOutcome<D, I> {
    match error {
        WorthQueryDeclarationEntryProgressionError::Entry(value) => match value {
            WorthQueryDeclarationAdmissionOrLegalityError::Admission(admission) => {
                declaration_admission_outcome(admission)
            }
            WorthQueryDeclarationAdmissionOrLegalityError::Legality(_) => declaration_outcome(
                WorthQueryContributionComposedOrchestrationCheckedKind::DeclarationDenied,
                WorthQueryContributionComposedStop::DeclarationDenied,
                WorthQueryDeclarationEntryOrchestrationStage::LegalityEstablished,
                "declaration legality denied contribution-composed orchestration",
                WorthQueryBindingLinkedArtifacts::new(),
                None,
                None,
                None,
            ),
        },
        WorthQueryDeclarationEntryProgressionError::Progression(value) => match value {
            WorthQueryDeclarationProgressionTerminalError::Deferred(progress) => {
                declaration_outcome(
                    WorthQueryContributionComposedOrchestrationCheckedKind::Deferred,
                    WorthQueryContributionComposedStop::Deferred,
                    WorthQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                    "declaration progression remained deferred",
                    WorthQueryBindingLinkedArtifacts::new().with_declaration_digest(format!(
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
            WorthQueryDeclarationProgressionTerminalError::Denied(progress) => declaration_outcome(
                WorthQueryContributionComposedOrchestrationCheckedKind::DeclarationDenied,
                WorthQueryContributionComposedStop::DeclarationDenied,
                WorthQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                "declaration progression was denied",
                WorthQueryBindingLinkedArtifacts::new().with_declaration_digest(format!(
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
            WorthQueryDeclarationProgressionTerminalError::Stale(progress) => declaration_outcome(
                WorthQueryContributionComposedOrchestrationCheckedKind::Stale,
                WorthQueryContributionComposedStop::Stale,
                WorthQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                "declaration progression is stale",
                WorthQueryBindingLinkedArtifacts::new()
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
            WorthQueryDeclarationProgressionTerminalError::RebindRequired(progress) => {
                declaration_outcome(
                    WorthQueryContributionComposedOrchestrationCheckedKind::RebindRequired,
                    WorthQueryContributionComposedStop::RebindRequired,
                    WorthQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                    "declaration progression requires rebind",
                    WorthQueryBindingLinkedArtifacts::new()
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
            WorthQueryDeclarationProgressionTerminalError::Failed(progress) => declaration_outcome(
                WorthQueryContributionComposedOrchestrationCheckedKind::Failed,
                WorthQueryContributionComposedStop::Failed,
                WorthQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
                "declaration progression failed",
                WorthQueryBindingLinkedArtifacts::new()
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
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    error: WorthQueryDeclarationEnvelopeTerminalError<D, I>,
) -> WorthQueryContributionComposedOrchestrationOutcome<D, I> {
    match error {
        WorthQueryDeclarationEnvelopeTerminalError::Deferred(value) => declaration_outcome(
            WorthQueryContributionComposedOrchestrationCheckedKind::DeclarationDenied,
            WorthQueryContributionComposedStop::DeclarationDenied,
            WorthQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
            value.reason(),
            linked_artifacts_for_envelope(value.envelope()),
            None,
            None,
            None,
        ),
        WorthQueryDeclarationEnvelopeTerminalError::Denied(value) => declaration_outcome(
            WorthQueryContributionComposedOrchestrationCheckedKind::DeclarationDenied,
            WorthQueryContributionComposedStop::DeclarationDenied,
            WorthQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
            value.reason(),
            linked_artifacts_for_envelope(value.envelope()),
            None,
            None,
            None,
        ),
        WorthQueryDeclarationEnvelopeTerminalError::Failed(value) => declaration_outcome(
            WorthQueryContributionComposedOrchestrationCheckedKind::Failed,
            WorthQueryContributionComposedStop::Failed,
            WorthQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
            value.reason(),
            linked_artifacts_for_envelope(value.envelope()),
            None,
            None,
            None,
        ),
    }
}

pub(super) fn linked_artifacts_for_envelope<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    envelope: &WorthQueryDeclarationEnvelope<D, I>,
) -> WorthQueryBindingLinkedArtifacts {
    let mut linked = WorthQueryBindingLinkedArtifacts::new()
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
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    outcome: &WorthQueryContributionComposedOrchestrationOutcome<D, I>,
) -> WorthQueryBindingLinkedArtifacts {
    match outcome {
        WorthQueryContributionComposedOrchestrationOutcome::Bound(value) => {
            linked_artifacts_for_envelope(value.envelope())
        }
        WorthQueryContributionComposedOrchestrationOutcome::Deferred(value)
        | WorthQueryContributionComposedOrchestrationOutcome::DeclarationDenied(value)
        | WorthQueryContributionComposedOrchestrationOutcome::ContributionDenied(value)
        | WorthQueryContributionComposedOrchestrationOutcome::Stale(value)
        | WorthQueryContributionComposedOrchestrationOutcome::RebindRequired(value)
        | WorthQueryContributionComposedOrchestrationOutcome::Unsupported(value)
        | WorthQueryContributionComposedOrchestrationOutcome::Failed(value) => {
            value.linked_artifacts().clone()
        }
    }
}

pub(super) fn contribution_digest_from_outcome<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    outcome: &WorthQueryContributionComposedOrchestrationOutcome<D, I>,
) -> Option<String> {
    match outcome {
        WorthQueryContributionComposedOrchestrationOutcome::Bound(value) => {
            Some(value.composition_for_reporting().to_string())
        }
        WorthQueryContributionComposedOrchestrationOutcome::Deferred(value)
        | WorthQueryContributionComposedOrchestrationOutcome::DeclarationDenied(value)
        | WorthQueryContributionComposedOrchestrationOutcome::ContributionDenied(value)
        | WorthQueryContributionComposedOrchestrationOutcome::Stale(value)
        | WorthQueryContributionComposedOrchestrationOutcome::RebindRequired(value)
        | WorthQueryContributionComposedOrchestrationOutcome::Unsupported(value)
        | WorthQueryContributionComposedOrchestrationOutcome::Failed(value) => {
            value.contribution_digest().map(str::to_string)
        }
    }
}

pub(super) fn declaration_outcome<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    kind: WorthQueryContributionComposedOrchestrationCheckedKind,
    stop: WorthQueryContributionComposedStop,
    stop_stage: WorthQueryDeclarationEntryOrchestrationStage,
    reason: impl Into<String>,
    linked_artifacts: WorthQueryBindingLinkedArtifacts,
    contribution_digest: Option<String>,
    declaration_aspect_record: Option<WorthQueryContributionComposedDeclarationAspectRecord>,
    primary_intent_descriptor: Option<WorthQueryContributionComposedIntentRequestDescriptor>,
) -> WorthQueryContributionComposedOrchestrationOutcome<D, I> {
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

pub(super) fn composed_outcome<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>(
    kind: WorthQueryContributionComposedOrchestrationCheckedKind,
    stop: WorthQueryContributionComposedStop,
    stop_stage: WorthQueryDeclarationEntryOrchestrationStage,
    reason: impl Into<String>,
    linked_artifacts: WorthQueryBindingLinkedArtifacts,
    contribution_digest: Option<String>,
    declaration_aspect_record: Option<WorthQueryContributionComposedDeclarationAspectRecord>,
    primary_intent_descriptor: Option<WorthQueryContributionComposedIntentRequestDescriptor>,
) -> WorthQueryContributionComposedOrchestrationOutcome<D, I> {
    let posture = WorthQueryContributionComposedOrchestrationPosture::new(
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
        WorthQueryContributionComposedOrchestrationCheckedKind::Deferred => {
            WorthQueryContributionComposedOrchestrationOutcome::Deferred(posture)
        }
        WorthQueryContributionComposedOrchestrationCheckedKind::DeclarationDenied => {
            WorthQueryContributionComposedOrchestrationOutcome::DeclarationDenied(posture)
        }
        WorthQueryContributionComposedOrchestrationCheckedKind::ContributionDenied => {
            WorthQueryContributionComposedOrchestrationOutcome::ContributionDenied(posture)
        }
        WorthQueryContributionComposedOrchestrationCheckedKind::Stale => {
            WorthQueryContributionComposedOrchestrationOutcome::Stale(posture)
        }
        WorthQueryContributionComposedOrchestrationCheckedKind::RebindRequired => {
            WorthQueryContributionComposedOrchestrationOutcome::RebindRequired(posture)
        }
        WorthQueryContributionComposedOrchestrationCheckedKind::Unsupported => {
            WorthQueryContributionComposedOrchestrationOutcome::Unsupported(posture)
        }
        WorthQueryContributionComposedOrchestrationCheckedKind::Failed => {
            WorthQueryContributionComposedOrchestrationOutcome::Failed(posture)
        }
    }
}

fn declaration_admission_outcome<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    error: WorthQueryDeclarationAdmissionError<D, I>,
) -> WorthQueryContributionComposedOrchestrationOutcome<D, I> {
    match error {
        WorthQueryDeclarationAdmissionError::Deferred(_) => declaration_outcome(
            WorthQueryContributionComposedOrchestrationCheckedKind::Deferred,
            WorthQueryContributionComposedStop::Deferred,
            WorthQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
            "declaration admission is deferred for this handle",
            WorthQueryBindingLinkedArtifacts::new(),
            None,
            None,
            None,
        ),
        WorthQueryDeclarationAdmissionError::AsyncDeferred(_) => declaration_outcome(
            WorthQueryContributionComposedOrchestrationCheckedKind::Deferred,
            WorthQueryContributionComposedStop::Deferred,
            WorthQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
            "async declaration support is deferred for this family",
            WorthQueryBindingLinkedArtifacts::new(),
            None,
            None,
            None,
        ),
        WorthQueryDeclarationAdmissionError::TemporalDeferred(_) => declaration_outcome(
            WorthQueryContributionComposedOrchestrationCheckedKind::Deferred,
            WorthQueryContributionComposedStop::Deferred,
            WorthQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
            "temporal declaration support is deferred for this family",
            WorthQueryBindingLinkedArtifacts::new(),
            None,
            None,
            None,
        ),
        WorthQueryDeclarationAdmissionError::Unsupported(_) => declaration_outcome(
            WorthQueryContributionComposedOrchestrationCheckedKind::Unsupported,
            WorthQueryContributionComposedStop::Unsupported,
            WorthQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
            "declaration family is unsupported for contribution-composed orchestration",
            WorthQueryBindingLinkedArtifacts::new(),
            None,
            None,
            None,
        ),
        WorthQueryDeclarationAdmissionError::AsyncUnsupported(_) => declaration_outcome(
            WorthQueryContributionComposedOrchestrationCheckedKind::Unsupported,
            WorthQueryContributionComposedStop::Unsupported,
            WorthQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
            "async declaration clauses are unsupported for this family",
            WorthQueryBindingLinkedArtifacts::new(),
            None,
            None,
            None,
        ),
        WorthQueryDeclarationAdmissionError::TemporalUnsupported(_) => declaration_outcome(
            WorthQueryContributionComposedOrchestrationCheckedKind::Unsupported,
            WorthQueryContributionComposedStop::Unsupported,
            WorthQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
            "temporal declaration clauses are unsupported for this family",
            WorthQueryBindingLinkedArtifacts::new(),
            None,
            None,
            None,
        ),
        WorthQueryDeclarationAdmissionError::InvalidContext(_) => declaration_outcome(
            WorthQueryContributionComposedOrchestrationCheckedKind::Unsupported,
            WorthQueryContributionComposedStop::Unsupported,
            WorthQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
            "declaration family is invalid in the current admitted context",
            WorthQueryBindingLinkedArtifacts::new(),
            None,
            None,
            None,
        ),
        WorthQueryDeclarationAdmissionError::Canonicalization(value) => declaration_outcome(
            WorthQueryContributionComposedOrchestrationCheckedKind::Failed,
            WorthQueryContributionComposedStop::Failed,
            WorthQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
            format!("{value:?}"),
            WorthQueryBindingLinkedArtifacts::new(),
            None,
            None,
            None,
        ),
    }
}
