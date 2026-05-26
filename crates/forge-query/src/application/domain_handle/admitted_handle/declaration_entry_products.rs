use crate::application::{
    forge_query_checked_declaration_envelope_orchestration_from_progressed_on_handle,
    forge_query_checked_declaration_receipt_orchestration_from_progressed_on_handle,
    forge_query_checked_declaration_route_orchestration_from_progressed_on_handle,
    forge_query_declaration_envelope_orchestration_from_progressed_on_handle,
    forge_query_declaration_envelope_orchestration_from_progressed_proof_on_handle,
    forge_query_declaration_receipt_orchestration_from_progressed_on_handle,
    forge_query_declaration_receipt_orchestration_from_progressed_proof_on_handle,
    forge_query_declaration_route_orchestration_from_progressed_on_handle,
    forge_query_declaration_route_orchestration_from_progressed_proof_on_handle,
    ForgeQueryAdmittedDeclarationProgression, ForgeQueryDeclarationEnvelope,
    ForgeQueryDeclarationEnvelopeChecked, ForgeQueryDeclarationEnvelopeOrchestrationTranscript,
    ForgeQueryDeclarationEnvelopeTerminalError, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationReceipt, ForgeQueryDeclarationReceiptChecked,
    ForgeQueryDeclarationReceiptOrchestrationTranscript, ForgeQueryDeclarationReceiptTerminalError,
    ForgeQueryDeclarationRouteIntent, ForgeQueryDeclarationRouteOrchestrationTranscript,
    ForgeQueryDeclarationRoutePlan, ForgeQueryDeclarationRoutePlanChecked,
    ForgeQueryDeclarationRoutePlanTerminalError, ForgeQueryDomainEntryMarker,
};

use super::ForgeQueryAdmittedConfiguredDomainHandle;
use crate::application::ForgeQueryDomainOperatingContext;

