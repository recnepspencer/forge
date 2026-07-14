use super::schedule::SecurityScopeHarnessSchedule;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SecurityScopeFailureKind {
    MetadataPreserved,
    PhysicalScopeDrift,
    StaleKeyPosture,
    WrongTenantScope,
    MissingAuthenticityRequirement,
    ReplayedCustodyPosture,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SecurityScopeHarnessScenario {
    failure_kind: SecurityScopeFailureKind,
    schedule: SecurityScopeHarnessSchedule,
}

impl SecurityScopeHarnessScenario {
    pub const fn metadata_preserved(schedule: SecurityScopeHarnessSchedule) -> Self {
        Self {
            failure_kind: SecurityScopeFailureKind::MetadataPreserved,
            schedule,
        }
    }

    pub const fn physical_scope_drift(schedule: SecurityScopeHarnessSchedule) -> Self {
        Self {
            failure_kind: SecurityScopeFailureKind::PhysicalScopeDrift,
            schedule,
        }
    }

    pub const fn stale_key_posture(schedule: SecurityScopeHarnessSchedule) -> Self {
        Self {
            failure_kind: SecurityScopeFailureKind::StaleKeyPosture,
            schedule,
        }
    }

    pub const fn wrong_tenant_scope(schedule: SecurityScopeHarnessSchedule) -> Self {
        Self {
            failure_kind: SecurityScopeFailureKind::WrongTenantScope,
            schedule,
        }
    }

    pub const fn missing_authenticity_requirement(schedule: SecurityScopeHarnessSchedule) -> Self {
        Self {
            failure_kind: SecurityScopeFailureKind::MissingAuthenticityRequirement,
            schedule,
        }
    }

    pub const fn replayed_custody_posture(schedule: SecurityScopeHarnessSchedule) -> Self {
        Self {
            failure_kind: SecurityScopeFailureKind::ReplayedCustodyPosture,
            schedule,
        }
    }

    pub const fn failure_kind(self) -> SecurityScopeFailureKind {
        self.failure_kind
    }

    pub const fn schedule(self) -> SecurityScopeHarnessSchedule {
        self.schedule
    }
}
