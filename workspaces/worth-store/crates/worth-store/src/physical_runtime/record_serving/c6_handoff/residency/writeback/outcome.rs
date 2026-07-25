use crate::physical_runtime::{
    PhysicalSignalSettlementOutcome, PhysicalWorkIdentity, SettledPhysicalWork,
};

use super::super::{C6AdmittedDirtyFrame, C6PhysicalWorkHandoffFailure};
use super::C6PhysicalWorkSettlement;

#[must_use = "writeback execution must join terminal settlement or retain its safe retry"]
pub enum C6PhysicalWritebackExecution {
    Settled(C6PhysicalWorkSettlement),
    Retryable(Box<C6RetryablePhysicalWriteback>),
}

#[must_use = "retryable writeback owns dirty residency and the original Signal lineage"]
pub struct C6RetryablePhysicalWriteback {
    settled: SettledPhysicalWork,
    signal: PhysicalSignalSettlementOutcome,
    dirty: C6AdmittedDirtyFrame,
}

#[derive(Debug)]
#[must_use = "failed writeback transition retains dirty residency ownership"]
pub struct C6PhysicalWritebackTransitionFailure {
    cause: C6PhysicalWorkHandoffFailure,
    dirty: C6AdmittedDirtyFrame,
}

impl C6PhysicalWritebackExecution {
    pub const fn settled(&self) -> Option<C6PhysicalWorkSettlement> {
        match self {
            Self::Settled(settlement) => Some(*settlement),
            Self::Retryable(_) => None,
        }
    }

    pub const fn retryable(&self) -> Option<&C6RetryablePhysicalWriteback> {
        match self {
            Self::Settled(_) => None,
            Self::Retryable(retryable) => Some(retryable),
        }
    }
}

impl C6RetryablePhysicalWriteback {
    pub(super) const fn new(
        settled: SettledPhysicalWork,
        signal: PhysicalSignalSettlementOutcome,
        dirty: C6AdmittedDirtyFrame,
    ) -> Self {
        Self {
            settled,
            signal,
            dirty,
        }
    }

    pub const fn identity(&self) -> PhysicalWorkIdentity {
        self.settled.intent().identity()
    }

    pub const fn settled(&self) -> &SettledPhysicalWork {
        &self.settled
    }

    pub const fn signal(&self) -> PhysicalSignalSettlementOutcome {
        self.signal
    }

    pub fn into_parts(self) -> (SettledPhysicalWork, C6AdmittedDirtyFrame) {
        (self.settled, self.dirty)
    }
}

impl C6PhysicalWritebackTransitionFailure {
    pub(super) const fn new(
        cause: C6PhysicalWorkHandoffFailure,
        dirty: C6AdmittedDirtyFrame,
    ) -> Self {
        Self { cause, dirty }
    }

    pub const fn cause(&self) -> C6PhysicalWorkHandoffFailure {
        self.cause
    }

    pub const fn dirty(&self) -> &C6AdmittedDirtyFrame {
        &self.dirty
    }

    pub fn into_dirty(self) -> C6AdmittedDirtyFrame {
        self.dirty
    }
}
