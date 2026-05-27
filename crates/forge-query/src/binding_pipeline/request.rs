use crate::application::{
    ForgeQueryDeclarationAspectContract, ForgeQueryDeclarationBridgeContinuationRequest,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationRouteIntent, ForgeQueryDomainEntryMarker,
};

use super::source::{
    ForgeQueryDeclarationContextCandidate, ForgeQueryEnvelopeContextCandidate,
    ForgeQueryEnvelopeResolverSubject, ForgeQueryProgressionContextCandidate,
    ForgeQueryReceiptResolverSubject, ForgeQueryRouteResolverSubject,
};
use super::ForgeQueryBindingSourceKind;

pub struct ForgeQueryContinuationBindingInput<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    bridge_request: ForgeQueryDeclarationBridgeContinuationRequest,
    subject: crate::application::ForgeQueryDeclarationBridgeRoutingInput<D, I>,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryContinuationBindingInput<D, I>
{
    pub fn bridge_request(&self) -> ForgeQueryDeclarationBridgeContinuationRequest {
        self.bridge_request
    }

    pub fn bridge_subject(
        &self,
    ) -> &crate::application::ForgeQueryDeclarationBridgeRoutingInput<D, I> {
        &self.subject
    }

    pub(crate) fn bridge(
        bridge_request: ForgeQueryDeclarationBridgeContinuationRequest,
        subject: crate::application::ForgeQueryDeclarationBridgeRoutingInput<D, I>,
    ) -> Self {
        Self {
            bridge_request,
            subject,
        }
    }

    pub(crate) fn into_bridge_parts(
        self,
    ) -> (
        ForgeQueryDeclarationBridgeContinuationRequest,
        crate::application::ForgeQueryDeclarationBridgeRoutingInput<D, I>,
    ) {
        (self.bridge_request, self.subject)
    }
}

