use super::{
    BackgroundIdleCapacityLease, BackgroundIoDebt, BackgroundPacingCounterSnapshot,
    BackgroundPacingDenial, BackgroundResourceBudget,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BackgroundPacingOutcome {
    Yield(BackgroundPacingYield),
    Deferred(BackgroundPacingDeferred),
    Denied(BackgroundPacingDenied),
    StaleRebindRequired(BackgroundPacingStaleRebindRequired),
    Throttled(BackgroundPacingThrottle),
    AdmittedWithDebt(BackgroundPacingAdmittedWithDebt),
    Violation(BackgroundPacingViolation),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackgroundPacingYield {
    counters: BackgroundPacingCounterSnapshot,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackgroundPacingDeferred {
    counters: BackgroundPacingCounterSnapshot,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackgroundPacingDenied {
    denial: BackgroundPacingDenial,
    counters: BackgroundPacingCounterSnapshot,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BackgroundPacingStaleRebindKind {
    Stale,
    RebindRequired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackgroundPacingStaleRebindRequired {
    kind: BackgroundPacingStaleRebindKind,
    counters: BackgroundPacingCounterSnapshot,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackgroundPacingThrottle {
    admitted: BackgroundResourceBudget,
    throttled: BackgroundResourceBudget,
    counters: BackgroundPacingCounterSnapshot,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackgroundPacingAdmittedWithDebt {
    lease: BackgroundIdleCapacityLease,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackgroundPacingViolation {
    debt: BackgroundIoDebt,
    counters: BackgroundPacingCounterSnapshot,
}

impl BackgroundPacingYield {
    pub(crate) const fn new(counters: BackgroundPacingCounterSnapshot) -> Self {
        Self { counters }
    }

    pub const fn counters(self) -> BackgroundPacingCounterSnapshot {
        self.counters
    }
}

impl BackgroundPacingDeferred {
    pub(crate) const fn new(counters: BackgroundPacingCounterSnapshot) -> Self {
        Self { counters }
    }

    pub const fn counters(self) -> BackgroundPacingCounterSnapshot {
        self.counters
    }
}

impl BackgroundPacingDenied {
    pub(crate) const fn new(
        denial: BackgroundPacingDenial,
        counters: BackgroundPacingCounterSnapshot,
    ) -> Self {
        Self { denial, counters }
    }

    pub const fn denial(self) -> BackgroundPacingDenial {
        self.denial
    }
    pub const fn counters(self) -> BackgroundPacingCounterSnapshot {
        self.counters
    }
}

impl BackgroundPacingStaleRebindRequired {
    pub(crate) const fn new(
        kind: BackgroundPacingStaleRebindKind,
        counters: BackgroundPacingCounterSnapshot,
    ) -> Self {
        Self { kind, counters }
    }

    pub const fn kind(self) -> BackgroundPacingStaleRebindKind {
        self.kind
    }
    pub const fn counters(self) -> BackgroundPacingCounterSnapshot {
        self.counters
    }
}

impl BackgroundPacingThrottle {
    pub(crate) const fn new(
        admitted: BackgroundResourceBudget,
        throttled: BackgroundResourceBudget,
        counters: BackgroundPacingCounterSnapshot,
    ) -> Self {
        Self {
            admitted,
            throttled,
            counters,
        }
    }

    pub const fn admitted_budget(self) -> BackgroundResourceBudget {
        self.admitted
    }
    pub const fn throttled_budget(self) -> BackgroundResourceBudget {
        self.throttled
    }
    pub const fn counters(self) -> BackgroundPacingCounterSnapshot {
        self.counters
    }
}

impl BackgroundPacingAdmittedWithDebt {
    pub(crate) const fn new(lease: BackgroundIdleCapacityLease) -> Self {
        Self { lease }
    }

    pub const fn lease(self) -> BackgroundIdleCapacityLease {
        self.lease
    }
    pub const fn debt(self) -> BackgroundIoDebt {
        self.lease.debt()
    }
    pub const fn counters(self) -> BackgroundPacingCounterSnapshot {
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
    pub const fn counters(self) -> BackgroundPacingCounterSnapshot {
        self.counters
    }
}
