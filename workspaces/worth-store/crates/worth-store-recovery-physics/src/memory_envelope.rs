#[cfg(feature = "legacy-certification-models")]
use worth_store_buffer_pool::{AdmittedBackgroundEnvelope, BackgroundWorkClass};
use worth_store_buffer_pool::{OperationAllocationGrant, OperationAllocationScope};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryMemoryEnvelope {
    allocation_scope: OperationAllocationScope,
    counters: RecoveryMemoryCounterSnapshot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryMemoryCounterSnapshot {
    resident_bytes: u64,
    resident_frames: u32,
    allocation_bytes: u64,
    pinned_pages: u32,
    copied_bytes: u64,
}

impl RecoveryMemoryEnvelope {
    pub fn from_allocation_grant(
        allocation: &OperationAllocationGrant,
        resident_frames: u32,
    ) -> Result<Self, RecoveryMemoryEnvelopeDenial> {
        if allocation.scope() != OperationAllocationScope::Recovery {
            return Err(RecoveryMemoryEnvelopeDenial::WrongAllocationScope {
                actual: allocation.scope(),
            });
        }
        Ok(Self {
            allocation_scope: allocation.scope(),
            counters: RecoveryMemoryCounterSnapshot {
                resident_bytes: allocation.bytes(),
                resident_frames,
                allocation_bytes: allocation.bytes(),
                pinned_pages: resident_frames,
                copied_bytes: 0,
            },
        })
    }

    #[cfg(feature = "legacy-certification-models")]
    pub fn from_admitted(
        envelope: AdmittedBackgroundEnvelope,
    ) -> Result<Self, RecoveryMemoryEnvelopeDenial> {
        if envelope.work_class() != BackgroundWorkClass::RecoveryPlanning {
            return Err(RecoveryMemoryEnvelopeDenial::WrongBackgroundEnvelopeClass {
                expected: BackgroundWorkClass::RecoveryPlanning,
                actual: envelope.work_class(),
            });
        }
        let counters = envelope.counters();
        Ok(Self {
            allocation_scope: OperationAllocationScope::Recovery,
            counters: RecoveryMemoryCounterSnapshot {
                resident_bytes: counters.resident_bytes_admitted(),
                resident_frames: counters.resident_frames_admitted(),
                allocation_bytes: counters.allocation_bytes_allocated(),
                pinned_pages: counters.pinned_pages_admitted(),
                copied_bytes: counters.copied_bytes(),
            },
        })
    }

    pub const fn allocation_scope(self) -> OperationAllocationScope {
        self.allocation_scope
    }

    pub const fn counters(self) -> RecoveryMemoryCounterSnapshot {
        self.counters
    }

    pub const fn proves_wal_recovery(self) -> bool {
        false
    }

    pub const fn proves_checkpoint_safety(self) -> bool {
        false
    }

    pub const fn proves_repair_behavior(self) -> bool {
        false
    }
}

impl RecoveryMemoryCounterSnapshot {
    pub const fn admitted(self) -> u32 {
        1
    }

    pub const fn resident_bytes_admitted(self) -> u64 {
        self.resident_bytes
    }

    pub const fn resident_frames_admitted(self) -> u32 {
        self.resident_frames
    }

    pub const fn allocation_bytes_allocated(self) -> u64 {
        self.allocation_bytes
    }

    pub const fn pinned_pages_admitted(self) -> u32 {
        self.pinned_pages
    }

    pub const fn copied_bytes(self) -> u64 {
        self.copied_bytes
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryMemoryEnvelopeDenial {
    WrongAllocationScope {
        actual: OperationAllocationScope,
    },
    #[cfg(feature = "legacy-certification-models")]
    WrongBackgroundEnvelopeClass {
        expected: BackgroundWorkClass,
        actual: BackgroundWorkClass,
    },
}
