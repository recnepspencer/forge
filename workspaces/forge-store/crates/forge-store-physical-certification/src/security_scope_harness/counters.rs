use super::{SecurityScopeHarnessOutcomeKind, SecurityScopeHarnessScenario};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SecurityScopeHarnessCounterSnapshot {
    scenarios_executed: u64,
    scope_admission_attempts: u64,
    readiness_acceptances: u64,
    denied_before_logical_decode: u64,
    stale_key_posture: u64,
    rebind_required: u64,
    physical_scope_drift: u64,
    wrong_tenant_scope: u64,
    missing_authenticity_requirement: u64,
    replayed_custody_posture: u64,
}

impl SecurityScopeHarnessCounterSnapshot {
    pub const fn start_scenario(_scenario: SecurityScopeHarnessScenario) -> Self {
        Self {
            scenarios_executed: 1,
            scope_admission_attempts: 1,
            readiness_acceptances: 0,
            denied_before_logical_decode: 0,
            stale_key_posture: 0,
            rebind_required: 0,
            physical_scope_drift: 0,
            wrong_tenant_scope: 0,
            missing_authenticity_requirement: 0,
            replayed_custody_posture: 0,
        }
    }

    pub const fn record_outcome(mut self, outcome: SecurityScopeHarnessOutcomeKind) -> Self {
        match outcome {
            SecurityScopeHarnessOutcomeKind::Admitted => {
                self.readiness_acceptances += 1;
            }
            SecurityScopeHarnessOutcomeKind::DeniedPhysicalScopeDrift => {
                self.denied_before_logical_decode += 1;
                self.physical_scope_drift += 1;
            }
            SecurityScopeHarnessOutcomeKind::StaleKeyPosture => {
                self.denied_before_logical_decode += 1;
                self.stale_key_posture += 1;
            }
            SecurityScopeHarnessOutcomeKind::RebindRequired => {
                self.denied_before_logical_decode += 1;
                self.rebind_required += 1;
            }
            SecurityScopeHarnessOutcomeKind::DeniedWrongTenantScope => {
                self.denied_before_logical_decode += 1;
                self.wrong_tenant_scope += 1;
            }
            SecurityScopeHarnessOutcomeKind::DeniedMissingAuthenticityRequirement => {
                self.denied_before_logical_decode += 1;
                self.missing_authenticity_requirement += 1;
            }
            SecurityScopeHarnessOutcomeKind::DeniedReplayedCustodyPosture => {
                self.denied_before_logical_decode += 1;
                self.replayed_custody_posture += 1;
            }
            SecurityScopeHarnessOutcomeKind::Failed => {}
        }
        self
    }

    pub const fn scenarios_executed(self) -> u64 {
        self.scenarios_executed
    }

    pub const fn scope_admission_attempts(self) -> u64 {
        self.scope_admission_attempts
    }

    pub const fn readiness_acceptances(self) -> u64 {
        self.readiness_acceptances
    }

    pub const fn denied_before_logical_decode(self) -> u64 {
        self.denied_before_logical_decode
    }

    pub const fn stale_key_posture(self) -> u64 {
        self.stale_key_posture
    }

    pub const fn rebind_required(self) -> u64 {
        self.rebind_required
    }

    pub const fn physical_scope_drift(self) -> u64 {
        self.physical_scope_drift
    }

    pub const fn wrong_tenant_scope(self) -> u64 {
        self.wrong_tenant_scope
    }

    pub const fn missing_authenticity_requirement(self) -> u64 {
        self.missing_authenticity_requirement
    }

    pub const fn replayed_custody_posture(self) -> u64 {
        self.replayed_custody_posture
    }
}
