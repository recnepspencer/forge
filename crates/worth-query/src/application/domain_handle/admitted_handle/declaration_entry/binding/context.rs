use crate::application::{
    WorthQueryCanonicalDeclarationArtifact, WorthQueryDeclarationEnvelopeInput,
    WorthQueryDeclarationInput, WorthQueryDeclarationReceiptInput,
    WorthQueryDeclarationRoutePlanInput, WorthQueryDomainEntryMarker,
};
use crate::binding_pipeline::{
    bind_continuation_request_from_context_on_handle, bind_declaration_from_context_on_handle,
    bind_envelope_request_from_context_on_handle, bind_receipt_request_from_context_on_handle,
    bind_route_request_from_context_on_handle, WorthQueryBindingChecked, WorthQueryBindingOutcome,
    WorthQueryBindingTranscript, WorthQueryContinuationBindingInput,
    WorthQueryContinuationBindingRequest, WorthQueryDeclarationBindingRequest,
    WorthQueryEnvelopeBindingRequest, WorthQueryReceiptBindingRequest,
    WorthQueryRouteBindingRequest,
};
use crate::ordinary_outcome::{ordinary_outcome_from_binding_outcome, WorthQueryOrdinaryOutcome};

use super::super::super::WorthQueryInstalledDomainDeclarationContext;
use crate::application::WorthQueryDomainOperatingContext;

