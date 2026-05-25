use crate::application::{
    forge_query_checked_declaration_envelope,
    forge_query_declaration_envelope_terminal_from_receipt_terminal,
    ForgeQueryAdmittedDeclarationProgression, ForgeQueryDeclarationEnvelope,
    ForgeQueryDeclarationEnvelopeChecked, ForgeQueryDeclarationEnvelopeInput,
    ForgeQueryDeclarationEnvelopeTerminalError, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationRouteIntent, ForgeQueryDomainEntryMarker,
};

use super::ForgeQueryAdmittedConfiguredDomainHandle;
use crate::application::ForgeQueryDomainOperatingContext;

impl<D: ForgeQueryDomainEntryMarker, C: ForgeQueryDomainOperatingContext<D>>
    ForgeQueryAdmittedConfiguredDomainHandle<D, C>
{
    pub fn envelope_routes<I>(
        &self,
        subject: ForgeQueryDeclarationEnvelopeInput<D, I>,
    ) -> Result<ForgeQueryDeclarationEnvelope<D, I>, ForgeQueryDeclarationEnvelopeTerminalError<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        match self.envelope_routes_checked(subject) {
            ForgeQueryDeclarationEnvelopeChecked::Enveloped(envelope) => Ok(envelope),
            ForgeQueryDeclarationEnvelopeChecked::Deferred(envelope) => Err(
                ForgeQueryDeclarationEnvelopeTerminalError::Deferred(envelope),
            ),
            ForgeQueryDeclarationEnvelopeChecked::Denied(envelope) => {
                Err(ForgeQueryDeclarationEnvelopeTerminalError::Denied(envelope))
            }
            ForgeQueryDeclarationEnvelopeChecked::Failed(envelope) => {
                Err(ForgeQueryDeclarationEnvelopeTerminalError::Failed(envelope))
            }
        }
    }

    pub fn envelope_routes_checked<I>(
        &self,
        subject: ForgeQueryDeclarationEnvelopeInput<D, I>,
    ) -> ForgeQueryDeclarationEnvelopeChecked<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        forge_query_checked_declaration_envelope(subject)
    }

    pub fn envelope_routes_from_progressed<I>(
        &self,
        progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
    ) -> Result<ForgeQueryDeclarationEnvelope<D, I>, ForgeQueryDeclarationEnvelopeTerminalError<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        let receipt = self
            .receipt_routes_from_progressed(progressed)
            .map_err(receipt_to_envelope_terminal)?;
        self.envelope_routes(ForgeQueryDeclarationEnvelopeInput::issued(receipt))
    }

    pub fn envelope_routes_from_progressed_with_intent<I>(
        &self,
        progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
        intent: ForgeQueryDeclarationRouteIntent,
    ) -> Result<ForgeQueryDeclarationEnvelope<D, I>, ForgeQueryDeclarationEnvelopeTerminalError<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        let receipt = self
            .receipt_routes_from_progressed_with_intent(progressed, intent)
            .map_err(receipt_to_envelope_terminal)?;
        self.envelope_routes(ForgeQueryDeclarationEnvelopeInput::issued(receipt))
    }

    pub fn declare_review_progress_describe_plan_receipt_and_envelope<I>(
        &self,
        input: I,
    ) -> Result<
        ForgeQueryDeclarationEnvelope<D, I>,
        crate::application::ForgeQueryDeclarationEntryEnvelopeError<D, I>,
    >
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        let receipt = self
            .declare_review_progress_describe_plan_and_receipt(input)
            .map_err(crate::application::ForgeQueryDeclarationEntryEnvelopeError::Entry)?;
        self.envelope_routes(ForgeQueryDeclarationEnvelopeInput::issued(receipt))
            .map_err(crate::application::ForgeQueryDeclarationEntryEnvelopeError::Envelope)
    }
}

fn receipt_to_envelope_terminal<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    error: crate::application::ForgeQueryDeclarationReceiptTerminalError<D, I>,
) -> ForgeQueryDeclarationEnvelopeTerminalError<D, I> {
    forge_query_declaration_envelope_terminal_from_receipt_terminal(error)
}
