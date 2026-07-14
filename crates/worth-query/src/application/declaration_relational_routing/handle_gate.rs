use crate::application::{
    WorthQueryDeclarationEnvelope, WorthQueryDeclarationInput, WorthQueryDomainEntryMarker,
};

use super::input::WorthQueryDeclarationRelationalRoutingInput;

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
    input: &WorthQueryDeclarationRelationalRoutingInput<D, I>,
) -> bool {
    let envelope = match input {
        WorthQueryDeclarationRelationalRoutingInput::Enveloped(envelope) => envelope,
        WorthQueryDeclarationRelationalRoutingInput::Deferred(envelope) => envelope.envelope(),
        WorthQueryDeclarationRelationalRoutingInput::Denied(envelope) => envelope.envelope(),
        WorthQueryDeclarationRelationalRoutingInput::Failed(envelope) => envelope.envelope(),
        WorthQueryDeclarationRelationalRoutingInput::EnvelopeChecked(_) => {
            unreachable!("checked input is lowered before handle matching")
        }
    };
    envelope_matches_handle(
        handle_identity_digest,
        operating_context_identity_digest,
        envelope,
    )
}
