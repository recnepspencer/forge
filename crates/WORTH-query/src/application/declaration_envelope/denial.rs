use crate::application::{
    WorthQueryDeclarationInput, WorthQueryDeclarationReceiptDenialCause,
    WorthQueryDeclarationRoutePlanDenialCause, WorthQueryDomainEntryMarker,
};

use super::artifact::WorthQueryDeclarationEnvelope;

macro_rules! define_envelope_terminal {
    ($name:ident) => {
        pub struct $name<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>> {
            envelope: WorthQueryDeclarationEnvelope<D, I>,
            reason: &'static str,
        }

        impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>> $name<D, I> {
            pub(crate) fn new(
                envelope: WorthQueryDeclarationEnvelope<D, I>,
                reason: &'static str,
            ) -> Self {
                Self { envelope, reason }
            }

            pub fn envelope(&self) -> &WorthQueryDeclarationEnvelope<D, I> {
                &self.envelope
            }

            pub fn reason(&self) -> &'static str {
                self.reason
            }

            pub(crate) fn into_envelope(self) -> WorthQueryDeclarationEnvelope<D, I> {
                self.envelope
            }
        }
    };
}

define_envelope_terminal!(WorthQueryDeclarationEnvelopeDeferred);
define_envelope_terminal!(WorthQueryDeclarationEnvelopeFailed);

pub struct WorthQueryDeclarationEnvelopeDenied<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    envelope: WorthQueryDeclarationEnvelope<D, I>,
    route_cause: Option<WorthQueryDeclarationRoutePlanDenialCause>,
    receipt_cause: Option<WorthQueryDeclarationReceiptDenialCause>,
    reason: &'static str,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryDeclarationEnvelopeDenied<D, I>
{
    pub(crate) fn new(
        envelope: WorthQueryDeclarationEnvelope<D, I>,
        route_cause: Option<WorthQueryDeclarationRoutePlanDenialCause>,
        receipt_cause: Option<WorthQueryDeclarationReceiptDenialCause>,
        reason: &'static str,
    ) -> Self {
        Self {
            envelope,
            route_cause,
            receipt_cause,
            reason,
        }
    }

    pub fn envelope(&self) -> &WorthQueryDeclarationEnvelope<D, I> {
        &self.envelope
    }

    pub fn route_cause(&self) -> Option<WorthQueryDeclarationRoutePlanDenialCause> {
        self.route_cause
    }

    pub fn receipt_cause(&self) -> Option<WorthQueryDeclarationReceiptDenialCause> {
        self.receipt_cause
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }

    pub(crate) fn into_envelope(self) -> WorthQueryDeclarationEnvelope<D, I> {
        self.envelope
    }
}

pub enum WorthQueryDeclarationEnvelopeTerminalError<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    Deferred(WorthQueryDeclarationEnvelopeDeferred<D, I>),
    Denied(WorthQueryDeclarationEnvelopeDenied<D, I>),
    Failed(WorthQueryDeclarationEnvelopeFailed<D, I>),
}

pub enum WorthQueryDeclarationEntryEnvelopeError<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    Entry(crate::application::WorthQueryDeclarationEntryReceiptError<D, I>),
    Envelope(WorthQueryDeclarationEnvelopeTerminalError<D, I>),
}
