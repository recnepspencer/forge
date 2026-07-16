#[cfg(test)]
use crate::application::{
    WorthQueryCanonicalDeclarationArtifact, WorthQueryDeclarationRoutePlanInput,
};
use crate::application::{WorthQueryDeclarationInput, WorthQueryDomainEntryMarker};
use crate::binding_pipeline::{
    bind_continuation_request_from_context_on_handle, WorthQueryBindingTranscript,
    WorthQueryContinuationBindingInput, WorthQueryContinuationBindingRequest,
};
#[cfg(test)]
use crate::binding_pipeline::{
    bind_declaration_from_context_on_handle, bind_route_request_from_context_on_handle,
    WorthQueryBindingOutcome, WorthQueryDeclarationBindingRequest, WorthQueryRouteBindingRequest,
};
#[cfg(test)]
use crate::ordinary_outcome::{ordinary_outcome_from_binding_outcome, WorthQueryOrdinaryOutcome};

use super::super::super::WorthQueryInstalledDomainDeclarationContext;
use crate::application::WorthQueryDomainOperatingContext;

impl<D: WorthQueryDomainEntryMarker, C: WorthQueryDomainOperatingContext<D>>
    WorthQueryInstalledDomainDeclarationContext<D, C>
{
    #[cfg(test)]
    pub(crate) fn bind_declaration_from_context<I>(
        &self,
        request: WorthQueryDeclarationBindingRequest<I>,
    ) -> WorthQueryBindingOutcome<WorthQueryCanonicalDeclarationArtifact<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        bind_declaration_from_context_on_handle(self, request).into_outcome()
    }

    #[cfg(test)]
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

    #[cfg(test)]
    pub(crate) fn bind_route_request_from_context<I>(
        &self,
        request: WorthQueryRouteBindingRequest<D, I>,
    ) -> WorthQueryBindingOutcome<WorthQueryDeclarationRoutePlanInput<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        bind_route_request_from_context_on_handle(self, request).into_outcome()
    }

    #[cfg(test)]
    pub(crate) fn bind_continuation_request_from_context<I>(
        &self,
        request: WorthQueryContinuationBindingRequest<D, I>,
    ) -> WorthQueryBindingOutcome<WorthQueryContinuationBindingInput<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        bind_continuation_request_from_context_on_handle(self, request).into_outcome()
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
