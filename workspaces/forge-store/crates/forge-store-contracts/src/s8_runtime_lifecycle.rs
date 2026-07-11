//! Shared S.8 runtime-simulation lifecycle vocabulary.
//!
//! This vocabulary names lifecycle facts; only a family execution owner may
//! attach one to its sealed receipt after the corresponding facade transition.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum S8RuntimeCase {
    Success,
    UnsupportedShapeDenial,
    StaleRebind,
    CorruptDerived,
    CorruptAuthority,
    Rebuild,
    MigrationRollback,
    HiddenScanDenial,
    Readmission,
    CostEnvelope,
}

impl S8RuntimeCase {
    pub const fn all() -> [Self; 10] {
        [
            Self::Success,
            Self::UnsupportedShapeDenial,
            Self::StaleRebind,
            Self::CorruptDerived,
            Self::CorruptAuthority,
            Self::Rebuild,
            Self::MigrationRollback,
            Self::HiddenScanDenial,
            Self::Readmission,
            Self::CostEnvelope,
        ]
    }

    pub const fn expected_outcome(self) -> S8RuntimeOutcome {
        match self {
            Self::Success => S8RuntimeOutcome::Succeeded,
            Self::UnsupportedShapeDenial => S8RuntimeOutcome::UnsupportedShapeDenied,
            Self::StaleRebind => S8RuntimeOutcome::Rebound,
            Self::CorruptDerived => S8RuntimeOutcome::DerivedCorruptionQuarantined,
            Self::CorruptAuthority => S8RuntimeOutcome::AuthorityCorruptionQuarantined,
            Self::Rebuild => S8RuntimeOutcome::Rebuilt,
            Self::MigrationRollback => S8RuntimeOutcome::MigrationRolledBack,
            Self::HiddenScanDenial => S8RuntimeOutcome::HiddenScanDenied,
            Self::Readmission => S8RuntimeOutcome::Readmitted,
            Self::CostEnvelope => S8RuntimeOutcome::CostEnvelopeHeld,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8RuntimeOutcome {
    Succeeded,
    UnsupportedShapeDenied,
    Rebound,
    DerivedCorruptionQuarantined,
    AuthorityCorruptionQuarantined,
    Rebuilt,
    MigrationRolledBack,
    HiddenScanDenied,
    Readmitted,
    CostEnvelopeHeld,
}

impl S8RuntimeOutcome {
    pub const fn satisfies(self, case: S8RuntimeCase) -> bool {
        matches!(
            (case, self),
            (S8RuntimeCase::Success, Self::Succeeded)
                | (
                    S8RuntimeCase::UnsupportedShapeDenial,
                    Self::UnsupportedShapeDenied
                )
                | (S8RuntimeCase::StaleRebind, Self::Rebound)
                | (
                    S8RuntimeCase::CorruptDerived,
                    Self::DerivedCorruptionQuarantined
                )
                | (
                    S8RuntimeCase::CorruptAuthority,
                    Self::AuthorityCorruptionQuarantined
                )
                | (S8RuntimeCase::Rebuild, Self::Rebuilt)
                | (S8RuntimeCase::MigrationRollback, Self::MigrationRolledBack)
                | (S8RuntimeCase::HiddenScanDenial, Self::HiddenScanDenied)
                | (S8RuntimeCase::Readmission, Self::Readmitted)
                | (S8RuntimeCase::CostEnvelope, Self::CostEnvelopeHeld)
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct S8RuntimeExecutionIdentity(u64);

impl S8RuntimeExecutionIdentity {
    pub const fn from_owner_seed(seed: u64) -> Self {
        Self(seed)
    }

    pub const fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8RuntimeScanPosture {
    OwnerBounded,
    FullStoreDenied,
    RebuildBounded,
    ReadmissionBounded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8RuntimeExactCounterEvidence {
    planned_units: u64,
    observed_units: u64,
}

impl S8RuntimeExactCounterEvidence {
    pub const fn new(planned_units: u64, observed_units: u64) -> Self {
        Self {
            planned_units,
            observed_units,
        }
    }

    pub const fn planned_units(self) -> u64 {
        self.planned_units
    }

    pub const fn observed_units(self) -> u64 {
        self.observed_units
    }

    pub const fn matches_plan(self) -> bool {
        self.planned_units == self.observed_units
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8RuntimeOwnerFact {
    execution_identity: S8RuntimeExecutionIdentity,
    case: S8RuntimeCase,
    outcome: S8RuntimeOutcome,
    scan_posture: S8RuntimeScanPosture,
    counters: S8RuntimeExactCounterEvidence,
}

impl S8RuntimeOwnerFact {
    pub const fn new(
        execution_identity: S8RuntimeExecutionIdentity,
        case: S8RuntimeCase,
        scan_posture: S8RuntimeScanPosture,
        counters: S8RuntimeExactCounterEvidence,
    ) -> Self {
        Self {
            execution_identity,
            case,
            outcome: case.expected_outcome(),
            scan_posture,
            counters,
        }
    }

    pub const fn execution_identity(self) -> S8RuntimeExecutionIdentity {
        self.execution_identity
    }

    pub const fn case(self) -> S8RuntimeCase {
        self.case
    }

    pub const fn outcome(self) -> S8RuntimeOutcome {
        self.outcome
    }

    pub const fn scan_posture(self) -> S8RuntimeScanPosture {
        self.scan_posture
    }

    pub const fn counters(self) -> S8RuntimeExactCounterEvidence {
        self.counters
    }

    pub const fn is_coherent(self) -> bool {
        self.outcome.satisfies(self.case) && self.counters.matches_plan()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owner_fact_binds_lifecycle_case_to_expected_outcome() {
        let fact = S8RuntimeOwnerFact::new(
            S8RuntimeExecutionIdentity::from_owner_seed(17),
            S8RuntimeCase::HiddenScanDenial,
            S8RuntimeScanPosture::FullStoreDenied,
            S8RuntimeExactCounterEvidence::new(4, 4),
        );

        assert_eq!(fact.outcome(), S8RuntimeOutcome::HiddenScanDenied);
        assert!(fact.is_coherent());
    }

    #[test]
    fn owner_fact_rejects_counter_projection_without_exact_match() {
        let fact = S8RuntimeOwnerFact::new(
            S8RuntimeExecutionIdentity::from_owner_seed(18),
            S8RuntimeCase::CostEnvelope,
            S8RuntimeScanPosture::OwnerBounded,
            S8RuntimeExactCounterEvidence::new(8, 7),
        );

        assert!(!fact.is_coherent());
    }
}
