use crate::application::{
    WorthQueryDeclarationEnvelopeChecked, WorthQueryDeclarationFamilyMarker,
    WorthQueryDeclarationInput, WorthQueryDomainEntryMarker,
};

use super::{
    checked::{WorthQueryDeclarationBridgeRoutingChecked, WorthQueryDeclarationBridgeRoutingInput},
    denial::{
        WorthQueryDeclarationBridgeRoutingDenialCause, WorthQueryDeclarationBridgeRoutingDenied,
    },
};

pub(super) fn lower_checked_input<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    checked: WorthQueryDeclarationEnvelopeChecked<D, I>,
) -> WorthQueryDeclarationBridgeRoutingInput<D, I> {
    match checked {
        WorthQueryDeclarationEnvelopeChecked::Enveloped(envelope) => {
            WorthQueryDeclarationBridgeRoutingInput::enveloped(envelope)
        }
        WorthQueryDeclarationEnvelopeChecked::Deferred(envelope) => {
            WorthQueryDeclarationBridgeRoutingInput::deferred(envelope)
        }
        WorthQueryDeclarationEnvelopeChecked::Denied(envelope) => {
            WorthQueryDeclarationBridgeRoutingInput::denied(envelope)
        }
        WorthQueryDeclarationEnvelopeChecked::Failed(envelope) => {
            WorthQueryDeclarationBridgeRoutingInput::failed(envelope)
        }
    }
}

pub(super) fn deny_non_success_mismatch<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    input: WorthQueryDeclarationBridgeRoutingInput<D, I>,
) -> WorthQueryDeclarationBridgeRoutingChecked<D, I> {
    match input {
        WorthQueryDeclarationBridgeRoutingInput::Deferred(envelope) => {
            WorthQueryDeclarationBridgeRoutingChecked::Denied(
                WorthQueryDeclarationBridgeRoutingDenied::new(
                    envelope.into_envelope(),
                    I::Family::bridge_continuation_contract().map(|contract| contract.request()),
                    I::Family::bridge_continuation_contract().map(|contract| contract.family()),
                    WorthQueryDeclarationBridgeRoutingDenialCause::BridgeEnvelopeMismatch,
                ),
            )
        }
        WorthQueryDeclarationBridgeRoutingInput::Denied(envelope) => {
            WorthQueryDeclarationBridgeRoutingChecked::Denied(
                WorthQueryDeclarationBridgeRoutingDenied::new(
                    envelope.into_envelope(),
                    I::Family::bridge_continuation_contract().map(|contract| contract.request()),
                    I::Family::bridge_continuation_contract().map(|contract| contract.family()),
                    WorthQueryDeclarationBridgeRoutingDenialCause::BridgeEnvelopeMismatch,
                ),
            )
        }
        WorthQueryDeclarationBridgeRoutingInput::Failed(envelope) => {
            WorthQueryDeclarationBridgeRoutingChecked::Denied(
                WorthQueryDeclarationBridgeRoutingDenied::new(
                    envelope.into_envelope(),
                    I::Family::bridge_continuation_contract().map(|contract| contract.request()),
                    I::Family::bridge_continuation_contract().map(|contract| contract.family()),
                    WorthQueryDeclarationBridgeRoutingDenialCause::BridgeEnvelopeMismatch,
                ),
            )
        }
        WorthQueryDeclarationBridgeRoutingInput::Enveloped(_)
        | WorthQueryDeclarationBridgeRoutingInput::EnvelopeChecked(_) => {
            unreachable!("covered envelopes use the covered-handle path")
        }
    }
}
