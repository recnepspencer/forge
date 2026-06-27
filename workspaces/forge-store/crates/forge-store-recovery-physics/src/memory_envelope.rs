use forge_store_buffer_pool::{
    AdmittedBackgroundEnvelope, AllocationScope, BackgroundEnvelopeCounterSnapshot,
    BackgroundWorkClass,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryMemoryEnvelope {
    envelope: AdmittedBackgroundEnvelope,
}

impl RecoveryMemoryEnvelope {
    pub fn from_admitted(
        envelope: AdmittedBackgroundEnvelope,
    ) -> Result<Self, RecoveryMemoryEnvelopeDenial> {
        if envelope.work_class() != BackgroundWorkClass::RecoveryPlanning {
            return Err(RecoveryMemoryEnvelopeDenial::WrongBackgroundEnvelopeClass {
                expected: BackgroundWorkClass::RecoveryPlanning,
                actual: envelope.work_class(),
            });
        }
        Ok(Self { envelope })
    }

    pub const fn allocation_scope(self) -> AllocationScope {
        self.envelope.allocation_scope()
    }

    pub const fn counters(self) -> BackgroundEnvelopeCounterSnapshot {
        self.envelope.counters()
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryMemoryEnvelopeDenial {
    WrongBackgroundEnvelopeClass {
        expected: BackgroundWorkClass,
        actual: BackgroundWorkClass,
    },
}
