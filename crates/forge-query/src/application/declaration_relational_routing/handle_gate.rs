use crate::application::{
    ForgeQueryDeclarationEnvelope, ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
};

use super::input::ForgeQueryDeclarationRelationalRoutingInput;

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
    input: &ForgeQueryDeclarationRelationalRoutingInput<D, I>,
) -> bool {
    let envelope = match input {
        ForgeQueryDeclarationRelationalRoutingInput::Enveloped(envelope) => envelope,
        ForgeQueryDeclarationRelationalRoutingInput::Deferred(envelope) => envelope.envelope(),
        ForgeQueryDeclarationRelationalRoutingInput::Denied(envelope) => envelope.envelope(),
        ForgeQueryDeclarationRelationalRoutingInput::Failed(envelope) => envelope.envelope(),
        ForgeQueryDeclarationRelationalRoutingInput::EnvelopeChecked(_) => {
            unreachable!("checked input is lowered before handle matching")
        }
    };
    envelope_matches_handle(
        handle_identity_digest,
        operating_context_identity_digest,
        envelope,
    )
}
