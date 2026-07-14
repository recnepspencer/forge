use worth_store_security::{
    StoreSecurityScopeAdmissionDenial, StoreSecurityScopeAdmissionFailure,
    StoreSecurityScopeAdmissionRebindRequired, StoreSecurityScopeAdmissionStale,
};

use super::{SecurityScopeFailureKind, SecurityScopeHarnessScenario};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityScopeHarnessOutcomeKind {
    Admitted,
    DeniedPhysicalScopeDrift,
    StaleKeyPosture,
    RebindRequired,
    DeniedWrongTenantScope,
    DeniedMissingAuthenticityRequirement,
    DeniedReplayedCustodyPosture,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SecurityScopeHarnessObservation {
    scenario: SecurityScopeHarnessScenario,
    outcome: SecurityScopeHarnessOutcomeKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SecurityScopeOracleVerdict {
    scenario: SecurityScopeHarnessScenario,
    outcome: SecurityScopeHarnessOutcomeKind,
    satisfied: bool,
    no_operator_authorization_claim: bool,
}

impl SecurityScopeHarnessObservation {
    pub const fn admitted(scenario: SecurityScopeHarnessScenario) -> Self {
        Self {
            scenario,
            outcome: SecurityScopeHarnessOutcomeKind::Admitted,
        }
    }

    pub const fn denied(
        scenario: SecurityScopeHarnessScenario,
        denial: StoreSecurityScopeAdmissionDenial,
    ) -> Self {
        Self {
            scenario,
            outcome: outcome_from_denial(denial),
        }
    }

    pub const fn stale(
        scenario: SecurityScopeHarnessScenario,
        _stale: StoreSecurityScopeAdmissionStale,
    ) -> Self {
        Self {
            scenario,
            outcome: SecurityScopeHarnessOutcomeKind::StaleKeyPosture,
        }
    }

    pub const fn rebind_required(
        scenario: SecurityScopeHarnessScenario,
        _rebind: StoreSecurityScopeAdmissionRebindRequired,
    ) -> Self {
        Self {
            scenario,
            outcome: SecurityScopeHarnessOutcomeKind::RebindRequired,
        }
    }

    pub const fn failed(
        scenario: SecurityScopeHarnessScenario,
        failure: StoreSecurityScopeAdmissionFailure,
    ) -> Self {
        Self {
            scenario,
            outcome: outcome_from_failure(failure),
        }
    }

    pub const fn scenario(self) -> SecurityScopeHarnessScenario {
        self.scenario
    }

    pub const fn outcome(self) -> SecurityScopeHarnessOutcomeKind {
        self.outcome
    }
}

impl SecurityScopeOracleVerdict {
    pub fn from_observation(observation: SecurityScopeHarnessObservation) -> Self {
        let scenario = observation.scenario();
        let outcome = observation.outcome();
        Self {
            scenario,
            outcome,
            satisfied: expected_outcome_for(scenario.failure_kind()) == outcome,
            no_operator_authorization_claim: true,
        }
    }

    pub const fn scenario(self) -> SecurityScopeHarnessScenario {
        self.scenario
    }

    pub const fn outcome(self) -> SecurityScopeHarnessOutcomeKind {
        self.outcome
    }

    pub const fn satisfied(self) -> bool {
        self.satisfied
    }

    pub const fn no_operator_authorization_claim(self) -> bool {
        self.no_operator_authorization_claim
    }
}

const fn outcome_from_denial(
    denial: StoreSecurityScopeAdmissionDenial,
) -> SecurityScopeHarnessOutcomeKind {
    match denial {
        StoreSecurityScopeAdmissionDenial::WrongPhysicalSecurityScope => {
            SecurityScopeHarnessOutcomeKind::DeniedPhysicalScopeDrift
        }
        StoreSecurityScopeAdmissionDenial::WrongTenantScope => {
            SecurityScopeHarnessOutcomeKind::DeniedWrongTenantScope
        }
        StoreSecurityScopeAdmissionDenial::MissingAuthenticityRequirement => {
            SecurityScopeHarnessOutcomeKind::DeniedMissingAuthenticityRequirement
        }
        StoreSecurityScopeAdmissionDenial::ReplayedAdmissionEvidence
        | StoreSecurityScopeAdmissionDenial::ExportedCustodyRequiresReadmission
        | StoreSecurityScopeAdmissionDenial::ImportedCustodyRequiresReadmission
        | StoreSecurityScopeAdmissionDenial::WrongCustodyPosture => {
            SecurityScopeHarnessOutcomeKind::DeniedReplayedCustodyPosture
        }
        _ => SecurityScopeHarnessOutcomeKind::Failed,
    }
}

const fn expected_outcome_for(
    failure: SecurityScopeFailureKind,
) -> SecurityScopeHarnessOutcomeKind {
    match failure {
        SecurityScopeFailureKind::MetadataPreserved => SecurityScopeHarnessOutcomeKind::Admitted,
        SecurityScopeFailureKind::PhysicalScopeDrift => {
            SecurityScopeHarnessOutcomeKind::DeniedPhysicalScopeDrift
        }
        SecurityScopeFailureKind::StaleKeyPosture => {
            SecurityScopeHarnessOutcomeKind::StaleKeyPosture
        }
        SecurityScopeFailureKind::WrongTenantScope => {
            SecurityScopeHarnessOutcomeKind::DeniedWrongTenantScope
        }
        SecurityScopeFailureKind::MissingAuthenticityRequirement => {
            SecurityScopeHarnessOutcomeKind::DeniedMissingAuthenticityRequirement
        }
        SecurityScopeFailureKind::ReplayedCustodyPosture => {
            SecurityScopeHarnessOutcomeKind::DeniedReplayedCustodyPosture
        }
    }
}

const fn outcome_from_failure(
    failure: StoreSecurityScopeAdmissionFailure,
) -> SecurityScopeHarnessOutcomeKind {
    match failure {
        StoreSecurityScopeAdmissionFailure::PhysicalAuthorityDrift => {
            SecurityScopeHarnessOutcomeKind::DeniedPhysicalScopeDrift
        }
    }
}
