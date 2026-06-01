use crate::application::{
    ForgeQueryDeclarationEnvelopeInput, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationReceiptInput, ForgeQueryDeclarationRoutePlanInput,
    ForgeQueryDomainEntryMarker,
};
use crate::binding_pipeline::{
    bind_continuation_from_target_on_handle, bind_envelope_from_target_on_handle,
    bind_receipt_from_target_on_handle, bind_route_from_target_on_handle, ForgeQueryBindingChecked,
    ForgeQueryBindingOutcome, ForgeQueryBindingTranscript, ForgeQueryContinuationBindingInput,
    ForgeQueryResolveContinuationFromTargetRequest, ForgeQueryResolveEnvelopeFromTargetRequest,
    ForgeQueryResolveReceiptFromTargetRequest, ForgeQueryResolveRouteFromTargetRequest,
};
use crate::ordinary_outcome::{ordinary_outcome_from_binding_outcome, ForgeQueryOrdinaryOutcome};

use super::super::super::ForgeQueryAdmittedConfiguredDomainHandle;
use crate::application::ForgeQueryDomainOperatingContext;

impl<D: ForgeQueryDomainEntryMarker, C: ForgeQueryDomainOperatingContext<D>>
    ForgeQueryAdmittedConfiguredDomainHandle<D, C>
{
    pub fn bind_route_from_target<I>(
        &self,
        request: ForgeQueryResolveRouteFromTargetRequest<D, I>,
    ) -> ForgeQueryBindingOutcome<ForgeQueryDeclarationRoutePlanInput<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        bind_route_from_target_on_handle(self, request).into_outcome()
    }

    pub fn bind_route_from_target_outcome<I>(
        &self,
        request: ForgeQueryResolveRouteFromTargetRequest<D, I>,
    ) -> ForgeQueryOrdinaryOutcome<ForgeQueryDeclarationRoutePlanInput<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        ordinary_outcome_from_binding_outcome(
            bind_route_from_target_on_handle(self, request).into_checked(),
        )
    }

    pub fn bind_route_from_target_checked<I>(
        &self,
        request: ForgeQueryResolveRouteFromTargetRequest<D, I>,
    ) -> ForgeQueryBindingChecked<ForgeQueryDeclarationRoutePlanInput<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        bind_route_from_target_on_handle(self, request).into_checked()
    }

    pub fn bind_route_from_target_proof<I>(
        &self,
        request: ForgeQueryResolveRouteFromTargetRequest<D, I>,
    ) -> ForgeQueryBindingTranscript<ForgeQueryDeclarationRoutePlanInput<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        bind_route_from_target_on_handle(self, request)
    }

    pub fn bind_receipt_from_target<I>(
        &self,
        request: ForgeQueryResolveReceiptFromTargetRequest<D, I>,
    ) -> ForgeQueryBindingOutcome<ForgeQueryDeclarationReceiptInput<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        bind_receipt_from_target_on_handle(self, request).into_outcome()
    }

    pub fn bind_receipt_from_target_outcome<I>(
        &self,
        request: ForgeQueryResolveReceiptFromTargetRequest<D, I>,
    ) -> ForgeQueryOrdinaryOutcome<ForgeQueryDeclarationReceiptInput<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        ordinary_outcome_from_binding_outcome(
            bind_receipt_from_target_on_handle(self, request).into_checked(),
        )
    }

    pub fn bind_receipt_from_target_checked<I>(
        &self,
        request: ForgeQueryResolveReceiptFromTargetRequest<D, I>,
    ) -> ForgeQueryBindingChecked<ForgeQueryDeclarationReceiptInput<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        bind_receipt_from_target_on_handle(self, request).into_checked()
    }

    pub fn bind_receipt_from_target_proof<I>(
        &self,
        request: ForgeQueryResolveReceiptFromTargetRequest<D, I>,
    ) -> ForgeQueryBindingTranscript<ForgeQueryDeclarationReceiptInput<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        bind_receipt_from_target_on_handle(self, request)
    }

    pub fn bind_envelope_from_target<I>(
        &self,
        request: ForgeQueryResolveEnvelopeFromTargetRequest<D, I>,
    ) -> ForgeQueryBindingOutcome<ForgeQueryDeclarationEnvelopeInput<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        bind_envelope_from_target_on_handle(self, request).into_outcome()
    }

    pub fn bind_envelope_from_target_outcome<I>(
        &self,
        request: ForgeQueryResolveEnvelopeFromTargetRequest<D, I>,
    ) -> ForgeQueryOrdinaryOutcome<ForgeQueryDeclarationEnvelopeInput<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        ordinary_outcome_from_binding_outcome(
            bind_envelope_from_target_on_handle(self, request).into_checked(),
        )
    }

    pub fn bind_envelope_from_target_checked<I>(
        &self,
        request: ForgeQueryResolveEnvelopeFromTargetRequest<D, I>,
    ) -> ForgeQueryBindingChecked<ForgeQueryDeclarationEnvelopeInput<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        bind_envelope_from_target_on_handle(self, request).into_checked()
    }

    pub fn bind_envelope_from_target_proof<I>(
        &self,
        request: ForgeQueryResolveEnvelopeFromTargetRequest<D, I>,
    ) -> ForgeQueryBindingTranscript<ForgeQueryDeclarationEnvelopeInput<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        bind_envelope_from_target_on_handle(self, request)
    }

    pub fn bind_continuation_from_target<I>(
        &self,
        request: ForgeQueryResolveContinuationFromTargetRequest<D, I>,
    ) -> ForgeQueryBindingOutcome<ForgeQueryContinuationBindingInput<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        bind_continuation_from_target_on_handle(self, request).into_outcome()
    }

    pub fn bind_continuation_from_target_outcome<I>(
        &self,
        request: ForgeQueryResolveContinuationFromTargetRequest<D, I>,
    ) -> ForgeQueryOrdinaryOutcome<ForgeQueryContinuationBindingInput<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        ordinary_outcome_from_binding_outcome(
            bind_continuation_from_target_on_handle(self, request).into_checked(),
        )
    }

    pub fn bind_continuation_from_target_checked<I>(
        &self,
        request: ForgeQueryResolveContinuationFromTargetRequest<D, I>,
    ) -> ForgeQueryBindingChecked<ForgeQueryContinuationBindingInput<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        bind_continuation_from_target_on_handle(self, request).into_checked()
    }

    pub fn bind_continuation_from_target_proof<I>(
        &self,
        request: ForgeQueryResolveContinuationFromTargetRequest<D, I>,
    ) -> ForgeQueryBindingTranscript<ForgeQueryContinuationBindingInput<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        bind_continuation_from_target_on_handle(self, request)
    }
}
