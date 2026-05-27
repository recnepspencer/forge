use crate::application::{
    ForgeQueryDeclarationBridgeContinuationFamily, ForgeQueryDeclarationBridgeContinuationRequest,
    ForgeQueryDeclarationEnvelope, ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationBridgeRoutingDenialCause {
    EnvelopeNotCoveredForBridgeRouting,
    NonBridgeRoutePlan,
    UnsupportedContinuationMode,
    UnsupportedTruthContext,
    BridgeAuthorityUnavailable,
    MissingRequiredAspect,
    AspectConflict,
    AuthorityAspectGap,
    AuthorityAspectAmbiguity,
    BridgeEnvelopeMismatch,
    BasisLifecycleMismatch,
}

impl ForgeQueryDeclarationBridgeRoutingDenialCause {
    pub fn reason(self) -> &'static str {
        match self {
            Self::EnvelopeNotCoveredForBridgeRouting => {
                "bridge continuation routing starts from covered envelope truth rather than non-success crossing posture"
            }
            Self::NonBridgeRoutePlan => {
                "the retained route plan does not currently admit a bridge slice"
            }
            Self::UnsupportedContinuationMode => {
                "this declaration family does not expose a supported bridge continuation-routing contract"
            }
            Self::UnsupportedTruthContext => {
                "the requested truth context is not supported for this bridge continuation family"
            }
            Self::BridgeAuthorityUnavailable => {
                "required bridge continuation capabilities are unavailable in this operating world"
            }
            Self::MissingRequiredAspect => {
                "the retained envelope publication does not expose the required bridge semantic slice"
            }
            Self::AspectConflict => {
                "the retained envelope publication conflicts with the required bridge semantic slice"
            }
            Self::AuthorityAspectGap => {
                "the retained envelope publication only partially covers the required bridge semantic slice"
            }
            Self::AuthorityAspectAmbiguity => {
                "multiple bridge mappings claim the same retained semantic slice"
            }
            Self::BridgeEnvelopeMismatch => {
                "the retained envelope truth and bridge continuation boundary expectations no longer agree"
            }
            Self::BasisLifecycleMismatch => {
                "the retained basis lifecycle posture does not satisfy this bridge continuation mode"
            }
        }
    }
}

macro_rules! define_bridge_terminal {
    ($name:ident) => {
        pub struct $name<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>> {
            envelope: ForgeQueryDeclarationEnvelope<D, I>,
            reason: &'static str,
        }

        impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>> $name<D, I> {
            pub(crate) fn new(
                envelope: ForgeQueryDeclarationEnvelope<D, I>,
                reason: &'static str,
            ) -> Self {
                Self { envelope, reason }
            }

            pub fn envelope(&self) -> &ForgeQueryDeclarationEnvelope<D, I> {
                &self.envelope
            }

            pub fn reason(&self) -> &'static str {
                self.reason
            }

            pub(crate) fn into_envelope(self) -> ForgeQueryDeclarationEnvelope<D, I> {
                self.envelope
            }
        }
    };
}

define_bridge_terminal!(ForgeQueryDeclarationBridgeRoutingDeferred);
define_bridge_terminal!(ForgeQueryDeclarationBridgeRoutingFailed);

pub struct ForgeQueryDeclarationBridgeRoutingDenied<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    envelope: ForgeQueryDeclarationEnvelope<D, I>,
    continuation_request: Option<ForgeQueryDeclarationBridgeContinuationRequest>,
    continuation_family: Option<ForgeQueryDeclarationBridgeContinuationFamily>,
    cause: ForgeQueryDeclarationBridgeRoutingDenialCause,
    reason: &'static str,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationBridgeRoutingDenied<D, I>
{
    pub(crate) fn new(
        envelope: ForgeQueryDeclarationEnvelope<D, I>,
        continuation_request: Option<ForgeQueryDeclarationBridgeContinuationRequest>,
        continuation_family: Option<ForgeQueryDeclarationBridgeContinuationFamily>,
        cause: ForgeQueryDeclarationBridgeRoutingDenialCause,
    ) -> Self {
        Self {
            envelope,
            continuation_request,
            continuation_family,
            cause,
            reason: cause.reason(),
        }
    }

    pub fn envelope(&self) -> &ForgeQueryDeclarationEnvelope<D, I> {
        &self.envelope
    }

    pub fn cause(&self) -> ForgeQueryDeclarationBridgeRoutingDenialCause {
        self.cause
    }

    pub fn continuation_request(&self) -> Option<ForgeQueryDeclarationBridgeContinuationRequest> {
        self.continuation_request
    }

    pub fn continuation_family(&self) -> Option<ForgeQueryDeclarationBridgeContinuationFamily> {
        self.continuation_family
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }

    pub(crate) fn into_envelope(self) -> ForgeQueryDeclarationEnvelope<D, I> {
        self.envelope
    }
}

pub enum ForgeQueryDeclarationBridgeRoutingTerminalError<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Deferred(ForgeQueryDeclarationBridgeRoutingDeferred<D, I>),
    Denied(ForgeQueryDeclarationBridgeRoutingDenied<D, I>),
    Failed(ForgeQueryDeclarationBridgeRoutingFailed<D, I>),
}

pub enum ForgeQueryDeclarationEntryBridgeRoutingError<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Entry(crate::application::ForgeQueryDeclarationEntryEnvelopeError<D, I>),
    Routing(ForgeQueryDeclarationBridgeRoutingTerminalError<D, I>),
}
