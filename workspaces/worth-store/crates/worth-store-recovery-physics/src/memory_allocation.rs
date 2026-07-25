use worth_store_buffer_pool::{
    OperationAllocationGrant, OperationAllocationObservation, OperationAllocationScope,
};

#[derive(Debug)]
pub struct RecoveryMemoryAllocation {
    grant: OperationAllocationGrant,
    observation: RecoveryMemoryObservation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryMemoryObservation {
    allocation: OperationAllocationObservation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryMemoryCounterSnapshot {
    allocation_bytes: u64,
}

impl RecoveryMemoryAllocation {
    pub fn from_allocation_grant(
        grant: OperationAllocationGrant,
    ) -> Result<Self, RecoveryMemoryAllocationDenial> {
        let allocation = grant.observation();
        if allocation.scope() != OperationAllocationScope::Recovery {
            return Err(RecoveryMemoryAllocationDenial::WrongAllocationScope {
                actual: allocation.scope(),
            });
        }
        Ok(Self {
            grant,
            observation: RecoveryMemoryObservation { allocation },
        })
    }

    pub const fn allocation_scope(&self) -> OperationAllocationScope {
        self.observation.allocation.scope()
    }

    pub const fn bytes(&self) -> u64 {
        self.grant.bytes()
    }

    pub const fn observation(&self) -> RecoveryMemoryObservation {
        self.observation
    }

    pub const fn counters(&self) -> RecoveryMemoryCounterSnapshot {
        RecoveryMemoryCounterSnapshot {
            allocation_bytes: self.grant.bytes(),
        }
    }
}

impl RecoveryMemoryObservation {
    pub const fn allocation(self) -> OperationAllocationObservation {
        self.allocation
    }

    pub const fn counters(self) -> RecoveryMemoryCounterSnapshot {
        RecoveryMemoryCounterSnapshot {
            allocation_bytes: self.allocation.bytes(),
        }
    }
}

impl RecoveryMemoryCounterSnapshot {
    pub const fn admitted(self) -> u32 {
        1
    }

    pub const fn resident_bytes_admitted(self) -> u64 {
        self.allocation_bytes
    }

    pub const fn resident_frames_admitted(self) -> u32 {
        0
    }

    pub const fn allocation_bytes_allocated(self) -> u64 {
        self.allocation_bytes
    }

    pub const fn pinned_pages_admitted(self) -> u32 {
        0
    }

    pub const fn copied_bytes(self) -> u64 {
        0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryMemoryAllocationDenial {
    WrongAllocationScope { actual: OperationAllocationScope },
}
