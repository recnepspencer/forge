use crate::application::{
    WorthQueryDeclarationEnvelopeChecked, WorthQueryDeclarationEnvelopeTerminalError,
    WorthQueryDeclarationInput, WorthQueryDeclarationReceiptChecked,
    WorthQueryDeclarationReceiptTerminalError, WorthQueryDeclarationRoutePlanChecked,
    WorthQueryDeclarationRoutePlanTerminalError, WorthQueryDomainEntryMarker,
};

pub(crate) fn route_orchestration_identity<D, I>(
    checked: &WorthQueryDeclarationRoutePlanChecked<D, I>,
) -> String
where
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
{
    match checked {
        WorthQueryDeclarationRoutePlanChecked::Planned(plan) => {
            format!("planned:{}", plan.route_plan_digest())
        }
        WorthQueryDeclarationRoutePlanChecked::Deferred(value) => {
            format!(
                "deferred:{}",
                value.foundational_evidence().support_digest()
            )
        }
        WorthQueryDeclarationRoutePlanChecked::Denied(value) => {
            format!(
                "denied:{}:{}",
                value.cause().as_str(),
                value.foundational_evidence().support_digest()
            )
        }
        WorthQueryDeclarationRoutePlanChecked::Failed(value) => {
            format!("failed:{}", value.foundational_evidence().support_digest())
        }
    }
}

pub(crate) fn receipt_orchestration_identity<D, I>(
    checked: &WorthQueryDeclarationReceiptChecked<D, I>,
) -> String
where
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
{
    match checked {
        WorthQueryDeclarationReceiptChecked::Issued(receipt) => {
            format!("issued:{:?}", receipt.receipt_digest())
        }
        WorthQueryDeclarationReceiptChecked::Deferred(value) => {
            format!("deferred:{:?}", value.receipt().receipt_digest())
        }
        WorthQueryDeclarationReceiptChecked::Denied(value) => {
            format!("denied:{:?}", value.receipt().receipt_digest())
        }
        WorthQueryDeclarationReceiptChecked::Failed(value) => {
            format!("failed:{:?}", value.receipt().receipt_digest())
        }
    }
}

pub(crate) fn envelope_orchestration_identity<D, I>(
    checked: &WorthQueryDeclarationEnvelopeChecked<D, I>,
) -> String
where
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
{
    match checked {
        WorthQueryDeclarationEnvelopeChecked::Enveloped(envelope) => {
            format!("enveloped:{:?}", envelope.envelope_digest())
        }
        WorthQueryDeclarationEnvelopeChecked::Deferred(value) => {
            format!("deferred:{:?}", value.envelope().envelope_digest())
        }
        WorthQueryDeclarationEnvelopeChecked::Denied(value) => {
            format!("denied:{:?}", value.envelope().envelope_digest())
        }
        WorthQueryDeclarationEnvelopeChecked::Failed(value) => {
            format!("failed:{:?}", value.envelope().envelope_digest())
        }
    }
}

pub(crate) fn route_terminal_from_checked<D, I>(
    checked: WorthQueryDeclarationRoutePlanChecked<D, I>,
) -> WorthQueryDeclarationRoutePlanTerminalError<D, I>
where
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
{
    match checked {
        WorthQueryDeclarationRoutePlanChecked::Planned(_) => {
            panic!("planned route should not be lowered into terminal error")
        }
        WorthQueryDeclarationRoutePlanChecked::Deferred(value) => {
            WorthQueryDeclarationRoutePlanTerminalError::Deferred(value)
        }
        WorthQueryDeclarationRoutePlanChecked::Denied(value) => {
            WorthQueryDeclarationRoutePlanTerminalError::Denied(value)
        }
        WorthQueryDeclarationRoutePlanChecked::Failed(value) => {
            WorthQueryDeclarationRoutePlanTerminalError::Failed(value)
        }
    }
}

pub(crate) fn receipt_terminal_from_checked<D, I>(
    checked: WorthQueryDeclarationReceiptChecked<D, I>,
) -> WorthQueryDeclarationReceiptTerminalError<D, I>
where
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
{
    match checked {
        WorthQueryDeclarationReceiptChecked::Issued(_) => {
            panic!("issued receipt should not be lowered into terminal error")
        }
        WorthQueryDeclarationReceiptChecked::Deferred(value) => {
            WorthQueryDeclarationReceiptTerminalError::Deferred(value)
        }
        WorthQueryDeclarationReceiptChecked::Denied(value) => {
            WorthQueryDeclarationReceiptTerminalError::Denied(value)
        }
        WorthQueryDeclarationReceiptChecked::Failed(value) => {
            WorthQueryDeclarationReceiptTerminalError::Failed(value)
        }
    }
}

pub(crate) fn envelope_terminal_from_checked<D, I>(
    checked: WorthQueryDeclarationEnvelopeChecked<D, I>,
) -> WorthQueryDeclarationEnvelopeTerminalError<D, I>
where
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
{
    match checked {
        WorthQueryDeclarationEnvelopeChecked::Enveloped(_) => {
            panic!("enveloped result should not be lowered into terminal error")
        }
        WorthQueryDeclarationEnvelopeChecked::Deferred(value) => {
            WorthQueryDeclarationEnvelopeTerminalError::Deferred(value)
        }
        WorthQueryDeclarationEnvelopeChecked::Denied(value) => {
            WorthQueryDeclarationEnvelopeTerminalError::Denied(value)
        }
        WorthQueryDeclarationEnvelopeChecked::Failed(value) => {
            WorthQueryDeclarationEnvelopeTerminalError::Failed(value)
        }
    }
}
