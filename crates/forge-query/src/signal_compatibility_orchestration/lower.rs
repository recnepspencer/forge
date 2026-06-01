use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationSignalCompatibilityChecked, ForgeQueryDeclarationSignalCompatibilityInput,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
};
use crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts;
use crate::identity::hash_parts;
use forge_foundational::facade::CanonicalDerivedDigest;

use super::ForgeQuerySignalCompatibilityOrchestrationOutcome;

pub(super) fn checked_and_linked_from_subject<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    subject: ForgeQueryDeclarationSignalCompatibilityInput<D, I>,
) -> (
    ForgeQueryDeclarationSignalCompatibilityChecked<D, I>,
    ForgeQueryBindingLinkedArtifacts,
) {
    let linked = linked_from_signal_subject(&subject);
    (handle.signal_compatibility_checked(subject), linked)
}

pub(super) fn handle_alignment_outcome<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    subject: &ForgeQueryDeclarationSignalCompatibilityInput<D, I>,
) -> Option<ForgeQuerySignalCompatibilityOrchestrationOutcome<D, I>> {
    let envelope = subject_envelope(subject);
    if envelope.operating_context_identity_digest() != handle.operating_context_identity_digest() {
        Some(
            ForgeQuerySignalCompatibilityOrchestrationOutcome::WrongWorld(
                "the retained signal subject was admitted in a different operating context"
                    .to_string(),
            ),
        )
    } else if envelope.handle_identity_digest() != handle.handle_identity_digest() {
        Some(
            ForgeQuerySignalCompatibilityOrchestrationOutcome::WrongHandle(
                "the retained signal subject was admitted on a different configured domain handle"
                    .to_string(),
            ),
        )
    } else {
        None
    }
}

pub(super) fn linked_from_orchestration_subject<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    subject: &ForgeQueryDeclarationSignalCompatibilityInput<D, I>,
) -> ForgeQueryBindingLinkedArtifacts {
    linked_from_signal_subject(subject)
}

pub(super) fn orchestration_digest(
    kind: &str,
    linked: &ForgeQueryBindingLinkedArtifacts,
    outcome: &str,
) -> String {
    hash_parts(&[
        "forge_query_signal_compatibility_orchestration_v1".to_string(),
        kind.to_string(),
        format!("outcome:{outcome}"),
        format!("linked:{linked:?}"),
    ])
}

fn linked_from_signal_subject<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    subject: &ForgeQueryDeclarationSignalCompatibilityInput<D, I>,
) -> ForgeQueryBindingLinkedArtifacts {
    linked_from_envelope(
        subject_envelope(subject),
        ForgeQueryBindingLinkedArtifacts::new(),
    )
}

fn subject_envelope<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    subject: &ForgeQueryDeclarationSignalCompatibilityInput<D, I>,
) -> &crate::application::ForgeQueryDeclarationEnvelope<D, I> {
    match subject {
        ForgeQueryDeclarationSignalCompatibilityInput::Enveloped(envelope) => envelope,
        ForgeQueryDeclarationSignalCompatibilityInput::Deferred(envelope) => envelope.envelope(),
        ForgeQueryDeclarationSignalCompatibilityInput::Denied(envelope) => envelope.envelope(),
        ForgeQueryDeclarationSignalCompatibilityInput::Failed(envelope) => envelope.envelope(),
        ForgeQueryDeclarationSignalCompatibilityInput::EnvelopeChecked(checked) => match checked {
            crate::application::ForgeQueryDeclarationEnvelopeChecked::Enveloped(envelope) => {
                envelope
            }
            crate::application::ForgeQueryDeclarationEnvelopeChecked::Deferred(envelope) => {
                envelope.envelope()
            }
            crate::application::ForgeQueryDeclarationEnvelopeChecked::Denied(envelope) => {
                envelope.envelope()
            }
            crate::application::ForgeQueryDeclarationEnvelopeChecked::Failed(envelope) => {
                envelope.envelope()
            }
        },
    }
}

fn linked_from_envelope<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    envelope: &crate::application::ForgeQueryDeclarationEnvelope<D, I>,
    mut linked: ForgeQueryBindingLinkedArtifacts,
) -> ForgeQueryBindingLinkedArtifacts {
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
