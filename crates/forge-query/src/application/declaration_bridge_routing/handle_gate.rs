use crate::application::{
    ForgeQueryDeclarationEnvelope, ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
};

use super::checked::ForgeQueryDeclarationBridgeRoutingInput;

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
    input: &ForgeQueryDeclarationBridgeRoutingInput<D, I>,
) -> bool {
    let envelope = match input {
        ForgeQueryDeclarationBridgeRoutingInput::Enveloped(envelope) => envelope,
        ForgeQueryDeclarationBridgeRoutingInput::Deferred(envelope) => envelope.envelope(),
        ForgeQueryDeclarationBridgeRoutingInput::Denied(envelope) => envelope.envelope(),
        ForgeQueryDeclarationBridgeRoutingInput::Failed(envelope) => envelope.envelope(),
        ForgeQueryDeclarationBridgeRoutingInput::EnvelopeChecked(_) => {
            unreachable!("checked input is lowered before handle matching")
        }
    };
    envelope_matches_handle(
        handle_identity_digest,
        operating_context_identity_digest,
        envelope,
    )
}
