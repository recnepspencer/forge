use worth_proof::TransitionOutcome;

use super::{
    WorthServerQueryHandoff, WorthServerQueryHandoffDeferred, WorthServerQueryHandoffDenial,
    WorthServerQueryHandoffFailure, WorthServerQueryHandoffRebindRequired,
    WorthServerQueryHandoffStale,
};

pub type WorthServerQueryHandoffOutcome = TransitionOutcome<
    WorthServerQueryHandoff,
    WorthServerQueryHandoffDenial,
    WorthServerQueryHandoffDeferred,
    WorthServerQueryHandoffStale,
    WorthServerQueryHandoffRebindRequired,
    WorthServerQueryHandoffFailure,
>;
