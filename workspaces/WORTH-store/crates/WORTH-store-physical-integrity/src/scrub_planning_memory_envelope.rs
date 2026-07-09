use worth_store_buffer_pool::{
    AdmittedBackgroundEnvelope, AllocationScope, BackgroundEnvelopeCounterSnapshot,
    BackgroundWorkClass,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScrubPlanningMemoryEnvelope {
    envelope: AdmittedBackgroundEnvelope,
}

impl ScrubPlanningMemoryEnvelope {
    pub fn from_admitted(
        envelope: AdmittedBackgroundEnvelope,
    ) -> Result<Self, ScrubPlanningMemoryEnvelopeDenial> {
        if envelope.work_class() != BackgroundWorkClass::ScrubPlanning {
            return Err(
                ScrubPlanningMemoryEnvelopeDenial::WrongBackgroundEnvelopeClass {
                    expected: BackgroundWorkClass::ScrubPlanning,
                    actual: envelope.work_class(),
                },
            );
        }
        Ok(Self { envelope })
    }

    pub const fn allocation_scope(self) -> AllocationScope {
        self.envelope.allocation_scope()
    }

    pub const fn allocation_bytes(self) -> u64 {
        self.envelope.allocation_bytes()
    }

    pub const fn pinned_pages(self) -> u32 {
        self.envelope.pinned_pages()
    }

    pub const fn counters(self) -> BackgroundEnvelopeCounterSnapshot {
        self.envelope.counters()
    }

    pub const fn proves_scrub_correctness(self) -> bool {
        false
    }

    pub const fn proves_corruption_localization(self) -> bool {
        false
    }

    pub const fn proves_repair_behavior(self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrubPlanningMemoryEnvelopeDenial {
    WrongBackgroundEnvelopeClass {
        expected: BackgroundWorkClass,
        actual: BackgroundWorkClass,
    },
}
