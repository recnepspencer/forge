use crate::application::{
    ForgeQueryDeclarationEnvelopeChecked, ForgeQueryDeclarationEnvelopeTerminalError,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationReceiptChecked,
    ForgeQueryDeclarationReceiptTerminalError, ForgeQueryDeclarationRoutePlanChecked,
    ForgeQueryDeclarationRoutePlanTerminalError, ForgeQueryDomainEntryMarker,
};

pub(crate) fn route_orchestration_identity<D, I>(
    checked: &ForgeQueryDeclarationRoutePlanChecked<D, I>,
) -> String
where
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
{
    match checked {
        ForgeQueryDeclarationRoutePlanChecked::Planned(plan) => {
            format!("planned:{}", plan.route_plan_digest())
        }
        ForgeQueryDeclarationRoutePlanChecked::Deferred(value) => {
            format!(
                "deferred:{}",
                value.foundational_evidence().support_digest()
            )
        }
        ForgeQueryDeclarationRoutePlanChecked::Denied(value) => {
            format!(
                "denied:{}:{}",
                value.cause().as_str(),
                value.foundational_evidence().support_digest()
            )
        }
        ForgeQueryDeclarationRoutePlanChecked::Failed(value) => {
            format!("failed:{}", value.foundational_evidence().support_digest())
        }
    }
}

pub(crate) fn receipt_orchestration_identity<D, I>(
    checked: &ForgeQueryDeclarationReceiptChecked<D, I>,
) -> String
where
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
{
    match checked {
        ForgeQueryDeclarationReceiptChecked::Issued(receipt) => {
            format!("issued:{:?}", receipt.receipt_digest())
        }
        ForgeQueryDeclarationReceiptChecked::Deferred(value) => {
            format!("deferred:{:?}", value.receipt().receipt_digest())
        }
        ForgeQueryDeclarationReceiptChecked::Denied(value) => {
            format!("denied:{:?}", value.receipt().receipt_digest())
        }
        ForgeQueryDeclarationReceiptChecked::Failed(value) => {
            format!("failed:{:?}", value.receipt().receipt_digest())
        }
    }
}

pub(crate) fn envelope_orchestration_identity<D, I>(
    checked: &ForgeQueryDeclarationEnvelopeChecked<D, I>,
) -> String
where
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
{
    match checked {
        ForgeQueryDeclarationEnvelopeChecked::Enveloped(envelope) => {
            format!("enveloped:{:?}", envelope.envelope_digest())
        }
        ForgeQueryDeclarationEnvelopeChecked::Deferred(value) => {
            format!("deferred:{:?}", value.envelope().envelope_digest())
        }
        ForgeQueryDeclarationEnvelopeChecked::Denied(value) => {
            format!("denied:{:?}", value.envelope().envelope_digest())
        }
        ForgeQueryDeclarationEnvelopeChecked::Failed(value) => {
            format!("failed:{:?}", value.envelope().envelope_digest())
        }
    }
}

pub(crate) fn route_terminal_from_checked<D, I>(
    checked: ForgeQueryDeclarationRoutePlanChecked<D, I>,
) -> ForgeQueryDeclarationRoutePlanTerminalError<D, I>
where
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
{
    match checked {
        ForgeQueryDeclarationRoutePlanChecked::Planned(_) => {
            panic!("planned route should not be lowered into terminal error")
        }
        ForgeQueryDeclarationRoutePlanChecked::Deferred(value) => {
            ForgeQueryDeclarationRoutePlanTerminalError::Deferred(value)
        }
        ForgeQueryDeclarationRoutePlanChecked::Denied(value) => {
            ForgeQueryDeclarationRoutePlanTerminalError::Denied(value)
        }
        ForgeQueryDeclarationRoutePlanChecked::Failed(value) => {
            ForgeQueryDeclarationRoutePlanTerminalError::Failed(value)
        }
    }
}

pub(crate) fn receipt_terminal_from_checked<D, I>(
    checked: ForgeQueryDeclarationReceiptChecked<D, I>,
) -> ForgeQueryDeclarationReceiptTerminalError<D, I>
where
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
{
    match checked {
        ForgeQueryDeclarationReceiptChecked::Issued(_) => {
            panic!("issued receipt should not be lowered into terminal error")
        }
        ForgeQueryDeclarationReceiptChecked::Deferred(value) => {
            ForgeQueryDeclarationReceiptTerminalError::Deferred(value)
        }
        ForgeQueryDeclarationReceiptChecked::Denied(value) => {
            ForgeQueryDeclarationReceiptTerminalError::Denied(value)
        }
        ForgeQueryDeclarationReceiptChecked::Failed(value) => {
            ForgeQueryDeclarationReceiptTerminalError::Failed(value)
        }
    }
}

pub(crate) fn envelope_terminal_from_checked<D, I>(
    checked: ForgeQueryDeclarationEnvelopeChecked<D, I>,
) -> ForgeQueryDeclarationEnvelopeTerminalError<D, I>
where
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
{
    match checked {
        ForgeQueryDeclarationEnvelopeChecked::Enveloped(_) => {
            panic!("enveloped result should not be lowered into terminal error")
        }
        ForgeQueryDeclarationEnvelopeChecked::Deferred(value) => {
            ForgeQueryDeclarationEnvelopeTerminalError::Deferred(value)
        }
        ForgeQueryDeclarationEnvelopeChecked::Denied(value) => {
            ForgeQueryDeclarationEnvelopeTerminalError::Denied(value)
        }
        ForgeQueryDeclarationEnvelopeChecked::Failed(value) => {
            ForgeQueryDeclarationEnvelopeTerminalError::Failed(value)
        }
    }
}
