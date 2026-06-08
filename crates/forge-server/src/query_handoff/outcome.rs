use forge_proof::TransitionOutcome;

use super::{
    ForgeServerQueryHandoff, ForgeServerQueryHandoffDeferred, ForgeServerQueryHandoffDenial,
    ForgeServerQueryHandoffFailure, ForgeServerQueryHandoffRebindRequired,
    ForgeServerQueryHandoffStale,
};

pub type ForgeServerQueryHandoffOutcome = TransitionOutcome<
    ForgeServerQueryHandoff,
    ForgeServerQueryHandoffDenial,
    ForgeServerQueryHandoffDeferred,
    ForgeServerQueryHandoffStale,
    ForgeServerQueryHandoffRebindRequired,
    ForgeServerQueryHandoffFailure,
>;
