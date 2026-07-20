use crate::application::{
    worth_query_checked_declaration_envelope,
    worth_query_declaration_envelope_terminal_from_receipt_terminal,
    WorthQueryAdmittedDeclarationProgression, WorthQueryDeclarationEnvelope,
    WorthQueryDeclarationEnvelopeChecked, WorthQueryDeclarationEnvelopeInput,
    WorthQueryDeclarationEnvelopeTerminalError, WorthQueryDeclarationInput,
    WorthQueryDeclarationRouteIntent, WorthQueryDomainEntryMarker,
};

use super::WorthQueryInstalledDomainDeclarationContext;
use crate::application::WorthQueryDomainOperatingContext;

impl<D: WorthQueryDomainEntryMarker, C: WorthQueryDomainOperatingContext<D>>
    WorthQueryInstalledDomainDeclarationContext<D, C>
{
    pub fn envelope_routes<I>(
        &self,
        subject: WorthQueryDeclarationEnvelopeInput<D, I>,
    ) -> Result<WorthQueryDeclarationEnvelope<D, I>, WorthQueryDeclarationEnvelopeTerminalError<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        match self.envelope_routes_checked(subject) {
            WorthQueryDeclarationEnvelopeChecked::Enveloped(envelope) => Ok(envelope),
            WorthQueryDeclarationEnvelopeChecked::Deferred(envelope) => Err(
                WorthQueryDeclarationEnvelopeTerminalError::Deferred(envelope),
            ),
            WorthQueryDeclarationEnvelopeChecked::Denied(envelope) => {
                Err(WorthQueryDeclarationEnvelopeTerminalError::Denied(envelope))
            }
            WorthQueryDeclarationEnvelopeChecked::Failed(envelope) => {
                Err(WorthQueryDeclarationEnvelopeTerminalError::Failed(envelope))
            }
        }
    }

    pub fn envelope_routes_checked<I>(
        &self,
        subject: WorthQueryDeclarationEnvelopeInput<D, I>,
    ) -> WorthQueryDeclarationEnvelopeChecked<D, I>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        worth_query_checked_declaration_envelope(subject)
    }

    pub fn envelope_routes_from_progressed<I>(
        &self,
        progressed: WorthQueryAdmittedDeclarationProgression<D, I>,
    ) -> Result<WorthQueryDeclarationEnvelope<D, I>, WorthQueryDeclarationEnvelopeTerminalError<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        let receipt = self
            .receipt_routes_from_progressed(progressed)
            .map_err(receipt_to_envelope_terminal)?;
        self.envelope_routes(WorthQueryDeclarationEnvelopeInput::issued(receipt))
    }

    pub fn envelope_routes_from_progressed_with_intent<I>(
        &self,
        progressed: WorthQueryAdmittedDeclarationProgression<D, I>,
        intent: WorthQueryDeclarationRouteIntent,
    ) -> Result<WorthQueryDeclarationEnvelope<D, I>, WorthQueryDeclarationEnvelopeTerminalError<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        let receipt = self
            .receipt_routes_from_progressed_with_intent(progressed, intent)
            .map_err(receipt_to_envelope_terminal)?;
        self.envelope_routes(WorthQueryDeclarationEnvelopeInput::issued(receipt))
    }

    pub fn declare_review_progress_describe_plan_receipt_and_envelope<I>(
        &self,
        input: I,
    ) -> Result<
        WorthQueryDeclarationEnvelope<D, I>,
        crate::application::WorthQueryDeclarationEntryEnvelopeError<D, I>,
    >
    where
        I: WorthQueryDeclarationInput<D>,
    {
        let receipt = self
            .declare_review_progress_describe_plan_and_receipt(input)
            .map_err(crate::application::WorthQueryDeclarationEntryEnvelopeError::Entry)?;
        self.envelope_routes(WorthQueryDeclarationEnvelopeInput::issued(receipt))
            .map_err(crate::application::WorthQueryDeclarationEntryEnvelopeError::Envelope)
    }
}

fn receipt_to_envelope_terminal<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    error: crate::application::WorthQueryDeclarationReceiptTerminalError<D, I>,
) -> WorthQueryDeclarationEnvelopeTerminalError<D, I> {
    worth_query_declaration_envelope_terminal_from_receipt_terminal(error)
}
