use crate::application::{
    ForgeQueryDeclarationEnvelope, ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
};

use super::checked::ForgeQueryDeclarationSignalCompatibilityInput;

pub(super) fn envelope_matches_handle<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle_identity_digest: &str,
    operating_context_identity_digest: &str,
    envelope: &ForgeQueryDeclarationEnvelope<D, I>,
) -> bool {
    envelope.handle_identity_digest() == handle_identity_digest
        && envelope.operating_context_identity_digest() == operating_context_identity_digest
}

pub(super) fn subject_matches_handle<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle_identity_digest: &str,
    operating_context_identity_digest: &str,
    input: &ForgeQueryDeclarationSignalCompatibilityInput<D, I>,
) -> bool {
    let envelope = match input {
        ForgeQueryDeclarationSignalCompatibilityInput::Enveloped(envelope) => envelope,
        ForgeQueryDeclarationSignalCompatibilityInput::Deferred(envelope) => envelope.envelope(),
        ForgeQueryDeclarationSignalCompatibilityInput::Denied(envelope) => envelope.envelope(),
        ForgeQueryDeclarationSignalCompatibilityInput::Failed(envelope) => envelope.envelope(),
        ForgeQueryDeclarationSignalCompatibilityInput::EnvelopeChecked(_) => {
            unreachable!("checked inputs are lowered before subject matching")
        }
    };
    envelope_matches_handle(
        handle_identity_digest,
        operating_context_identity_digest,
        envelope,
    )
}
