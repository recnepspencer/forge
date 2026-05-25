use crate::application::{
    ForgeQueryDeclarationInput, ForgeQueryDeclarationReceiptDenialCause,
    ForgeQueryDeclarationRoutePlanDenialCause, ForgeQueryDomainEntryMarker,
};

use super::artifact::ForgeQueryDeclarationEnvelope;

macro_rules! define_envelope_terminal {
    ($name:ident) => {
        pub struct $name<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>> {
            envelope: ForgeQueryDeclarationEnvelope<D, I>,
            reason: &'static str,
        }

        impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>> $name<D, I> {
            pub(crate) fn new(
                envelope: ForgeQueryDeclarationEnvelope<D, I>,
                reason: &'static str,
            ) -> Self {
                Self { envelope, reason }
            }

            pub fn envelope(&self) -> &ForgeQueryDeclarationEnvelope<D, I> {
                &self.envelope
            }

            pub fn reason(&self) -> &'static str {
                self.reason
            }

            pub(crate) fn into_envelope(self) -> ForgeQueryDeclarationEnvelope<D, I> {
                self.envelope
            }
        }
    };
}

define_envelope_terminal!(ForgeQueryDeclarationEnvelopeDeferred);
define_envelope_terminal!(ForgeQueryDeclarationEnvelopeFailed);

pub struct ForgeQueryDeclarationEnvelopeDenied<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    envelope: ForgeQueryDeclarationEnvelope<D, I>,
    route_cause: Option<ForgeQueryDeclarationRoutePlanDenialCause>,
    receipt_cause: Option<ForgeQueryDeclarationReceiptDenialCause>,
    reason: &'static str,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationEnvelopeDenied<D, I>
{
    pub(crate) fn new(
        envelope: ForgeQueryDeclarationEnvelope<D, I>,
        route_cause: Option<ForgeQueryDeclarationRoutePlanDenialCause>,
        receipt_cause: Option<ForgeQueryDeclarationReceiptDenialCause>,
        reason: &'static str,
    ) -> Self {
        Self {
            envelope,
            route_cause,
            receipt_cause,
            reason,
        }
    }

    pub fn envelope(&self) -> &ForgeQueryDeclarationEnvelope<D, I> {
        &self.envelope
    }

    pub fn route_cause(&self) -> Option<ForgeQueryDeclarationRoutePlanDenialCause> {
        self.route_cause
    }

    pub fn receipt_cause(&self) -> Option<ForgeQueryDeclarationReceiptDenialCause> {
        self.receipt_cause
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }

    pub(crate) fn into_envelope(self) -> ForgeQueryDeclarationEnvelope<D, I> {
        self.envelope
    }
}

pub enum ForgeQueryDeclarationEnvelopeTerminalError<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Deferred(ForgeQueryDeclarationEnvelopeDeferred<D, I>),
    Denied(ForgeQueryDeclarationEnvelopeDenied<D, I>),
    Failed(ForgeQueryDeclarationEnvelopeFailed<D, I>),
}

pub enum ForgeQueryDeclarationEntryEnvelopeError<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Entry(crate::application::ForgeQueryDeclarationEntryReceiptError<D, I>),
    Envelope(ForgeQueryDeclarationEnvelopeTerminalError<D, I>),
}
