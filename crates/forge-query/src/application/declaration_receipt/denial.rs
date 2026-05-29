use crate::application::{
    ForgeQueryDeclarationInput, ForgeQueryDeclarationRouteIntent,
    ForgeQueryDeclarationRoutePlanDenialCause, ForgeQueryDomainEntryMarker,
};

use super::artifact::ForgeQueryDeclarationReceipt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationReceiptDenialCause {
    MissingRoutePlan,
    UnsupportedReceiptKind,
    ReceiptMaterializationMismatch,
    RouteIntegrityMismatch,
}

impl ForgeQueryDeclarationReceiptDenialCause {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingRoutePlan => "missing_route_plan",
            Self::UnsupportedReceiptKind => "unsupported_receipt_kind",
            Self::ReceiptMaterializationMismatch => "receipt_materialization_mismatch",
            Self::RouteIntegrityMismatch => "route_integrity_mismatch",
        }
    }

    pub fn reason(&self) -> &'static str {
        match self {
            Self::MissingRoutePlan => {
                "receipt construction requires retained route truth rather than loose declaration evidence"
            }
            Self::UnsupportedReceiptKind => {
                "this declaration route kind is not yet a supported Query receipt crossing"
            }
            Self::ReceiptMaterializationMismatch => {
                "the receipt boundary could not materialize a matching foundational receipt artifact"
            }
            Self::RouteIntegrityMismatch => {
                "the retained route proof and receipt boundary truth no longer agree"
            }
        }
    }
}

macro_rules! define_receipt_terminal {
    ($name:ident) => {
        pub struct $name<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>> {
            receipt: ForgeQueryDeclarationReceipt<D, I>,
            route_intent: Option<ForgeQueryDeclarationRouteIntent>,
            reason: &'static str,
        }

        impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>> $name<D, I> {
            pub(crate) fn new(
                receipt: ForgeQueryDeclarationReceipt<D, I>,
                route_intent: Option<ForgeQueryDeclarationRouteIntent>,
                reason: &'static str,
            ) -> Self {
                Self {
                    receipt,
                    route_intent,
                    reason,
                }
            }

            pub fn receipt(&self) -> &ForgeQueryDeclarationReceipt<D, I> {
                &self.receipt
            }

            pub fn route_intent(&self) -> Option<ForgeQueryDeclarationRouteIntent> {
                self.route_intent
            }

            pub fn reason(&self) -> &'static str {
                self.reason
            }
        }
    };
}

define_receipt_terminal!(ForgeQueryDeclarationReceiptDeferred);
define_receipt_terminal!(ForgeQueryDeclarationReceiptFailed);

pub struct ForgeQueryDeclarationReceiptDenied<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    receipt: ForgeQueryDeclarationReceipt<D, I>,
    route_intent: Option<ForgeQueryDeclarationRouteIntent>,
    route_cause: Option<ForgeQueryDeclarationRoutePlanDenialCause>,
    receipt_cause: Option<ForgeQueryDeclarationReceiptDenialCause>,
    reason: &'static str,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationReceiptDenied<D, I>
{
    pub(crate) fn from_route_cause(
        receipt: ForgeQueryDeclarationReceipt<D, I>,
        route_intent: Option<ForgeQueryDeclarationRouteIntent>,
        cause: ForgeQueryDeclarationRoutePlanDenialCause,
    ) -> Self {
        Self {
            receipt,
            route_intent,
            route_cause: Some(cause),
            receipt_cause: None,
            reason: cause.reason(),
        }
    }

    pub(crate) fn from_receipt_cause(
        receipt: ForgeQueryDeclarationReceipt<D, I>,
        route_intent: Option<ForgeQueryDeclarationRouteIntent>,
        cause: ForgeQueryDeclarationReceiptDenialCause,
    ) -> Self {
        Self {
            receipt,
            route_intent,
            route_cause: None,
            receipt_cause: Some(cause),
            reason: cause.reason(),
        }
    }

    pub fn receipt(&self) -> &ForgeQueryDeclarationReceipt<D, I> {
        &self.receipt
    }

    pub fn route_intent(&self) -> Option<ForgeQueryDeclarationRouteIntent> {
        self.route_intent
    }

    pub fn route_cause(&self) -> Option<ForgeQueryDeclarationRoutePlanDenialCause> {
        self.route_cause
    }

    pub fn receipt_cause(&self) -> Option<ForgeQueryDeclarationReceiptDenialCause> {
        self.receipt_cause
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }
}

pub enum ForgeQueryDeclarationReceiptTerminalError<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Deferred(ForgeQueryDeclarationReceiptDeferred<D, I>),
    Denied(ForgeQueryDeclarationReceiptDenied<D, I>),
    Failed(ForgeQueryDeclarationReceiptFailed<D, I>),
}

pub enum ForgeQueryDeclarationEntryReceiptError<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Entry(crate::application::ForgeQueryDeclarationEntryProgressionError<D, I>),
    Receipt(ForgeQueryDeclarationReceiptTerminalError<D, I>),
}
