#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ReclaimPolicyCounterSnapshot {
    admission_requests: u64,
    admitted: u64,
    denied: u64,
    executed: u64,
    violations: u64,
    protected_reachability_checks: u64,
    security_scope_checks: u64,
    byte_interpretation_observations: u64,
    non_claim_handoffs: u64,
}

impl ReclaimPolicyCounterSnapshot {
    pub const fn start_request() -> Self {
        Self {
            admission_requests: 1,
            admitted: 0,
            denied: 0,
            executed: 0,
            violations: 0,
            protected_reachability_checks: 0,
            security_scope_checks: 0,
            byte_interpretation_observations: 0,
            non_claim_handoffs: 0,
        }
    }

    pub const fn with_admitted(mut self) -> Self {
        self.admitted += 1;
        self
    }

    pub const fn with_denied(mut self) -> Self {
        self.denied += 1;
        self
    }

    pub const fn with_executed(mut self) -> Self {
        self.executed += 1;
        self
    }

    pub const fn with_violation(mut self) -> Self {
        self.violations += 1;
        self
    }

    pub const fn with_protected_reachability_check(mut self) -> Self {
        self.protected_reachability_checks += 1;
        self
    }

    pub const fn with_security_scope_check(mut self) -> Self {
        self.security_scope_checks += 1;
        self
    }

    pub const fn with_byte_interpretation_observation(mut self) -> Self {
        self.byte_interpretation_observations += 1;
        self
    }

    pub const fn with_non_claim_handoff(mut self) -> Self {
        self.non_claim_handoffs += 1;
        self
    }

    pub const fn admission_requests(self) -> u64 {
        self.admission_requests
    }
    pub const fn admitted(self) -> u64 {
        self.admitted
    }
    pub const fn denied(self) -> u64 {
        self.denied
    }
    pub const fn executed(self) -> u64 {
        self.executed
    }
    pub const fn violations(self) -> u64 {
        self.violations
    }
    pub const fn protected_reachability_checks(self) -> u64 {
        self.protected_reachability_checks
    }
    pub const fn security_scope_checks(self) -> u64 {
        self.security_scope_checks
    }
    pub const fn byte_interpretation_observations(self) -> u64 {
        self.byte_interpretation_observations
    }
    pub const fn non_claim_handoffs(self) -> u64 {
        self.non_claim_handoffs
    }
}
