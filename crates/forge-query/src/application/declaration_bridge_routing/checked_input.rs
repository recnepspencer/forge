use crate::application::{
    ForgeQueryDeclarationEnvelopeChecked, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
};

use super::{
    checked::{ForgeQueryDeclarationBridgeRoutingChecked, ForgeQueryDeclarationBridgeRoutingInput},
    denial::{
        ForgeQueryDeclarationBridgeRoutingDenialCause, ForgeQueryDeclarationBridgeRoutingDenied,
    },
};

pub(super) fn lower_checked_input<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    checked: ForgeQueryDeclarationEnvelopeChecked<D, I>,
) -> ForgeQueryDeclarationBridgeRoutingInput<D, I> {
    match checked {
        ForgeQueryDeclarationEnvelopeChecked::Enveloped(envelope) => {
            ForgeQueryDeclarationBridgeRoutingInput::enveloped(envelope)
        }
        ForgeQueryDeclarationEnvelopeChecked::Deferred(envelope) => {
            ForgeQueryDeclarationBridgeRoutingInput::deferred(envelope)
        }
        ForgeQueryDeclarationEnvelopeChecked::Denied(envelope) => {
            ForgeQueryDeclarationBridgeRoutingInput::denied(envelope)
        }
        ForgeQueryDeclarationEnvelopeChecked::Failed(envelope) => {
            ForgeQueryDeclarationBridgeRoutingInput::failed(envelope)
        }
    }
}

pub(super) fn deny_non_success_mismatch<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    input: ForgeQueryDeclarationBridgeRoutingInput<D, I>,
) -> ForgeQueryDeclarationBridgeRoutingChecked<D, I> {
    match input {
        ForgeQueryDeclarationBridgeRoutingInput::Deferred(envelope) => {
            ForgeQueryDeclarationBridgeRoutingChecked::Denied(
                ForgeQueryDeclarationBridgeRoutingDenied::new(
                    envelope.into_envelope(),
                    I::Family::bridge_continuation_contract().map(|contract| contract.request()),
                    I::Family::bridge_continuation_contract().map(|contract| contract.family()),
                    ForgeQueryDeclarationBridgeRoutingDenialCause::BridgeEnvelopeMismatch,
                ),
            )
        }
        ForgeQueryDeclarationBridgeRoutingInput::Denied(envelope) => {
            ForgeQueryDeclarationBridgeRoutingChecked::Denied(
                ForgeQueryDeclarationBridgeRoutingDenied::new(
                    envelope.into_envelope(),
                    I::Family::bridge_continuation_contract().map(|contract| contract.request()),
                    I::Family::bridge_continuation_contract().map(|contract| contract.family()),
                    ForgeQueryDeclarationBridgeRoutingDenialCause::BridgeEnvelopeMismatch,
                ),
            )
        }
        ForgeQueryDeclarationBridgeRoutingInput::Failed(envelope) => {
            ForgeQueryDeclarationBridgeRoutingChecked::Denied(
                ForgeQueryDeclarationBridgeRoutingDenied::new(
                    envelope.into_envelope(),
                    I::Family::bridge_continuation_contract().map(|contract| contract.request()),
                    I::Family::bridge_continuation_contract().map(|contract| contract.family()),
                    ForgeQueryDeclarationBridgeRoutingDenialCause::BridgeEnvelopeMismatch,
                ),
            )
        }
        ForgeQueryDeclarationBridgeRoutingInput::Enveloped(_)
        | ForgeQueryDeclarationBridgeRoutingInput::EnvelopeChecked(_) => {
            unreachable!("covered envelopes use the covered-handle path")
        }
    }
}
