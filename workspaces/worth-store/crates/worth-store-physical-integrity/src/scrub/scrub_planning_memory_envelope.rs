#[cfg(feature = "legacy-certification-models")]
use worth_store_buffer_pool::{
    AdmittedBackgroundEnvelope, AllocationScope, BackgroundEnvelopeCounterSnapshot,
    BackgroundWorkClass,
};
use worth_store_buffer_pool::{OperationAllocationGrant, PhysicalOperationAllocationScope};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScrubPlanningMemoryEnvelope {
    allocation_bytes: u64,
    pinned_pages: u32,
    #[cfg(feature = "legacy-certification-models")]
    legacy_scope: Option<AllocationScope>,
    #[cfg(feature = "legacy-certification-models")]
    legacy_counters: Option<BackgroundEnvelopeCounterSnapshot>,
}

impl ScrubPlanningMemoryEnvelope {
    pub fn from_allocation_grant(
        allocation: &OperationAllocationGrant,
        pinned_pages: u32,
    ) -> Result<Self, ScrubPlanningMemoryEnvelopeDenial> {
        if allocation.scope() != PhysicalOperationAllocationScope::Scrub {
            return Err(ScrubPlanningMemoryEnvelopeDenial::WrongAllocationScope {
                actual: allocation.scope(),
            });
        }
        if pinned_pages == 0 {
            return Err(ScrubPlanningMemoryEnvelopeDenial::EmptyPinBudget);
        }
        Ok(Self {
            allocation_bytes: allocation.bytes(),
            pinned_pages,
            #[cfg(feature = "legacy-certification-models")]
            legacy_scope: None,
            #[cfg(feature = "legacy-certification-models")]
            legacy_counters: None,
        })
    }

    #[cfg(feature = "legacy-certification-models")]
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
        Ok(Self {
            allocation_bytes: envelope.allocation_bytes(),
            pinned_pages: envelope.pinned_pages(),
            legacy_scope: Some(envelope.allocation_scope()),
            legacy_counters: Some(envelope.counters()),
        })
    }

    pub const fn allocation_bytes(self) -> u64 {
        self.allocation_bytes
    }

    pub const fn pinned_pages(self) -> u32 {
        self.pinned_pages
    }

    #[cfg(feature = "legacy-certification-models")]
    pub fn allocation_scope(self) -> AllocationScope {
        self.legacy_scope
            .expect("legacy envelope retains its scope")
    }

    #[cfg(feature = "legacy-certification-models")]
    pub fn counters(self) -> BackgroundEnvelopeCounterSnapshot {
        self.legacy_counters
            .expect("legacy envelope retains its counters")
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
    WrongAllocationScope {
        actual: PhysicalOperationAllocationScope,
    },
    EmptyPinBudget,
    #[cfg(feature = "legacy-certification-models")]
    WrongBackgroundEnvelopeClass {
        expected: BackgroundWorkClass,
        actual: BackgroundWorkClass,
    },
}
