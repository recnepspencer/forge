use serde::{Deserialize, Serialize};

use super::{ClockTick, ReadyTemporalWake, TemporalWakeId, ValidatedClockAdvance, WakeOrdinal};

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

/// Cost-honest summary of an accepted clock advance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemporalClockAdvanceSummary {
    validated_advance: ValidatedClockAdvance,
    frontier_before: TemporalFrontierSnapshot,
    frontier_after: TemporalFrontierSnapshot,
    promoted_wake_count: u64,
    ready_selection_deferred: bool,
}

impl TemporalClockAdvanceSummary {
    pub(crate) fn new(
        validated_advance: ValidatedClockAdvance,
        frontier_before: TemporalFrontierSnapshot,
        frontier_after: TemporalFrontierSnapshot,
    ) -> Self {
        Self {
            validated_advance,
            frontier_before,
            frontier_after,
            promoted_wake_count: 0,
            ready_selection_deferred: true,
        }
    }

    pub fn validated_advance(&self) -> ValidatedClockAdvance {
        self.validated_advance
    }

    pub fn frontier_before(&self) -> TemporalFrontierSnapshot {
        self.frontier_before
    }

    pub fn frontier_after(&self) -> TemporalFrontierSnapshot {
        self.frontier_after
    }

    pub fn promoted_wake_count(&self) -> u64 {
        self.promoted_wake_count
    }

    pub fn ready_selection_deferred(&self) -> bool {
        self.ready_selection_deferred
    }
}

/// Cost-honest summary of due-wake promotion over the temporal frontier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemporalReadyPromotionSummary {
    frontier_before: TemporalFrontierSnapshot,
    frontier_after: TemporalFrontierSnapshot,
    ready_wakes: Vec<ReadyTemporalWake>,
    promoted_wake_count: u64,
    broad_scan_denial_count_delta: u64,
}

impl TemporalReadyPromotionSummary {
    pub(crate) fn new(
        frontier_before: TemporalFrontierSnapshot,
        frontier_after: TemporalFrontierSnapshot,
        ready_wakes: Vec<ReadyTemporalWake>,
        broad_scan_denial_count_delta: u64,
    ) -> Self {
        let promoted_wake_count = ready_wakes.len() as u64;
        Self {
            frontier_before,
            frontier_after,
            ready_wakes,
            promoted_wake_count,
            broad_scan_denial_count_delta,
        }
    }

    pub fn frontier_before(&self) -> TemporalFrontierSnapshot {
        self.frontier_before
    }

    pub fn frontier_after(&self) -> TemporalFrontierSnapshot {
        self.frontier_after
    }

    pub fn ready_wakes(&self) -> &[ReadyTemporalWake] {
        &self.ready_wakes
    }

    pub fn promoted_wake_count(&self) -> u64 {
        self.promoted_wake_count
    }

    pub fn broad_scan_denial_count_delta(&self) -> u64 {
        self.broad_scan_denial_count_delta
    }
}
