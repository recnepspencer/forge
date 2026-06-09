use forge_proof::TransitionOutcome;

use super::{
    ForgeServerAdmission, ForgeServerDenial, ForgeServerMiddlewareDeferred,
    ForgeServerMiddlewareFailure, ForgeServerMiddlewareRebindRequired, ForgeServerMiddlewareStale,
};

pub type ForgeServerAdmissionOutcome = TransitionOutcome<
    ForgeServerAdmission,
    ForgeServerDenial,
    ForgeServerMiddlewareDeferred,
    ForgeServerMiddlewareStale,
    ForgeServerMiddlewareRebindRequired,
    ForgeServerMiddlewareFailure,
>;
