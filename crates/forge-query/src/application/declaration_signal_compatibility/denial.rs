use crate::application::{
    ForgeQueryDeclarationEnvelope, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationSignalExecutionFamily, ForgeQueryDomainEntryMarker,
};
use crate::basis_lifecycle::BasisFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationSignalCompatibilityDenialCause {
    EnvelopeNotCoveredForSignalCompatibility,
    SignalFamilyUnsupported,
    SignalBasisMismatch,
    MissingRequiredAspect,
    AspectConflict,
    AuthorityAspectGap,
    SignalCompatibilityMismatch,
    SignalExecutionFamilyUnavailable,
}

impl ForgeQueryDeclarationSignalCompatibilityDenialCause {
    pub fn reason(self) -> &'static str {
        match self {
            Self::EnvelopeNotCoveredForSignalCompatibility => {
                "signal compatibility starts from covered envelope truth rather than non-success crossing posture"
            }
            Self::SignalFamilyUnsupported => {
                "this declaration family is not structurally signal-compatible"
            }
            Self::SignalBasisMismatch => {
                "the retained envelope truth does not currently satisfy the required basis-sensitive signal continuation posture"
            }
            Self::MissingRequiredAspect => {
                "the retained envelope publication does not expose the required signal dependency slice"
            }
            Self::AspectConflict => {
                "the retained envelope publication conflicts with the required signal dependency slice"
            }
            Self::AuthorityAspectGap => {
                "the retained envelope publication only partially covers the required signal dependency slice"
            }
            Self::SignalCompatibilityMismatch => {
                "the retained envelope truth and signal compatibility boundary expectations no longer agree"
            }
            Self::SignalExecutionFamilyUnavailable => {
                "this declaration family does not expose a supported signal compatibility contract"
            }
        }
    }
}

macro_rules! define_signal_terminal {
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

define_signal_terminal!(ForgeQueryDeclarationSignalCompatibilityDeferred);
define_signal_terminal!(ForgeQueryDeclarationSignalCompatibilityFailed);

pub struct ForgeQueryDeclarationSignalCompatibilityDenied<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    envelope: ForgeQueryDeclarationEnvelope<D, I>,
    execution_family: Option<ForgeQueryDeclarationSignalExecutionFamily>,
    basis_families: Vec<BasisFamily>,
    cause: ForgeQueryDeclarationSignalCompatibilityDenialCause,
    reason: &'static str,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationSignalCompatibilityDenied<D, I>
{
    pub(crate) fn new(
        envelope: ForgeQueryDeclarationEnvelope<D, I>,
        execution_family: Option<ForgeQueryDeclarationSignalExecutionFamily>,
        basis_families: Vec<BasisFamily>,
        cause: ForgeQueryDeclarationSignalCompatibilityDenialCause,
    ) -> Self {
        Self {
            envelope,
            execution_family,
            basis_families,
            cause,
            reason: cause.reason(),
        }
    }

    pub fn envelope(&self) -> &ForgeQueryDeclarationEnvelope<D, I> {
        &self.envelope
    }

    pub fn cause(&self) -> ForgeQueryDeclarationSignalCompatibilityDenialCause {
        self.cause
    }

    pub fn execution_family(&self) -> Option<ForgeQueryDeclarationSignalExecutionFamily> {
        self.execution_family
    }

    pub fn basis_families(&self) -> &[BasisFamily] {
        &self.basis_families
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }

    pub(crate) fn into_envelope(self) -> ForgeQueryDeclarationEnvelope<D, I> {
        self.envelope
    }
}

pub enum ForgeQueryDeclarationSignalCompatibilityTerminalError<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Deferred(ForgeQueryDeclarationSignalCompatibilityDeferred<D, I>),
    Denied(ForgeQueryDeclarationSignalCompatibilityDenied<D, I>),
    Failed(ForgeQueryDeclarationSignalCompatibilityFailed<D, I>),
}

pub enum ForgeQueryDeclarationEntrySignalCompatibilityError<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Entry(crate::application::ForgeQueryDeclarationEntryEnvelopeError<D, I>),
    Compatibility(ForgeQueryDeclarationSignalCompatibilityTerminalError<D, I>),
}
