use worth_store_io_scheduler::{
    BackgroundIdleCapacityLease, BackgroundIoPressureClass, BackgroundPacingCounterSnapshot,
    BackgroundResourceBudget,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BlobCompactionPacingDenial {
    WrongSchedulerClass { actual: BackgroundIoPressureClass },
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct BlobCompactionPacingAdmission {
    lease: BackgroundIdleCapacityLease,
}

impl BlobCompactionPacingAdmission {
    pub(crate) fn from_scheduler_lease(
        lease: BackgroundIdleCapacityLease,
    ) -> Result<Self, BlobCompactionPacingDenial> {
        if lease.class() != BackgroundIoPressureClass::CompactionRewrite {
            return Err(BlobCompactionPacingDenial::WrongSchedulerClass {
                actual: lease.class(),
            });
        }
        Ok(Self { lease })
    }

    pub(crate) const fn counters(&self) -> BackgroundPacingCounterSnapshot {
        self.lease.counters()
    }

    pub(crate) const fn admitted_budget(&self) -> BackgroundResourceBudget {
        self.lease.admitted_budget()
    }
}
