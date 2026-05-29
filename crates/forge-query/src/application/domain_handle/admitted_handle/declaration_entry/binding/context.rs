use crate::application::{
    ForgeQueryCanonicalDeclarationArtifact, ForgeQueryDeclarationEnvelopeInput,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationReceiptInput,
    ForgeQueryDeclarationRoutePlanInput, ForgeQueryDomainEntryMarker,
};
use crate::binding_pipeline::{
    bind_continuation_request_from_context_on_handle, bind_declaration_from_context_on_handle,
    bind_envelope_request_from_context_on_handle, bind_receipt_request_from_context_on_handle,
    bind_route_request_from_context_on_handle, ForgeQueryBindingChecked, ForgeQueryBindingOutcome,
    ForgeQueryBindingTranscript, ForgeQueryContinuationBindingInput,
    ForgeQueryContinuationBindingRequest, ForgeQueryDeclarationBindingRequest,
    ForgeQueryEnvelopeBindingRequest, ForgeQueryReceiptBindingRequest,
    ForgeQueryRouteBindingRequest,
};
use crate::ordinary_outcome::{ordinary_outcome_from_binding_outcome, ForgeQueryOrdinaryOutcome};

use super::super::super::ForgeQueryAdmittedConfiguredDomainHandle;
use crate::application::ForgeQueryDomainOperatingContext;