macro_rules! request_common {
    () => {
        pub fn required_aspect_contract(&self) -> &ForgeQueryDeclarationAspectContract {
            &self.required_aspect_contract
        }
        pub fn allowed_sources(&self) -> &[ForgeQueryBindingSourceKind] {
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

macro_rules! resolver_request_common {
    () => {
        pub fn required_aspect_contract(&self) -> &ForgeQueryDeclarationAspectContract {
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
        pub fn required_aspect_contract(&self) -> &ForgeQueryDeclarationAspectContract {
            &self.required_aspect_contract
        }
        pub fn allowed_sources(&self) -> &[ForgeQueryBindingSourceKind] {
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

pub struct ForgeQueryDeclarationBindingRequest<I> {
    candidates: Vec<ForgeQueryDeclarationContextCandidate<I>>,
    required_aspect_contract: ForgeQueryDeclarationAspectContract,
    allowed_sources: Vec<ForgeQueryBindingSourceKind>,
    allow_compatible_superset: bool,
    partial_is_narrowing_required: bool,
}

impl<I> ForgeQueryDeclarationBindingRequest<I> {
    pub fn new(
        candidates: Vec<ForgeQueryDeclarationContextCandidate<I>>,
        required_aspect_contract: ForgeQueryDeclarationAspectContract,
        allowed_sources: Vec<ForgeQueryBindingSourceKind>,
    ) -> Self {
        Self {
            candidates,
            required_aspect_contract,
            allowed_sources,
            allow_compatible_superset: true,
            partial_is_narrowing_required: true,
        }
    }

    request_common!();

    pub fn with_exact_fit_only(mut self) -> Self {
        self.allow_compatible_superset = false;
        self
    }

    pub fn with_partial_denial(mut self) -> Self {
        self.partial_is_narrowing_required = false;
        self
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        Vec<ForgeQueryDeclarationContextCandidate<I>>,
        ForgeQueryDeclarationAspectContract,
        Vec<ForgeQueryBindingSourceKind>,
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

macro_rules! context_request {
    ($name:ident, $candidate:ty) => {
        pub struct $name<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>> {
            candidates: Vec<$candidate>,
            required_aspect_contract: ForgeQueryDeclarationAspectContract,
            allowed_sources: Vec<ForgeQueryBindingSourceKind>,
            allow_compatible_superset: bool,
            partial_is_narrowing_required: bool,
            route_intent: Option<ForgeQueryDeclarationRouteIntent>,
        }

        impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>> $name<D, I> {
            pub fn new(
                candidates: Vec<$candidate>,
                required_aspect_contract: ForgeQueryDeclarationAspectContract,
                allowed_sources: Vec<ForgeQueryBindingSourceKind>,
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

            context_request_common!();

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
                route_intent: ForgeQueryDeclarationRouteIntent,
            ) -> Self {
                self.route_intent = Some(route_intent);
                self
            }

            pub fn route_intent(&self) -> Option<ForgeQueryDeclarationRouteIntent> {
                self.route_intent
            }

            pub(crate) fn into_parts(
                self,
            ) -> (
                Vec<$candidate>,
                ForgeQueryDeclarationAspectContract,
                Vec<ForgeQueryBindingSourceKind>,
                bool,
                bool,
                Option<ForgeQueryDeclarationRouteIntent>,
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

context_request!(ForgeQueryRouteBindingRequest, ForgeQueryProgressionContextCandidate<D, I>);
context_request!(ForgeQueryReceiptBindingRequest, ForgeQueryProgressionContextCandidate<D, I>);
context_request!(ForgeQueryEnvelopeBindingRequest, ForgeQueryProgressionContextCandidate<D, I>);

pub struct ForgeQueryContinuationBindingRequest<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    candidates: Vec<ForgeQueryEnvelopeContextCandidate<D, I>>,
    required_aspect_contract: ForgeQueryDeclarationAspectContract,
    allowed_sources: Vec<ForgeQueryBindingSourceKind>,
    allow_compatible_superset: bool,
    partial_is_narrowing_required: bool,
    bridge_request: Option<ForgeQueryDeclarationBridgeContinuationRequest>,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryContinuationBindingRequest<D, I>
{
    pub fn new(
        candidates: Vec<ForgeQueryEnvelopeContextCandidate<D, I>>,
        required_aspect_contract: ForgeQueryDeclarationAspectContract,
        allowed_sources: Vec<ForgeQueryBindingSourceKind>,
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
        bridge_request: ForgeQueryDeclarationBridgeContinuationRequest,
    ) -> Self {
        self.bridge_request = Some(bridge_request);
        self
    }

    pub fn bridge_request(&self) -> Option<ForgeQueryDeclarationBridgeContinuationRequest> {
        self.bridge_request
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        Vec<ForgeQueryEnvelopeContextCandidate<D, I>>,
        ForgeQueryDeclarationAspectContract,
        Vec<ForgeQueryBindingSourceKind>,
        bool,
        bool,
        Option<ForgeQueryDeclarationBridgeContinuationRequest>,
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

macro_rules! resolver_request {
    ($name:ident, $source:ty) => {
        pub struct $name<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>> {
            source: $source,
            required_aspect_contract: ForgeQueryDeclarationAspectContract,
            allow_compatible_superset: bool,
            partial_is_narrowing_required: bool,
            route_intent: Option<ForgeQueryDeclarationRouteIntent>,
        }

        impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>> $name<D, I> {
            pub fn new(
                source: $source,
                required_aspect_contract: ForgeQueryDeclarationAspectContract,
            ) -> Self {
                Self {
                    source,
                    required_aspect_contract,
                    allow_compatible_superset: true,
                    partial_is_narrowing_required: true,
                    route_intent: None,
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

            pub fn with_route_intent(
                mut self,
                route_intent: ForgeQueryDeclarationRouteIntent,
            ) -> Self {
                self.route_intent = Some(route_intent);
                self
            }

            pub fn route_intent(&self) -> Option<ForgeQueryDeclarationRouteIntent> {
                self.route_intent
            }

            pub(crate) fn into_parts(
                self,
            ) -> (
                $source,
                ForgeQueryDeclarationAspectContract,
                bool,
                bool,
                Option<ForgeQueryDeclarationRouteIntent>,
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

resolver_request!(ForgeQueryResolveRouteFromTargetRequest, ForgeQueryRouteResolverSubject<D, I>);
resolver_request!(ForgeQueryResolveReceiptFromTargetRequest, ForgeQueryReceiptResolverSubject<D, I>);
resolver_request!(ForgeQueryResolveEnvelopeFromTargetRequest, ForgeQueryEnvelopeResolverSubject<D, I>);

pub struct ForgeQueryResolveContinuationFromTargetRequest<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    envelope: crate::application::ForgeQueryDeclarationEnvelope<D, I>,
    required_aspect_contract: ForgeQueryDeclarationAspectContract,
    allow_compatible_superset: bool,
    partial_is_narrowing_required: bool,
    bridge_request: Option<ForgeQueryDeclarationBridgeContinuationRequest>,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryResolveContinuationFromTargetRequest<D, I>
{
    pub fn new(
        envelope: crate::application::ForgeQueryDeclarationEnvelope<D, I>,
        required_aspect_contract: ForgeQueryDeclarationAspectContract,
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
        bridge_request: ForgeQueryDeclarationBridgeContinuationRequest,
    ) -> Self {
        self.bridge_request = Some(bridge_request);
        self
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        crate::application::ForgeQueryDeclarationEnvelope<D, I>,
        ForgeQueryDeclarationAspectContract,
        bool,
        bool,
        Option<ForgeQueryDeclarationBridgeContinuationRequest>,
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
