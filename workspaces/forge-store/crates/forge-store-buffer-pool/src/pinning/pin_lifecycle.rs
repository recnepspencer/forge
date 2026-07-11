use crate::{
    LeaseScope, PageLeaseId, PinLifecycleCounterSnapshot, ResidentFrameCounterSnapshot,
    ResidentFrameIdentity,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnpinnedPageReceipt {
    lease_id: PageLeaseId,
    identity: ResidentFrameIdentity,
    counters: PinLifecycleCounterSnapshot,
}

impl UnpinnedPageReceipt {
    pub(crate) const fn new(
        lease_id: PageLeaseId,
        identity: ResidentFrameIdentity,
        counters: PinLifecycleCounterSnapshot,
    ) -> Self {
        Self {
            lease_id,
            identity,
            counters,
        }
    }

    pub const fn lease_id(self) -> PageLeaseId {
        self.lease_id
    }

    pub const fn identity(self) -> ResidentFrameIdentity {
        self.identity
    }

    pub const fn counters(self) -> PinLifecycleCounterSnapshot {
        self.counters
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LeaseLeakReport {
    scope: LeaseScope,
    leaked_pin_count: u64,
    pin_counters: PinLifecycleCounterSnapshot,
}

impl LeaseLeakReport {
    pub(crate) const fn new(
        scope: LeaseScope,
        leaked_pin_count: u64,
        pin_counters: PinLifecycleCounterSnapshot,
    ) -> Self {
        Self {
            scope,
            leaked_pin_count,
            pin_counters,
        }
    }

    pub const fn scope(self) -> LeaseScope {
        self.scope
    }

    pub const fn leaked_pin_count(self) -> u64 {
        self.leaked_pin_count
    }

    pub const fn pin_counters(self) -> PinLifecycleCounterSnapshot {
        self.pin_counters
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PinLifecycleCloseoutReport {
    pin_counters: PinLifecycleCounterSnapshot,
    resident_counters: ResidentFrameCounterSnapshot,
}

impl PinLifecycleCloseoutReport {
    pub(crate) const fn new(
        pin_counters: PinLifecycleCounterSnapshot,
        resident_counters: ResidentFrameCounterSnapshot,
    ) -> Self {
        Self {
            pin_counters,
            resident_counters,
        }
    }

    pub const fn pin_counters(self) -> PinLifecycleCounterSnapshot {
        self.pin_counters
    }

    pub const fn resident_counters(self) -> ResidentFrameCounterSnapshot {
        self.resident_counters
    }
}
