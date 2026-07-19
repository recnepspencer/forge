#[cfg(test)]
use crate::application::WorthQueryDeclarationRoutePlanInput;
use crate::application::{WorthQueryDeclarationInput, WorthQueryDomainEntryMarker};
use crate::binding_pipeline::{
    bind_continuation_from_target_on_handle, WorthQueryBindingTranscript,
    WorthQueryContinuationBindingInput, WorthQueryResolveContinuationFromTargetRequest,
};
#[cfg(test)]
use crate::binding_pipeline::{
    bind_route_from_target_on_handle, WorthQueryBindingOutcome,
    WorthQueryResolveRouteFromTargetRequest,
};

use super::super::super::WorthQueryInstalledDomainDeclarationContext;
use crate::application::WorthQueryDomainOperatingContext;

impl<D: WorthQueryDomainEntryMarker, C: WorthQueryDomainOperatingContext<D>>
    WorthQueryInstalledDomainDeclarationContext<D, C>
{
    #[cfg(test)]
    pub(crate) fn bind_route_from_target<I>(
        &self,
        request: WorthQueryResolveRouteFromTargetRequest<D, I>,
    ) -> WorthQueryBindingOutcome<WorthQueryDeclarationRoutePlanInput<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        bind_route_from_target_on_handle(self, request).into_outcome()
    }

    #[cfg(test)]
    pub(crate) fn bind_route_from_target_proof<I>(
        &self,
        request: WorthQueryResolveRouteFromTargetRequest<D, I>,
    ) -> WorthQueryBindingTranscript<WorthQueryDeclarationRoutePlanInput<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        bind_route_from_target_on_handle(self, request)
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
