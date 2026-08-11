use std::collections::BTreeMap;

use crate::data::temporal::{
    PreviousValueRevision, ReadyTemporalWake, RetiredTemporalWake, RuntimeClockBasis,
    ScheduledTemporalWake, TemporalWakeId, TemporalWakeOwner, WakeOrdinal,
};

mod admission;
mod clock;
mod frontier;
mod lifecycle;
mod observation;
mod previous_value;
mod restoration;
mod retirement;

/// Runtime-owned temporal state for authoritative clock basis semantics and wake lifecycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::logic::transaction::runtime) struct TemporalRuntimeState {
    pub(super) clock_basis: RuntimeClockBasis,
    pub(super) previous_value_capability_epoch: u64,
    pub(super) next_wake_id: TemporalWakeId,
    pub(super) next_wake_ordinal: WakeOrdinal,
    pub(super) next_previous_value_revision: PreviousValueRevision,
    pub(super) scheduled_wakes: BTreeMap<TemporalWakeId, ScheduledTemporalWake>,
    pub(super) scheduled_frontier:
        BTreeMap<crate::data::temporal::ClockTick, BTreeMap<WakeOrdinal, TemporalWakeId>>,
    pub(super) ready_wakes: BTreeMap<TemporalWakeId, ReadyTemporalWake>,
    pub(super) ready_frontier: BTreeMap<WakeOrdinal, TemporalWakeId>,
    pub(super) owner_frontier: BTreeMap<TemporalWakeOwner, BTreeMap<WakeOrdinal, TemporalWakeId>>,
    pub(super) retired_wakes: BTreeMap<TemporalWakeId, RetiredTemporalWake>,
}

impl Default for TemporalRuntimeState {
    fn default() -> Self {
        Self {
            clock_basis: RuntimeClockBasis::default(),
            previous_value_capability_epoch: 0,
            next_wake_id: TemporalWakeId::new(0),
            next_wake_ordinal: WakeOrdinal::ZERO,
            next_previous_value_revision: PreviousValueRevision::ZERO,
            scheduled_wakes: BTreeMap::new(),
            scheduled_frontier: BTreeMap::new(),
            ready_wakes: BTreeMap::new(),
            ready_frontier: BTreeMap::new(),
            owner_frontier: BTreeMap::new(),
            retired_wakes: BTreeMap::new(),
        }
    }
}
