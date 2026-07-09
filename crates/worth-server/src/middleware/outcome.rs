use worth_proof::TransitionOutcome;

use super::{
    WorthServerAdmission, WorthServerDenial, WorthServerMiddlewareDeferred,
    WorthServerMiddlewareFailure, WorthServerMiddlewareRebindRequired, WorthServerMiddlewareStale,
};

pub type WorthServerAdmissionOutcome = TransitionOutcome<
    WorthServerAdmission,
    WorthServerDenial,
    WorthServerMiddlewareDeferred,
    WorthServerMiddlewareStale,
    WorthServerMiddlewareRebindRequired,
    WorthServerMiddlewareFailure,
>;
