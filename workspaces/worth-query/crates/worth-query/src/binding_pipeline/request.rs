#[cfg(test)]
use crate::application::WorthQueryDeclarationRouteIntent;
use crate::application::{
    WorthQueryDeclarationAspectContract, WorthQueryDeclarationBridgeContinuationRequest,
    WorthQueryDeclarationInput, WorthQueryDomainEntryMarker,
};

use super::source::WorthQueryEnvelopeContextCandidate;
#[cfg(test)]
use super::source::{
    WorthQueryDeclarationContextCandidate, WorthQueryProgressionContextCandidate,
    WorthQueryRouteResolverSubject,
};
use super::WorthQueryBindingSourceKind;

pub struct WorthQueryContinuationBindingInput<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    bridge_request: WorthQueryDeclarationBridgeContinuationRequest,
    subject: crate::application::WorthQueryDeclarationBridgeRoutingInput<D, I>,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryContinuationBindingInput<D, I>
{
    pub fn bridge_request(&self) -> WorthQueryDeclarationBridgeContinuationRequest {
        self.bridge_request
    }

    pub fn bridge_subject(
        &self,
    ) -> &crate::application::WorthQueryDeclarationBridgeRoutingInput<D, I> {
        &self.subject
    }

    pub(crate) fn bridge(
        bridge_request: WorthQueryDeclarationBridgeContinuationRequest,
        subject: crate::application::WorthQueryDeclarationBridgeRoutingInput<D, I>,
    ) -> Self {
        Self {
            bridge_request,
            subject,
        }
    }

    pub(crate) fn into_bridge_parts(
        self,
    ) -> (
        WorthQueryDeclarationBridgeContinuationRequest,
        crate::application::WorthQueryDeclarationBridgeRoutingInput<D, I>,
    ) {
        (self.bridge_request, self.subject)
    }
}

macro_rules! resolver_request_common {
    () => {
        pub fn required_aspect_contract(&self) -> &WorthQueryDeclarationAspectContract {
            &self.required_aspect_contract
        }
        pub fn allow_compatible_superset(&self) -> bool {
            self.allow_compatible_superset
        }
        pub fn partial_is_narrowing_required(&self) -> bool {
            self.partial_is_narrowing_required
        }
    };
}

macro_rules! context_request_common {
    () => {
        pub fn required_aspect_contract(&self) -> &WorthQueryDeclarationAspectContract {
            &self.required_aspect_contract
        }
        pub fn allowed_sources(&self) -> &[WorthQueryBindingSourceKind] {
            &self.allowed_sources
        }
        pub fn allow_compatible_superset(&self) -> bool {
            self.allow_compatible_superset
        }
        pub fn partial_is_narrowing_required(&self) -> bool {
            self.partial_is_narrowing_required
        }
    };
}

#[cfg(test)]
pub struct WorthQueryDeclarationBindingRequest<I> {
    candidates: Vec<WorthQueryDeclarationContextCandidate<I>>,
    required_aspect_contract: WorthQueryDeclarationAspectContract,
    allowed_sources: Vec<WorthQueryBindingSourceKind>,
    allow_compatible_superset: bool,
    partial_is_narrowing_required: bool,
}

#[cfg(test)]
impl<I> WorthQueryDeclarationBindingRequest<I> {
    pub fn new(
        candidates: Vec<WorthQueryDeclarationContextCandidate<I>>,
        required_aspect_contract: WorthQueryDeclarationAspectContract,
        allowed_sources: Vec<WorthQueryBindingSourceKind>,
    ) -> Self {
        Self {
            candidates,
            required_aspect_contract,
            allowed_sources,
            allow_compatible_superset: true,
            partial_is_narrowing_required: true,
        }
    }

    #[cfg(test)]
    pub(crate) fn into_parts(
        self,
    ) -> (
        Vec<WorthQueryDeclarationContextCandidate<I>>,
        WorthQueryDeclarationAspectContract,
        Vec<WorthQueryBindingSourceKind>,
        bool,
        bool,
    ) {
        (
            self.candidates,
            self.required_aspect_contract,
            self.allowed_sources,
            self.allow_compatible_superset,
            self.partial_is_narrowing_required,
        )
    }
}

#[cfg(test)]
macro_rules! context_request {
    ($name:ident, $candidate:ty) => {
        pub struct $name<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>> {
            candidates: Vec<$candidate>,
            required_aspect_contract: WorthQueryDeclarationAspectContract,
            allowed_sources: Vec<WorthQueryBindingSourceKind>,
            allow_compatible_superset: bool,
            partial_is_narrowing_required: bool,
            route_intent: Option<WorthQueryDeclarationRouteIntent>,
        }

        impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>> $name<D, I> {
            pub fn new(
                candidates: Vec<$candidate>,
                required_aspect_contract: WorthQueryDeclarationAspectContract,
                allowed_sources: Vec<WorthQueryBindingSourceKind>,
            ) -> Self {
                Self {
                    candidates,
                    required_aspect_contract,
                    allowed_sources,
                    allow_compatible_superset: true,
                    partial_is_narrowing_required: true,
                    route_intent: None,
                }
            }

            #[cfg(test)]
            pub(crate) fn into_parts(
                self,
            ) -> (
                Vec<$candidate>,
                WorthQueryDeclarationAspectContract,
                Vec<WorthQueryBindingSourceKind>,
                bool,
                bool,
                Option<WorthQueryDeclarationRouteIntent>,
            ) {
                (
                    self.candidates,
                    self.required_aspect_contract,
                    self.allowed_sources,
                    self.allow_compatible_superset,
                    self.partial_is_narrowing_required,
                    self.route_intent,
                )
            }
        }
    };
}

