use super::{BackgroundDebtKind, BackgroundResourceBudget};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackgroundPacingCounterSnapshot {
    requested: BackgroundResourceBudget,
    idle_available: BackgroundResourceBudget,
    admitted: BackgroundResourceBudget,
    throttled: BackgroundResourceBudget,
    denied: BackgroundResourceBudget,
    revoked: BackgroundResourceBudget,
    debt: BackgroundResourceBudget,
    compaction_debt: BackgroundResourceBudget,
    checkpoint_flush_debt: BackgroundResourceBudget,
    scrub_pressure: BackgroundResourceBudget,
    replication_prep_pressure: BackgroundResourceBudget,
    blob_contention: BackgroundResourceBudget,
    backup_pressure: BackgroundResourceBudget,
    repair_pressure: BackgroundResourceBudget,
    yield_events: u64,
    deferred_events: u64,
    denied_events: u64,
    revoke_events: u64,
    throttle_events: u64,
    admitted_with_debt_events: u64,
    violation_events: u64,
    foreground_pressure_events: u64,
}

impl BackgroundPacingCounterSnapshot {
    pub(crate) const fn yield_now(
        requested: BackgroundResourceBudget,
        idle_available: BackgroundResourceBudget,
        foreground_pressure_events: u64,
    ) -> Self {
        Self::new(requested, idle_available, BackgroundResourceBudget::new())
            .with_yield(1, foreground_pressure_events)
    }

    pub(crate) const fn deferred(
        requested: BackgroundResourceBudget,
        idle_available: BackgroundResourceBudget,
    ) -> Self {
        Self::new(requested, idle_available, BackgroundResourceBudget::new()).with_deferred(1)
    }

    pub(crate) const fn denied(
        requested: BackgroundResourceBudget,
        idle_available: BackgroundResourceBudget,
        denied: BackgroundResourceBudget,
    ) -> Self {
        Self::new(requested, idle_available, BackgroundResourceBudget::new()).with_denied(denied, 1)
    }

    pub(crate) const fn throttled(
        requested: BackgroundResourceBudget,
        idle_available: BackgroundResourceBudget,
        admitted: BackgroundResourceBudget,
        throttled: BackgroundResourceBudget,
    ) -> Self {
        Self::new(requested, idle_available, admitted).with_throttled(throttled, 1)
    }

    pub(crate) const fn admitted_with_debt(
        requested: BackgroundResourceBudget,
        idle_available: BackgroundResourceBudget,
        admitted: BackgroundResourceBudget,
        debt: BackgroundResourceBudget,
        kind: BackgroundDebtKind,
    ) -> Self {
        Self::new(requested, idle_available, admitted).with_debt(debt, kind, 1)
    }

    pub(crate) const fn violation(
        requested: BackgroundResourceBudget,
        idle_available: BackgroundResourceBudget,
        admitted: BackgroundResourceBudget,
        debt: BackgroundResourceBudget,
        kind: BackgroundDebtKind,
        foreground_pressure_events: u64,
    ) -> Self {
        Self::new(requested, idle_available, admitted)
            .with_debt(debt, kind, 0)
            .with_violation(1, foreground_pressure_events)
    }

    pub(crate) const fn revoked(
        prior: Self,
        revoked: BackgroundResourceBudget,
        foreground_pressure_events: u64,
    ) -> Self {
        prior.with_revoked(revoked, 1, foreground_pressure_events)
    }

