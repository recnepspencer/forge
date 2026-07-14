use serde::{Deserialize, Serialize};

use crate::data::aspect::AspectVersion;
use crate::data::handle::NodeId;
use crate::data::output::OutputIdentity;
use crate::state::SignalBranchId;

use super::{ClockTick, ReadyTemporalWake, TemporalWakeId, WakeOrdinal};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PreviousValueRevision(u64);

impl PreviousValueRevision {
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemporalPreviousValueAccess {
    branch_id: SignalBranchId,
    capability_epoch: u64,
    wake_id: TemporalWakeId,
    ready_ordinal: WakeOrdinal,
    ready_tick: ClockTick,
}

impl TemporalPreviousValueAccess {
    pub(crate) fn from_ready_wake(
        branch_id: SignalBranchId,
        capability_epoch: u64,
        wake: &ReadyTemporalWake,
    ) -> Self {
        Self {
            branch_id,
            capability_epoch,
            wake_id: wake.id(),
            ready_ordinal: wake.ready_ordinal(),
            ready_tick: wake.ready_tick(),
        }
    }

    pub fn branch_id(&self) -> SignalBranchId {
        self.branch_id
    }

    pub fn capability_epoch(&self) -> u64 {
        self.capability_epoch
    }

    pub fn wake_id(&self) -> TemporalWakeId {
        self.wake_id
    }

    pub fn ready_ordinal(&self) -> WakeOrdinal {
        self.ready_ordinal
    }

    pub fn ready_tick(&self) -> ClockTick {
        self.ready_tick
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemporalPreviousValueReference {
    revision: PreviousValueRevision,
    branch_id: SignalBranchId,
    access_wake_id: TemporalWakeId,
    node: NodeId,
    captured_at_tick: ClockTick,
    aspect_version: AspectVersion,
    output_identity: Option<OutputIdentity>,
}

impl TemporalPreviousValueReference {
    pub(crate) fn new(
        revision: PreviousValueRevision,
        access: &TemporalPreviousValueAccess,
        node: NodeId,
        aspect_version: AspectVersion,
        output_identity: Option<OutputIdentity>,
    ) -> Self {
        Self {
            revision,
            branch_id: access.branch_id(),
            access_wake_id: access.wake_id(),
            node,
            captured_at_tick: access.ready_tick(),
            aspect_version,
            output_identity,
        }
    }

    pub fn revision(&self) -> PreviousValueRevision {
        self.revision
    }

    pub fn access_wake_id(&self) -> TemporalWakeId {
        self.access_wake_id
    }

    pub fn branch_id(&self) -> SignalBranchId {
        self.branch_id
    }

    pub fn node(&self) -> NodeId {
        self.node
    }

    pub fn captured_at_tick(&self) -> ClockTick {
        self.captured_at_tick
    }

    pub fn aspect_version(&self) -> AspectVersion {
        self.aspect_version
    }

    pub fn output_identity(&self) -> Option<&OutputIdentity> {
        self.output_identity.as_ref()
    }
}
