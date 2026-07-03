use forge_store_security::{
    StoreSecurityScopeAdmissionDenial, StoreSecurityScopeAdmissionFailure,
    StoreSecurityScopeAdmissionRebindRequired, StoreSecurityScopeAdmissionStale,
};

use super::{S51SecurityScopeFailureKind, S51SecurityScopeHarnessScenario};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S51SecurityScopeHarnessOutcomeKind {
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
pub struct S51SecurityScopeHarnessObservation {
    scenario: S51SecurityScopeHarnessScenario,
    outcome: S51SecurityScopeHarnessOutcomeKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S51SecurityScopeOracleVerdict {
    scenario: S51SecurityScopeHarnessScenario,
    outcome: S51SecurityScopeHarnessOutcomeKind,
    satisfied: bool,
    no_s11_claim: bool,
}

impl S51SecurityScopeHarnessObservation {
    pub const fn admitted(scenario: S51SecurityScopeHarnessScenario) -> Self {
        Self {
            scenario,
            outcome: S51SecurityScopeHarnessOutcomeKind::Admitted,
        }
    }

    pub const fn denied(
        scenario: S51SecurityScopeHarnessScenario,
        denial: StoreSecurityScopeAdmissionDenial,
    ) -> Self {
        Self {
            scenario,
            outcome: outcome_from_denial(denial),
        }
    }

    pub const fn stale(
        scenario: S51SecurityScopeHarnessScenario,
        _stale: StoreSecurityScopeAdmissionStale,
    ) -> Self {
        Self {
            scenario,
            outcome: S51SecurityScopeHarnessOutcomeKind::StaleKeyPosture,
        }
    }

    pub const fn rebind_required(
        scenario: S51SecurityScopeHarnessScenario,
        _rebind: StoreSecurityScopeAdmissionRebindRequired,
    ) -> Self {
        Self {
            scenario,
            outcome: S51SecurityScopeHarnessOutcomeKind::RebindRequired,
        }
    }

    pub const fn failed(
        scenario: S51SecurityScopeHarnessScenario,
        failure: StoreSecurityScopeAdmissionFailure,
    ) -> Self {
        Self {
            scenario,
            outcome: outcome_from_failure(failure),
        }
    }

    pub const fn scenario(self) -> S51SecurityScopeHarnessScenario {
        self.scenario
    }

    pub const fn outcome(self) -> S51SecurityScopeHarnessOutcomeKind {
        self.outcome
    }
}

impl S51SecurityScopeOracleVerdict {
    pub fn from_observation(observation: S51SecurityScopeHarnessObservation) -> Self {
        let scenario = observation.scenario();
        let outcome = observation.outcome();
        Self {
            scenario,
            outcome,
            satisfied: expected_outcome_for(scenario.failure_kind()) == outcome,
            no_s11_claim: true,
        }
    }

    pub const fn scenario(self) -> S51SecurityScopeHarnessScenario {
        self.scenario
    }

    pub const fn outcome(self) -> S51SecurityScopeHarnessOutcomeKind {
        self.outcome
    }

    pub const fn satisfied(self) -> bool {
        self.satisfied
    }

    pub const fn no_s11_claim(self) -> bool {
        self.no_s11_claim
    }
}

const fn outcome_from_denial(
    denial: StoreSecurityScopeAdmissionDenial,
) -> S51SecurityScopeHarnessOutcomeKind {
    match denial {
        StoreSecurityScopeAdmissionDenial::WrongPhysicalSecurityScope => {
            S51SecurityScopeHarnessOutcomeKind::DeniedPhysicalScopeDrift
        }
        StoreSecurityScopeAdmissionDenial::WrongTenantScope => {
            S51SecurityScopeHarnessOutcomeKind::DeniedWrongTenantScope
        }
        StoreSecurityScopeAdmissionDenial::MissingAuthenticityRequirement => {
            S51SecurityScopeHarnessOutcomeKind::DeniedMissingAuthenticityRequirement
        }
        StoreSecurityScopeAdmissionDenial::ReplayedAdmissionEvidence
        | StoreSecurityScopeAdmissionDenial::ExportedCustodyRequiresReadmission
        | StoreSecurityScopeAdmissionDenial::ImportedCustodyRequiresReadmission
        | StoreSecurityScopeAdmissionDenial::WrongCustodyPosture => {
            S51SecurityScopeHarnessOutcomeKind::DeniedReplayedCustodyPosture
        }
        _ => S51SecurityScopeHarnessOutcomeKind::Failed,
    }
}

const fn expected_outcome_for(
    failure: S51SecurityScopeFailureKind,
) -> S51SecurityScopeHarnessOutcomeKind {
    match failure {
        S51SecurityScopeFailureKind::MetadataPreserved => {
            S51SecurityScopeHarnessOutcomeKind::Admitted
        }
        S51SecurityScopeFailureKind::PhysicalScopeDrift => {
            S51SecurityScopeHarnessOutcomeKind::DeniedPhysicalScopeDrift
        }
        S51SecurityScopeFailureKind::StaleKeyPosture => {
            S51SecurityScopeHarnessOutcomeKind::StaleKeyPosture
        }
        S51SecurityScopeFailureKind::WrongTenantScope => {
            S51SecurityScopeHarnessOutcomeKind::DeniedWrongTenantScope
        }
        S51SecurityScopeFailureKind::MissingAuthenticityRequirement => {
            S51SecurityScopeHarnessOutcomeKind::DeniedMissingAuthenticityRequirement
        }
        S51SecurityScopeFailureKind::ReplayedCustodyPosture => {
            S51SecurityScopeHarnessOutcomeKind::DeniedReplayedCustodyPosture
        }
    }
}

const fn outcome_from_failure(
    failure: StoreSecurityScopeAdmissionFailure,
) -> S51SecurityScopeHarnessOutcomeKind {
    match failure {
        StoreSecurityScopeAdmissionFailure::PhysicalAuthorityDrift => {
            S51SecurityScopeHarnessOutcomeKind::DeniedPhysicalScopeDrift
        }
    }
}