impl<D: WorthQueryDomainEntryMarker, C: WorthQueryDomainOperatingContext<D>>
    WorthQueryInstalledDomainDeclarationContext<D, C>
{
    pub(crate) fn bind_declaration_from_context<I>(
        &self,
        request: WorthQueryDeclarationBindingRequest<I>,
    ) -> WorthQueryBindingOutcome<WorthQueryCanonicalDeclarationArtifact<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        bind_declaration_from_context_on_handle(self, request).into_outcome()
    }

    pub(crate) fn bind_declaration_from_context_outcome<I>(
        &self,
        request: WorthQueryDeclarationBindingRequest<I>,
    ) -> WorthQueryOrdinaryOutcome<WorthQueryCanonicalDeclarationArtifact<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        ordinary_outcome_from_binding_outcome(
            bind_declaration_from_context_on_handle(self, request).into_checked(),
        )
    }

    pub(crate) fn bind_declaration_from_context_checked<I>(
        &self,
        request: WorthQueryDeclarationBindingRequest<I>,
    ) -> WorthQueryBindingChecked<WorthQueryCanonicalDeclarationArtifact<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        bind_declaration_from_context_on_handle(self, request).into_checked()
    }

    pub(crate) fn bind_declaration_from_context_proof<I>(
        &self,
        request: WorthQueryDeclarationBindingRequest<I>,
    ) -> WorthQueryBindingTranscript<WorthQueryCanonicalDeclarationArtifact<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        bind_declaration_from_context_on_handle(self, request)
    }

    pub(crate) fn bind_route_request_from_context<I>(
        &self,
        request: WorthQueryRouteBindingRequest<D, I>,
    ) -> WorthQueryBindingOutcome<WorthQueryDeclarationRoutePlanInput<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        bind_route_request_from_context_on_handle(self, request).into_outcome()
    }

    pub(crate) fn bind_route_request_from_context_outcome<I>(
        &self,
        request: WorthQueryRouteBindingRequest<D, I>,
    ) -> WorthQueryOrdinaryOutcome<WorthQueryDeclarationRoutePlanInput<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        ordinary_outcome_from_binding_outcome(
            bind_route_request_from_context_on_handle(self, request).into_checked(),
        )
    }

    pub(crate) fn bind_route_request_from_context_checked<I>(
        &self,
        request: WorthQueryRouteBindingRequest<D, I>,
    ) -> WorthQueryBindingChecked<WorthQueryDeclarationRoutePlanInput<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        bind_route_request_from_context_on_handle(self, request).into_checked()
    }

    pub(crate) fn bind_route_request_from_context_proof<I>(
        &self,
        request: WorthQueryRouteBindingRequest<D, I>,
    ) -> WorthQueryBindingTranscript<WorthQueryDeclarationRoutePlanInput<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        bind_route_request_from_context_on_handle(self, request)
    }

    pub(crate) fn bind_receipt_request_from_context<I>(
        &self,
        request: WorthQueryReceiptBindingRequest<D, I>,
    ) -> WorthQueryBindingOutcome<WorthQueryDeclarationReceiptInput<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        bind_receipt_request_from_context_on_handle(self, request).into_outcome()
    }

    pub(crate) fn bind_receipt_request_from_context_outcome<I>(
        &self,
        request: WorthQueryReceiptBindingRequest<D, I>,
    ) -> WorthQueryOrdinaryOutcome<WorthQueryDeclarationReceiptInput<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        ordinary_outcome_from_binding_outcome(
            bind_receipt_request_from_context_on_handle(self, request).into_checked(),
        )
    }

    pub(crate) fn bind_receipt_request_from_context_checked<I>(
        &self,
        request: WorthQueryReceiptBindingRequest<D, I>,
    ) -> WorthQueryBindingChecked<WorthQueryDeclarationReceiptInput<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        bind_receipt_request_from_context_on_handle(self, request).into_checked()
    }

    pub(crate) fn bind_receipt_request_from_context_proof<I>(
        &self,
        request: WorthQueryReceiptBindingRequest<D, I>,
    ) -> WorthQueryBindingTranscript<WorthQueryDeclarationReceiptInput<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        bind_receipt_request_from_context_on_handle(self, request)
    }

    pub(crate) fn bind_envelope_request_from_context<I>(
        &self,
        request: WorthQueryEnvelopeBindingRequest<D, I>,
    ) -> WorthQueryBindingOutcome<WorthQueryDeclarationEnvelopeInput<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        bind_envelope_request_from_context_on_handle(self, request).into_outcome()
    }

    pub(crate) fn bind_envelope_request_from_context_outcome<I>(
        &self,
        request: WorthQueryEnvelopeBindingRequest<D, I>,
    ) -> WorthQueryOrdinaryOutcome<WorthQueryDeclarationEnvelopeInput<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        ordinary_outcome_from_binding_outcome(
            bind_envelope_request_from_context_on_handle(self, request).into_checked(),
        )
    }

    pub(crate) fn bind_envelope_request_from_context_checked<I>(
        &self,
        request: WorthQueryEnvelopeBindingRequest<D, I>,
    ) -> WorthQueryBindingChecked<WorthQueryDeclarationEnvelopeInput<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        bind_envelope_request_from_context_on_handle(self, request).into_checked()
    }

    pub(crate) fn bind_envelope_request_from_context_proof<I>(
        &self,
        request: WorthQueryEnvelopeBindingRequest<D, I>,
    ) -> WorthQueryBindingTranscript<WorthQueryDeclarationEnvelopeInput<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        bind_envelope_request_from_context_on_handle(self, request)
    }

    pub(crate) fn bind_continuation_request_from_context<I>(
        &self,
        request: WorthQueryContinuationBindingRequest<D, I>,
    ) -> WorthQueryBindingOutcome<WorthQueryContinuationBindingInput<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        bind_continuation_request_from_context_on_handle(self, request).into_outcome()
    }

    pub(crate) fn bind_continuation_request_from_context_outcome<I>(
        &self,
        request: WorthQueryContinuationBindingRequest<D, I>,
    ) -> WorthQueryOrdinaryOutcome<WorthQueryContinuationBindingInput<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        ordinary_outcome_from_binding_outcome(
            bind_continuation_request_from_context_on_handle(self, request).into_checked(),
        )
    }

    pub(crate) fn bind_continuation_request_from_context_checked<I>(
        &self,
        request: WorthQueryContinuationBindingRequest<D, I>,
    ) -> WorthQueryBindingChecked<WorthQueryContinuationBindingInput<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        bind_continuation_request_from_context_on_handle(self, request).into_checked()
    }

    pub(crate) fn bind_continuation_request_from_context_proof<I>(
        &self,
        request: WorthQueryContinuationBindingRequest<D, I>,
    ) -> WorthQueryBindingTranscript<WorthQueryContinuationBindingInput<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        bind_continuation_request_from_context_on_handle(self, request)
    }
}
