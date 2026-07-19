mod domain;
mod proof;

pub(super) use domain::{
    admitted_handle, AspectRichEnvelopeFamily, EnvelopeInput, FailedEnvelopeFamily,
    MixedEnvelopeFamily, RelationalEnvelopeFamily, RequiredIntentEnvelopeFamily,
    SignalEnvelopeFamily,
};
pub(super) use proof::{
    envelope_checked_from_receipt, progressed, route_checked_from_input, route_checked_with_intent,
    DeferredEnvelopeFamily,
};
