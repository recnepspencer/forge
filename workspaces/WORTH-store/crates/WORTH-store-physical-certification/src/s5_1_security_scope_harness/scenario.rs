use super::schedule::S51SecurityScopeHarnessSchedule;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum S51SecurityScopeFailureKind {
    MetadataPreserved,
    PhysicalScopeDrift,
    StaleKeyPosture,
    WrongTenantScope,
    MissingAuthenticityRequirement,
    ReplayedCustodyPosture,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S51SecurityScopeHarnessScenario {
    failure_kind: S51SecurityScopeFailureKind,
    schedule: S51SecurityScopeHarnessSchedule,
}

impl S51SecurityScopeHarnessScenario {
    pub const fn metadata_preserved(schedule: S51SecurityScopeHarnessSchedule) -> Self {
        Self {
            failure_kind: S51SecurityScopeFailureKind::MetadataPreserved,
            schedule,
        }
    }

    pub const fn physical_scope_drift(schedule: S51SecurityScopeHarnessSchedule) -> Self {
        Self {
            failure_kind: S51SecurityScopeFailureKind::PhysicalScopeDrift,
            schedule,
        }
    }

    pub const fn stale_key_posture(schedule: S51SecurityScopeHarnessSchedule) -> Self {
        Self {
            failure_kind: S51SecurityScopeFailureKind::StaleKeyPosture,
            schedule,
        }
    }

    pub const fn wrong_tenant_scope(schedule: S51SecurityScopeHarnessSchedule) -> Self {
        Self {
            failure_kind: S51SecurityScopeFailureKind::WrongTenantScope,
            schedule,
        }
    }

    pub const fn missing_authenticity_requirement(
        schedule: S51SecurityScopeHarnessSchedule,
    ) -> Self {
        Self {
            failure_kind: S51SecurityScopeFailureKind::MissingAuthenticityRequirement,
            schedule,
        }
    }

    pub const fn replayed_custody_posture(schedule: S51SecurityScopeHarnessSchedule) -> Self {
        Self {
            failure_kind: S51SecurityScopeFailureKind::ReplayedCustodyPosture,
            schedule,
        }
    }

    pub const fn failure_kind(self) -> S51SecurityScopeFailureKind {
        self.failure_kind
    }

    pub const fn schedule(self) -> S51SecurityScopeHarnessSchedule {
        self.schedule
    }
}
