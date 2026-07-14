use serde::{Deserialize, Serialize};

use crate::data::handle::NodeId;

use super::{ClockDomain, ClockTick, TemporalCondition};

/// Stable identifier for runtime-owned temporal wakes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub struct TemporalWakeId(u64);

impl TemporalWakeId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

/// Monotonic ordinal assigned as wakes enter lifecycle boundaries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub struct WakeOrdinal(u64);

impl WakeOrdinal {
    pub const ZERO: Self = Self(0);

    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn get(self) -> u64 {
        self.0
    }

    pub(crate) fn next(self) -> Self {
        Self(self.0.saturating_add(1))
    }
}

/// Framework-owned owner identity for active temporal wake lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TemporalWakeOwner {
    Manual,
    Node(NodeId),
    ResourceNode(NodeId),
}

/// Runtime-owned proof that a temporal wake has been admitted but is not yet ready.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScheduledTemporalWake {
    id: TemporalWakeId,
    ordinal: WakeOrdinal,
    owner: TemporalWakeOwner,
    condition: TemporalCondition,
    due_tick: ClockTick,
}

impl ScheduledTemporalWake {
    pub(crate) fn new(
        id: TemporalWakeId,
        ordinal: WakeOrdinal,
        owner: TemporalWakeOwner,
        condition: TemporalCondition,
        due_tick: ClockTick,
    ) -> Self {
        Self {
            id,
            ordinal,
            owner,
            condition,
            due_tick,
        }
    }

    pub fn id(&self) -> TemporalWakeId {
        self.id
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

    pub fn due_tick(&self) -> ClockTick {
        self.due_tick
    }

    pub fn clock_domain(&self) -> ClockDomain {
        self.condition.clock_domain()
    }
}

/// Runtime-owned proof that a temporal wake crossed readiness at a specific clock tick.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadyTemporalWake {
    id: TemporalWakeId,
    scheduled_ordinal: WakeOrdinal,
    ready_ordinal: WakeOrdinal,
    owner: TemporalWakeOwner,
    condition: TemporalCondition,
    due_tick: ClockTick,
    ready_tick: ClockTick,
}

impl ReadyTemporalWake {
    pub(crate) fn from_scheduled(
        wake: ScheduledTemporalWake,
        ready_ordinal: WakeOrdinal,
        ready_tick: ClockTick,
    ) -> Self {
        Self {
            id: wake.id,
            scheduled_ordinal: wake.ordinal,
            ready_ordinal,
            owner: wake.owner,
            condition: wake.condition,
            due_tick: wake.due_tick,
            ready_tick,
        }
    }

    pub fn id(&self) -> TemporalWakeId {
        self.id
    }

    pub fn scheduled_ordinal(&self) -> WakeOrdinal {
        self.scheduled_ordinal
    }

    pub fn ready_ordinal(&self) -> WakeOrdinal {
        self.ready_ordinal
    }

    pub fn owner(&self) -> TemporalWakeOwner {
        self.owner
    }

    pub fn condition(&self) -> &TemporalCondition {
        &self.condition
    }

    pub fn due_tick(&self) -> ClockTick {
        self.due_tick
    }

    pub fn ready_tick(&self) -> ClockTick {
        self.ready_tick
    }

    pub fn clock_domain(&self) -> ClockDomain {
        self.condition.clock_domain()
    }
}

/// Reason a runtime-owned temporal wake left the active lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TemporalWakeRetirementReason {
    Consumed,
    Cancelled,
    Superseded,
    BranchRestored,
    Disposed,
}

/// Runtime-owned proof that a wake can no longer participate in active readiness.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetiredTemporalWake {
    id: TemporalWakeId,
    active_ordinal: WakeOrdinal,
    retired_ordinal: WakeOrdinal,
    owner: TemporalWakeOwner,
    retired_tick: ClockTick,
    reason: TemporalWakeRetirementReason,
}

