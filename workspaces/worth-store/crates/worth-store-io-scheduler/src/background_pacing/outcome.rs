use super::{
    BackgroundIdleCapacityLease, BackgroundIoDebt, BackgroundIoPressureClass,
    BackgroundPacingCounterSnapshot, BackgroundPacingDenial, BackgroundResourceBudget,
};

#[derive(Debug, Eq, PartialEq)]
pub enum BackgroundPacingOutcome {
    Yield(BackgroundPacingYield),
    Deferred(BackgroundPacingDeferred),
    Denied(BackgroundPacingDenied),
    Throttled(Box<BackgroundPacingThrottle>),
    AdmittedWithDebt(Box<BackgroundPacingAdmittedWithDebt>),
    Violation(BackgroundPacingViolation),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackgroundPacingYield {
    class: BackgroundIoPressureClass,
    counters: BackgroundPacingCounterSnapshot,
}

impl BackgroundPacingOutcome {
    pub fn class(&self) -> BackgroundIoPressureClass {
        match self {
            Self::Yield(outcome) => outcome.class(),
            Self::Deferred(outcome) => outcome.class(),
            Self::Denied(outcome) => outcome.class(),
            Self::Throttled(outcome) => outcome.class(),
            Self::AdmittedWithDebt(outcome) => outcome.class(),
            Self::Violation(outcome) => outcome.class(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackgroundPacingDeferred {
    class: BackgroundIoPressureClass,
    counters: BackgroundPacingCounterSnapshot,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackgroundPacingDenied {
    class: BackgroundIoPressureClass,
    denial: BackgroundPacingDenial,
    counters: BackgroundPacingCounterSnapshot,
}

#[derive(Debug, Eq, PartialEq)]
pub struct BackgroundPacingThrottle {
    class: BackgroundIoPressureClass,
    admitted: BackgroundResourceBudget,
    throttled: BackgroundResourceBudget,
    counters: BackgroundPacingCounterSnapshot,
    lease: Option<BackgroundIdleCapacityLease>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct BackgroundPacingAdmittedWithDebt {
    lease: BackgroundIdleCapacityLease,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackgroundPacingViolation {
    debt: BackgroundIoDebt,
    counters: BackgroundPacingCounterSnapshot,
}

impl BackgroundPacingYield {
    pub(crate) const fn new(
        class: BackgroundIoPressureClass,
        counters: BackgroundPacingCounterSnapshot,
    ) -> Self {
        Self { class, counters }
    }

    pub const fn class(self) -> BackgroundIoPressureClass {
        self.class
    }
    pub const fn counters(self) -> BackgroundPacingCounterSnapshot {
        self.counters
    }
}

impl BackgroundPacingDeferred {
    pub(crate) const fn new(
        class: BackgroundIoPressureClass,
        counters: BackgroundPacingCounterSnapshot,
    ) -> Self {
        Self { class, counters }
    }

    pub const fn class(self) -> BackgroundIoPressureClass {
        self.class
    }
    pub const fn counters(self) -> BackgroundPacingCounterSnapshot {
        self.counters
    }
}

impl BackgroundPacingDenied {
    pub(crate) const fn new(
        class: BackgroundIoPressureClass,
        denial: BackgroundPacingDenial,
        counters: BackgroundPacingCounterSnapshot,
    ) -> Self {
        Self {
            class,
            denial,
            counters,
        }
    }

    pub const fn class(self) -> BackgroundIoPressureClass {
        self.class
    }
    pub const fn denial(self) -> BackgroundPacingDenial {
        self.denial
    }
    pub const fn counters(self) -> BackgroundPacingCounterSnapshot {
        self.counters
    }
}

impl BackgroundPacingThrottle {
    pub(crate) const fn new(
        class: BackgroundIoPressureClass,
        admitted: BackgroundResourceBudget,
        throttled: BackgroundResourceBudget,
        counters: BackgroundPacingCounterSnapshot,
        lease: Option<BackgroundIdleCapacityLease>,
    ) -> Self {
        Self {
            class,
            admitted,
            throttled,
            counters,
            lease,
        }
    }

    pub const fn class(&self) -> BackgroundIoPressureClass {
        self.class
    }
    pub const fn admitted_budget(&self) -> BackgroundResourceBudget {
        self.admitted
    }
    pub const fn throttled_budget(&self) -> BackgroundResourceBudget {
        self.throttled
    }
    pub const fn counters(&self) -> BackgroundPacingCounterSnapshot {
        self.counters
    }
    pub fn into_lease(self) -> Option<BackgroundIdleCapacityLease> {
        self.lease
    }
}

impl BackgroundPacingAdmittedWithDebt {
    pub(crate) const fn new(lease: BackgroundIdleCapacityLease) -> Self {
        Self { lease }
    }

    pub const fn into_lease(self) -> BackgroundIdleCapacityLease {
        self.lease
    }
    pub const fn class(&self) -> BackgroundIoPressureClass {
        self.lease.class()
    }
    pub const fn debt(&self) -> BackgroundIoDebt {
        self.lease.debt()
    }
    pub const fn admitted_budget(&self) -> BackgroundResourceBudget {
        self.lease.admitted_budget()
    }
    pub const fn counters(&self) -> BackgroundPacingCounterSnapshot {
        self.lease.counters()
    }
}

impl BackgroundPacingViolation {
    pub(crate) const fn new(
        debt: BackgroundIoDebt,
        counters: BackgroundPacingCounterSnapshot,
    ) -> Self {
        Self { debt, counters }
    }

    pub const fn causal_debt(self) -> BackgroundIoDebt {
        self.debt
    }
    pub const fn class(self) -> BackgroundIoPressureClass {
        self.debt.class()
    }
    pub const fn counters(self) -> BackgroundPacingCounterSnapshot {
        self.counters
    }
}
