use core::num::NonZeroU64;

use super::{
    BackgroundIoDebt, BackgroundIoPressureClass, BackgroundPacingAdmissionBasis,
    BackgroundPacingCounterSnapshot, BackgroundResourceBudget,
};
use crate::SecureIoPreservationReceipt;

#[derive(Debug, Eq, PartialEq)]
pub struct BackgroundIdleCapacityLease {
    class: BackgroundIoPressureClass,
    admitted: BackgroundResourceBudget,
    debt: BackgroundIoDebt,
    basis: BackgroundPacingAdmissionBasis,
    counters: BackgroundPacingCounterSnapshot,
    secure_io: Option<SecureIoPreservationReceipt>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackgroundLeaseRevocation {
    class: BackgroundIoPressureClass,
    revoked: BackgroundResourceBudget,
    basis: BackgroundPacingAdmissionBasis,
    counters: BackgroundPacingCounterSnapshot,
    secure_io: Option<SecureIoPreservationReceipt>,
}

impl BackgroundIdleCapacityLease {
    pub(crate) const fn new(
        class: BackgroundIoPressureClass,
        admitted: BackgroundResourceBudget,
        debt: BackgroundIoDebt,
        basis: BackgroundPacingAdmissionBasis,
        counters: BackgroundPacingCounterSnapshot,
        secure_io: Option<SecureIoPreservationReceipt>,
    ) -> Self {
        Self {
            class,
            admitted,
            debt,
            basis,
            counters,
            secure_io,
        }
    }

    pub const fn class(&self) -> BackgroundIoPressureClass {
        self.class
    }
    pub const fn admitted_budget(&self) -> BackgroundResourceBudget {
        self.admitted
    }
    pub const fn debt(&self) -> BackgroundIoDebt {
        self.debt
    }
    pub const fn basis(&self) -> BackgroundPacingAdmissionBasis {
        self.basis
    }
    pub const fn counters(&self) -> BackgroundPacingCounterSnapshot {
        self.counters
    }
    pub const fn secure_io(&self) -> Option<SecureIoPreservationReceipt> {
        self.secure_io
    }

    pub const fn revoke_for_foreground_pressure(
        self,
        foreground_pressure_events: NonZeroU64,
    ) -> BackgroundLeaseRevocation {
        BackgroundLeaseRevocation {
            class: self.class,
            revoked: self.admitted,
            basis: self.basis,
            counters: BackgroundPacingCounterSnapshot::revoked(
                self.counters,
                self.admitted,
                foreground_pressure_events.get(),
            ),
            secure_io: self.secure_io,
        }
    }
}

impl BackgroundLeaseRevocation {
    pub const fn class(self) -> BackgroundIoPressureClass {
        self.class
    }
    pub const fn revoked_budget(self) -> BackgroundResourceBudget {
        self.revoked
    }
    pub const fn basis(self) -> BackgroundPacingAdmissionBasis {
        self.basis
    }
    pub const fn counters(self) -> BackgroundPacingCounterSnapshot {
        self.counters
    }
    pub const fn secure_io(self) -> Option<SecureIoPreservationReceipt> {
        self.secure_io
    }
}
