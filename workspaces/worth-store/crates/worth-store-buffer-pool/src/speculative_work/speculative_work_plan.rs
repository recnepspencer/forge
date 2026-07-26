use crate::{
    AllocationGrant, AllocationReceipt, PhysicalSpeculativeWorkKind, PrefetchWindow,
    SpeculativeWorkCounterSnapshot,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpeculativeWorkReplayIdentity {
    kind: PhysicalSpeculativeWorkKind,
    resident_frames_requested: u32,
    dirty_pages_requested: u32,
    allocation_bytes_requested: u64,
    resident_frames_at_lowering: u32,
    dirty_pages_at_lowering: u32,
}

impl SpeculativeWorkReplayIdentity {
    pub(crate) const fn new(
        kind: PhysicalSpeculativeWorkKind,
        resident_frames_requested: u32,
        dirty_pages_requested: u32,
        allocation_bytes_requested: u64,
        resident_frames_at_lowering: u32,
        dirty_pages_at_lowering: u32,
    ) -> Self {
        Self {
            kind,
            resident_frames_requested,
            dirty_pages_requested,
            allocation_bytes_requested,
            resident_frames_at_lowering,
            dirty_pages_at_lowering,
        }
    }

    pub const fn kind(self) -> PhysicalSpeculativeWorkKind {
        self.kind
    }

    pub const fn resident_frames_requested(self) -> u32 {
        self.resident_frames_requested
    }

    pub const fn dirty_pages_requested(self) -> u32 {
        self.dirty_pages_requested
    }

    pub const fn allocation_bytes_requested(self) -> u64 {
        self.allocation_bytes_requested
    }

    pub const fn resident_frames_at_lowering(self) -> u32 {
        self.resident_frames_at_lowering
    }

    pub const fn dirty_pages_at_lowering(self) -> u32 {
        self.dirty_pages_at_lowering
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ReadAheadPlan {
    window: PrefetchWindow,
    replay_identity: SpeculativeWorkReplayIdentity,
    allocation_grant: Option<AllocationGrant>,
    counters: SpeculativeWorkCounterSnapshot,
}

impl ReadAheadPlan {
    pub(crate) const fn new(
        window: PrefetchWindow,
        replay_identity: SpeculativeWorkReplayIdentity,
        allocation_grant: Option<AllocationGrant>,
        counters: SpeculativeWorkCounterSnapshot,
    ) -> Self {
        Self {
            window,
            replay_identity,
            allocation_grant,
            counters,
        }
    }

    pub const fn window(&self) -> PrefetchWindow {
        self.window
    }

    pub const fn replay_identity(&self) -> SpeculativeWorkReplayIdentity {
        self.replay_identity
    }

    pub const fn counters(&self) -> SpeculativeWorkCounterSnapshot {
        self.counters
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        PrefetchWindow,
        SpeculativeWorkReplayIdentity,
        Option<AllocationGrant>,
        SpeculativeWorkCounterSnapshot,
    ) {
        (
            self.window,
            self.replay_identity,
            self.allocation_grant,
            self.counters,
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PrefetchPlan {
    window: PrefetchWindow,
    replay_identity: SpeculativeWorkReplayIdentity,
    allocation_grant: Option<AllocationGrant>,
    counters: SpeculativeWorkCounterSnapshot,
}

impl PrefetchPlan {
    pub(crate) const fn new(
        window: PrefetchWindow,
        replay_identity: SpeculativeWorkReplayIdentity,
        allocation_grant: Option<AllocationGrant>,
        counters: SpeculativeWorkCounterSnapshot,
    ) -> Self {
        Self {
            window,
            replay_identity,
            allocation_grant,
            counters,
        }
    }

    pub const fn window(&self) -> PrefetchWindow {
        self.window
    }

    pub const fn replay_identity(&self) -> SpeculativeWorkReplayIdentity {
        self.replay_identity
    }

    pub const fn counters(&self) -> SpeculativeWorkCounterSnapshot {
        self.counters
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        PrefetchWindow,
        SpeculativeWorkReplayIdentity,
        Option<AllocationGrant>,
        SpeculativeWorkCounterSnapshot,
    ) {
        (
            self.window,
            self.replay_identity,
            self.allocation_grant,
            self.counters,
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct WriteBehindPlan {
    replay_identity: SpeculativeWorkReplayIdentity,
    allocation_grant: Option<AllocationGrant>,
    counters: SpeculativeWorkCounterSnapshot,
}

impl WriteBehindPlan {
    pub(crate) const fn new(
        replay_identity: SpeculativeWorkReplayIdentity,
        allocation_grant: Option<AllocationGrant>,
        counters: SpeculativeWorkCounterSnapshot,
    ) -> Self {
        Self {
            replay_identity,
            allocation_grant,
            counters,
        }
    }

    pub const fn replay_identity(&self) -> SpeculativeWorkReplayIdentity {
        self.replay_identity
    }

    pub const fn counters(&self) -> SpeculativeWorkCounterSnapshot {
        self.counters
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        SpeculativeWorkReplayIdentity,
        Option<AllocationGrant>,
        SpeculativeWorkCounterSnapshot,
    ) {
        (self.replay_identity, self.allocation_grant, self.counters)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReadAheadAdmission {
    replay_identity: SpeculativeWorkReplayIdentity,
    allocation_receipt: Option<AllocationReceipt>,
    counters: SpeculativeWorkCounterSnapshot,
}

impl ReadAheadAdmission {
    pub(crate) const fn new(
        replay_identity: SpeculativeWorkReplayIdentity,
        allocation_receipt: Option<AllocationReceipt>,
        counters: SpeculativeWorkCounterSnapshot,
    ) -> Self {
        Self {
            replay_identity,
            allocation_receipt,
            counters,
        }
    }

    pub const fn replay_identity(self) -> SpeculativeWorkReplayIdentity {
        self.replay_identity
    }

    pub const fn allocation_receipt(self) -> Option<AllocationReceipt> {
        self.allocation_receipt
    }

    pub const fn counters(self) -> SpeculativeWorkCounterSnapshot {
        self.counters
    }

    pub const fn proves_io_qos(self) -> bool {
        false
    }

    pub const fn proves_queue_depth_correctness(self) -> bool {
        false
    }

    pub const fn proves_backend_pacing(self) -> bool {
        false
    }

    pub const fn proves_fsync_policy(self) -> bool {
        false
    }

    pub const fn proves_fairness(self) -> bool {
        false
    }

    pub const fn proves_throughput_improvement(self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrefetchAdmission {
    replay_identity: SpeculativeWorkReplayIdentity,
    allocation_receipt: Option<AllocationReceipt>,
    counters: SpeculativeWorkCounterSnapshot,
}

impl PrefetchAdmission {
    pub(crate) const fn new(
        replay_identity: SpeculativeWorkReplayIdentity,
        allocation_receipt: Option<AllocationReceipt>,
        counters: SpeculativeWorkCounterSnapshot,
    ) -> Self {
        Self {
            replay_identity,
            allocation_receipt,
            counters,
        }
    }

    pub const fn replay_identity(self) -> SpeculativeWorkReplayIdentity {
        self.replay_identity
    }

    pub const fn allocation_receipt(self) -> Option<AllocationReceipt> {
        self.allocation_receipt
    }

    pub const fn counters(self) -> SpeculativeWorkCounterSnapshot {
        self.counters
    }

    pub const fn proves_io_qos(self) -> bool {
        false
    }

    pub const fn proves_queue_depth_correctness(self) -> bool {
        false
    }

    pub const fn proves_backend_pacing(self) -> bool {
        false
    }

    pub const fn proves_fsync_policy(self) -> bool {
        false
    }

    pub const fn proves_fairness(self) -> bool {
        false
    }

    pub const fn proves_throughput_improvement(self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WriteBehindAdmission {
    replay_identity: SpeculativeWorkReplayIdentity,
    allocation_receipt: Option<AllocationReceipt>,
    counters: SpeculativeWorkCounterSnapshot,
}

impl WriteBehindAdmission {
    pub(crate) const fn new(
        replay_identity: SpeculativeWorkReplayIdentity,
        allocation_receipt: Option<AllocationReceipt>,
        counters: SpeculativeWorkCounterSnapshot,
    ) -> Self {
        Self {
            replay_identity,
            allocation_receipt,
            counters,
        }
    }

    pub const fn replay_identity(self) -> SpeculativeWorkReplayIdentity {
        self.replay_identity
    }

    pub const fn allocation_receipt(self) -> Option<AllocationReceipt> {
        self.allocation_receipt
    }

    pub const fn counters(self) -> SpeculativeWorkCounterSnapshot {
        self.counters
    }

    pub const fn proves_io_qos(self) -> bool {
        false
    }

    pub const fn proves_queue_depth_correctness(self) -> bool {
        false
    }

    pub const fn proves_backend_pacing(self) -> bool {
        false
    }

    pub const fn proves_fsync_policy(self) -> bool {
        false
    }

    pub const fn proves_fairness(self) -> bool {
        false
    }

    pub const fn proves_throughput_improvement(self) -> bool {
        false
    }
}
