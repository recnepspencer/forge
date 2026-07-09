use crate::application::{
    WorthQueryAdmittedConfiguredDomainHandle, WorthQueryDeclarationInput,
    WorthQueryDeclarationSignalCompatibilityChecked, WorthQueryDeclarationSignalCompatibilityInput,
    WorthQueryDomainEntryMarker, WorthQueryDomainOperatingContext,
};
use crate::binding_pipeline::WorthQueryBindingLinkedArtifacts;
use crate::identity::hash_parts;
use worth_foundational::facade::CanonicalDerivedDigest;

use super::WorthQuerySignalCompatibilityOrchestrationOutcome;

pub(super) fn checked_and_linked_from_subject<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D>,
>(
    handle: &WorthQueryAdmittedConfiguredDomainHandle<D, C>,
    subject: WorthQueryDeclarationSignalCompatibilityInput<D, I>,
) -> (
    WorthQueryDeclarationSignalCompatibilityChecked<D, I>,
    WorthQueryBindingLinkedArtifacts,
) {
    let linked = linked_from_signal_subject(&subject);
    (handle.signal_compatibility_checked(subject), linked)
}

pub(super) fn handle_alignment_outcome<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D>,
>(
    handle: &WorthQueryAdmittedConfiguredDomainHandle<D, C>,
    subject: &WorthQueryDeclarationSignalCompatibilityInput<D, I>,
) -> Option<WorthQuerySignalCompatibilityOrchestrationOutcome<D, I>> {
    let envelope = subject_envelope(subject);
    if envelope.operating_context_identity_digest() != handle.operating_context_identity_digest() {
        Some(
            WorthQuerySignalCompatibilityOrchestrationOutcome::WrongWorld(
                "the retained signal subject was admitted in a different operating context"
                    .to_string(),
            ),
        )
    } else if envelope.handle_identity_digest() != handle.handle_identity_digest() {
        Some(
            WorthQuerySignalCompatibilityOrchestrationOutcome::WrongHandle(
                "the retained signal subject was admitted on a different configured domain handle"
                    .to_string(),
            ),
        )
    } else {
        None
    }
}

pub(super) fn linked_from_orchestration_subject<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    subject: &WorthQueryDeclarationSignalCompatibilityInput<D, I>,
) -> WorthQueryBindingLinkedArtifacts {
    linked_from_signal_subject(subject)
}

pub(super) fn orchestration_digest(
    kind: &str,
    linked: &WorthQueryBindingLinkedArtifacts,
    outcome: &str,
) -> String {
    hash_parts(&[
        "worth_query_signal_compatibility_orchestration_v1".to_string(),
        kind.to_string(),
        format!("outcome:{outcome}"),
        format!("linked:{linked:?}"),
    ])
}

fn linked_from_signal_subject<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>(
    subject: &WorthQueryDeclarationSignalCompatibilityInput<D, I>,
) -> WorthQueryBindingLinkedArtifacts {
    linked_from_envelope(
        subject_envelope(subject),
        WorthQueryBindingLinkedArtifacts::new(),
    )
}

fn subject_envelope<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>(
    subject: &WorthQueryDeclarationSignalCompatibilityInput<D, I>,
) -> &crate::application::WorthQueryDeclarationEnvelope<D, I> {
    match subject {
        WorthQueryDeclarationSignalCompatibilityInput::Enveloped(envelope) => envelope,
        WorthQueryDeclarationSignalCompatibilityInput::Deferred(envelope) => envelope.envelope(),
        WorthQueryDeclarationSignalCompatibilityInput::Denied(envelope) => envelope.envelope(),
        WorthQueryDeclarationSignalCompatibilityInput::Failed(envelope) => envelope.envelope(),
        WorthQueryDeclarationSignalCompatibilityInput::EnvelopeChecked(checked) => match checked {
            crate::application::WorthQueryDeclarationEnvelopeChecked::Enveloped(envelope) => {
                envelope
            }
            crate::application::WorthQueryDeclarationEnvelopeChecked::Deferred(envelope) => {
                envelope.envelope()
            }
            crate::application::WorthQueryDeclarationEnvelopeChecked::Denied(envelope) => {
                envelope.envelope()
            }
            crate::application::WorthQueryDeclarationEnvelopeChecked::Failed(envelope) => {
                envelope.envelope()
            }
        },
    }
}

fn linked_from_envelope<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>(
    envelope: &crate::application::WorthQueryDeclarationEnvelope<D, I>,
    mut linked: WorthQueryBindingLinkedArtifacts,
) -> WorthQueryBindingLinkedArtifacts {
    linked = linked
        .with_declaration_digest(envelope.declaration_digest())
        .with_receipt_digest(canonical_digest_token(envelope.receipt_digest()))
        .with_envelope_digest(canonical_digest_token(envelope.envelope_digest()));
    if let Some(progression_digest) = envelope.progression_digest() {
        linked = linked.with_progression_digest(progression_digest);
    }
    if let Some(route_plan_digest) = envelope.route_plan_digest() {
        linked = linked.with_route_plan_digest(route_plan_digest);
    }
    linked
}

fn canonical_digest_token(digest: &CanonicalDerivedDigest) -> String {
    let hex = digest
        .value()
        .bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    format!("{}:{hex}", digest.metadata().algorithm().id().as_str())
}