impl RetiredTemporalWake {
    pub(crate) fn new(
        id: TemporalWakeId,
        active_ordinal: WakeOrdinal,
        retired_ordinal: WakeOrdinal,
        owner: TemporalWakeOwner,
        retired_tick: ClockTick,
        reason: TemporalWakeRetirementReason,
    ) -> Self {
        Self {
            id,
            active_ordinal,
            retired_ordinal,
            owner,
            retired_tick,
            reason,
        }
    }

    pub fn id(&self) -> TemporalWakeId {
        self.id
    }

    pub fn active_ordinal(&self) -> WakeOrdinal {
        self.active_ordinal
    }

    pub fn retired_ordinal(&self) -> WakeOrdinal {
        self.retired_ordinal
    }

    pub fn owner(&self) -> TemporalWakeOwner {
        self.owner
    }

    pub fn retired_tick(&self) -> ClockTick {
        self.retired_tick
    }

    pub fn reason(&self) -> TemporalWakeRetirementReason {
        self.reason
    }
}

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

/// Cost-honest summary of runtime-owned temporal admission work.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TemporalWakeAdmissionSummary {
    scheduled: Vec<ScheduledTemporalWake>,
    rescheduled: Vec<TemporalWakeReschedule>,
    reused: Vec<TemporalWakeReuse>,
}

impl TemporalWakeAdmissionSummary {
    pub(crate) fn record_scheduled(&mut self, wake: ScheduledTemporalWake) {
        self.scheduled.push(wake);
    }

    pub(crate) fn record_policy_supersession(&mut self, reschedule: TemporalWakeReschedule) {
        self.rescheduled.push(reschedule);
    }

    pub(crate) fn record_reschedule(&mut self, reschedule: TemporalWakeReschedule) {
        self.rescheduled.push(reschedule);
    }

    pub(crate) fn record_reused(&mut self, reuse: TemporalWakeReuse) {
        self.reused.push(reuse);
    }

    pub(crate) fn extend(&mut self, other: Self) {
        self.scheduled.extend(other.scheduled);
        self.rescheduled.extend(other.rescheduled);
        self.reused.extend(other.reused);
    }

    pub fn scheduled(&self) -> &[ScheduledTemporalWake] {
        &self.scheduled
    }

    pub fn rescheduled(&self) -> &[TemporalWakeReschedule] {
        &self.rescheduled
    }

    pub fn reused(&self) -> &[TemporalWakeReuse] {
        &self.reused
    }

    pub fn scheduled_count(&self) -> u64 {
        self.scheduled.len() as u64
    }

    pub fn rescheduled_count(&self) -> u64 {
        self.rescheduled.len() as u64
    }

    pub fn reused_count(&self) -> u64 {
        self.reused.len() as u64
    }

    pub fn total_decision_count(&self) -> u64 {
        self.scheduled_count()
            .saturating_add(self.rescheduled_count())
            .saturating_add(self.reused_count())
    }
}

/// Cost-honest summary of runtime-owned temporal wake state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TemporalWakeSummary {
    scheduled_count: u64,
    ready_count: u64,
    retired_count: u64,
    next_wake_id: TemporalWakeId,
    next_wake_ordinal: WakeOrdinal,
}

impl TemporalWakeSummary {
    pub(crate) fn new(
        scheduled_count: usize,
        ready_count: usize,
        retired_count: usize,
        next_wake_id: TemporalWakeId,
        next_wake_ordinal: WakeOrdinal,
    ) -> Self {
        Self {
            scheduled_count: scheduled_count as u64,
            ready_count: ready_count as u64,
            retired_count: retired_count as u64,
            next_wake_id,
            next_wake_ordinal,
        }
    }

    pub fn scheduled_count(self) -> u64 {
        self.scheduled_count
    }

    pub fn ready_count(self) -> u64 {
        self.ready_count
    }

    pub fn retired_count(self) -> u64 {
        self.retired_count
    }

    pub fn next_wake_id(self) -> TemporalWakeId {
        self.next_wake_id
    }

    pub fn next_wake_ordinal(self) -> WakeOrdinal {
        self.next_wake_ordinal
    }
}
