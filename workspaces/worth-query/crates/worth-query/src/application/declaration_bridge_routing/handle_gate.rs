use crate::application::{
    WorthQueryDeclarationEnvelope, WorthQueryDeclarationInput, WorthQueryDomainEntryMarker,
};

use super::checked::WorthQueryDeclarationBridgeRoutingInput;

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
    input: &WorthQueryDeclarationBridgeRoutingInput<D, I>,
) -> bool {
    let envelope = match input {
        WorthQueryDeclarationBridgeRoutingInput::Enveloped(envelope) => envelope,
        WorthQueryDeclarationBridgeRoutingInput::Deferred(envelope) => envelope.envelope(),
        WorthQueryDeclarationBridgeRoutingInput::Denied(envelope) => envelope.envelope(),
        WorthQueryDeclarationBridgeRoutingInput::Failed(envelope) => envelope.envelope(),
        WorthQueryDeclarationBridgeRoutingInput::EnvelopeChecked(_) => {
            unreachable!("checked input is lowered before handle matching")
        }
    };
    envelope_matches_handle(
        handle_identity_digest,
        operating_context_identity_digest,
        envelope,
    )
}
