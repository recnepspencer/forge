use serde::{Deserialize, Serialize};

use super::{
    RetiredTemporalWake, ScheduledTemporalWake, TemporalWakeId, TemporalWakeOwner,
    TemporalWakeRetirementReason, WakeOrdinal,
};
use crate::data::temporal::{ClockTick, TemporalCondition};

/// Runtime-owned proof that one wake lifecycle was explicitly superseded by a new schedule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemporalWakeReschedule {
    retired: RetiredTemporalWake,
    scheduled: ScheduledTemporalWake,
}

impl TemporalWakeReschedule {
    pub(crate) fn new(retired: RetiredTemporalWake, scheduled: ScheduledTemporalWake) -> Self {
        Self { retired, scheduled }
    }

    pub fn retired(&self) -> &RetiredTemporalWake {
        &self.retired
    }

    pub fn scheduled(&self) -> &ScheduledTemporalWake {
        &self.scheduled
    }
}

/// Runtime-owned proof that an existing wake lifecycle was intentionally reused.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemporalWakeReuse {
    wake_id: TemporalWakeId,
    ordinal: WakeOrdinal,
    owner: TemporalWakeOwner,
    condition: TemporalCondition,
    original_due_tick: ClockTick,
    attempted_due_tick: ClockTick,
    decision_tick: ClockTick,
}

impl TemporalWakeReuse {
    pub(crate) fn from_scheduled(
        scheduled: &ScheduledTemporalWake,
        attempted_due_tick: ClockTick,
        decision_tick: ClockTick,
    ) -> Self {
        Self {
            wake_id: scheduled.id(),
            ordinal: scheduled.ordinal(),
            owner: scheduled.owner(),
            condition: scheduled.condition().clone(),
            original_due_tick: scheduled.due_tick(),
            attempted_due_tick,
            decision_tick,
        }
    }

    pub fn wake_id(&self) -> TemporalWakeId {
        self.wake_id
    }

    pub fn ordinal(&self) -> WakeOrdinal {
        self.ordinal
    }

    pub fn owner(&self) -> TemporalWakeOwner {
        self.owner
    }

    pub fn condition(&self) -> &TemporalCondition {
        &self.condition
    }

    pub fn original_due_tick(&self) -> ClockTick {
        self.original_due_tick
    }

    pub fn attempted_due_tick(&self) -> ClockTick {
        self.attempted_due_tick
    }

    pub fn decision_tick(&self) -> ClockTick {
        self.decision_tick
    }
}

/// Runtime-owned proof that an interval wake was consumed and regenerated under policy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntervalWakeRegeneration {
    retired: RetiredTemporalWake,
    scheduled: ScheduledTemporalWake,
    suppressed_interval_count: u64,
}

impl IntervalWakeRegeneration {
    pub(crate) fn new(
        retired: RetiredTemporalWake,
        scheduled: ScheduledTemporalWake,
        suppressed_interval_count: u64,
    ) -> Self {
        Self {
            retired,
            scheduled,
            suppressed_interval_count,
        }
    }

    pub fn retired(&self) -> &RetiredTemporalWake {
        &self.retired
    }

    pub fn scheduled(&self) -> &ScheduledTemporalWake {
        &self.scheduled
    }

    pub fn suppressed_interval_count(&self) -> u64 {
        self.suppressed_interval_count
    }
}

/// Runtime-owned proof that all active wakes for one owner were retired together.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemporalWakeRetirementBatch {
    owner: TemporalWakeOwner,
    reason: TemporalWakeRetirementReason,
    retired: Vec<RetiredTemporalWake>,
}

impl TemporalWakeRetirementBatch {
    pub(crate) fn new(
        owner: TemporalWakeOwner,
        reason: TemporalWakeRetirementReason,
        retired: Vec<RetiredTemporalWake>,
    ) -> Self {
        Self {
            owner,
            reason,
            retired,
        }
    }

    pub fn owner(&self) -> TemporalWakeOwner {
        self.owner
    }

    pub fn reason(&self) -> TemporalWakeRetirementReason {
        self.reason
    }

    pub fn retired(&self) -> &[RetiredTemporalWake] {
        &self.retired
    }
}
