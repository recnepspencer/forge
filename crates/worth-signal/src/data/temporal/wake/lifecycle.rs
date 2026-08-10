use serde::{Deserialize, Serialize};

use crate::data::handle::NodeId;

use crate::data::temporal::{ClockDomain, ClockTick, TemporalCondition};

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