#[cfg(test)]
context_request!(WorthQueryRouteBindingRequest, WorthQueryProgressionContextCandidate<D, I>);

pub struct WorthQueryContinuationBindingRequest<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    candidates: Vec<WorthQueryEnvelopeContextCandidate<D, I>>,
    required_aspect_contract: WorthQueryDeclarationAspectContract,
    allowed_sources: Vec<WorthQueryBindingSourceKind>,
    allow_compatible_superset: bool,
    partial_is_narrowing_required: bool,
    bridge_request: Option<WorthQueryDeclarationBridgeContinuationRequest>,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryContinuationBindingRequest<D, I>
{
    pub fn new(
        candidates: Vec<WorthQueryEnvelopeContextCandidate<D, I>>,
        required_aspect_contract: WorthQueryDeclarationAspectContract,
        allowed_sources: Vec<WorthQueryBindingSourceKind>,
    ) -> Self {
        Self {
            candidates,
            required_aspect_contract,
            allowed_sources,
            allow_compatible_superset: true,
            partial_is_narrowing_required: true,
            bridge_request: None,
        }
    }

    context_request_common!();

    pub fn with_exact_fit_only(mut self) -> Self {
        self.allow_compatible_superset = false;
        self
    }

    pub fn with_partial_denial(mut self) -> Self {
        self.partial_is_narrowing_required = false;
        self
    }

    pub fn with_bridge_request(
        mut self,
        bridge_request: WorthQueryDeclarationBridgeContinuationRequest,
    ) -> Self {
        self.bridge_request = Some(bridge_request);
        self
    }

    pub fn bridge_request(&self) -> Option<WorthQueryDeclarationBridgeContinuationRequest> {
        self.bridge_request
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        Vec<WorthQueryEnvelopeContextCandidate<D, I>>,
        WorthQueryDeclarationAspectContract,
        Vec<WorthQueryBindingSourceKind>,
        bool,
        bool,
        Option<WorthQueryDeclarationBridgeContinuationRequest>,
    ) {
        (
            self.candidates,
            self.required_aspect_contract,
            self.allowed_sources,
            self.allow_compatible_superset,
            self.partial_is_narrowing_required,
            self.bridge_request,
        )
    }
}

#[cfg(test)]
macro_rules! resolver_request {
    ($name:ident, $source:ty) => {
        pub struct $name<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>> {
            source: $source,
            required_aspect_contract: WorthQueryDeclarationAspectContract,
            allow_compatible_superset: bool,
            partial_is_narrowing_required: bool,
            route_intent: Option<WorthQueryDeclarationRouteIntent>,
        }

        impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>> $name<D, I> {
            pub fn new(
                source: $source,
                required_aspect_contract: WorthQueryDeclarationAspectContract,
            ) -> Self {
                Self {
                    source,
                    required_aspect_contract,
                    allow_compatible_superset: true,
                    partial_is_narrowing_required: true,
                    route_intent: None,
                }
            }

            pub fn with_exact_fit_only(mut self) -> Self {
                self.allow_compatible_superset = false;
                self
            }

            pub fn with_partial_denial(mut self) -> Self {
                self.partial_is_narrowing_required = false;
                self
            }

            pub fn with_route_intent(
                mut self,
                route_intent: WorthQueryDeclarationRouteIntent,
            ) -> Self {
                self.route_intent = Some(route_intent);
                self
            }

            #[cfg(test)]
            pub(crate) fn into_parts(
                self,
            ) -> (
                $source,
                WorthQueryDeclarationAspectContract,
                bool,
                bool,
                Option<WorthQueryDeclarationRouteIntent>,
            ) {
                (
                    self.source,
                    self.required_aspect_contract,
                    self.allow_compatible_superset,
                    self.partial_is_narrowing_required,
                    self.route_intent,
                )
            }
        }
    };
}

#[cfg(test)]
resolver_request!(WorthQueryResolveRouteFromTargetRequest, WorthQueryRouteResolverSubject<D, I>);

pub struct WorthQueryResolveContinuationFromTargetRequest<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    envelope: crate::application::WorthQueryDeclarationEnvelope<D, I>,
    required_aspect_contract: WorthQueryDeclarationAspectContract,
    allow_compatible_superset: bool,
    partial_is_narrowing_required: bool,
    bridge_request: Option<WorthQueryDeclarationBridgeContinuationRequest>,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryResolveContinuationFromTargetRequest<D, I>
{
    pub fn new(
        envelope: crate::application::WorthQueryDeclarationEnvelope<D, I>,
        required_aspect_contract: WorthQueryDeclarationAspectContract,
    ) -> Self {
        Self {
            envelope,
            required_aspect_contract,
            allow_compatible_superset: true,
            partial_is_narrowing_required: true,
            bridge_request: None,
        }
    }

    resolver_request_common!();

    pub fn with_exact_fit_only(mut self) -> Self {
        self.allow_compatible_superset = false;
        self
    }

    pub fn with_partial_denial(mut self) -> Self {
        self.partial_is_narrowing_required = false;
        self
    }

    pub fn with_bridge_request(
        mut self,
        bridge_request: WorthQueryDeclarationBridgeContinuationRequest,
    ) -> Self {
        self.bridge_request = Some(bridge_request);
        self
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        crate::application::WorthQueryDeclarationEnvelope<D, I>,
        WorthQueryDeclarationAspectContract,
        bool,
        bool,
        Option<WorthQueryDeclarationBridgeContinuationRequest>,
    ) {
        (
            self.envelope,
            self.required_aspect_contract,
            self.allow_compatible_superset,
            self.partial_is_narrowing_required,
            self.bridge_request,
        )
    }
}