impl<D: ForgeQueryDomainEntryMarker, C: ForgeQueryDomainOperatingContext<D>>
    ForgeQueryAdmittedConfiguredDomainHandle<D, C>
{
    pub fn bind_declaration_from_context<I>(
        &self,
        request: ForgeQueryDeclarationBindingRequest<I>,
    ) -> ForgeQueryBindingOutcome<ForgeQueryCanonicalDeclarationArtifact<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        bind_declaration_from_context_on_handle(self, request).into_outcome()
    }

    pub fn bind_declaration_from_context_outcome<I>(
        &self,
        request: ForgeQueryDeclarationBindingRequest<I>,
    ) -> ForgeQueryOrdinaryOutcome<ForgeQueryCanonicalDeclarationArtifact<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        ordinary_outcome_from_binding_outcome(
            bind_declaration_from_context_on_handle(self, request).into_checked(),
        )
    }

    pub fn bind_declaration_from_context_checked<I>(
        &self,
        request: ForgeQueryDeclarationBindingRequest<I>,
    ) -> ForgeQueryBindingChecked<ForgeQueryCanonicalDeclarationArtifact<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        bind_declaration_from_context_on_handle(self, request).into_checked()
    }

    pub fn bind_declaration_from_context_proof<I>(
        &self,
        request: ForgeQueryDeclarationBindingRequest<I>,
    ) -> ForgeQueryBindingTranscript<ForgeQueryCanonicalDeclarationArtifact<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        bind_declaration_from_context_on_handle(self, request)
    }

    pub fn bind_route_request_from_context<I>(
        &self,
        request: ForgeQueryRouteBindingRequest<D, I>,
    ) -> ForgeQueryBindingOutcome<ForgeQueryDeclarationRoutePlanInput<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        bind_route_request_from_context_on_handle(self, request).into_outcome()
    }

    pub fn bind_route_request_from_context_outcome<I>(
        &self,
        request: ForgeQueryRouteBindingRequest<D, I>,
    ) -> ForgeQueryOrdinaryOutcome<ForgeQueryDeclarationRoutePlanInput<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        ordinary_outcome_from_binding_outcome(
            bind_route_request_from_context_on_handle(self, request).into_checked(),
        )
    }

    pub fn bind_route_request_from_context_checked<I>(
        &self,
        request: ForgeQueryRouteBindingRequest<D, I>,
    ) -> ForgeQueryBindingChecked<ForgeQueryDeclarationRoutePlanInput<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        bind_route_request_from_context_on_handle(self, request).into_checked()
    }

    pub fn bind_route_request_from_context_proof<I>(
        &self,
        request: ForgeQueryRouteBindingRequest<D, I>,
    ) -> ForgeQueryBindingTranscript<ForgeQueryDeclarationRoutePlanInput<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        bind_route_request_from_context_on_handle(self, request)
    }

    pub fn bind_receipt_request_from_context<I>(
        &self,
        request: ForgeQueryReceiptBindingRequest<D, I>,
    ) -> ForgeQueryBindingOutcome<ForgeQueryDeclarationReceiptInput<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        bind_receipt_request_from_context_on_handle(self, request).into_outcome()
    }

    pub fn bind_receipt_request_from_context_outcome<I>(
        &self,
        request: ForgeQueryReceiptBindingRequest<D, I>,
    ) -> ForgeQueryOrdinaryOutcome<ForgeQueryDeclarationReceiptInput<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        ordinary_outcome_from_binding_outcome(
            bind_receipt_request_from_context_on_handle(self, request).into_checked(),
        )
    }

    pub fn bind_receipt_request_from_context_checked<I>(
        &self,
        request: ForgeQueryReceiptBindingRequest<D, I>,
    ) -> ForgeQueryBindingChecked<ForgeQueryDeclarationReceiptInput<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        bind_receipt_request_from_context_on_handle(self, request).into_checked()
    }

    pub fn bind_receipt_request_from_context_proof<I>(
        &self,
        request: ForgeQueryReceiptBindingRequest<D, I>,
    ) -> ForgeQueryBindingTranscript<ForgeQueryDeclarationReceiptInput<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        bind_receipt_request_from_context_on_handle(self, request)
    }

    pub fn bind_envelope_request_from_context<I>(
        &self,
        request: ForgeQueryEnvelopeBindingRequest<D, I>,
    ) -> ForgeQueryBindingOutcome<ForgeQueryDeclarationEnvelopeInput<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        bind_envelope_request_from_context_on_handle(self, request).into_outcome()
    }

    pub fn bind_envelope_request_from_context_outcome<I>(
        &self,
        request: ForgeQueryEnvelopeBindingRequest<D, I>,
    ) -> ForgeQueryOrdinaryOutcome<ForgeQueryDeclarationEnvelopeInput<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        ordinary_outcome_from_binding_outcome(
            bind_envelope_request_from_context_on_handle(self, request).into_checked(),
        )
    }

    pub fn bind_envelope_request_from_context_checked<I>(
        &self,
        request: ForgeQueryEnvelopeBindingRequest<D, I>,
    ) -> ForgeQueryBindingChecked<ForgeQueryDeclarationEnvelopeInput<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        bind_envelope_request_from_context_on_handle(self, request).into_checked()
    }

    pub fn bind_envelope_request_from_context_proof<I>(
        &self,
        request: ForgeQueryEnvelopeBindingRequest<D, I>,
    ) -> ForgeQueryBindingTranscript<ForgeQueryDeclarationEnvelopeInput<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        bind_envelope_request_from_context_on_handle(self, request)
    }

    pub fn bind_continuation_request_from_context<I>(
        &self,
        request: ForgeQueryContinuationBindingRequest<D, I>,
    ) -> ForgeQueryBindingOutcome<ForgeQueryContinuationBindingInput<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        bind_continuation_request_from_context_on_handle(self, request).into_outcome()
    }

    pub fn bind_continuation_request_from_context_outcome<I>(
        &self,
        request: ForgeQueryContinuationBindingRequest<D, I>,
    ) -> ForgeQueryOrdinaryOutcome<ForgeQueryContinuationBindingInput<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        ordinary_outcome_from_binding_outcome(
            bind_continuation_request_from_context_on_handle(self, request).into_checked(),
        )
    }

    pub fn bind_continuation_request_from_context_checked<I>(
        &self,
        request: ForgeQueryContinuationBindingRequest<D, I>,
    ) -> ForgeQueryBindingChecked<ForgeQueryContinuationBindingInput<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        bind_continuation_request_from_context_on_handle(self, request).into_checked()
    }

    pub fn bind_continuation_request_from_context_proof<I>(
        &self,
        request: ForgeQueryContinuationBindingRequest<D, I>,
    ) -> ForgeQueryBindingTranscript<ForgeQueryContinuationBindingInput<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        bind_continuation_request_from_context_on_handle(self, request)
    }
}
