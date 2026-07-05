#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SecureIoCounterStrength {
    Exact,
    Derived,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SecureIoPreservationCounterSnapshot {
    scope_checks: u64,
    backend_posture_checks: u64,
    denied_checks: u64,
    strength: SecureIoCounterStrength,
}

impl SecureIoPreservationCounterSnapshot {
    pub const fn start() -> Self {
        Self {
            scope_checks: 0,
            backend_posture_checks: 0,
            denied_checks: 0,
            strength: SecureIoCounterStrength::Exact,
        }
    }

    pub const fn checked_scope(mut self) -> Self {
        self.scope_checks += 1;
        self
    }

    pub const fn checked_backend_posture(mut self) -> Self {
        self.backend_posture_checks += 1;
        self
    }

    pub const fn denied(mut self) -> Self {
        self.denied_checks += 1;
        self
    }

    pub const fn derived(mut self) -> Self {
        self.strength = SecureIoCounterStrength::Derived;
        self
    }

    pub const fn scope_checks(self) -> u64 {
        self.scope_checks
    }

    pub const fn backend_posture_checks(self) -> u64 {
        self.backend_posture_checks
    }

    pub const fn denied_checks(self) -> u64 {
        self.denied_checks
    }

    pub const fn strength(self) -> SecureIoCounterStrength {
        self.strength
    }
}
