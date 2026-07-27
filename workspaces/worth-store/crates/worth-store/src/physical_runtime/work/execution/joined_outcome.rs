use super::super::SettledPhysicalWork;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalSignalSettlementOutcome {
    Committed,
    ReconciledFromPhysicalTruth,
    DerivedStateUnavailable,
}

pub struct PhysicalWorkExecutionOutcome {
    settled: SettledPhysicalWork,
    signal: PhysicalSignalSettlementOutcome,
    residency_writeback: Option<super::PhysicalResidencyWritebackCompletion>,
}

pub struct PhysicalWorkExecutionBatchOutcome {
    executions: Box<[PhysicalWorkExecutionOutcome]>,
    denied_before_effect: Box<[PhysicalWorkBatchDenial]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalWorkBatchDenial {
    identity: super::super::PhysicalWorkIdentity,
    denial: super::super::PhysicalWorkPreEffectDenial,
}

impl PhysicalWorkExecutionOutcome {
    pub(in crate::physical_runtime) const fn new(
        settled: SettledPhysicalWork,
        signal: PhysicalSignalSettlementOutcome,
        residency_writeback: Option<super::PhysicalResidencyWritebackCompletion>,
    ) -> Self {
        Self {
            settled,
            signal,
            residency_writeback,
        }
    }

    pub const fn settled(&self) -> &SettledPhysicalWork {
        &self.settled
    }

    pub const fn signal(&self) -> PhysicalSignalSettlementOutcome {
        self.signal
    }

    pub fn into_settled(self) -> SettledPhysicalWork {
        self.settled
    }

    pub(in crate::physical_runtime) fn into_residency_writeback_parts(
        self,
    ) -> (
        SettledPhysicalWork,
        PhysicalSignalSettlementOutcome,
        Option<super::PhysicalResidencyWritebackCompletion>,
    ) {
        (self.settled, self.signal, self.residency_writeback)
    }
}

impl PhysicalWorkExecutionBatchOutcome {
    pub(in crate::physical_runtime) fn new(
        executions: Vec<PhysicalWorkExecutionOutcome>,
        denied_before_effect: Vec<PhysicalWorkBatchDenial>,
    ) -> Self {
        Self {
            executions: executions.into_boxed_slice(),
            denied_before_effect: denied_before_effect.into_boxed_slice(),
        }
    }

    pub const fn executions(&self) -> &[PhysicalWorkExecutionOutcome] {
        &self.executions
    }

    pub const fn denied_before_effect(&self) -> &[PhysicalWorkBatchDenial] {
        &self.denied_before_effect
    }
}

impl PhysicalWorkBatchDenial {
    pub(in crate::physical_runtime) const fn new(
        identity: super::super::PhysicalWorkIdentity,
        denial: super::super::PhysicalWorkPreEffectDenial,
    ) -> Self {
        Self { identity, denial }
    }

    pub const fn identity(self) -> super::super::PhysicalWorkIdentity {
        self.identity
    }

    pub const fn denial(self) -> super::super::PhysicalWorkPreEffectDenial {
        self.denial
    }
}
