use serde::{Deserialize, Serialize};

use super::{ClockTick, TemporalWakeId, WakeOrdinal};

/// Cost-honest snapshot of the indexed temporal frontier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TemporalFrontierSnapshot {
    scheduled_frontier_width: u64,
    ready_frontier_width: u64,
    next_due_tick: Option<ClockTick>,
    next_due_wake_id: Option<TemporalWakeId>,
    next_due_wake_ordinal: Option<WakeOrdinal>,
    next_ready_wake_id: Option<TemporalWakeId>,
    next_ready_wake_ordinal: Option<WakeOrdinal>,
}

impl TemporalFrontierSnapshot {
    pub(crate) fn new(
        scheduled_frontier_width: usize,
        ready_frontier_width: usize,
        next_due_tick: Option<ClockTick>,
        next_due_wake_id: Option<TemporalWakeId>,
        next_due_wake_ordinal: Option<WakeOrdinal>,
        next_ready_wake_id: Option<TemporalWakeId>,
        next_ready_wake_ordinal: Option<WakeOrdinal>,
    ) -> Self {
        Self {
            scheduled_frontier_width: scheduled_frontier_width as u64,
            ready_frontier_width: ready_frontier_width as u64,
            next_due_tick,
            next_due_wake_id,
            next_due_wake_ordinal,
            next_ready_wake_id,
            next_ready_wake_ordinal,
        }
    }

    pub fn scheduled_frontier_width(self) -> u64 {
        self.scheduled_frontier_width
    }

    pub fn ready_frontier_width(self) -> u64 {
        self.ready_frontier_width
    }

    pub fn next_due_tick(self) -> Option<ClockTick> {
        self.next_due_tick
    }

    pub fn next_due_wake_id(self) -> Option<TemporalWakeId> {
        self.next_due_wake_id
    }

    pub fn next_due_wake_ordinal(self) -> Option<WakeOrdinal> {
        self.next_due_wake_ordinal
    }

    pub fn next_ready_wake_id(self) -> Option<TemporalWakeId> {
        self.next_ready_wake_id
    }

    pub fn next_ready_wake_ordinal(self) -> Option<WakeOrdinal> {
        self.next_ready_wake_ordinal
    }
}