impl<D: ForgeQueryDomainEntryMarker, C: ForgeQueryDomainOperatingContext<D>>
    ForgeQueryAdmittedConfiguredDomainHandle<D, C>
{
    pub fn orchestrate_routes_from_progressed<I>(
        &self,
        progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
    ) -> Result<
        ForgeQueryDeclarationRoutePlan<D, I>,
        ForgeQueryDeclarationRoutePlanTerminalError<D, I>,
    >
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        forge_query_declaration_route_orchestration_from_progressed_on_handle(
            self, progressed, None,
        )
    }

    pub fn orchestrate_routes_from_progressed_with_intent<I>(
        &self,
        progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
        intent: ForgeQueryDeclarationRouteIntent,
    ) -> Result<
        ForgeQueryDeclarationRoutePlan<D, I>,
        ForgeQueryDeclarationRoutePlanTerminalError<D, I>,
    >
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        forge_query_declaration_route_orchestration_from_progressed_on_handle(
            self,
            progressed,
            Some(intent),
        )
    }

    pub fn orchestrate_routes_from_progressed_checked<I>(
        &self,
        progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
    ) -> ForgeQueryDeclarationRoutePlanChecked<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        forge_query_checked_declaration_route_orchestration_from_progressed_on_handle(
            self, progressed, None,
        )
    }

    pub fn orchestrate_routes_from_progressed_checked_with_intent<I>(
        &self,
        progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
        intent: ForgeQueryDeclarationRouteIntent,
    ) -> ForgeQueryDeclarationRoutePlanChecked<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        forge_query_checked_declaration_route_orchestration_from_progressed_on_handle(
            self,
            progressed,
            Some(intent),
        )
    }

    pub fn orchestrate_routes_from_progressed_proof<I>(
        &self,
        progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
    ) -> ForgeQueryDeclarationRouteOrchestrationTranscript<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        forge_query_declaration_route_orchestration_from_progressed_proof_on_handle(
            self, progressed, None,
        )
    }

    pub fn orchestrate_routes_from_progressed_proof_with_intent<I>(
        &self,
        progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
        intent: ForgeQueryDeclarationRouteIntent,
    ) -> ForgeQueryDeclarationRouteOrchestrationTranscript<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        forge_query_declaration_route_orchestration_from_progressed_proof_on_handle(
            self,
            progressed,
            Some(intent),
        )
    }

    pub fn orchestrate_receipt_from_progressed<I>(
        &self,
        progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
    ) -> Result<ForgeQueryDeclarationReceipt<D, I>, ForgeQueryDeclarationReceiptTerminalError<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        forge_query_declaration_receipt_orchestration_from_progressed_on_handle(
            self, progressed, None,
        )
    }

    pub fn orchestrate_receipt_from_progressed_with_intent<I>(
        &self,
        progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
        intent: ForgeQueryDeclarationRouteIntent,
    ) -> Result<ForgeQueryDeclarationReceipt<D, I>, ForgeQueryDeclarationReceiptTerminalError<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        forge_query_declaration_receipt_orchestration_from_progressed_on_handle(
            self,
            progressed,
            Some(intent),
        )
    }

    pub fn orchestrate_receipt_from_progressed_checked<I>(
        &self,
        progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
    ) -> ForgeQueryDeclarationReceiptChecked<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        forge_query_checked_declaration_receipt_orchestration_from_progressed_on_handle(
            self, progressed, None,
        )
    }

    pub fn orchestrate_receipt_from_progressed_checked_with_intent<I>(
        &self,
        progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
        intent: ForgeQueryDeclarationRouteIntent,
    ) -> ForgeQueryDeclarationReceiptChecked<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        forge_query_checked_declaration_receipt_orchestration_from_progressed_on_handle(
            self,
            progressed,
            Some(intent),
        )
    }

    pub fn orchestrate_receipt_from_progressed_proof<I>(
        &self,
        progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
    ) -> ForgeQueryDeclarationReceiptOrchestrationTranscript<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        forge_query_declaration_receipt_orchestration_from_progressed_proof_on_handle(
            self, progressed, None,
        )
    }

    pub fn orchestrate_receipt_from_progressed_proof_with_intent<I>(
        &self,
        progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
        intent: ForgeQueryDeclarationRouteIntent,
    ) -> ForgeQueryDeclarationReceiptOrchestrationTranscript<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        forge_query_declaration_receipt_orchestration_from_progressed_proof_on_handle(
            self,
            progressed,
            Some(intent),
        )
    }

    pub fn orchestrate_envelope_from_progressed<I>(
        &self,
        progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
    ) -> Result<ForgeQueryDeclarationEnvelope<D, I>, ForgeQueryDeclarationEnvelopeTerminalError<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        forge_query_declaration_envelope_orchestration_from_progressed_on_handle(
            self, progressed, None,
        )
    }

    pub fn orchestrate_envelope_from_progressed_with_intent<I>(
        &self,
        progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
        intent: ForgeQueryDeclarationRouteIntent,
    ) -> Result<ForgeQueryDeclarationEnvelope<D, I>, ForgeQueryDeclarationEnvelopeTerminalError<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        forge_query_declaration_envelope_orchestration_from_progressed_on_handle(
            self,
            progressed,
            Some(intent),
        )
    }

    pub fn orchestrate_envelope_from_progressed_checked<I>(
        &self,
        progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
    ) -> ForgeQueryDeclarationEnvelopeChecked<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        forge_query_checked_declaration_envelope_orchestration_from_progressed_on_handle(
            self, progressed, None,
        )
    }

    pub fn orchestrate_envelope_from_progressed_checked_with_intent<I>(
        &self,
        progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
        intent: ForgeQueryDeclarationRouteIntent,
    ) -> ForgeQueryDeclarationEnvelopeChecked<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        forge_query_checked_declaration_envelope_orchestration_from_progressed_on_handle(
            self,
            progressed,
            Some(intent),
        )
    }

    pub fn orchestrate_envelope_from_progressed_proof<I>(
        &self,
        progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
    ) -> ForgeQueryDeclarationEnvelopeOrchestrationTranscript<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        forge_query_declaration_envelope_orchestration_from_progressed_proof_on_handle(
            self, progressed, None,
        )
    }

    pub fn orchestrate_envelope_from_progressed_proof_with_intent<I>(
        &self,
        progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
        intent: ForgeQueryDeclarationRouteIntent,
    ) -> ForgeQueryDeclarationEnvelopeOrchestrationTranscript<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        forge_query_declaration_envelope_orchestration_from_progressed_proof_on_handle(
            self,
            progressed,
            Some(intent),
        )
    }
}
