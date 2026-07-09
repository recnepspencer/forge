use crate::application::{
    worth_query_checked_declaration_envelope_orchestration_from_progressed_on_handle,
    worth_query_checked_declaration_receipt_orchestration_from_progressed_on_handle,
    worth_query_checked_declaration_route_orchestration_from_progressed_on_handle,
    worth_query_declaration_envelope_orchestration_from_progressed_on_handle,
    worth_query_declaration_envelope_orchestration_from_progressed_proof_on_handle,
    worth_query_declaration_receipt_orchestration_from_progressed_on_handle,
    worth_query_declaration_receipt_orchestration_from_progressed_proof_on_handle,
    worth_query_declaration_route_orchestration_from_progressed_on_handle,
    worth_query_declaration_route_orchestration_from_progressed_proof_on_handle,
    WorthQueryAdmittedDeclarationProgression, WorthQueryDeclarationEnvelope,
    WorthQueryDeclarationEnvelopeChecked, WorthQueryDeclarationEnvelopeOrchestrationTranscript,
    WorthQueryDeclarationEnvelopeTerminalError, WorthQueryDeclarationInput,
    WorthQueryDeclarationReceipt, WorthQueryDeclarationReceiptChecked,
    WorthQueryDeclarationReceiptOrchestrationTranscript, WorthQueryDeclarationReceiptTerminalError,
    WorthQueryDeclarationRouteIntent, WorthQueryDeclarationRouteOrchestrationTranscript,
    WorthQueryDeclarationRoutePlan, WorthQueryDeclarationRoutePlanChecked,
    WorthQueryDeclarationRoutePlanTerminalError, WorthQueryDomainEntryMarker,
};

use super::super::WorthQueryAdmittedConfiguredDomainHandle;
use crate::application::WorthQueryDomainOperatingContext;

