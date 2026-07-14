use crate::application::{
    WorthQueryDeclarationEnvelopeInput, WorthQueryDeclarationInput,
    WorthQueryDeclarationReceiptInput, WorthQueryDeclarationRoutePlanInput,
    WorthQueryDomainEntryMarker,
};
use crate::binding_pipeline::{
    bind_continuation_from_target_on_handle, bind_envelope_from_target_on_handle,
    bind_receipt_from_target_on_handle, bind_route_from_target_on_handle, WorthQueryBindingChecked,
    WorthQueryBindingOutcome, WorthQueryBindingTranscript, WorthQueryContinuationBindingInput,
    WorthQueryResolveContinuationFromTargetRequest, WorthQueryResolveEnvelopeFromTargetRequest,
    WorthQueryResolveReceiptFromTargetRequest, WorthQueryResolveRouteFromTargetRequest,
};
use crate::ordinary_outcome::{ordinary_outcome_from_binding_outcome, WorthQueryOrdinaryOutcome};

use super::super::super::WorthQueryInstalledDomainDeclarationContext;
use crate::application::WorthQueryDomainOperatingContext;

impl<D: WorthQueryDomainEntryMarker, C: WorthQueryDomainOperatingContext<D>>
    WorthQueryInstalledDomainDeclarationContext<D, C>
{
    pub(crate) fn bind_route_from_target<I>(
        &self,
        request: WorthQueryResolveRouteFromTargetRequest<D, I>,
    ) -> WorthQueryBindingOutcome<WorthQueryDeclarationRoutePlanInput<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        bind_route_from_target_on_handle(self, request).into_outcome()
    }

    pub(crate) fn bind_route_from_target_outcome<I>(
        &self,
        request: WorthQueryResolveRouteFromTargetRequest<D, I>,
    ) -> WorthQueryOrdinaryOutcome<WorthQueryDeclarationRoutePlanInput<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        ordinary_outcome_from_binding_outcome(
            bind_route_from_target_on_handle(self, request).into_checked(),
        )
    }

    pub(crate) fn bind_route_from_target_checked<I>(
        &self,
        request: WorthQueryResolveRouteFromTargetRequest<D, I>,
    ) -> WorthQueryBindingChecked<WorthQueryDeclarationRoutePlanInput<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        bind_route_from_target_on_handle(self, request).into_checked()
    }

    pub(crate) fn bind_route_from_target_proof<I>(
        &self,
        request: WorthQueryResolveRouteFromTargetRequest<D, I>,
    ) -> WorthQueryBindingTranscript<WorthQueryDeclarationRoutePlanInput<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        bind_route_from_target_on_handle(self, request)
    }

    pub(crate) fn bind_receipt_from_target<I>(
        &self,
        request: WorthQueryResolveReceiptFromTargetRequest<D, I>,
    ) -> WorthQueryBindingOutcome<WorthQueryDeclarationReceiptInput<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        bind_receipt_from_target_on_handle(self, request).into_outcome()
    }

    pub(crate) fn bind_receipt_from_target_outcome<I>(
        &self,
        request: WorthQueryResolveReceiptFromTargetRequest<D, I>,
    ) -> WorthQueryOrdinaryOutcome<WorthQueryDeclarationReceiptInput<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        ordinary_outcome_from_binding_outcome(
            bind_receipt_from_target_on_handle(self, request).into_checked(),
        )
    }

    pub(crate) fn bind_receipt_from_target_checked<I>(
        &self,
        request: WorthQueryResolveReceiptFromTargetRequest<D, I>,
    ) -> WorthQueryBindingChecked<WorthQueryDeclarationReceiptInput<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        bind_receipt_from_target_on_handle(self, request).into_checked()
    }

    pub(crate) fn bind_receipt_from_target_proof<I>(
        &self,
        request: WorthQueryResolveReceiptFromTargetRequest<D, I>,
    ) -> WorthQueryBindingTranscript<WorthQueryDeclarationReceiptInput<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        bind_receipt_from_target_on_handle(self, request)
    }

    pub(crate) fn bind_envelope_from_target<I>(
        &self,
        request: WorthQueryResolveEnvelopeFromTargetRequest<D, I>,
    ) -> WorthQueryBindingOutcome<WorthQueryDeclarationEnvelopeInput<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        bind_envelope_from_target_on_handle(self, request).into_outcome()
    }

    pub(crate) fn bind_envelope_from_target_outcome<I>(
        &self,
        request: WorthQueryResolveEnvelopeFromTargetRequest<D, I>,
    ) -> WorthQueryOrdinaryOutcome<WorthQueryDeclarationEnvelopeInput<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        ordinary_outcome_from_binding_outcome(
            bind_envelope_from_target_on_handle(self, request).into_checked(),
        )
    }

    pub(crate) fn bind_envelope_from_target_checked<I>(
        &self,
        request: WorthQueryResolveEnvelopeFromTargetRequest<D, I>,
    ) -> WorthQueryBindingChecked<WorthQueryDeclarationEnvelopeInput<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        bind_envelope_from_target_on_handle(self, request).into_checked()
    }

    pub(crate) fn bind_envelope_from_target_proof<I>(
        &self,
        request: WorthQueryResolveEnvelopeFromTargetRequest<D, I>,
    ) -> WorthQueryBindingTranscript<WorthQueryDeclarationEnvelopeInput<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        bind_envelope_from_target_on_handle(self, request)
    }

    pub(crate) fn bind_continuation_from_target<I>(
        &self,
        request: WorthQueryResolveContinuationFromTargetRequest<D, I>,
    ) -> WorthQueryBindingOutcome<WorthQueryContinuationBindingInput<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        bind_continuation_from_target_on_handle(self, request).into_outcome()
    }

    pub(crate) fn bind_continuation_from_target_outcome<I>(
        &self,
        request: WorthQueryResolveContinuationFromTargetRequest<D, I>,
    ) -> WorthQueryOrdinaryOutcome<WorthQueryContinuationBindingInput<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        ordinary_outcome_from_binding_outcome(
            bind_continuation_from_target_on_handle(self, request).into_checked(),
        )
    }

    pub(crate) fn bind_continuation_from_target_checked<I>(
        &self,
        request: WorthQueryResolveContinuationFromTargetRequest<D, I>,
    ) -> WorthQueryBindingChecked<WorthQueryContinuationBindingInput<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        bind_continuation_from_target_on_handle(self, request).into_checked()
    }

    pub(crate) fn bind_continuation_from_target_proof<I>(
        &self,
        request: WorthQueryResolveContinuationFromTargetRequest<D, I>,
    ) -> WorthQueryBindingTranscript<WorthQueryContinuationBindingInput<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        bind_continuation_from_target_on_handle(self, request)
    }
}
