use crate::application::{
    ForgeQueryDeclarationEnvelope, ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationRelationalRoutingDenialCause {
    EnvelopeNotCoveredForRelationalRouting,
    NonRelationalRoutePlan,
    UnsupportedRelationalTruthClaim,
    RelationalAuthorityUnavailable,
    RelationalEnvelopeMismatch,
}

impl ForgeQueryDeclarationRelationalRoutingDenialCause {
    pub fn reason(self) -> &'static str {
        match self {
            Self::EnvelopeNotCoveredForRelationalRouting => {
                "relational truth routing starts from covered envelope truth rather than non-success crossing posture"
            }
            Self::NonRelationalRoutePlan => {
                "the retained route plan does not currently admit a relational slice"
            }
            Self::UnsupportedRelationalTruthClaim => {
                "this declaration family does not expose a supported relational truth-routing contract"
            }
            Self::RelationalAuthorityUnavailable => {
                "required relational authority capabilities are unavailable in this operating world"
            }
            Self::RelationalEnvelopeMismatch => {
                "the retained envelope truth and relational boundary expectations no longer agree"
            }
        }
    }
}

macro_rules! define_relational_terminal {
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

define_relational_terminal!(ForgeQueryDeclarationRelationalRoutingDeferred);
define_relational_terminal!(ForgeQueryDeclarationRelationalRoutingFailed);

pub struct ForgeQueryDeclarationRelationalRoutingDenied<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    envelope: ForgeQueryDeclarationEnvelope<D, I>,
    cause: ForgeQueryDeclarationRelationalRoutingDenialCause,
    reason: &'static str,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationRelationalRoutingDenied<D, I>
{
    pub(crate) fn new(
        envelope: ForgeQueryDeclarationEnvelope<D, I>,
        cause: ForgeQueryDeclarationRelationalRoutingDenialCause,
    ) -> Self {
        Self {
            envelope,
            cause,
            reason: cause.reason(),
        }
    }

    pub fn envelope(&self) -> &ForgeQueryDeclarationEnvelope<D, I> {
        &self.envelope
    }

    pub fn cause(&self) -> ForgeQueryDeclarationRelationalRoutingDenialCause {
        self.cause
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }

    pub(crate) fn into_envelope(self) -> ForgeQueryDeclarationEnvelope<D, I> {
        self.envelope
    }
}

pub enum ForgeQueryDeclarationRelationalRoutingTerminalError<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Deferred(ForgeQueryDeclarationRelationalRoutingDeferred<D, I>),
    Denied(ForgeQueryDeclarationRelationalRoutingDenied<D, I>),
    Failed(ForgeQueryDeclarationRelationalRoutingFailed<D, I>),
}

pub enum ForgeQueryDeclarationEntryRelationalRoutingError<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Entry(crate::application::ForgeQueryDeclarationEntryEnvelopeError<D, I>),
    Routing(ForgeQueryDeclarationRelationalRoutingTerminalError<D, I>),
}