    pub const fn requested(self) -> BackgroundResourceBudget {
        self.requested
    }
    pub const fn idle_available(self) -> BackgroundResourceBudget {
        self.idle_available
    }
    pub const fn admitted_budget(self) -> BackgroundResourceBudget {
        self.admitted
    }
    pub const fn throttled_budget(self) -> BackgroundResourceBudget {
        self.throttled
    }
    pub const fn denied_budget(self) -> BackgroundResourceBudget {
        self.denied
    }
    pub const fn revoked_budget(self) -> BackgroundResourceBudget {
        self.revoked
    }
    pub const fn debt_budget(self) -> BackgroundResourceBudget {
        self.debt
    }
    pub const fn compaction_debt(self) -> BackgroundResourceBudget {
        self.compaction_debt
    }
    pub const fn checkpoint_flush_debt(self) -> BackgroundResourceBudget {
        self.checkpoint_flush_debt
    }
    pub const fn scrub_pressure(self) -> BackgroundResourceBudget {
        self.scrub_pressure
    }
    pub const fn replication_prep_pressure(self) -> BackgroundResourceBudget {
        self.replication_prep_pressure
    }
    pub const fn blob_contention(self) -> BackgroundResourceBudget {
        self.blob_contention
    }
    pub const fn backup_pressure(self) -> BackgroundResourceBudget {
        self.backup_pressure
    }
    pub const fn repair_pressure(self) -> BackgroundResourceBudget {
        self.repair_pressure
    }
    pub const fn yield_events(self) -> u64 {
        self.yield_events
    }
    pub const fn deferred_events(self) -> u64 {
        self.deferred_events
    }
    pub const fn denied_events(self) -> u64 {
        self.denied_events
    }
    pub const fn revoke_events(self) -> u64 {
        self.revoke_events
    }
    pub const fn throttle_events(self) -> u64 {
        self.throttle_events
    }
    pub const fn admitted_with_debt_events(self) -> u64 {
        self.admitted_with_debt_events
    }
    pub const fn violation_events(self) -> u64 {
        self.violation_events
    }
    pub const fn foreground_pressure_events(self) -> u64 {
        self.foreground_pressure_events
    }

    const fn new(
        requested: BackgroundResourceBudget,
        idle_available: BackgroundResourceBudget,
        admitted: BackgroundResourceBudget,
    ) -> Self {
        Self {
            requested,
            idle_available,
            admitted,
            throttled: BackgroundResourceBudget::new(),
            denied: BackgroundResourceBudget::new(),
            revoked: BackgroundResourceBudget::new(),
            debt: BackgroundResourceBudget::new(),
            compaction_debt: BackgroundResourceBudget::new(),
            checkpoint_flush_debt: BackgroundResourceBudget::new(),
            scrub_pressure: BackgroundResourceBudget::new(),
            replication_prep_pressure: BackgroundResourceBudget::new(),
            blob_contention: BackgroundResourceBudget::new(),
            backup_pressure: BackgroundResourceBudget::new(),
            repair_pressure: BackgroundResourceBudget::new(),
            yield_events: 0,
            deferred_events: 0,
            denied_events: 0,
            revoke_events: 0,
            throttle_events: 0,
            admitted_with_debt_events: 0,
            violation_events: 0,
            foreground_pressure_events: 0,
        }
    }

    const fn with_yield(mut self, events: u64, foreground_pressure_events: u64) -> Self {
        self.yield_events = events;
        self.foreground_pressure_events = foreground_pressure_events;
        self
    }

    const fn with_deferred(mut self, events: u64) -> Self {
        self.deferred_events = events;
        self
    }

    const fn with_denied(mut self, denied: BackgroundResourceBudget, events: u64) -> Self {
        self.denied = denied;
        self.denied_events = events;
        self
    }

    const fn with_revoked(
        mut self,
        revoked: BackgroundResourceBudget,
        events: u64,
        foreground_pressure_events: u64,
    ) -> Self {
        self.revoked = revoked;
        self.revoke_events = events;
        self.foreground_pressure_events = foreground_pressure_events;
        self
    }

    const fn with_throttled(mut self, throttled: BackgroundResourceBudget, events: u64) -> Self {
        self.throttled = throttled;
        self.throttle_events = events;
        self
    }

    const fn with_debt(
        mut self,
        debt: BackgroundResourceBudget,
        kind: BackgroundDebtKind,
        events: u64,
    ) -> Self {
        self.debt = debt;
        match kind {
            BackgroundDebtKind::CompactionDebt => self.compaction_debt = debt,
            BackgroundDebtKind::CheckpointFlushDebt => self.checkpoint_flush_debt = debt,
            BackgroundDebtKind::ScrubPressure => self.scrub_pressure = debt,
            BackgroundDebtKind::ReplicationPrepPressure => self.replication_prep_pressure = debt,
            BackgroundDebtKind::BlobContention => self.blob_contention = debt,
            BackgroundDebtKind::BackupPressure => self.backup_pressure = debt,
            BackgroundDebtKind::RepairPressure => self.repair_pressure = debt,
        }
        self.admitted_with_debt_events = events;
        self
    }

    const fn with_violation(mut self, events: u64, foreground_pressure_events: u64) -> Self {
        self.violation_events = events;
        self.foreground_pressure_events = foreground_pressure_events;
        self
    }
}