impl<D: WorthQueryDomainEntryMarker, C: WorthQueryDomainOperatingContext<D>>
    WorthQueryAdmittedConfiguredDomainHandle<D, C>
{
    pub fn orchestrate_routes_from_progressed<I>(
        &self,
        progressed: WorthQueryAdmittedDeclarationProgression<D, I>,
    ) -> Result<
        WorthQueryDeclarationRoutePlan<D, I>,
        WorthQueryDeclarationRoutePlanTerminalError<D, I>,
    >
    where
        I: WorthQueryDeclarationInput<D>,
    {
        worth_query_declaration_route_orchestration_from_progressed_on_handle(
            self, progressed, None,
        )
    }

    pub fn orchestrate_routes_from_progressed_with_intent<I>(
        &self,
        progressed: WorthQueryAdmittedDeclarationProgression<D, I>,
        intent: WorthQueryDeclarationRouteIntent,
    ) -> Result<
        WorthQueryDeclarationRoutePlan<D, I>,
        WorthQueryDeclarationRoutePlanTerminalError<D, I>,
    >
    where
        I: WorthQueryDeclarationInput<D>,
    {
        worth_query_declaration_route_orchestration_from_progressed_on_handle(
            self,
            progressed,
            Some(intent),
        )
    }

    pub fn orchestrate_routes_from_progressed_checked<I>(
        &self,
        progressed: WorthQueryAdmittedDeclarationProgression<D, I>,
    ) -> WorthQueryDeclarationRoutePlanChecked<D, I>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        worth_query_checked_declaration_route_orchestration_from_progressed_on_handle(
            self, progressed, None,
        )
    }

    pub fn orchestrate_routes_from_progressed_checked_with_intent<I>(
        &self,
        progressed: WorthQueryAdmittedDeclarationProgression<D, I>,
        intent: WorthQueryDeclarationRouteIntent,
    ) -> WorthQueryDeclarationRoutePlanChecked<D, I>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        worth_query_checked_declaration_route_orchestration_from_progressed_on_handle(
            self,
            progressed,
            Some(intent),
        )
    }

    pub fn orchestrate_routes_from_progressed_proof<I>(
        &self,
        progressed: WorthQueryAdmittedDeclarationProgression<D, I>,
    ) -> WorthQueryDeclarationRouteOrchestrationTranscript<D, I>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        worth_query_declaration_route_orchestration_from_progressed_proof_on_handle(
            self, progressed, None,
        )
    }

    pub fn orchestrate_routes_from_progressed_proof_with_intent<I>(
        &self,
        progressed: WorthQueryAdmittedDeclarationProgression<D, I>,
        intent: WorthQueryDeclarationRouteIntent,
    ) -> WorthQueryDeclarationRouteOrchestrationTranscript<D, I>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        worth_query_declaration_route_orchestration_from_progressed_proof_on_handle(
            self,
            progressed,
            Some(intent),
        )
    }

    pub fn orchestrate_receipt_from_progressed<I>(
        &self,
        progressed: WorthQueryAdmittedDeclarationProgression<D, I>,
    ) -> Result<WorthQueryDeclarationReceipt<D, I>, WorthQueryDeclarationReceiptTerminalError<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        worth_query_declaration_receipt_orchestration_from_progressed_on_handle(
            self, progressed, None,
        )
    }

    pub fn orchestrate_receipt_from_progressed_with_intent<I>(
        &self,
        progressed: WorthQueryAdmittedDeclarationProgression<D, I>,
        intent: WorthQueryDeclarationRouteIntent,
    ) -> Result<WorthQueryDeclarationReceipt<D, I>, WorthQueryDeclarationReceiptTerminalError<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        worth_query_declaration_receipt_orchestration_from_progressed_on_handle(
            self,
            progressed,
            Some(intent),
        )
    }

    pub fn orchestrate_receipt_from_progressed_checked<I>(
        &self,
        progressed: WorthQueryAdmittedDeclarationProgression<D, I>,
    ) -> WorthQueryDeclarationReceiptChecked<D, I>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        worth_query_checked_declaration_receipt_orchestration_from_progressed_on_handle(
            self, progressed, None,
        )
    }

    pub fn orchestrate_receipt_from_progressed_checked_with_intent<I>(
        &self,
        progressed: WorthQueryAdmittedDeclarationProgression<D, I>,
        intent: WorthQueryDeclarationRouteIntent,
    ) -> WorthQueryDeclarationReceiptChecked<D, I>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        worth_query_checked_declaration_receipt_orchestration_from_progressed_on_handle(
            self,
            progressed,
            Some(intent),
        )
    }

    pub fn orchestrate_receipt_from_progressed_proof<I>(
        &self,
        progressed: WorthQueryAdmittedDeclarationProgression<D, I>,
    ) -> WorthQueryDeclarationReceiptOrchestrationTranscript<D, I>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        worth_query_declaration_receipt_orchestration_from_progressed_proof_on_handle(
            self, progressed, None,
        )
    }

    pub fn orchestrate_receipt_from_progressed_proof_with_intent<I>(
        &self,
        progressed: WorthQueryAdmittedDeclarationProgression<D, I>,
        intent: WorthQueryDeclarationRouteIntent,
    ) -> WorthQueryDeclarationReceiptOrchestrationTranscript<D, I>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        worth_query_declaration_receipt_orchestration_from_progressed_proof_on_handle(
            self,
            progressed,
            Some(intent),
        )
    }

    pub fn orchestrate_envelope_from_progressed<I>(
        &self,
        progressed: WorthQueryAdmittedDeclarationProgression<D, I>,
    ) -> Result<WorthQueryDeclarationEnvelope<D, I>, WorthQueryDeclarationEnvelopeTerminalError<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        worth_query_declaration_envelope_orchestration_from_progressed_on_handle(
            self, progressed, None,
        )
    }

    pub fn orchestrate_envelope_from_progressed_with_intent<I>(
        &self,
        progressed: WorthQueryAdmittedDeclarationProgression<D, I>,
        intent: WorthQueryDeclarationRouteIntent,
    ) -> Result<WorthQueryDeclarationEnvelope<D, I>, WorthQueryDeclarationEnvelopeTerminalError<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        worth_query_declaration_envelope_orchestration_from_progressed_on_handle(
            self,
            progressed,
            Some(intent),
        )
    }

    pub fn orchestrate_envelope_from_progressed_checked<I>(
        &self,
        progressed: WorthQueryAdmittedDeclarationProgression<D, I>,
    ) -> WorthQueryDeclarationEnvelopeChecked<D, I>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        worth_query_checked_declaration_envelope_orchestration_from_progressed_on_handle(
            self, progressed, None,
        )
    }

    pub fn orchestrate_envelope_from_progressed_checked_with_intent<I>(
        &self,
        progressed: WorthQueryAdmittedDeclarationProgression<D, I>,
        intent: WorthQueryDeclarationRouteIntent,
    ) -> WorthQueryDeclarationEnvelopeChecked<D, I>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        worth_query_checked_declaration_envelope_orchestration_from_progressed_on_handle(
            self,
            progressed,
            Some(intent),
        )
    }

    pub fn orchestrate_envelope_from_progressed_proof<I>(
        &self,
        progressed: WorthQueryAdmittedDeclarationProgression<D, I>,
    ) -> WorthQueryDeclarationEnvelopeOrchestrationTranscript<D, I>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        worth_query_declaration_envelope_orchestration_from_progressed_proof_on_handle(
            self, progressed, None,
        )
    }

    pub fn orchestrate_envelope_from_progressed_proof_with_intent<I>(
        &self,
        progressed: WorthQueryAdmittedDeclarationProgression<D, I>,
        intent: WorthQueryDeclarationRouteIntent,
    ) -> WorthQueryDeclarationEnvelopeOrchestrationTranscript<D, I>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        worth_query_declaration_envelope_orchestration_from_progressed_proof_on_handle(
            self,
            progressed,
            Some(intent),
        )
    }
}
