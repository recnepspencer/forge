use crate::physical_runtime::{
    PhysicalEffectIdentity, PhysicalSignalSettlementOutcome, PhysicalWorkEffectFate,
    PhysicalWorkIdentity, PhysicalWorkRecoveryDisposition, SettledPhysicalWork,
};

use super::AdmittedDirtyFrame;

#[must_use = "writeback execution must be classified as clean, retryable, or inspection-required"]
pub enum PhysicalWritebackExecution {
    Clean(PhysicalWritebackSettlement),
    Retryable(RetryablePhysicalWriteback),
    InspectionRequired(PhysicalWritebackInspectionRequired),
}

pub struct RetryablePhysicalWriteback {
    #[cfg(feature = "certification-test-authority")]
    settled: SettledPhysicalWork,
    settlement: PhysicalWritebackSettlement,
    dirty: AdmittedDirtyFrame,
}

pub struct PhysicalWritebackInspectionRequired {
    settlement: PhysicalWritebackSettlement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalWritebackSettlement {
    identity: PhysicalWorkIdentity,
    effect: Option<PhysicalEffectIdentity>,
    effect_fate: PhysicalWorkEffectFate,
    recovery: PhysicalWorkRecoveryDisposition,
    signal: PhysicalSignalSettlementOutcome,
}

impl PhysicalWritebackSettlement {
    pub(super) fn from_settled(
        settled: &SettledPhysicalWork,
        signal: PhysicalSignalSettlementOutcome,
    ) -> Self {
        Self {
            identity: settled.intent().identity(),
            effect: settled.effect_identity(),
            effect_fate: settled.evidence().fate(),
            recovery: settled.recovery_disposition(),
            signal,
        }
    }

    pub const fn identity(self) -> PhysicalWorkIdentity {
        self.identity
    }

    pub const fn effect(self) -> Option<PhysicalEffectIdentity> {
        self.effect
    }

    pub const fn effect_fate(self) -> PhysicalWorkEffectFate {
        self.effect_fate
    }

    pub const fn recovery(self) -> PhysicalWorkRecoveryDisposition {
        self.recovery
    }

    pub const fn signal(self) -> PhysicalSignalSettlementOutcome {
        self.signal
    }
}

impl RetryablePhysicalWriteback {
    pub(super) fn new(
        settled: SettledPhysicalWork,
        settlement: PhysicalWritebackSettlement,
        dirty: AdmittedDirtyFrame,
    ) -> Self {
        #[cfg(not(feature = "certification-test-authority"))]
        let _ = settled;
        Self {
            #[cfg(feature = "certification-test-authority")]
            settled,
            settlement,
            dirty,
        }
    }

    #[cfg(feature = "certification-test-authority")]
    pub const fn settled(&self) -> &SettledPhysicalWork {
        &self.settled
    }

    pub const fn settlement(&self) -> PhysicalWritebackSettlement {
        self.settlement
    }

    #[cfg(feature = "certification-test-authority")]
    pub fn into_parts(self) -> (SettledPhysicalWork, AdmittedDirtyFrame) {
        (self.settled, self.dirty)
    }

    pub(in crate::physical_runtime::record_serving) fn into_dirty(self) -> AdmittedDirtyFrame {
        self.dirty
    }
}

impl PhysicalWritebackInspectionRequired {
    pub(super) const fn new(settlement: PhysicalWritebackSettlement) -> Self {
        Self { settlement }
    }

    pub const fn settlement(&self) -> PhysicalWritebackSettlement {
        self.settlement
    }
}
