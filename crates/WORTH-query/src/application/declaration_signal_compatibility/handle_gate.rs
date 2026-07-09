use crate::application::{
    WorthQueryDeclarationEnvelope, WorthQueryDeclarationInput, WorthQueryDomainEntryMarker,
};

use super::checked::WorthQueryDeclarationSignalCompatibilityInput;

pub(super) fn envelope_matches_handle<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    handle_identity_digest: &str,
    operating_context_identity_digest: &str,
    envelope: &WorthQueryDeclarationEnvelope<D, I>,
) -> bool {
    envelope.handle_identity_digest() == handle_identity_digest
        && envelope.operating_context_identity_digest() == operating_context_identity_digest
}

pub(super) fn subject_matches_handle<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    handle_identity_digest: &str,
    operating_context_identity_digest: &str,
    input: &WorthQueryDeclarationSignalCompatibilityInput<D, I>,
) -> bool {
    let envelope = match input {
        WorthQueryDeclarationSignalCompatibilityInput::Enveloped(envelope) => envelope,
        WorthQueryDeclarationSignalCompatibilityInput::Deferred(envelope) => envelope.envelope(),
        WorthQueryDeclarationSignalCompatibilityInput::Denied(envelope) => envelope.envelope(),
        WorthQueryDeclarationSignalCompatibilityInput::Failed(envelope) => envelope.envelope(),
        WorthQueryDeclarationSignalCompatibilityInput::EnvelopeChecked(_) => {
            unreachable!("checked inputs are lowered before subject matching")
        }
    };
    envelope_matches_handle(
        handle_identity_digest,
        operating_context_identity_digest,
        envelope,
    )
}
