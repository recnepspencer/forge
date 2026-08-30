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
    pub(super) scheduled_wakes: im::OrdMap<TemporalWakeId, ScheduledTemporalWake>,
    pub(super) scheduled_frontier:
        im::OrdMap<crate::data::temporal::ClockTick, im::OrdMap<WakeOrdinal, TemporalWakeId>>,
    pub(super) ready_wakes: im::OrdMap<TemporalWakeId, ReadyTemporalWake>,
    pub(super) ready_frontier: im::OrdMap<WakeOrdinal, TemporalWakeId>,
    pub(super) owner_frontier:
        im::OrdMap<TemporalWakeOwner, im::OrdMap<WakeOrdinal, TemporalWakeId>>,
    pub(super) retired_wakes: im::OrdMap<TemporalWakeId, RetiredTemporalWake>,
}

impl Default for TemporalRuntimeState {
    fn default() -> Self {
        Self {
            clock_basis: RuntimeClockBasis::default(),
            previous_value_capability_epoch: 0,
            next_wake_id: TemporalWakeId::new(0),
            next_wake_ordinal: WakeOrdinal::ZERO,
            next_previous_value_revision: PreviousValueRevision::ZERO,
            scheduled_wakes: im::OrdMap::new(),
            scheduled_frontier: im::OrdMap::new(),
            ready_wakes: im::OrdMap::new(),
            ready_frontier: im::OrdMap::new(),
            owner_frontier: im::OrdMap::new(),
            retired_wakes: im::OrdMap::new(),
        }
    }
}

#[cfg(test)]
impl TemporalRuntimeState {
    pub(super) fn shares_storage_with(&self, other: &Self) -> bool {
        self.scheduled_wakes.ptr_eq(&other.scheduled_wakes)
            && self.scheduled_frontier.ptr_eq(&other.scheduled_frontier)
            && self.ready_wakes.ptr_eq(&other.ready_wakes)
            && self.ready_frontier.ptr_eq(&other.ready_frontier)
            && self.owner_frontier.ptr_eq(&other.owner_frontier)
            && self.retired_wakes.ptr_eq(&other.retired_wakes)
    }
}
